//! L'aspect des habitats : la maison vue en coupe, comme sur l'image de référence (`docs/images/image_maison.jpg`).
//!
//! Toit de chaume et mur de rondins au nord (avec fenêtres et cheminée de pierre), murs de bois sur les côtés, mur
//! sud bas avec la porte, sol en planches, paillasson, puis les meubles dans leur orientation. La lueur du feu et
//! des fenêtres est dessinée après la lumière du jour. La logique de placement du décor est pure et testée.

use crate::render::View;
use crate::sprites::{self, Sprite, Turns};
use afk_core::{Game, HabitatClass, HabitatSpec, Rotation, TilePos};
use macroquad::prelude::*;

/// Où poser le décor du mur nord d'un habitat : la colonne de la cheminée et celles des fenêtres.
#[derive(Debug, PartialEq, Eq)]
pub struct WallDecor {
    pub fireplace: i32,
    pub windows: Vec<i32>,
}

/// Le nombre de fenêtres d'une classe d'habitat.
pub fn window_count(class: HabitatClass) -> usize {
    match class {
        HabitatClass::Small => 1,
        HabitatClass::Medium => 2,
        HabitatClass::Large => 3,
    }
}

/// Place la cheminée et les fenêtres sur le mur nord : jamais sur la porte, jamais l'une sur l'autre, la cheminée au
/// plus près du milieu avec un foyer libre devant elle (ni meuble, ni lit, ni entrée), les fenêtres aussi loin que
/// possible de la porte, de la cheminée et les unes des autres. C'est un décor du mur : il ne prend aucune tuile de sol.
pub fn wall_decor(h: &HabitatSpec) -> WallDecor {
    let span: Vec<i32> = (h.x + 1..h.x + h.width() - 1).collect();
    let door_x = (h.door.y == h.y).then_some(h.door.x);
    let candidates: Vec<i32> = span.iter().copied().filter(|&x| Some(x) != door_x).collect();
    let solid = h.solid_tiles();
    let beds = h.bed_tiles();
    let hearth_free = |x: i32| {
        let below = TilePos::new(x, h.y + 1);
        !solid.contains(&below) && !beds.contains(&below) && below != h.entrance()
    };
    let middle = h.x + h.width() / 2;
    let by_middle = |a: &i32, b: &i32| (a - middle).abs().cmp(&(b - middle).abs()).then(a.cmp(b));
    let mut free: Vec<i32> = candidates.iter().copied().filter(|&x| hearth_free(x)).collect();
    free.sort_by(by_middle);
    let fireplace = free.first().copied().unwrap_or_else(|| {
        let mut any = candidates.clone();
        any.sort_by(by_middle);
        any[0]
    });
    let mut taken: Vec<i32> = vec![fireplace];
    if let Some(d) = door_x {
        taken.push(d);
    }
    let mut windows = Vec::new();
    for _ in 0..window_count(h.class) {
        let pick = |spaced: bool| {
            candidates
                .iter()
                .copied()
                .filter(|x| !windows.contains(x) && *x != fireplace)
                // deux fenêtres ne se touchent pas, tant qu'il reste de la place pour les écarter
                .filter(|x| !spaced || windows.iter().all(|w| (w - x).abs() >= 2))
                .max_by_key(|&x| (taken.iter().map(|t| (t - x).abs()).min().unwrap_or(0), x))
        };
        let best = pick(true).or_else(|| pick(false));
        match best {
            Some(x) => {
                windows.push(x);
                taken.push(x);
            }
            None => break,
        }
    }
    windows.sort();
    WallDecor { fireplace, windows }
}

/// Les sprites des habitats : construits une fois au lancement (ils ont besoin de la fenêtre).
pub struct Art {
    floors: Vec<Sprite>,
    wall_north: Sprite,
    wall_north_door: Sprite,
    wall_south: Sprite,
    wall_south_door: Vec<Sprite>,
    wall_west: Sprite,
    wall_east: Sprite,
    side_door_west: Vec<Sprite>,
    side_door_east: Vec<Sprite>,
    window_day: Sprite,
    window_lit: Sprite,
    fireplace: Vec<Sprite>,
    doormat: Sprite,
    bed: Turns,
    table: Turns,
    shelf: Turns,
    trough: Turns,
}

impl Art {
    pub fn new() -> Art {
        let many = |f: &dyn Fn(usize) -> sprites::PixelSprite, n: usize| (0..n).map(|v| Sprite::new(&f(v))).collect::<Vec<_>>();
        Art {
            floors: many(&sprites::floor, 4),
            wall_north: Sprite::new(&sprites::wall_north()),
            wall_north_door: Sprite::new(&sprites::wall_north_door()),
            wall_south: Sprite::new(&sprites::wall_south()),
            wall_south_door: many(&sprites::wall_south_door, 4),
            wall_west: Sprite::new(&sprites::wall_side(false)),
            wall_east: Sprite::new(&sprites::wall_side(true)),
            side_door_west: many(&|v| sprites::wall_side_door(false, v), 4),
            side_door_east: many(&|v| sprites::wall_side_door(true, v), 4),
            window_day: Sprite::new(&sprites::window(false)),
            window_lit: Sprite::new(&sprites::window(true)),
            fireplace: many(&sprites::fireplace, 3),
            doormat: Sprite::new(&sprites::doormat()),
            // le lit est dessiné tête en haut : c'est la rotation de 90°
            bed: Turns::new(&sprites::bed_head_up(), 1),
            table: Turns::new(&sprites::table(), 0),
            shelf: Turns::new(&sprites::shelf(), 0),
            trough: Turns::new(&sprites::trough(), 0),
        }
    }
}

/// Cette tuile appartient-elle à un habitat (murs compris) ? Le dessin des tuiles la laisse alors à `draw_habitats`.
pub fn is_habitat_tile(game: &Game, tile: TilePos) -> bool {
    game.habitats().iter().any(|h| h.covers(tile))
}

/// L'échelle d'affichage : la taille d'une tuile à l'écran divisée par la tuile de base de 16 pixels (un entier).
fn scale(view: &View) -> f32 {
    view.tile_px / crate::camera::BASE_TILE_PX
}

/// L'une des quatre variantes de sol, choisie par la position pour que les planches ne se répètent pas.
fn floor_variant(t: TilePos) -> usize {
    ((t.x * 7 + t.y * 13 + (t.x / 2) * (t.y / 3)).rem_euclid(4)) as usize
}

fn quarter_turns(r: Rotation) -> u8 {
    r.quarter_turns()
}

/// Dessine tous les habitats visibles : sol, murs, décor mural, paillasson, puis meubles.
pub fn draw_habitats(game: &Game, view: &View, art: &Art) {
    let s = scale(view);
    let night = game.clock().daylight() < 0.5;
    let frame = ((game.time() * 6.0) as usize) % 3;
    for h in game.habitats() {
        let center = TilePos::new(h.x + h.width() / 2, h.y + h.height() / 2).center();
        if !view.point_visible(center, h.width().max(h.height()) as f32) {
            continue;
        }
        let (x0, y0, x1, y1) = (h.x, h.y, h.x + h.width() - 1, h.y + h.height() - 1);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let t = TilePos::new(x, y);
                let (px, py) = view.tile_origin(t);
                let v = floor_variant(t);
                let is_door = t == h.door;
                let sprite = if y == y0 {
                    if is_door { &art.wall_north_door } else { &art.wall_north }
                } else if y == y1 {
                    if is_door { &art.wall_south_door[v] } else { &art.wall_south }
                } else if x == x0 {
                    if is_door { &art.side_door_west[v] } else { &art.wall_west }
                } else if x == x1 {
                    if is_door { &art.side_door_east[v] } else { &art.wall_east }
                } else {
                    &art.floors[v]
                };
                sprite.draw(px, py, s);
            }
        }
        // le décor du mur nord : fenêtres, puis cheminée dont le conduit dépasse du toit
        let decor = wall_decor(h);
        for &wx in &decor.windows {
            let (px, py) = view.tile_origin(TilePos::new(wx, y0));
            let window = if night { &art.window_lit } else { &art.window_day };
            window.draw(px + 4.0 * s, py + 6.0 * s, s);
        }
        let (fx, fy) = view.tile_origin(TilePos::new(decor.fireplace, y0));
        art.fireplace[frame].draw(fx, fy - 4.0 * s, s);
        // le paillasson derrière la porte, puis les meubles
        let (mx, my) = view.tile_origin(h.entrance());
        art.doormat.draw(mx, my, s);
        for bed in &h.beds {
            let (bx, by) = view.tile_origin(bed.anchor);
            art.bed.get(quarter_turns(bed.rotation)).draw(bx, by, s);
        }
        for (piece, turns) in [(&h.table, &art.table), (&h.shelf, &art.shelf), (&h.trough, &art.trough)] {
            let (px, py) = view.tile_origin(piece.anchor);
            turns.get(quarter_turns(piece.rotation)).draw(px, py, s);
        }
    }
}

/// La lueur du feu et des fenêtres, à dessiner après la lumière du jour pour qu'elle reste visible la nuit.
pub fn draw_habitat_lights(game: &Game, view: &View) {
    let day = game.clock().daylight();
    let dark = 1.0 - day;
    let pulse = 0.85 + 0.15 * (game.time() * 5.0).sin();
    for h in game.habitats() {
        let center = TilePos::new(h.x + h.width() / 2, h.y + h.height() / 2).center();
        if !view.point_visible(center, h.width().max(h.height()) as f32) {
            continue;
        }
        let decor = wall_decor(h);
        let t = view.tile_px;
        // le feu éclaire le sol devant la cheminée, un peu même en plein jour
        let glow = |tx: i32, ty: i32, alpha: f32| {
            let (px, py) = view.tile_origin(TilePos::new(tx, ty));
            draw_rectangle(px, py, t, t, Color::new(1.0, 0.62, 0.2, alpha));
        };
        let base = (0.05 + 0.13 * dark) * pulse;
        glow(decor.fireplace, h.y + 1, base * 1.6);
        glow(decor.fireplace - 1, h.y + 1, base);
        glow(decor.fireplace + 1, h.y + 1, base);
        glow(decor.fireplace, h.y + 2, base * 0.8);
        // les fenêtres éclairées la nuit laissent tomber une lueur sur le sol
        if dark > 0.3 {
            for &wx in &decor.windows {
                glow(wx, h.y + 1, 0.12 * dark);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use afk_core::Furniture;
    use Rotation::{Deg0, Deg90};

    /// Une pièce à l'image : étagère au nord, lits verticaux à l'est, table au centre, baquet au sud-est.
    fn house(class: HabitatClass, door: (i32, i32)) -> HabitatSpec {
        let (x, y) = (10, 10);
        let beds = match class {
            HabitatClass::Small => vec![Furniture::new(x + 5, y + 1, Deg90)],
            HabitatClass::Medium => vec![
                Furniture::new(x + 5, y + 1, Deg90),
                Furniture::new(x + 6, y + 1, Deg90),
                Furniture::new(x + 7, y + 1, Deg90),
            ],
            HabitatClass::Large => vec![
                Furniture::new(x + 5, y + 1, Deg90),
                Furniture::new(x + 6, y + 1, Deg90),
                Furniture::new(x + 7, y + 1, Deg90),
                Furniture::new(x + 5, y + 4, Deg90),
                Furniture::new(x + 6, y + 4, Deg90),
                Furniture::new(x + 7, y + 4, Deg90),
            ],
        };
        let trough = match class {
            HabitatClass::Small => Furniture::new(x + 5, y + 3, Deg0),
            HabitatClass::Medium => Furniture::new(x + 7, y + 3, Deg0),
            HabitatClass::Large => Furniture::new(x + 1, y + 5, Deg0),
        };
        let table_row = if class == HabitatClass::Large { 3 } else { 2 };
        HabitatSpec::new(x, y, class, door, &beds, Furniture::new(x + 2, y + table_row, Deg0), Furniture::new(x + 1, y + 1, Deg0), trough)
    }

    fn check(h: &HabitatSpec) -> WallDecor {
        assert_eq!(h.validate(), Ok(()), "{:?}", h);
        let d = wall_decor(h);
        let span = h.x + 1..h.x + h.width() - 1;
        assert!(span.contains(&d.fireplace), "la cheminée est dans le mur nord");
        for w in &d.windows {
            assert!(span.contains(w), "fenêtre {w} hors du mur");
            assert_ne!(*w, d.fireplace, "une fenêtre sur la cheminée");
        }
        if h.door.y == h.y {
            assert_ne!(d.fireplace, h.door.x, "la cheminée est sur la porte");
            assert!(!d.windows.contains(&h.door.x), "une fenêtre est sur la porte");
        }
        let mut sorted = d.windows.clone();
        sorted.dedup();
        assert_eq!(sorted.len(), d.windows.len(), "deux fenêtres au même endroit");
        d
    }

    #[test]
    fn each_class_gets_one_two_or_three_windows_and_a_fireplace() {
        for (class, expected) in [(HabitatClass::Small, 1), (HabitatClass::Medium, 2), (HabitatClass::Large, 3)] {
            let h = house(class, (14, 10 + class.size().1 - 1)); // porte au sud
            let d = check(&h);
            assert_eq!(d.windows.len(), expected, "{class:?}");
        }
    }

    #[test]
    fn the_fireplace_never_sits_on_a_door_in_the_north_wall_and_has_a_free_hearth() {
        for door_x in 13..=15 { // une porte au-dessus de l'étagère boucherait l'entrée
            let h = house(HabitatClass::Small, (door_x, 10));
            let d = check(&h);
            let below = TilePos::new(d.fireplace, h.y + 1);
            assert!(!h.solid_tiles().contains(&below), "le foyer est bouché par un meuble (porte en {door_x})");
            assert!(!h.bed_tiles().contains(&below), "le foyer est sur un lit (porte en {door_x})");
        }
    }

    #[test]
    fn the_wall_decor_takes_no_floor_tile() {
        // le décor est une pure fonction de la colonne : il ne modifie ni les meubles ni la place libre
        let h = house(HabitatClass::Medium, (19, 12));
        let before = (h.solid_tiles(), h.bed_tiles());
        let _ = wall_decor(&h);
        assert_eq!((h.solid_tiles(), h.bed_tiles()), before);
    }

    #[test]
    fn windows_are_spread_far_from_the_fireplace_and_from_each_other() {
        let h = house(HabitatClass::Large, (13, 16));
        let d = check(&h);
        for pair in d.windows.windows(2) {
            assert!(pair[1] - pair[0] >= 2, "fenêtres trop proches : {:?}", d.windows);
        }
    }

    #[test]
    fn floor_variants_are_always_in_range() {
        for y in 0..50 {
            for x in 0..50 {
                assert!(floor_variant(TilePos::new(x, y)) < 4);
            }
        }
    }

    #[test]
    fn the_scale_is_a_whole_number_at_every_zoom_level() {
        for z in crate::camera::ZOOM_LEVELS {
            let s = z / crate::camera::BASE_TILE_PX;
            assert_eq!(s.fract(), 0.0);
        }
    }

    // ------------------------------------------------------------ aperçu (hors test normal)

    /// Un tampon RGBA pour composer un aperçu sans fenêtre.
    struct Preview {
        w: usize,
        h: usize,
        rgba: Vec<u8>,
    }

    impl Preview {
        fn blit(&mut self, sprite: &sprites::PixelSprite, x: i32, y: i32, scale: i32) {
            for sy in 0..sprite.h {
                for sx in 0..sprite.w {
                    let c = sprites::color_of(sprite.at(sx, sy)).unwrap();
                    if c[3] == 0 {
                        continue;
                    }
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let (px, py) = (x + sx as i32 * scale + dx, y + sy as i32 * scale + dy);
                            if px >= 0 && py >= 0 && (px as usize) < self.w && (py as usize) < self.h {
                                let i = (py as usize * self.w + px as usize) * 4;
                                self.rgba[i..i + 4].copy_from_slice(&c);
                            }
                        }
                    }
                }
            }
        }

        fn write_bmp(&self, path: &str) {
            let row = (self.w * 3 + 3) / 4 * 4;
            let size = 54 + row * self.h;
            let mut out = Vec::with_capacity(size);
            out.extend_from_slice(b"BM");
            out.extend_from_slice(&(size as u32).to_le_bytes());
            out.extend_from_slice(&[0, 0, 0, 0, 54, 0, 0, 0, 40, 0, 0, 0]);
            out.extend_from_slice(&(self.w as i32).to_le_bytes());
            out.extend_from_slice(&(self.h as i32).to_le_bytes());
            out.extend_from_slice(&[1, 0, 24, 0, 0, 0, 0, 0]);
            out.extend_from_slice(&((row * self.h) as u32).to_le_bytes());
            out.extend_from_slice(&[0; 16]);
            for y in (0..self.h).rev() {
                for x in 0..self.w {
                    let i = (y * self.w + x) * 4;
                    out.extend_from_slice(&[self.rgba[i + 2], self.rgba[i + 1], self.rgba[i]]);
                }
                out.extend(std::iter::repeat(0).take(row - self.w * 3));
            }
            std::fs::write(path, out).unwrap();
        }
    }

    /// Compose une maison comme le fait `draw_habitats`, avec des sprites en mémoire.
    fn compose(preview: &mut Preview, h: &HabitatSpec, ox: i32, oy: i32, scale: i32, night: bool) {
        let t = 16 * scale;
        let at = |x: i32, y: i32| (ox + (x - h.x) * t, oy + (y - h.y) * t);
        let floor = |x: i32, y: i32| sprites::floor((((x * 7 + y * 13 + (x / 2) * (y / 3)).rem_euclid(4)) as usize));
        let (x1, y1) = (h.x + h.width() - 1, h.y + h.height() - 1);
        for y in h.y..=y1 {
            for x in h.x..=x1 {
                let (px, py) = at(x, y);
                let door = TilePos::new(x, y) == h.door;
                let v = (x * 7 + y * 13 + (x / 2) * (y / 3)).rem_euclid(4) as usize;
                let sprite = if y == h.y {
                    if door { sprites::wall_north_door() } else { sprites::wall_north() }
                } else if y == y1 {
                    if door { sprites::wall_south_door(v) } else { sprites::wall_south() }
                } else if x == h.x {
                    if door { sprites::wall_side_door(false, v) } else { sprites::wall_side(false) }
                } else if x == x1 {
                    if door { sprites::wall_side_door(true, v) } else { sprites::wall_side(true) }
                } else {
                    floor(x, y)
                };
                preview.blit(&sprite, px, py, scale);
            }
        }
        let decor = wall_decor(h);
        for &wx in &decor.windows {
            let (px, py) = at(wx, h.y);
            preview.blit(&sprites::window(night), px + 4 * scale, py + 6 * scale, scale);
        }
        let (fx, fy) = at(decor.fireplace, h.y);
        preview.blit(&sprites::fireplace(1), fx, fy - 4 * scale, scale);
        let (mx, my) = at(h.entrance().x, h.entrance().y);
        preview.blit(&sprites::doormat(), mx, my, scale);
        for bed in &h.beds {
            let (bx, by) = at(bed.anchor.x, bed.anchor.y);
            let turns = (bed.rotation.quarter_turns() + 4 - 1) % 4;
            preview.blit(&sprites::bed_head_up().rotated(turns), bx, by, scale);
        }
        for (piece, sprite) in [(&h.table, sprites::table()), (&h.shelf, sprites::shelf()), (&h.trough, sprites::trough())] {
            let (px, py) = at(piece.anchor.x, piece.anchor.y);
            preview.blit(&sprite.rotated(piece.rotation.quarter_turns()), px, py, scale);
        }
    }

    /// Écrit un aperçu BMP des maisons du village dans le fichier donné par `PREVIEW_OUT` :
    /// `PREVIEW_OUT=... cargo test -p afk-game preview_houses -- --ignored`.
    #[test]
    #[ignore]
    fn preview_houses() {
        let Ok(path) = std::env::var("PREVIEW_OUT") else { return };
        let scenario = afk_core::Scenario::village_with_farmers(4);
        let scale = 4;
        let mut preview = Preview { w: 1500, h: 700, rgba: vec![0; 1500 * 700 * 4] };
        for px in preview.rgba.chunks_mut(4) {
            px.copy_from_slice(&[40, 28, 22, 255]);
        }
        // la grande maison du paysan, une petite maison (forgeron), une moyenne (caserne) ; une petite la nuit
        compose(&mut preview, &scenario.habitats[0], 10, 10, scale, false);
        compose(&mut preview, &scenario.habitats[1], 10, 460, scale / 2 * 2 / 2 * 2, true);
        compose(&mut preview, &scenario.habitats[3], 560, 10, scale / 2 * 2, false);
        preview.write_bmp(&path);
    }
}
