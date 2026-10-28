//! Types mathématiques minimaux, propres au cœur de jeu.
//! Toutes les coordonnées du monde sont exprimées en **tuiles** (pas en pixels) :
//! la taille d'affichage d'une tuile est l'affaire du rendu.

use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(self, other: Vec2) -> f32 {
        (self - other).length()
    }

    pub fn normalized_or_zero(self) -> Vec2 {
        let len = self.length();
        if len > 1e-6 {
            self * (1.0 / len)
        } else {
            Vec2::ZERO
        }
    }

    /// Tuile contenant ce point.
    pub fn tile(self) -> TilePos {
        TilePos::new(self.x.floor() as i32, self.y.floor() as i32)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x + o.x, self.y + o.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f32) -> Vec2 {
        Vec2::new(self.x * k, self.y * k)
    }
}

/// Coordonnées entières d'une tuile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    pub const fn new(x: i32, y: i32) -> Self {
        TilePos { x, y }
    }

    /// Centre de la tuile, en coordonnées monde.
    pub fn center(self) -> Vec2 {
        Vec2::new(self.x as f32 + 0.5, self.y as f32 + 0.5)
    }
}

/// Un quart de tour dans le sens des aiguilles d'une montre, à partir de la pose de base d'un meuble ou d'un
/// bâtiment : empreinte horizontale (2×1 pour une pièce de deux tuiles), tête de lit à gauche, façade d'étagère
/// tournée vers le bas (le sud). Tout ce qui se pose dans le jeu peut se tourner de la même façon.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    pub const ALL: [Rotation; 4] = [Rotation::Deg0, Rotation::Deg90, Rotation::Deg180, Rotation::Deg270];

    /// Un quart de tour de plus, dans le sens des aiguilles d'une montre.
    pub fn clockwise(self) -> Rotation {
        match self {
            Rotation::Deg0 => Rotation::Deg90,
            Rotation::Deg90 => Rotation::Deg180,
            Rotation::Deg180 => Rotation::Deg270,
            Rotation::Deg270 => Rotation::Deg0,
        }
    }

    /// Le nombre de quarts de tour (0 à 3).
    pub fn quarter_turns(self) -> u8 {
        match self {
            Rotation::Deg0 => 0,
            Rotation::Deg90 => 1,
            Rotation::Deg180 => 2,
            Rotation::Deg270 => 3,
        }
    }

    /// Largeur et hauteur d'une pièce de base `(w, h)` une fois tournée : les deux échangent aux quarts de tour impairs.
    pub fn footprint(self, (w, h): (i32, i32)) -> (i32, i32) {
        if self.quarter_turns() % 2 == 1 {
            (h, w)
        } else {
            (w, h)
        }
    }
}

#[cfg(test)]
mod rotation_tests {
    use super::*;

    #[test]
    fn four_quarter_turns_make_a_full_turn_and_two_tile_pieces_swap_their_footprint() {
        let mut r = Rotation::Deg0;
        for _ in 0..4 {
            r = r.clockwise();
        }
        assert_eq!(r, Rotation::Deg0);
        assert_eq!(Rotation::Deg0.footprint((2, 1)), (2, 1));
        assert_eq!(Rotation::Deg90.footprint((2, 1)), (1, 2));
        assert_eq!(Rotation::Deg180.footprint((2, 1)), (2, 1));
        assert_eq!(Rotation::Deg270.footprint((2, 1)), (1, 2));
        assert_eq!(Rotation::Deg90.footprint((1, 1)), (1, 1));
    }
}
