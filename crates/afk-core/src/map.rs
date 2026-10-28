//! La carte à tuiles : types de terrain, collisions et recherche de chemin.
//! Aucune notion de couleur ou de sprite ici : le rendu décide comment afficher chaque `Tile`.

use crate::math::{TilePos, Vec2};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

/// Comment un personnage choisit son trajet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathMode {
    /// Il suit les routes : la route coûte 1, l'herbe (et le reste) ce surcoût.
    Roads(f32),
    /// Plus court chemin en distance, sans regarder les routes (voleur en fuite, gardes qui le poursuivent).
    Direct,
}

/// Surcoût de l'herbe par rapport à la route, par défaut (voir `Config::grass_cost`).
pub const DEFAULT_GRASS_COST: f32 = 3.0;

const STRAIGHT_COST: u32 = 1000;
const DIAG_COST: u32 = 1414;
/// Ordre fixe des voisins : il départage les chemins de même coût de façon déterministe.
const NEIGHBORS: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Grass,
    Path,
    Water,
    Field,
    Wall,
    Floor,
    Tree,
    /// Un lit : praticable (on s'y couche), posé sur un sol intérieur.
    Bed,
    /// Une enclume : c'est un obstacle, on forge depuis une case voisine en la regardant.
    Anvil,
    /// Une tuile de filon de minerai : solide, on mine depuis une case voisine en la regardant.
    Ore,
    /// Un coffre (contenant de ressources), posé à l'extérieur des bâtiments : solide, on y accède
    /// depuis une case voisine.
    Crate,
    /// Un meuble d'habitat solide (table, étagère ou bac d'eau) : le dessin retrouve le meuble et son
    /// orientation dans la description de l'habitat.
    Furniture,
    /// Une pousse d'arbre : praticable, redevient un arbre après un délai.
    Sapling,
}

impl Tile {
    pub fn is_solid(self) -> bool {
        matches!(self, Tile::Water | Tile::Wall | Tile::Tree | Tile::Anvil | Tile::Ore | Tile::Crate | Tile::Furniture)
    }
}

/// Part nord de la case d'une enclume (en tuiles) où l'on peut se tenir : l'enclume, vue de trois-quarts, n'occupe
/// que le bas de sa case. C'est là que les pieds du forgeron se posent quand il forge (voir `forge_post`).
pub const ANVIL_BACK: f32 = 0.22;

pub struct TileMap {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl TileMap {
    pub fn new(width: i32, height: i32, fill: Tile) -> Self {
        TileMap {
            width,
            height,
            tiles: vec![fill; (width * height) as usize],
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn in_bounds(&self, p: TilePos) -> bool {
        (0..self.width).contains(&p.x) && (0..self.height).contains(&p.y)
    }

    /// Tuile à cette position. Hors de la carte : de l'eau (infranchissable).
    pub fn get(&self, p: TilePos) -> Tile {
        if self.in_bounds(p) {
            self.tiles[(p.y * self.width + p.x) as usize]
        } else {
            Tile::Water
        }
    }

    pub fn set(&mut self, p: TilePos, tile: Tile) {
        if self.in_bounds(p) {
            self.tiles[(p.y * self.width + p.x) as usize] = tile;
        }
    }

    /// Toutes les tuiles de ce type, dans l'ordre de lecture (haut vers bas, gauche vers droite).
    pub fn find_tiles(&self, tile: Tile) -> Vec<TilePos> {
        let mut found = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = TilePos::new(x, y);
                if self.get(p) == tile {
                    found.push(p);
                }
            }
        }
        found
    }

    /// Les cases praticables directement autour de `p` (4 directions).
    pub fn walkable_neighbors(&self, p: TilePos) -> Vec<TilePos> {
        [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .iter()
            .map(|&(dx, dy)| TilePos::new(p.x + dx, p.y + dy))
            .filter(|&n| self.in_bounds(n) && !self.get(n).is_solid())
            .collect()
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, tile: Tile) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.set(TilePos::new(xx, yy), tile);
            }
        }
    }

    /// Bâtiment : murs sur le pourtour, sol à l'intérieur, une porte (tuile de sol dans le mur).
    pub fn add_building(&mut self, x: i32, y: i32, w: i32, h: i32, door: TilePos) {
        self.fill_rect(x, y, w, h, Tile::Wall);
        self.fill_rect(x + 1, y + 1, w - 2, h - 2, Tile::Floor);
        self.set(door, Tile::Floor);
    }

    /// Une boîte carrée centrée sur `center` (demi-côté `half`, en tuiles) touche-t-elle un obstacle ?
    pub fn is_box_blocked(&self, center: Vec2, half: f32) -> bool {
        [(-half, -half), (half, -half), (-half, half), (half, half)].iter().any(|&(dx, dy)| {
            let p = Vec2::new(center.x + dx, center.y + dy);
            let tile = p.tile();
            match self.get(tile) {
                // le haut de la case de l'enclume est derrière elle : le forgeron s'y tient pour forger
                Tile::Anvil => p.y - tile.y as f32 >= ANVIL_BACK,
                t => t.is_solid(),
            }
        })
    }

    /// Une boîte de demi-côté `half` peut-elle aller en ligne droite de `a` à `b` sans toucher d'obstacle ?
    pub fn is_segment_clear(&self, a: Vec2, b: Vec2, half: f32) -> bool {
        let steps = (a.distance(b) / 0.1).ceil().max(1.0) as i32;
        (0..=steps).all(|i| {
            let t = i as f32 / steps as f32;
            !self.is_box_blocked(a + (b - a) * t, half)
        })
    }

    /// Plus court chemin de `from` à `to` en mode `Roads` avec le surcoût d'herbe par défaut.
    /// Le résultat exclut `from` et inclut `to` ; il est vide si `to` est inaccessible
    /// (ou si `from == to`).
    pub fn find_path(&self, from: TilePos, to: TilePos) -> VecDeque<TilePos> {
        self.find_path_mode(from, to, PathMode::Roads(DEFAULT_GRASS_COST))
    }

    pub fn find_path_mode(&self, from: TilePos, to: TilePos, mode: PathMode) -> VecDeque<TilePos> {
        self.find_path_cost(from, to, mode).map(|(path, _)| path).unwrap_or_default()
    }

    /// Coût d'une case selon le mode : la route et le sol des bâtiments coûtent 1, le reste `grass_cost`.
    fn tile_cost(&self, p: TilePos, mode: PathMode) -> f32 {
        match mode {
            PathMode::Direct => 1.0,
            PathMode::Roads(grass) => match self.get(p) {
                Tile::Path | Tile::Floor | Tile::Bed => 1.0,
                _ => grass.max(1.0),
            },
        }
    }

    /// Plus court chemin (8 directions, sans couper le coin d'un obstacle) et son coût.
    /// Un pas coûte sa longueur (1, ou √2 en diagonale) fois le coût de la case d'arrivée.
    /// `None` si `to` est inaccessible ; un chemin vide de coût 0 si `from == to`.
    pub fn find_path_cost(&self, from: TilePos, to: TilePos, mode: PathMode) -> Option<(VecDeque<TilePos>, f32)> {
        if !self.in_bounds(from) || !self.in_bounds(to) || self.get(to).is_solid() {
            return None;
        }
        if from == to {
            return Some((VecDeque::new(), 0.0));
        }
        let idx = |p: TilePos| (p.y * self.width + p.x) as usize;
        let count = (self.width * self.height) as usize;
        let mut best = vec![u32::MAX; count];
        let mut prev: Vec<Option<TilePos>> = vec![None; count];
        let mut heap = BinaryHeap::new();
        // coûts en millièmes ; heuristique octile (admissible : une case coûte au moins 1)
        let h = |p: TilePos| {
            let (dx, dy) = ((p.x - to.x).unsigned_abs(), (p.y - to.y).unsigned_abs());
            let (lo, hi) = (dx.min(dy), dx.max(dy));
            lo * DIAG_COST + (hi - lo) * STRAIGHT_COST
        };
        best[idx(from)] = 0;
        heap.push((Reverse(h(from)), Reverse(0u32), Reverse(idx(from) as u32)));

        while let Some((_, Reverse(g), Reverse(ci))) = heap.pop() {
            if g > best[ci as usize] {
                continue;
            }
            let cur = TilePos::new(ci as i32 % self.width, ci as i32 / self.width);
            if cur == to {
                let mut path = VecDeque::new();
                let mut p = cur;
                while p != from {
                    path.push_front(p);
                    p = prev[idx(p)].expect("chaîne de prédécesseurs cassée");
                }
                return Some((path, g as f32 / STRAIGHT_COST as f32));
            }
            for (dx, dy) in NEIGHBORS {
                let next = TilePos::new(cur.x + dx, cur.y + dy);
                if !self.in_bounds(next) || self.get(next).is_solid() {
                    continue;
                }
                let diagonal = dx != 0 && dy != 0;
                if diagonal
                    && (self.get(TilePos::new(cur.x + dx, cur.y)).is_solid() || self.get(TilePos::new(cur.x, cur.y + dy)).is_solid())
                {
                    continue; // pas de coin coupé
                }
                let base = if diagonal { DIAG_COST } else { STRAIGHT_COST };
                let ng = g + (base as f32 * self.tile_cost(next, mode)) as u32;
                if ng < best[idx(next)] {
                    best[idx(next)] = ng;
                    prev[idx(next)] = Some(cur);
                    heap.push((Reverse(ng + h(next)), Reverse(ng), Reverse(idx(next) as u32)));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open(w: i32, h: i32) -> TileMap {
        TileMap::new(w, h, Tile::Grass)
    }

    fn p(x: i32, y: i32) -> TilePos {
        TilePos::new(x, y)
    }

    #[test]
    fn diagonal_in_open_ground() {
        let map = open(12, 12);
        let path = map.find_path_mode(p(1, 1), p(6, 6), PathMode::Direct);
        assert_eq!(path.len(), 5);
        assert!(path.iter().enumerate().all(|(i, t)| *t == p(2 + i as i32, 2 + i as i32)));
    }

    #[test]
    fn no_corner_cutting() {
        let mut map = open(6, 6);
        map.set(p(3, 2), Tile::Wall);
        map.set(p(2, 3), Tile::Wall);
        let path = map.find_path_mode(p(2, 2), p(3, 3), PathMode::Direct);
        assert!(path.len() > 1, "la diagonale entre deux murs en coin est interdite");
        let mut prev = p(2, 2);
        for &t in &path {
            let (dx, dy) = (t.x - prev.x, t.y - prev.y);
            if dx != 0 && dy != 0 {
                assert!(!map.get(p(prev.x + dx, prev.y)).is_solid() && !map.get(p(prev.x, prev.y + dy)).is_solid());
            }
            prev = t;
        }
    }

    #[test]
    fn unreachable_goal_is_empty_and_result_is_deterministic() {
        let mut map = open(8, 8);
        for t in [p(4, 4), p(5, 4), p(6, 4), p(4, 5), p(6, 5), p(4, 6), p(5, 6), p(6, 6)] {
            map.set(t, Tile::Wall);
        }
        assert!(map.find_path(p(0, 0), p(5, 5)).is_empty(), "case entourée de murs");
        assert!(map.find_path(p(0, 0), p(4, 4)).is_empty(), "but solide");
        assert_eq!(map.find_path(p(0, 0), p(7, 7)), map.find_path(p(0, 0), p(7, 7)));
    }

    #[test]
    fn roads_mode_follows_a_nearby_road() {
        let mut map = open(30, 12);
        map.fill_rect(0, 3, 30, 1, Tile::Path);
        let path = map.find_path_mode(p(0, 5), p(28, 5), PathMode::Roads(3.0));
        let on_road = path.iter().filter(|t| map.get(**t) == Tile::Path).count();
        assert!(on_road >= 20, "il longe la route : {on_road} cases sur route");
    }

    #[test]
    fn roads_mode_crosses_grass_when_detour_is_too_long() {
        let mut map = open(30, 30);
        map.fill_rect(0, 25, 30, 1, Tile::Path); // route loin du trajet
        let path = map.find_path_mode(p(0, 5), p(20, 5), PathMode::Roads(3.0));
        assert!(path.iter().all(|t| map.get(*t) != Tile::Path));
    }

    #[test]
    fn roads_mode_leaves_the_road_for_an_off_road_goal() {
        let mut map = open(30, 12);
        map.fill_rect(0, 3, 30, 1, Tile::Path);
        let path = map.find_path_mode(p(0, 3), p(20, 8), PathMode::Roads(3.0));
        assert_eq!(*path.back().unwrap(), p(20, 8));
        let first_grass = path.iter().position(|t| map.get(*t) != Tile::Path).unwrap();
        assert!(first_grass >= 10, "il reste sur la route avant de la quitter : {first_grass}");
    }

    #[test]
    fn direct_mode_ignores_roads() {
        let mut map = open(30, 12);
        map.fill_rect(0, 1, 30, 1, Tile::Path);
        map.fill_rect(29, 1, 1, 10, Tile::Path);
        let direct = map.find_path_mode(p(0, 9), p(28, 9), PathMode::Direct);
        assert_eq!(direct.len(), 28);
        assert!(direct.iter().all(|t| map.get(*t) != Tile::Path));
    }

    #[test]
    fn direct_cuts_where_roads_follows() {
        let mut map = open(30, 30);
        map.fill_rect(0, 2, 28, 1, Tile::Path);
        map.fill_rect(27, 2, 1, 26, Tile::Path); // route en L
        let roads = map.find_path_mode(p(0, 2), p(27, 27), PathMode::Roads(3.0));
        let direct = map.find_path_mode(p(0, 2), p(27, 27), PathMode::Direct);
        let on_road = |path: &VecDeque<TilePos>| path.iter().filter(|t| map.get(**t) == Tile::Path).count();
        assert!(on_road(&roads) > on_road(&direct));
        assert!(direct.len() < roads.len());
    }

    #[test]
    fn path_cost_counts_diagonal_as_sqrt2() {
        let map = open(8, 8);
        let (path, cost) = map.find_path_cost(p(1, 1), p(2, 2), PathMode::Direct).unwrap();
        assert_eq!(path.len(), 1);
        assert!((cost - 1.414).abs() < 0.001);
        let (_, straight) = map.find_path_cost(p(1, 1), p(4, 1), PathMode::Direct).unwrap();
        assert!((straight - 3.0).abs() < 0.001);
        assert!(map.find_path_cost(p(1, 1), p(1, 1), PathMode::Direct).unwrap().0.is_empty());
    }
}
