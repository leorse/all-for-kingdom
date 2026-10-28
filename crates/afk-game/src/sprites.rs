//! Les sprites : des images vectorielles dessinées dans `assets/blacksmith/` (voir `build.sh`), rendues
//! une fois en atlas PNG et embarquées dans le binaire. Chaque image est une cellule de 64 x 80
//! pixels logiques (le PNG est à 2x pour rester net) ; à l'écran, la cellule est mise à l'échelle comme
//! tout le reste (32 px par tuile = échelle 1).
//!
//! Dans l'atlas, un personnage debout est ancré entre ses pieds ; le lit et le dormeur sont ancrés au
//! centre de la tuile. Le sprite dépasse de sa tuile : la vue est inclinée, comme dans Zelda sur SNES.
//! Seul le forgeron a des sprites pour l'instant ; les autres métiers gardent leurs formes simples.

use afk_core::{Activity, Character, Facing, WorkKind};
use macroquad::prelude::*;

const CELL_W: f32 = 64.0;
const CELL_H: f32 = 80.0;
/// L'atlas est rendu à cette échelle (pixels de texture par pixel logique).
const ATLAS_SCALE: f32 = 2.0;
/// Entre les pieds d'un personnage debout, dans sa cellule.
const FEET: (f32, f32) = (32.0, 58.0);
/// Centre de la tuile, pour le lit et le dormeur.
const BED_CENTER: (f32, f32) = (32.0, 40.0);
/// Les pieds sont dessinés sous le centre du personnage (en pixels à l'échelle 1).
const FEET_BELOW_CENTER: f32 = 10.0;

// lignes de l'atlas
const ROW_WALK_DOWN: usize = 0;
const ROW_WALK_UP: usize = 1;
const ROW_WALK_SIDE: usize = 2;
const ROW_IDLE: usize = 3;
const ROW_FORGE: usize = 4;
const ROW_BED: usize = 5;

const WALK_FRAMES: f32 = 6.0;
const FORGE_FRAMES: f32 = 4.0;

pub struct Sprites {
    blacksmith: Texture2D,
}

impl Sprites {
    pub fn load() -> Sprites {
        let png = include_bytes!("../assets/blacksmith/blacksmith.png");
        let blacksmith = Texture2D::from_file_with_format(png, Some(ImageFormat::Png));
        blacksmith.set_filter(FilterMode::Linear);
        Sprites { blacksmith }
    }

    /// Dessine la cellule (`col`, `row`) de façon que son point d'ancrage tombe en `at`.
    fn draw_cell(&self, col: usize, row: usize, anchor: (f32, f32), at: (f32, f32), s: f32, flip: bool) {
        let (cw, ch) = (CELL_W * ATLAS_SCALE, CELL_H * ATLAS_SCALE);
        draw_texture_ex(
            &self.blacksmith,
            at.0 - anchor.0 * s,
            at.1 - anchor.1 * s,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(CELL_W * s, CELL_H * s)),
                source: Some(Rect::new(col as f32 * cw, row as f32 * ch, cw, ch)),
                flip_x: flip,
                ..Default::default()
            },
        );
    }

    /// Un lit, centré sur la tuile dont le centre est en `center` (pixels écran).
    pub fn draw_bed(&self, center: (f32, f32), s: f32) {
        self.draw_cell(0, ROW_BED, BED_CENTER, center, s, false);
    }

    /// Le forgeron : marche, forge, dort ou attend, selon son état. `at` : sa position à l'écran.
    /// `by_player` : un joueur le dirige (il marche plus vite, donc la cadence des pas aussi).
    /// Renvoie la hauteur (en pixels à l'échelle 1) au-dessus de ses pieds où se trouve le haut de sa tête,
    /// pour poser un marqueur ou une barre au-dessus.
    pub fn draw_blacksmith(&self, c: &Character, at: (f32, f32), s: f32, time: f32, by_player: bool) -> f32 {
        let feet = (at.0, at.1 + FEET_BELOW_CENTER * s);
        let t = time + c.id.0 as f32 * 0.37;

        if c.activity == Activity::Sleeping {
            let breath = (time * 0.7) as usize % 2;
            self.draw_cell(1 + breath, ROW_BED, BED_CENTER, at, s, false);
            return 0.0;
        }

        if matches!(c.work.map(|w| w.kind), Some(WorkKind::Forge)) {
            let frame = (t * 5.0) as usize % FORGE_FRAMES as usize;
            self.draw_cell(frame, ROW_FORGE, FEET, feet, s, false);
            return 48.0;
        }

        let flip = c.facing == Facing::Right;
        if c.activity == Activity::Walking {
            let fps = if by_player { 9.0 } else { 5.0 };
            let frame = (t * fps) as usize % WALK_FRAMES as usize;
            let row = match c.facing {
                Facing::Down => ROW_WALK_DOWN,
                Facing::Up => ROW_WALK_UP,
                Facing::Left | Facing::Right => ROW_WALK_SIDE,
            };
            self.draw_cell(frame, row, FEET, feet, s, flip);
        } else {
            let col = match c.facing {
                Facing::Down => 0,
                Facing::Up => 1,
                Facing::Left | Facing::Right => 2,
            };
            self.draw_cell(col, ROW_IDLE, FEET, feet, s, flip);
        }
        48.0
    }
}
