//! La carte à tuiles : types de terrain, collisions et recherche de chemin.
//! Aucune notion de couleur ou de sprite ici : le rendu décide comment afficher chaque `Tile`.

use crate::math::{TilePos, Vec2};
use std::collections::VecDeque;

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
    /// Le pied d'un lit de 2×1 (la tête est une `Bed`) : praticable.
    BedFoot,
    /// La moitié gauche d'une table de 2×1 : meuble solide.
    Table,
    /// La moitié droite d'une table : meuble solide.
    TableEnd,
    /// La moitié gauche d'une étagère de 2×1 : meuble solide.
    Shelf,
    /// La moitié droite d'une étagère : meuble solide.
    ShelfEnd,
    /// Un bac d'eau d'habitat (1×1) : meuble solide.
    Trough,
    /// Une pousse d'arbre : praticable, redevient un arbre après un délai.
    Sapling,
}

impl Tile {
    pub fn is_solid(self) -> bool {
        matches!(self, Tile::Water | Tile::Wall | Tile::Tree | Tile::Anvil | Tile::Ore | Tile::Crate | Tile::Table | Tile::TableEnd | Tile::Shelf | Tile::ShelfEnd | Tile::Trough)
    }
}

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
        [(-half, -half), (half, -half), (-half, half), (half, half)]
            .iter()
            .any(|&(dx, dy)| self.get(Vec2::new(center.x + dx, center.y + dy).tile()).is_solid())
    }

    /// Plus court chemin (4 directions) de `from` à `to`.
    /// Le résultat exclut `from` et inclut `to` ; il est vide si `to` est inaccessible
    /// (ou si `from == to`).
    pub fn find_path(&self, from: TilePos, to: TilePos) -> VecDeque<TilePos> {
        if !self.in_bounds(from) || !self.in_bounds(to) || self.get(to).is_solid() {
            return VecDeque::new();
        }
        let idx = |p: TilePos| (p.y * self.width + p.x) as usize;
        let count = (self.width * self.height) as usize;
        let mut prev: Vec<Option<TilePos>> = vec![None; count];
        let mut seen = vec![false; count];
        let mut queue = VecDeque::new();

        seen[idx(from)] = true;
        queue.push_back(from);

        while let Some(cur) = queue.pop_front() {
            if cur == to {
                let mut path = VecDeque::new();
                let mut p = cur;
                while p != from {
                    path.push_front(p);
                    p = prev[idx(p)].expect("chaîne de prédécesseurs cassée");
                }
                return path;
            }
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let next = TilePos::new(cur.x + dx, cur.y + dy);
                if !self.in_bounds(next) || seen[idx(next)] || self.get(next).is_solid() {
                    continue;
                }
                seen[idx(next)] = true;
                prev[idx(next)] = Some(cur);
                queue.push_back(next);
            }
        }
        VecDeque::new()
    }
}
