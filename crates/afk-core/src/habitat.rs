//! Les habitats : des bâtiments de taille fixe où l'on dort.
//!
//! Il y a trois classes : la petite maison (7×5, une place), la maison moyenne (9×5, jusqu'à trois places)
//! et la grande maison (9×7, jusqu'à six places). Une « place » est un lit ; les places d'un même habitat
//! se partagent une table, une étagère et un bac d'eau (pièce commune).
//!
//! Le mobilier a une taille fixe : le lit, la table et l'étagère font 2 tuiles, le bac 1. Chaque meuble se
//! pose dans une des quatre rotations (`Rotation`), à partir de sa pose de base : empreinte horizontale 2×1,
//! tête de lit à gauche, façade d'étagère tournée vers le bas. Un quart de tour fait passer un meuble de 2 tuiles de
//! 2×1 à 1×2. L'ancre d'un meuble est la tuile en haut à gauche de son empreinte tournée.

use crate::map::{Tile, TileMap};
use crate::math::{Rotation, TilePos};
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

/// Un meuble posé : l'ancre (tuile en haut à gauche de son empreinte tournée) et sa rotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Furniture {
    pub anchor: TilePos,
    pub rotation: Rotation,
}

impl Furniture {
    pub fn new(x: i32, y: i32, rotation: Rotation) -> Furniture {
        Furniture { anchor: TilePos::new(x, y), rotation }
    }

    /// Les tuiles de l'empreinte d'un meuble de `len` tuiles (1 ou 2) selon sa rotation.
    pub fn tiles(&self, len: i32) -> Vec<TilePos> {
        let (w, h) = self.rotation.footprint((len, 1));
        let mut tiles = Vec::new();
        for dy in 0..h {
            for dx in 0..w {
                tiles.push(TilePos::new(self.anchor.x + dx, self.anchor.y + dy));
            }
        }
        tiles
    }

    /// Pour un lit de deux tuiles : la tuile de la tête, qui désigne la place. À 0° elle est à gauche, à 90° en haut,
    /// à 180° à droite et à 270° en bas.
    pub fn head(&self) -> TilePos {
        match self.rotation {
            Rotation::Deg0 | Rotation::Deg90 => self.anchor,
            Rotation::Deg180 => TilePos::new(self.anchor.x + 1, self.anchor.y),
            Rotation::Deg270 => TilePos::new(self.anchor.x, self.anchor.y + 1),
        }
    }

    /// Pour une étagère : le côté vers lequel sa façade regarde, en `(dx, dy)`. À 0° vers le bas (le sud), puis
    /// un quart de tour dans le sens des aiguilles d'une montre : à 90° vers la gauche, à 180° vers le haut, à 270°
    /// vers la droite.
    pub fn front(&self) -> (i32, i32) {
        match self.rotation {
            Rotation::Deg0 => (0, 1),
            Rotation::Deg90 => (-1, 0),
            Rotation::Deg180 => (0, -1),
            Rotation::Deg270 => (1, 0),
        }
    }
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
    /// Les lits (2 tuiles) : une place chacun.
    pub beds: Vec<Furniture>,
    /// La table (2 tuiles).
    pub table: Furniture,
    /// L'étagère (2 tuiles).
    pub shelf: Furniture,
    /// Le bac d'eau (1 tuile).
    pub trough: Furniture,
}

impl HabitatSpec {
    pub fn new(
        x: i32,
        y: i32,
        class: HabitatClass,
        door: (i32, i32),
        beds: &[Furniture],
        table: Furniture,
        shelf: Furniture,
        trough: Furniture,
    ) -> HabitatSpec {
        HabitatSpec { x, y, class, door: TilePos::new(door.0, door.1), beds: beds.to_vec(), table, shelf, trough }
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

    /// Les places : la tuile de tête de chaque lit (celle que désigne `home`).
    pub fn bed_homes(&self) -> Vec<TilePos> {
        self.beds.iter().map(|b| b.head()).collect()
    }

    /// Les tuiles de tous les lits (tête et pied).
    pub fn bed_tiles(&self) -> Vec<TilePos> {
        self.beds.iter().flat_map(|b| b.tiles(2)).collect()
    }

    /// Les tuiles des meubles infranchissables : table, étagère et bac.
    pub fn solid_tiles(&self) -> Vec<TilePos> {
        let mut tiles = self.table.tiles(2);
        tiles.extend(self.shelf.tiles(2));
        tiles.extend(self.trough.tiles(1));
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
        // chaque meuble tient dans l'intérieur (une fois tourné), et deux meubles ne se chevauchent pas
        let mut occupied: Vec<TilePos> = Vec::new();
        let furniture: Vec<(&str, Vec<TilePos>)> = self
            .beds
            .iter()
            .map(|b| ("un lit", b.tiles(2)))
            .chain([
                ("la table", self.table.tiles(2)),
                ("l'étagère", self.shelf.tiles(2)),
                ("le bac d'eau", self.trough.tiles(1)),
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

    /// Pose l'habitat sur la carte : murs, sol, porte, puis meubles (`Tile::Bed` praticable pour les lits,
    /// `Tile::Furniture` solide pour le reste). Panique si l'habitat est non conforme.
    pub fn build(&self, map: &mut TileMap) {
        if let Err(why) = self.validate() {
            panic!("habitat non conforme en ({}, {}) : {why}", self.x, self.y);
        }
        map.add_building(self.x, self.y, self.width(), self.height(), self.door);
        for t in self.bed_tiles() {
            map.set(t, Tile::Bed);
        }
        for t in self.solid_tiles() {
            map.set(t, Tile::Furniture);
        }
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
    use Rotation::{Deg0, Deg180, Deg270, Deg90};

    /// Petite maison en (4,23), porte en haut en (8,23). Pose de l'image : étagère contre le mur nord, lit vertical
    /// tête en haut contre le mur est, table au centre, baquet dans le coin sud-est.
    fn small() -> HabitatSpec {
        HabitatSpec::new(
            4,
            23,
            HabitatClass::Small,
            (8, 23),
            &[Furniture::new(9, 24, Deg90)],
            Furniture::new(6, 25, Deg0),
            Furniture::new(5, 24, Deg0),
            Furniture::new(9, 26, Deg0),
        )
    }

    fn medium() -> HabitatSpec {
        HabitatSpec::new(
            4,
            23,
            HabitatClass::Medium,
            (8, 23),
            &[Furniture::new(9, 24, Deg90), Furniture::new(10, 24, Deg90), Furniture::new(11, 24, Deg90)],
            Furniture::new(6, 25, Deg0),
            Furniture::new(5, 24, Deg0),
            Furniture::new(11, 26, Deg0),
        )
    }

    fn large() -> HabitatSpec {
        HabitatSpec::new(
            4,
            23,
            HabitatClass::Large,
            (8, 23),
            &[
                Furniture::new(9, 24, Deg90),
                Furniture::new(10, 24, Deg90),
                Furniture::new(11, 24, Deg90),
                Furniture::new(9, 27, Deg90),
                Furniture::new(10, 27, Deg90),
                Furniture::new(11, 27, Deg90),
            ],
            Furniture::new(6, 26, Deg0),
            Furniture::new(5, 24, Deg0),
            Furniture::new(5, 28, Deg0),
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
    fn a_two_tile_piece_swaps_between_2x1_and_1x2_with_its_rotation() {
        let at = |r| Furniture::new(10, 10, r).tiles(2);
        let t = |x, y| TilePos::new(x, y);
        assert_eq!(at(Deg0), vec![t(10, 10), t(11, 10)]);
        assert_eq!(at(Deg90), vec![t(10, 10), t(10, 11)]);
        assert_eq!(at(Deg180), vec![t(10, 10), t(11, 10)]);
        assert_eq!(at(Deg270), vec![t(10, 10), t(10, 11)]);
        // le bac ne couvre qu'une tuile, quelle que soit sa rotation
        for r in Rotation::ALL {
            assert_eq!(Furniture::new(10, 10, r).tiles(1), vec![t(10, 10)]);
        }
    }

    #[test]
    fn the_head_of_a_bed_follows_its_rotation() {
        let t = |x, y| TilePos::new(x, y);
        assert_eq!(Furniture::new(10, 10, Deg0).head(), t(10, 10), "à 0° : à gauche");
        assert_eq!(Furniture::new(10, 10, Deg90).head(), t(10, 10), "à 90° : en haut");
        assert_eq!(Furniture::new(10, 10, Deg180).head(), t(11, 10), "à 180° : à droite");
        assert_eq!(Furniture::new(10, 10, Deg270).head(), t(10, 11), "à 270° : en bas");
        // la tuile de tête est toujours l'une des deux tuiles du lit
        for r in Rotation::ALL {
            let bed = Furniture::new(10, 10, r);
            assert!(bed.tiles(2).contains(&bed.head()));
        }
    }

    #[test]
    fn a_shelf_front_turns_clockwise_from_south() {
        assert_eq!(Furniture::new(0, 0, Deg0).front(), (0, 1));
        assert_eq!(Furniture::new(0, 0, Deg90).front(), (-1, 0));
        assert_eq!(Furniture::new(0, 0, Deg180).front(), (0, -1));
        assert_eq!(Furniture::new(0, 0, Deg270).front(), (1, 0));
    }

    #[test]
    fn a_vertical_bed_has_its_head_up_and_the_place_is_the_head_tile() {
        let house = small();
        assert_eq!(house.bed_homes(), vec![TilePos::new(9, 24)]);
        assert_eq!(house.bed_tiles(), vec![TilePos::new(9, 24), TilePos::new(9, 25)]);
    }

    #[test]
    fn building_puts_beds_underfoot_and_everything_else_in_the_way() {
        let mut map = TileMap::new(30, 40, Tile::Grass);
        small().build(&mut map);
        for t in [TilePos::new(9, 24), TilePos::new(9, 25)] {
            assert_eq!(map.get(t), Tile::Bed);
            assert!(!map.get(t).is_solid(), "un lit se traverse");
        }
        for t in [TilePos::new(6, 25), TilePos::new(7, 25), TilePos::new(5, 24), TilePos::new(6, 24), TilePos::new(9, 26)] {
            assert_eq!(map.get(t), Tile::Furniture, "{t:?}");
            assert!(map.get(t).is_solid(), "{t:?} doit bloquer");
        }
        // le bac ne couvre qu'une tuile : sa voisine est du sol
        assert_eq!(map.get(TilePos::new(8, 26)), Tile::Floor);
        assert!(!map.find_path(TilePos::new(8, 22), TilePos::new(9, 25)).is_empty(), "on peut rejoindre le pied du lit");
    }

    #[test]
    fn a_rotated_bed_is_accepted_in_every_orientation_when_it_fits() {
        for rotation in Rotation::ALL {
            let mut house = small();
            // 90° et 270° : verticaux (1×2) ; 0° et 180° : horizontaux (2×1), posés dans la rangée libre du bas
            house.beds = vec![if rotation.quarter_turns() % 2 == 1 { Furniture::new(9, 24, rotation) } else { Furniture::new(8, 26, rotation) }];
            house.trough = Furniture::new(5, 26, Deg0);
            assert_eq!(house.validate(), Ok(()), "{rotation:?}");
        }
    }

    #[test]
    fn a_small_house_with_two_beds_is_refused() {
        let mut house = small();
        house.beds.push(Furniture::new(8, 26, Deg0));
        assert!(house.validate().unwrap_err().contains("maximum 1"));
    }

    #[test]
    fn a_house_without_bed_is_refused() {
        let mut house = small();
        house.beds.clear();
        assert!(house.validate().is_err());
    }

    #[test]
    fn a_rotated_piece_that_sticks_out_or_overlaps_is_refused() {
        // un lit vertical tout en bas de l'intérieur déborde dans le mur sud
        let mut house = small();
        house.beds = vec![Furniture::new(9, 26, Deg90)];
        house.trough = Furniture::new(5, 26, Deg0);
        assert!(house.validate().unwrap_err().contains("déborde"));
        // la table, tournée, chevauche le lit
        let mut house = small();
        house.table = Furniture::new(9, 25, Deg90);
        assert!(house.validate().unwrap_err().contains("chevauche"));
        // le bac sur l'étagère
        let mut house = small();
        house.trough = Furniture::new(6, 24, Deg0);
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
    fn a_bed_walled_in_by_rotated_furniture_is_refused() {
        // lit vertical en (9,24)-(9,25) dans le coin nord-est : une table verticale à gauche et le bac dessous l'emmurent
        // mais le lit est praticable ; on emmure donc l'entrée elle-même avec l'étagère et la table
        let house = HabitatSpec::new(
            4,
            23,
            HabitatClass::Small,
            (6, 23),
            &[Furniture::new(9, 25, Deg0)], // lit horizontal en bas à droite
            Furniture::new(8, 24, Deg90),   // table verticale (8,24)-(8,25)
            Furniture::new(8, 26, Deg0),    // étagère (8,26)-(9,26)
            Furniture::new(7, 24, Deg0),
        );
        let why = house.validate().unwrap_err();
        assert!(why.contains("joignable") || why.contains("déborde"), "{why}");
    }

    #[test]
    fn a_house_without_enough_free_room_is_refused() {
        let mut house = large();
        house.beds.extend([Furniture::new(5, 25, Deg90), Furniture::new(6, 25, Deg90)]);
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
