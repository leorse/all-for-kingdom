//! Les habitats : des bâtiments de taille fixe où l'on dort.
//!
//! Il y a trois classes : la petite maison (7×5, une place), la maison moyenne (9×5, jusqu'à trois places)
//! et la grande maison (9×7, jusqu'à six places). Une « place » est un lit ; les places d'un même habitat
//! se partagent une table, une étagère et un bac d'eau (pièce commune).
//!
//! Le mobilier a une taille fixe : le lit, la table et l'étagère font 2×1 tuiles (largeur 2, hauteur 1),
//! le bac 1×1. Un meuble de 2×1 est repéré par sa tuile de gauche ; pour un lit, c'est la tête, qui désigne
//! la place (`home` du personnage).

use crate::map::{Tile, TileMap};
use crate::math::TilePos;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HabitatClass {
    Small,
    Medium,
    Large,
}

impl HabitatClass {
    /// Largeur et hauteur, murs compris.
    pub fn size(self) -> (i32, i32) {
        match self {
            HabitatClass::Small => (7, 5),
            HabitatClass::Medium => (9, 5),
            HabitatClass::Large => (9, 7),
        }
    }

    /// Nombre maximal de places (de lits).
    pub fn max_places(self) -> usize {
        match self {
            HabitatClass::Small => 1,
            HabitatClass::Medium => 3,
            HabitatClass::Large => 6,
        }
    }

    /// La classe d'après, si l'habitat peut encore être agrandi.
    pub fn next(self) -> Option<HabitatClass> {
        match self {
            HabitatClass::Small => Some(HabitatClass::Medium),
            HabitatClass::Medium => Some(HabitatClass::Large),
            HabitatClass::Large => None,
        }
    }
}

/// Les deux tuiles (gauche, droite) d'un meuble de 2×1.
fn pair(left: TilePos) -> [TilePos; 2] {
    [left, TilePos::new(left.x + 1, left.y)]
}

/// Un habitat tel que le scénario le décrit.
#[derive(Clone, Debug)]
pub struct HabitatSpec {
    /// Coin haut-gauche (le mur).
    pub x: i32,
    pub y: i32,
    pub class: HabitatClass,
    /// La porte, dans un mur (hors angle).
    pub door: TilePos,
    /// Les lits (2×1) : tuile de tête, à gauche. Une place chacun.
    pub beds: Vec<TilePos>,
    /// La table (2×1) : tuile de gauche.
    pub table: TilePos,
    /// L'étagère (2×1) : tuile de gauche.
    pub shelf: TilePos,
    /// Le bac d'eau (1×1).
    pub trough: TilePos,
}

impl HabitatSpec {
    pub fn new(
        x: i32,
        y: i32,
        class: HabitatClass,
        door: (i32, i32),
        beds: &[(i32, i32)],
        table: (i32, i32),
        shelf: (i32, i32),
        trough: (i32, i32),
    ) -> HabitatSpec {
        let tile = |(x, y): (i32, i32)| TilePos::new(x, y);
        HabitatSpec {
            x,
            y,
            class,
            door: tile(door),
            beds: beds.iter().map(|&b| tile(b)).collect(),
            table: tile(table),
            shelf: tile(shelf),
            trough: tile(trough),
        }
    }

    pub fn width(&self) -> i32 {
        self.class.size().0
    }

    pub fn height(&self) -> i32 {
        self.class.size().1
    }

    /// Le bâtiment (murs compris) couvre-t-il cette tuile ?
    pub fn covers(&self, t: TilePos) -> bool {
        (self.x..self.x + self.width()).contains(&t.x) && (self.y..self.y + self.height()).contains(&t.y)
    }

    /// L'intérieur (sans les murs) contient-il cette tuile ?
    pub fn interior_contains(&self, t: TilePos) -> bool {
        (self.x + 1..self.x + self.width() - 1).contains(&t.x) && (self.y + 1..self.y + self.height() - 1).contains(&t.y)
    }

    /// La case intérieure juste derrière la porte.
    pub fn entrance(&self) -> TilePos {
        let (w, h) = (self.width(), self.height());
        if self.door.y == self.y {
            TilePos::new(self.door.x, self.y + 1)
        } else if self.door.y == self.y + h - 1 {
            TilePos::new(self.door.x, self.y + h - 2)
        } else if self.door.x == self.x {
            TilePos::new(self.x + 1, self.door.y)
        } else {
            TilePos::new(self.x + w - 2, self.door.y)
        }
    }

    /// Nombre de tuiles de l'intérieur.
    pub fn interior_size(&self) -> usize {
        ((self.width() - 2) * (self.height() - 2)) as usize
    }

    /// Les tuiles de tous les lits (tête et pied).
    pub fn bed_tiles(&self) -> Vec<TilePos> {
        self.beds.iter().flat_map(|&b| pair(b)).collect()
    }

    /// Les tuiles des meubles infranchissables : table, étagère et bac.
    pub fn solid_tiles(&self) -> Vec<TilePos> {
        let mut tiles: Vec<TilePos> = pair(self.table).to_vec();
        tiles.extend(pair(self.shelf));
        tiles.push(self.trough);
        tiles
    }

    /// Cet habitat respecte-t-il les règles ? Renvoie la première règle violée.
    pub fn validate(&self) -> Result<(), String> {
        let (w, h) = (self.width(), self.height());
        // la porte : sur le pourtour, jamais dans un angle
        let on_horizontal = (self.door.y == self.y || self.door.y == self.y + h - 1) && (self.x + 1..self.x + w - 1).contains(&self.door.x);
        let on_vertical = (self.door.x == self.x || self.door.x == self.x + w - 1) && (self.y + 1..self.y + h - 1).contains(&self.door.y);
        if !on_horizontal && !on_vertical {
            return Err(format!("la porte {:?} n'est pas dans un mur (hors angle)", self.door));
        }
        // les lits
        if self.beds.is_empty() {
            return Err("un habitat a au moins un lit".into());
        }
        if self.beds.len() > self.class.max_places() {
            return Err(format!("{} lits pour {:?} (maximum {})", self.beds.len(), self.class, self.class.max_places()));
        }
        // chaque meuble tient dans l'intérieur, et deux meubles ne se chevauchent pas
        let mut occupied: Vec<TilePos> = Vec::new();
        let furniture: Vec<(&str, Vec<TilePos>)> = self
            .beds
            .iter()
            .map(|&b| ("un lit", pair(b).to_vec()))
            .chain([
                ("la table", pair(self.table).to_vec()),
                ("l'étagère", pair(self.shelf).to_vec()),
                ("le bac d'eau", vec![self.trough]),
            ])
            .collect();
        for (name, tiles) in &furniture {
            for &t in tiles {
                if !self.interior_contains(t) {
                    return Err(format!("{name} déborde de l'intérieur en {t:?}"));
                }
                if occupied.contains(&t) {
                    return Err(format!("{name} chevauche un autre meuble en {t:?}"));
                }
                occupied.push(t);
            }
        }
        // de la place pour circuler : au moins trois cases libres de tout meuble
        let free = self.interior_size() - occupied.len();
        if free < 3 {
            return Err(format!("seulement {free} cases libres dans l'intérieur"));
        }
        // chaque tuile de lit est joignable depuis l'entrée sans traverser un meuble solide
        let solid = self.solid_tiles();
        let entrance = self.entrance();
        if solid.contains(&entrance) {
            return Err("un meuble bouche l'entrée".into());
        }
        let mut seen = vec![entrance];
        let mut queue = VecDeque::from([entrance]);
        while let Some(cur) = queue.pop_front() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let next = TilePos::new(cur.x + dx, cur.y + dy);
                if self.interior_contains(next) && !solid.contains(&next) && !seen.contains(&next) {
                    seen.push(next);
                    queue.push_back(next);
                }
            }
        }
        for t in self.bed_tiles() {
            if !seen.contains(&t) {
                return Err(format!("le lit en {t:?} n'est pas joignable depuis l'entrée"));
            }
        }
        Ok(())
    }

    /// Pose l'habitat sur la carte : murs, sol, porte, puis meubles. Panique si l'habitat est non conforme.
    pub fn build(&self, map: &mut TileMap) {
        if let Err(why) = self.validate() {
            panic!("habitat non conforme en ({}, {}) : {why}", self.x, self.y);
        }
        map.add_building(self.x, self.y, self.width(), self.height(), self.door);
        for &bed in &self.beds {
            let [head, foot] = pair(bed);
            map.set(head, Tile::Bed);
            map.set(foot, Tile::BedFoot);
        }
        let [t0, t1] = pair(self.table);
        map.set(t0, Tile::Table);
        map.set(t1, Tile::TableEnd);
        let [s0, s1] = pair(self.shelf);
        map.set(s0, Tile::Shelf);
        map.set(s1, Tile::ShelfEnd);
        map.set(self.trough, Tile::Trough);
    }

    /// Peut-on agrandir l'habitat ? La classe suivante, ancrée au même coin haut-gauche, ne doit couvrir
    /// que des cases libres (herbe ou chemin) dans la carte, hors de l'emprise actuelle.
    pub fn can_expand(&self, map: &TileMap) -> bool {
        let Some(next) = self.class.next() else { return false };
        let (w, h) = next.size();
        for y in self.y..self.y + h {
            for x in self.x..self.x + w {
                let t = TilePos::new(x, y);
                if self.covers(t) {
                    continue;
                }
                if !map.in_bounds(t) || !matches!(map.get(t), Tile::Grass | Tile::Path) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Petite maison en (4,23) : intérieur x 5..9, y 24..26 ; porte en haut en (8,23).
    fn small() -> HabitatSpec {
        HabitatSpec::new(4, 23, HabitatClass::Small, (8, 23), &[(5, 24)], (8, 26), (5, 26), (9, 24))
    }

    fn medium() -> HabitatSpec {
        HabitatSpec::new(4, 23, HabitatClass::Medium, (8, 23), &[(5, 24), (10, 24), (5, 26)], (8, 26), (10, 26), (7, 24))
    }

    fn large() -> HabitatSpec {
        HabitatSpec::new(
            4,
            23,
            HabitatClass::Large,
            (8, 23),
            &[(5, 24), (10, 24), (5, 26), (10, 26), (5, 28), (10, 28)],
            (7, 26),
            (7, 28),
            (9, 26),
        )
    }

    #[test]
    fn each_class_has_its_fixed_size_and_places() {
        assert_eq!((HabitatClass::Small.size(), HabitatClass::Small.max_places()), ((7, 5), 1));
        assert_eq!((HabitatClass::Medium.size(), HabitatClass::Medium.max_places()), ((9, 5), 3));
        assert_eq!((HabitatClass::Large.size(), HabitatClass::Large.max_places()), ((9, 7), 6));
        assert_eq!(HabitatClass::Small.next(), Some(HabitatClass::Medium));
        assert_eq!(HabitatClass::Medium.next(), Some(HabitatClass::Large));
        assert_eq!(HabitatClass::Large.next(), None);
    }

    #[test]
    fn interiors_are_5x3_7x3_and_7x5() {
        assert_eq!(small().interior_size(), 15);
        assert_eq!(medium().interior_size(), 21);
        assert_eq!(large().interior_size(), 35);
    }

    #[test]
    fn well_formed_houses_of_every_class_are_accepted() {
        for house in [small(), medium(), large()] {
            assert_eq!(house.validate(), Ok(()), "{:?}", house.class);
        }
    }

    #[test]
    fn a_bed_covers_two_tiles_head_left_foot_right_and_both_can_be_walked_on() {
        let mut map = TileMap::new(30, 40, Tile::Grass);
        small().build(&mut map);
        assert_eq!(map.get(TilePos::new(5, 24)), Tile::Bed);
        assert_eq!(map.get(TilePos::new(6, 24)), Tile::BedFoot);
        assert!(!map.get(TilePos::new(5, 24)).is_solid() && !map.get(TilePos::new(6, 24)).is_solid());
        assert!(!map.find_path(TilePos::new(8, 22), TilePos::new(6, 24)).is_empty(), "on peut rejoindre le pied du lit");
    }

    #[test]
    fn table_and_shelf_cover_two_tiles_and_the_trough_one_and_all_are_solid() {
        let mut map = TileMap::new(30, 40, Tile::Grass);
        small().build(&mut map);
        for (tile, expected) in [
            ((8, 26), Tile::Table),
            ((9, 26), Tile::TableEnd),
            ((5, 26), Tile::Shelf),
            ((6, 26), Tile::ShelfEnd),
            ((9, 24), Tile::Trough),
        ] {
            let t = TilePos::new(tile.0, tile.1);
            assert_eq!(map.get(t), expected);
            assert!(map.get(t).is_solid(), "{expected:?} doit bloquer");
        }
        // le bac ne couvre qu'une tuile : sa voisine est du sol
        assert_eq!(map.get(TilePos::new(9, 25)), Tile::Floor);
    }

    #[test]
    fn a_small_house_with_two_beds_is_refused() {
        let mut house = small();
        house.beds.push(TilePos::new(5, 25));
        assert!(house.validate().unwrap_err().contains("maximum 1"));
    }

    #[test]
    fn a_house_without_bed_is_refused() {
        let mut house = small();
        house.beds.clear();
        assert!(house.validate().is_err());
    }

    #[test]
    fn furniture_that_sticks_out_or_overlaps_is_refused() {
        // le pied du lit dépasse dans le mur
        let mut house = small();
        house.beds = vec![TilePos::new(9, 25)];
        assert!(house.validate().unwrap_err().contains("déborde"));
        // la table chevauche le lit
        let mut house = small();
        house.table = TilePos::new(6, 24);
        assert!(house.validate().unwrap_err().contains("chevauche"));
        // le bac sur l'étagère
        let mut house = small();
        house.trough = TilePos::new(6, 26);
        assert!(house.validate().unwrap_err().contains("chevauche"));
    }

    #[test]
    fn a_door_in_a_corner_or_off_the_wall_is_refused() {
        let mut house = small();
        house.door = TilePos::new(4, 23);
        assert!(house.validate().is_err());
        house.door = TilePos::new(8, 24);
        assert!(house.validate().is_err());
    }

    #[test]
    fn a_bed_walled_in_by_furniture_is_refused() {
        // lit en (5,24)-(6,24) dans le coin : l'étagère dessous et le bac à droite l'emmurent
        let house = HabitatSpec::new(4, 23, HabitatClass::Small, (8, 23), &[(5, 24)], (8, 26), (5, 25), (7, 24));
        let why = house.validate().unwrap_err();
        assert!(why.contains("joignable"), "{why}");
    }

    #[test]
    fn a_house_without_enough_free_room_is_refused() {
        // 15 tuiles - lit 2 - table 2 - étagère 2 - bac 1 = 8 libres : on remplit avec un lit de trop en grande maison
        let mut house = large();
        house.beds.extend([TilePos::new(5, 25), TilePos::new(5, 27)]);
        assert!(house.validate().is_err());
    }

    #[test]
    fn expansion_needs_free_ground_around() {
        let house = small();
        let mut map = TileMap::new(40, 40, Tile::Grass);
        house.build(&mut map);
        assert!(house.can_expand(&map), "dégagé : on peut passer à la maison moyenne");
        map.set(TilePos::new(12, 25), Tile::Tree); // dans les deux colonnes ajoutées (x 11..12)
        assert!(!house.can_expand(&map), "un arbre gêne");
        map.set(TilePos::new(12, 25), Tile::Path);
        assert!(house.can_expand(&map), "un chemin ne gêne pas");
        map.set(TilePos::new(12, 27), Tile::Water);
        assert!(!house.can_expand(&map), "l'eau gêne");
    }

    #[test]
    fn expansion_is_refused_at_the_map_edge_by_a_crate_and_for_a_large_house() {
        let house = small();
        let mut edge = TileMap::new(12, 40, Tile::Grass); // trop étroite pour 9 de large depuis x=4
        house.build(&mut edge);
        assert!(!house.can_expand(&edge));
        let mut map = TileMap::new(40, 40, Tile::Grass);
        house.build(&mut map);
        map.set(TilePos::new(12, 24), Tile::Crate);
        assert!(!house.can_expand(&map), "un coffre gêne");
        let free = TileMap::new(40, 40, Tile::Grass);
        assert!(!large().can_expand(&free), "déjà au maximum");
        // la maison moyenne grandit vers le bas : deux rangées de plus
        let mut down = TileMap::new(40, 40, Tile::Grass);
        medium().build(&mut down);
        assert!(medium().can_expand(&down));
        down.set(TilePos::new(6, 29), Tile::Tree);
        assert!(!medium().can_expand(&down));
    }
}
