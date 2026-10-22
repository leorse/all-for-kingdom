//! Les forêts et la repousse des arbres.
//!
//! Une forêt est une zone rectangulaire de la carte : seuls les arbres qui s'y trouvent se coupent
//! (les arbres isolés restent du décor). Un arbre abattu laisse une pousse (`Tile::Sapling`) qui
//! redevient un arbre après un délai en heures de jeu. La forêt ne s'étend jamais : elle peut donc
//! s'épuiser si on la coupe plus vite qu'elle ne repousse.

use crate::map::{Tile, TileMap};
use crate::math::TilePos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forest {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Forest {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Forest {
        Forest { x, y, w, h }
    }

    pub fn contains(&self, tile: TilePos) -> bool {
        (self.x..self.x + self.w).contains(&tile.x) && (self.y..self.y + self.h).contains(&tile.y)
    }

    /// Les arbres actuellement debout dans la forêt, dans l'ordre de lecture.
    pub fn trees(&self, map: &TileMap) -> Vec<TilePos> {
        let mut trees = Vec::new();
        for y in self.y..self.y + self.h {
            for x in self.x..self.x + self.w {
                let p = TilePos::new(x, y);
                if map.get(p) == Tile::Tree {
                    trees.push(p);
                }
            }
        }
        trees
    }

    /// Remplit la forêt d'arbres sur l'herbe, en laissant un couloir libre toutes les trois colonnes :
    /// chaque arbre touche ainsi un couloir et les bûcherons peuvent circuler.
    pub fn plant(&self, map: &mut TileMap) {
        for y in self.y..self.y + self.h {
            for x in self.x..self.x + self.w {
                let p = TilePos::new(x, y);
                if (x - self.x) % 3 != 0 && map.get(p) == Tile::Grass {
                    map.set(p, Tile::Tree);
                }
            }
        }
    }
}

/// Une cabane : le lit du bûcheron (dans le bâtiment) et la forêt qu'il exploite.
#[derive(Clone, Copy, Debug)]
pub struct LodgeSpec {
    /// Indice de la forêt dans la liste des forêts.
    pub forest: usize,
    pub bed: TilePos,
}

/// Les pousses en attente de redevenir des arbres.
#[derive(Default)]
pub struct Saplings {
    /// Tuile et heures de jeu restantes avant la repousse.
    pending: Vec<(TilePos, f32)>,
}

impl Saplings {
    pub fn new() -> Saplings {
        Saplings::default()
    }

    /// Un arbre vient d'être abattu : sa case devient une pousse.
    pub fn plant(&mut self, map: &mut TileMap, tile: TilePos, regrow_hours: f32) {
        map.set(tile, Tile::Sapling);
        self.pending.push((tile, regrow_hours));
    }

    pub fn count(&self) -> usize {
        self.pending.len()
    }

    /// Fait avancer le temps. Une pousse dont le délai est écoulé redevient un arbre, sauf si
    /// `is_occupied` dit qu'un personnage s'y tient : elle attend alors que la case soit libre.
    pub fn tick(&mut self, dt_hours: f32, map: &mut TileMap, is_occupied: impl Fn(TilePos) -> bool) {
        for (tile, left) in &mut self.pending {
            *left = (*left - dt_hours).max(0.0);
            if *left == 0.0 && !is_occupied(*tile) {
                map.set(*tile, Tile::Tree);
            }
        }
        self.pending.retain(|&(tile, _)| map.get(tile) == Tile::Sapling);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grass() -> TileMap {
        TileMap::new(20, 12, Tile::Grass)
    }

    #[test]
    fn a_forest_keeps_a_free_corridor_every_third_column() {
        let mut map = grass();
        let forest = Forest::new(2, 2, 9, 5);
        forest.plant(&mut map);
        for y in 2..7 {
            for x in [2, 5, 8] {
                assert_ne!(map.get(TilePos::new(x, y)), Tile::Tree, "couloir bouché en {x},{y}");
            }
        }
        assert_eq!(forest.trees(&map).len(), 6 * 5);
        assert!(forest.contains(TilePos::new(2, 2)) && !forest.contains(TilePos::new(11, 2)));
    }

    #[test]
    fn a_sapling_regrows_after_its_delay_and_never_before() {
        let mut map = grass();
        let mut saplings = Saplings::new();
        let tile = TilePos::new(3, 3);
        saplings.plant(&mut map, tile, 10.0);
        assert_eq!(map.get(tile), Tile::Sapling);
        assert!(!map.get(tile).is_solid());
        saplings.tick(9.0, &mut map, |_| false);
        assert_eq!(map.get(tile), Tile::Sapling);
        saplings.tick(1.0, &mut map, |_| false);
        assert_eq!(map.get(tile), Tile::Tree);
        assert_eq!(saplings.count(), 0);
    }

    #[test]
    fn an_occupied_sapling_waits_without_trapping_anyone() {
        let mut map = grass();
        let mut saplings = Saplings::new();
        let tile = TilePos::new(3, 3);
        saplings.plant(&mut map, tile, 1.0);
        saplings.tick(5.0, &mut map, |t| t == tile);
        assert_eq!(map.get(tile), Tile::Sapling, "quelqu'un est dessus");
        assert_eq!(saplings.count(), 1);
        saplings.tick(0.1, &mut map, |_| false);
        assert_eq!(map.get(tile), Tile::Tree);
    }
}
