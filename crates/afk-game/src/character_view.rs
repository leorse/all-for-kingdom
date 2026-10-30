//! L'affichage des personnages dessinés : leur planche (embarquée dans le binaire), l'image à montrer et sa place à
//! l'écran, pieds sur la position du personnage. Le gabarit (cellules, animations) est dans `character_sprites`.

use crate::character_sprites::{self as cs, Frame, SheetSpec, CELL};
use crate::render::View;
use afk_core::{Activity, Character, Role};
use macroquad::prelude::*;

/// Le point de la cellule de sommeil posé sur la case de la tête du lit : le centre de la tuile du haut.
const SLEEP_ANCHOR: (f32, f32) = (32.0, 16.0);

const FORGERON_PNG: &[u8] = include_bytes!("../assets/characters/forgeron.png");

/// Une planche chargée en texture.
pub struct Sheet {
    pub spec: SheetSpec,
    texture: Texture2D,
}

/// Les planches des métiers déjà dessinés.
pub struct Sheets {
    pub blacksmith: Sheet,
}

impl Sheets {
    pub fn new() -> Sheets {
        Sheets { blacksmith: Sheet::load(cs::BLACKSMITH, FORGERON_PNG) }
    }

    /// La planche d'un métier, s'il en a une ; sinon il garde son dessin d'avant.
    pub fn for_role(&self, role: Role) -> Option<&Sheet> {
        match role {
            Role::Blacksmith => Some(&self.blacksmith),
            _ => None,
        }
    }
}

impl Sheet {
    fn load(spec: SheetSpec, png: &[u8]) -> Sheet {
        let texture = Texture2D::from_file_with_format(png, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Nearest); // jamais de lissage : les pixels restent nets
        Sheet { spec, texture }
    }

    /// Dessine une cellule, le point `anchor` de la cellule posé en `at` (pixels écran), tournée de `rotation` radians
    /// autour de ce point (des quarts de tour seulement, pour rester net).
    fn draw_cell(&self, frame: Frame, at: (f32, f32), anchor: (f32, f32), scale: f32, rotation: f32) {
        let (ax, ay) = (at.0.round(), at.1.round());
        let size = CELL as f32;
        draw_texture_ex(
            &self.texture,
            ax - anchor.0 * scale,
            ay - anchor.1 * scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size * scale, size * scale)),
                source: Some(Rect::new(frame.col as f32 * size, frame.row as f32 * size, size, size)),
                rotation,
                pivot: Some(vec2(ax, ay)),
                ..Default::default()
            },
        );
    }

    /// Dessine le personnage, pieds sur sa position. Endormi, il est couché dans son lit : `sleep_dir` est la direction
    /// de sa tête vers ses pieds (le pied du lit).
    pub fn draw_character(&self, c: &Character, view: &View, t: f32, sleep_dir: (f32, f32)) {
        let scale = scale(view);
        let at = view.to_screen(c.pos);
        let frame = cs::frame_for(&self.spec, c.activity, c.facing, c.work.is_some(), t, c.id.0);
        if c.activity == Activity::Sleeping {
            self.draw_cell(frame, at, SLEEP_ANCHOR, scale, sleep_rotation(sleep_dir));
            return;
        }
        // l'ombre au sol, sous les pieds
        draw_ellipse(at.0, at.1 + 1.0 * scale, 9.0 * scale, 2.5 * scale, 0.0, Color::new(0.0, 0.0, 0.0, 0.22));
        self.draw_cell(frame, at, cs::ANCHOR, scale, 0.0);
    }

    /// Dessine un objet fixe de la planche (l'enclume...), ancré comme un personnage sur le point `center`.
    pub fn draw_extra(&self, name: &str, center: afk_core::Vec2, view: &View) {
        if let Some(frame) = self.spec.extra(name) {
            self.draw_cell(frame, view.to_screen(center), cs::ANCHOR, scale(view), 0.0);
        }
    }
}

/// L'échelle d'affichage des dessins sur la tuile de base de 32 pixels (un entier).
pub fn scale(view: &View) -> f32 {
    view.tile_px / crate::camera::BASE_TILE_PX
}

/// La rotation (en radians, sens horaire à l'écran) qui amène le dormeur dessiné tête en haut, pieds en bas, dans la
/// direction `(fx, fy)` de sa tête vers ses pieds.
fn sleep_rotation((fx, fy): (f32, f32)) -> f32 {
    use std::f32::consts::{FRAC_PI_2, PI};
    if fx > 0.5 {
        -FRAC_PI_2
    } else if fx < -0.5 {
        FRAC_PI_2
    } else if fy < -0.5 {
        PI
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprites::PALETTE;

    #[test]
    fn the_embedded_sheet_has_the_layout_size_palette_colours_and_no_empty_cell() {
        let image = Image::from_file_with_format(FORGERON_PNG, Some(ImageFormat::Png)).expect("PNG lisible");
        let (w, h) = cs::BLACKSMITH.size();
        assert_eq!((image.width as u32, image.height as u32), (w, h));
        for (x, y) in (0..h).flat_map(|y| (0..w).map(move |x| (x, y))) {
            let i = ((y * w + x) * 4) as usize;
            let p = &image.bytes[i..i + 4];
            let ok = p[3] == 0 || (p[3] == 255 && PALETTE.iter().any(|(_, c)| c[..] == *p));
            assert!(ok, "pixel ({x}, {y}) {p:?} hors palette");
        }
        for (id, col, row) in cs::BLACKSMITH.cells() {
            let filled = (0..CELL).any(|y| {
                (0..CELL).any(|x| {
                    let i = (((row * CELL + y) * w + col * CELL + x) * 4) as usize;
                    image.bytes[i + 3] != 0
                })
            });
            assert!(filled, "l'image « {id} » est vide");
        }
    }

    #[test]
    fn a_sleeper_is_turned_with_his_bed() {
        use std::f32::consts::{FRAC_PI_2, PI};
        assert_eq!(sleep_rotation((0.0, 1.0)), 0.0); // dessiné tête en haut
        assert_eq!(sleep_rotation((0.0, -1.0)), PI);
        // couché vers la droite (lit de base) : un quart de tour dans le sens inverse des aiguilles d'une montre
        assert_eq!(sleep_rotation((1.0, 0.0)), -FRAC_PI_2);
        assert_eq!(sleep_rotation((-1.0, 0.0)), FRAC_PI_2);
    }
}
