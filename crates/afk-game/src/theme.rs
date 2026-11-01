//! Le thème médiéval de l'interface : bois sombre, parchemin, filet d'or, rivets de fer. Tout est dessiné avec des
//! rectangles et des cercles (pas de police ni d'image à charger) ; les textes restent en ASCII, comme le reste du HUD.

use macroquad::prelude::*;

pub const WOOD_DARK: Color = Color::new(0.20, 0.12, 0.07, 1.0);
pub const WOOD: Color = Color::new(0.34, 0.21, 0.11, 1.0);
pub const WOOD_LIGHT: Color = Color::new(0.47, 0.31, 0.17, 1.0);
pub const PARCHMENT: Color = Color::new(0.91, 0.82, 0.62, 1.0);
pub const PARCHMENT_DARK: Color = Color::new(0.80, 0.69, 0.47, 1.0);
pub const PARCHMENT_LIGHT: Color = Color::new(0.96, 0.89, 0.72, 1.0);
pub const GOLD: Color = Color::new(0.80, 0.63, 0.16, 1.0);
pub const GOLD_LIGHT: Color = Color::new(0.95, 0.82, 0.38, 1.0);
pub const INK: Color = Color::new(0.21, 0.13, 0.08, 1.0);
pub const INK_SOFT: Color = Color::new(0.38, 0.27, 0.16, 1.0);
pub const IRON: Color = Color::new(0.30, 0.30, 0.34, 1.0);
/// La zone d'un plan que le royaume peut payer.
pub const PLAN_OK: Color = Color::new(0.27, 0.55, 0.92, 1.0);
/// La zone d'un plan que le royaume ne peut pas payer (encore).
pub const PLAN_SHORT: Color = Color::new(0.86, 0.26, 0.22, 1.0);
/// La zone d'un plan qu'on ne peut pas poser là.
pub const PLAN_BLOCKED: Color = Color::new(0.55, 0.52, 0.48, 1.0);

pub fn with_alpha(c: Color, a: f32) -> Color {
    Color::new(c.r, c.g, c.b, a)
}

/// Le texte du thème : encre sombre sur parchemin, avec un léger relief.
pub fn ink(text: &str, x: f32, y: f32, size: f32) {
    draw_text(text, x + 1.0, y + 1.0, size, with_alpha(PARCHMENT_LIGHT, 0.7));
    draw_text(text, x, y, size, INK);
}

/// Texte clair sur bois sombre.
pub fn gilt(text: &str, x: f32, y: f32, size: f32) {
    draw_text(text, x + 1.0, y + 2.0, size, with_alpha(BLACK, 0.8));
    draw_text(text, x, y, size, GOLD_LIGHT);
}

/// Un rivet de fer : un petit disque avec un reflet.
fn rivet(x: f32, y: f32, r: f32) {
    draw_circle(x, y, r + 1.0, with_alpha(BLACK, 0.6));
    draw_circle(x, y, r, IRON);
    draw_circle(x - r * 0.3, y - r * 0.3, r * 0.4, Color::new(0.7, 0.7, 0.74, 1.0));
}

/// Un cadre de bois à filet d'or, rempli de parchemin : le fond de tous les panneaux.
pub fn panel(x: f32, y: f32, w: f32, h: f32) {
    // l'ombre portée, puis le cadre de bois avec ses veines
    draw_rectangle(x + 4.0, y + 5.0, w, h, with_alpha(BLACK, 0.35));
    draw_rectangle(x, y, w, h, WOOD_DARK);
    draw_rectangle(x + 2.0, y + 2.0, w - 4.0, h - 4.0, WOOD);
    for i in 0..((h / 9.0) as i32) {
        let ly = y + 6.0 + i as f32 * 9.0;
        draw_line(x + 3.0, ly, x + w - 3.0, ly + if i % 2 == 0 { 1.0 } else { -1.0 }, 1.0, with_alpha(WOOD_DARK, 0.35));
    }
    // le filet d'or, puis le parchemin
    draw_rectangle_lines(x + 6.0, y + 6.0, w - 12.0, h - 12.0, 2.0, GOLD);
    draw_rectangle(x + 9.0, y + 9.0, w - 18.0, h - 18.0, PARCHMENT);
    // un peu de patine sur le parchemin : des bords plus sombres
    draw_rectangle(x + 9.0, y + h - 22.0, w - 18.0, 13.0, with_alpha(PARCHMENT_DARK, 0.45));
    draw_rectangle(x + 9.0, y + 9.0, w - 18.0, 6.0, with_alpha(PARCHMENT_LIGHT, 0.5));
    for &(cx, cy) in &[(x + 6.0, y + 6.0), (x + w - 6.0, y + 6.0), (x + 6.0, y + h - 6.0), (x + w - 6.0, y + h - 6.0)] {
        rivet(cx, cy, 3.5);
    }
}

/// Un bandeau de titre : une planche sombre qui déborde du panneau, titre doré, fanions aux deux bouts.
pub fn ribbon(x: f32, y: f32, w: f32, title: &str) {
    let h = 30.0;
    draw_rectangle(x - 8.0, y + 2.0, w + 16.0, h, with_alpha(BLACK, 0.4));
    draw_rectangle(x - 8.0, y, w + 16.0, h, WOOD_DARK);
    draw_rectangle(x - 6.0, y + 2.0, w + 12.0, h - 4.0, WOOD_LIGHT);
    draw_rectangle_lines(x - 6.0, y + 2.0, w + 12.0, h - 4.0, 2.0, GOLD);
    // les extrémités en queue d'aronde
    for &(ex, dir) in &[(x - 8.0, 1.0_f32), (x + w + 8.0, -1.0_f32)] {
        draw_triangle(vec2(ex, y), vec2(ex, y + h), vec2(ex + dir * 9.0, y + h / 2.0), WOOD_DARK);
    }
    let size = 24.0;
    let tw = measure_text(title, None, size as u16, 1.0).width;
    gilt(title, x + (w - tw) / 2.0, y + 22.0, size);
}

/// Un bouton de parchemin cerclé de bois ; `hot` quand la souris le survole, `down` quand il est choisi.
pub fn button(x: f32, y: f32, w: f32, h: f32, hot: bool, down: bool) {
    draw_rectangle(x, y + 2.0, w, h, with_alpha(BLACK, 0.3));
    draw_rectangle(x, y, w, h, if down { GOLD } else { WOOD });
    let fill = if down { PARCHMENT_LIGHT } else if hot { PARCHMENT_LIGHT } else { PARCHMENT };
    draw_rectangle(x + 2.0, y + 2.0, w - 4.0, h - 4.0, fill);
    draw_rectangle_lines(x + 2.0, y + 2.0, w - 4.0, h - 4.0, 1.0, if hot || down { GOLD } else { PARCHMENT_DARK });
}

/// Une planche de bois à filet d'or, sans parchemin : le fond des pastilles et des compteurs du HUD (texte doré dessus).
pub fn plank(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x + 3.0, y + 4.0, w, h, with_alpha(BLACK, 0.35));
    draw_rectangle(x, y, w, h, WOOD_DARK);
    draw_rectangle(x + 2.0, y + 2.0, w - 4.0, h - 4.0, WOOD);
    draw_rectangle(x + 2.0, y + 2.0, w - 4.0, 3.0, with_alpha(WOOD_LIGHT, 0.7));
    draw_rectangle_lines(x + 4.0, y + 4.0, w - 8.0, h - 8.0, 1.0, with_alpha(GOLD, 0.8));
}

/// Une touche de clavier : un capuchon de parchemin sur une tranche sombre, la lettre au centre.
pub fn keycap(x: f32, y: f32, size: f32, label: &str) {
    draw_rectangle(x, y + 2.0, size, size, INK);
    draw_rectangle(x + 1.0, y, size - 2.0, size - 2.0, PARCHMENT_LIGHT);
    draw_rectangle_lines(x + 1.0, y, size - 2.0, size - 2.0, 1.0, PARCHMENT_DARK);
    let text_size = size * 0.72;
    let w = measure_text(label, None, text_size as u16, 1.0).width;
    draw_text(label, x + (size - w) / 2.0, y + (size - 2.0) / 2.0 + text_size * 0.32, text_size, INK);
}

