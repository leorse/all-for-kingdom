//! Les filons de minerai et les mines.
//!
//! Un filon est un bloc de 2×2 tuiles, infini, dont on connaît la nature et la qualité. Sur la carte il
//! n'est qu'une tuile solide (`Tile::Ore`) : la nature et la qualité vivent dans la liste des filons.
//! Une mine est la maison d'un mineur (son lit), accolée à son filon.

use crate::map::{Tile, TileMap};
use crate::math::TilePos;
use crate::resources::Resource;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OreKind {
    Iron,
    Copper,
    Gold,
    Stone,
}

impl OreKind {
    /// La ressource que ce minerai ajoute au royaume.
    pub fn resource(self) -> Resource {
        match self {
            OreKind::Iron => Resource::Iron,
            OreKind::Copper => Resource::Copper,
            OreKind::Gold => Resource::Gold,
            OreKind::Stone => Resource::Stone,
        }
    }
}

/// La pureté de la roche : moins il y a d'impuretés, moins il faut de coups de pioche.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    Pure,
    Medium,
    Impure,
}

impl Quality {
    /// Nombre de points (impuretés) dessinés sur chaque tuile du filon.
    pub fn dots(self) -> u32 {
        match self {
            Quality::Pure => 1,
            Quality::Medium => 2,
            Quality::Impure => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vein {
    /// Tuile en haut à gauche du bloc de 2×2.
    pub origin: TilePos,
    pub kind: OreKind,
    pub quality: Quality,
}

impl Vein {
    pub fn new(x: i32, y: i32, kind: OreKind, quality: Quality) -> Vein {
        Vein { origin: TilePos::new(x, y), kind, quality }
    }

    pub fn tiles(&self) -> [TilePos; 4] {
        let o = self.origin;
        [
            TilePos::new(o.x, o.y),
            TilePos::new(o.x + 1, o.y),
            TilePos::new(o.x, o.y + 1),
            TilePos::new(o.x + 1, o.y + 1),
        ]
    }

    pub fn contains(&self, tile: TilePos) -> bool {
        (self.origin.x..self.origin.x + 2).contains(&tile.x) && (self.origin.y..self.origin.y + 2).contains(&tile.y)
    }

    /// Pose le filon sur la carte.
    pub fn place(&self, map: &mut TileMap) {
        map.fill_rect(self.origin.x, self.origin.y, 2, 2, Tile::Ore);
    }

    /// Les cases praticables qui touchent le filon par un côté (huit au maximum) : les postes des mineurs.
    pub fn access_tiles(&self, map: &TileMap) -> Vec<TilePos> {
        let o = self.origin;
        let mut tiles = Vec::new();
        for i in 0..2 {
            tiles.push(TilePos::new(o.x + i, o.y - 1));
            tiles.push(TilePos::new(o.x + i, o.y + 2));
            tiles.push(TilePos::new(o.x - 1, o.y + i));
            tiles.push(TilePos::new(o.x + 2, o.y + i));
        }
        tiles.retain(|&t| map.in_bounds(t) && !map.get(t).is_solid());
        tiles
    }
}

/// Une mine : le lit du mineur (dans le bâtiment accolé) et le filon qu'il exploite.
#[derive(Clone, Copy, Debug)]
pub struct MineSpec {
    /// Indice du filon dans la liste des filons.
    pub vein: usize,
    pub bed: TilePos,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_vein_blocks_the_way_and_has_eight_access_tiles() {
        let mut map = TileMap::new(12, 12, Tile::Grass);
        let vein = Vein::new(5, 5, OreKind::Iron, Quality::Pure);
        vein.place(&mut map);
        for t in vein.tiles() {
            assert!(map.get(t).is_solid());
            assert!(vein.contains(t));
        }
        assert!(!vein.contains(TilePos::new(7, 5)));
        assert!(map.find_path(TilePos::new(4, 4), vein.tiles()[0]).is_empty());
        let access = vein.access_tiles(&map);
        assert_eq!(access.len(), 8);
        assert!(access.iter().all(|&t| !map.get(t).is_solid()));
    }

    #[test]
    fn solid_neighbors_are_not_access_tiles() {
        let mut map = TileMap::new(12, 12, Tile::Grass);
        map.fill_rect(5, 4, 2, 1, Tile::Wall);
        let vein = Vein::new(5, 5, OreKind::Gold, Quality::Impure);
        vein.place(&mut map);
        assert_eq!(vein.access_tiles(&map).len(), 6);
    }

    #[test]
    fn purer_rock_has_fewer_dots() {
        assert!(Quality::Pure.dots() < Quality::Medium.dots());
        assert!(Quality::Medium.dots() < Quality::Impure.dots());
    }
}
