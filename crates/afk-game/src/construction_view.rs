//! L'aspect du camp de base et des chantiers : le feu de camp et les couchages, puis les fondations, l'échafaudage qui
//! (les planches qui recouvrent la zone avec l'avancement), et l'étiquette de chaque chantier. Les bâtiments finis sont dessinés par `habitat_view`.

use crate::render::View;
use crate::theme;
use afk_core::{Game, PlanKind, Resource, Site, SiteState, Tile, TilePos};
use macroquad::prelude::*;

/// Le nom d'un type de plan, en ASCII (la police par défaut ne gère pas les accents).
pub fn plan_name(kind: PlanKind) -> &'static str {
    match kind {
        PlanKind::House => "Maison",
        PlanKind::Warehouse => "Entrepot",
        PlanKind::Tavern => "Taverne",
        PlanKind::Forge => "Forge",
    }
}

/// Le feu de camp : un cercle de pierres, deux bûches croisées et des flammes qui dansent.
pub fn draw_campfire(game: &Game, view: &View) {
    let s = view.tile_px / 32.0;
    let t = game.time();
    for tile in game.map().find_tiles(Tile::Campfire) {
        if !view.point_visible(tile.center(), 2.0) {
            continue;
        }
        let (px, py) = view.tile_origin(tile);
        // les pierres
        for i in 0..8 {
            let a = i as f32 / 8.0 * std::f32::consts::TAU;
            let (cx, cy) = (px + 16.0 * s + a.cos() * 11.0 * s, py + 19.0 * s + a.sin() * 7.0 * s);
            draw_rectangle(cx - 3.0 * s, cy - 2.0 * s, 6.0 * s, 4.0 * s, Color::from_rgba(112, 108, 112, 255));
            draw_rectangle(cx - 3.0 * s, cy - 2.0 * s, 6.0 * s, 1.5 * s, Color::from_rgba(150, 146, 150, 255));
        }
        draw_rectangle(px + 7.0 * s, py + 16.0 * s, 18.0 * s, 8.0 * s, Color::from_rgba(40, 28, 22, 255)); // les braises
        // deux bûches croisées
        draw_line(px + 8.0 * s, py + 23.0 * s, px + 24.0 * s, py + 17.0 * s, 4.0 * s, Color::from_rgba(96, 62, 34, 255));
        draw_line(px + 8.0 * s, py + 17.0 * s, px + 24.0 * s, py + 23.0 * s, 4.0 * s, Color::from_rgba(116, 78, 44, 255));
        // les flammes : trois langues qui oscillent
        for (i, (dx, base_h)) in [(-5.0_f32, 9.0_f32), (0.0, 14.0), (5.0, 8.0)].into_iter().enumerate() {
            let flick = ((t * 9.0 + i as f32 * 2.1).sin() * 0.5 + 0.5) * 4.0;
            let h = (base_h + flick) * s;
            let cx = px + 16.0 * s + dx * s;
            draw_triangle(
                vec2(cx - 4.5 * s, py + 21.0 * s),
                vec2(cx + 4.5 * s, py + 21.0 * s),
                vec2(cx + ((t * 7.0 + i as f32).sin()) * 1.5 * s, py + 21.0 * s - h),
                Color::from_rgba(240, 120, 30, 255),
            );
            draw_triangle(
                vec2(cx - 2.5 * s, py + 21.0 * s),
                vec2(cx + 2.5 * s, py + 21.0 * s),
                vec2(cx, py + 21.0 * s - h * 0.65),
                Color::from_rgba(255, 214, 80, 255),
            );
        }
    }
}

/// La lueur du feu de camp sur le sol, à dessiner après la lumière du jour pour qu'elle reste visible la nuit.
pub fn draw_campfire_light(game: &Game, view: &View) {
    let dark = 1.0 - game.clock().daylight();
    let pulse = 0.85 + 0.15 * (game.time() * 5.0).sin();
    for tile in game.map().find_tiles(Tile::Campfire) {
        if !view.point_visible(tile.center(), 6.0) {
            continue;
        }
        let c = tile.center();
        let (x, y) = view.to_screen(c);
        let r = view.tile_px * 4.2;
        for (k, a) in [(1.0_f32, 0.05_f32), (0.72, 0.07), (0.45, 0.09), (0.22, 0.12)] {
            draw_circle(x, y, r * k, Color::new(1.0, 0.6, 0.2, (a + 0.16 * dark) * pulse));
        }
    }
}

/// Un couchage du camp : une paillasse, une couverture roulée et un oreiller, posés à même le sol.
pub fn draw_bedroll(view: &View, tile: TilePos) {
    let s = view.tile_px / 32.0;
    let (px, py) = view.tile_origin(tile);
    draw_rectangle(px + 2.0 * s, py + 20.0 * s, 28.0 * s, 4.0 * s, Color::from_rgba(40, 60, 40, 120)); // ombre
    draw_rectangle(px + 3.0 * s, py + 9.0 * s, 26.0 * s, 14.0 * s, Color::from_rgba(200, 176, 110, 255)); // paillasse
    draw_rectangle(px + 3.0 * s, py + 9.0 * s, 26.0 * s, 2.0 * s, Color::from_rgba(226, 204, 140, 255));
    draw_rectangle(px + 11.0 * s, py + 10.0 * s, 17.0 * s, 12.0 * s, Color::from_rgba(150, 62, 56, 255)); // couverture
    draw_rectangle(px + 11.0 * s, py + 10.0 * s, 17.0 * s, 2.0 * s, Color::from_rgba(190, 96, 84, 255));
    draw_rectangle(px + 5.0 * s, py + 11.0 * s, 8.0 * s, 9.0 * s, Color::from_rgba(236, 232, 218, 255)); // oreiller
}

/// Une couleur de bois ou de pierre pour les piquets et les murs en construction.
const PLANK: Color = Color::new(0.55, 0.38, 0.21, 1.0);
const PLANK_DARK: Color = Color::new(0.36, 0.24, 0.13, 1.0);
const STONE: Color = Color::new(0.62, 0.60, 0.58, 1.0);

/// Les chantiers : terre damée et bordure de pierre pour les fondations, échafaudage et murs qui montent avec
/// l'avancement, étiquette de l'état au-dessus.
pub fn draw_sites(game: &Game, view: &View) {
    for site in game.sites().iter().filter(|s| s.state != SiteState::Done) {
        let center = site.center().center();
        if !view.point_visible(center, site.w.max(site.h) as f32) {
            continue;
        }
        draw_foundation(game, view, site);
        draw_site_label(game, view, site);
    }
}

fn draw_foundation(game: &Game, view: &View, site: &Site) {
    let t = view.tile_px;
    let s = t / 32.0;
    let (px, py) = view.tile_origin(TilePos::new(site.x, site.y));
    let (w, h) = (site.w as f32 * t, site.h as f32 * t);
    // la terre damée, avec des planches posées en travers
    draw_rectangle(px, py, w, h, Color::from_rgba(122, 96, 66, 255));
    for ty in 0..site.h {
        for tx in 0..site.w {
            let shade = if (tx + ty) % 2 == 0 { 0.0 } else { 0.04 };
            draw_rectangle(px + tx as f32 * t, py + ty as f32 * t, t, t, Color::new(0.0, 0.0, 0.0, shade));
        }
    }
    for ty in (1..site.h).step_by(2) {
        draw_rectangle(px, py + ty as f32 * t - 1.0, w, 2.0, Color::from_rgba(92, 70, 46, 150));
    }
    // la bordure de pierre de la fondation
    let b = 4.0 * s;
    draw_rectangle(px, py, w, b, STONE);
    draw_rectangle(px, py + h - b, w, b, STONE);
    draw_rectangle(px, py, b, h, STONE);
    draw_rectangle(px + w - b, py, b, h, STONE);

    let p = site.progress.clamp(0.0, 1.0);
    match site.state {
        SiteState::Waiting | SiteState::Delivering => {
            // des piquets aux angles et un cordeau : le terrain est tracé, rien ne bouge encore
            for &(cx, cy) in &[(px, py), (px + w, py), (px, py + h), (px + w, py + h)] {
                draw_rectangle(cx - 2.0 * s, cy - 10.0 * s, 4.0 * s, 14.0 * s, PLANK);
                draw_rectangle(cx - 2.0 * s, cy - 10.0 * s, 4.0 * s, 3.0 * s, theme::GOLD_LIGHT);
            }
            let dash = 8.0 * s;
            let mut x = px;
            while x < px + w {
                draw_line(x, py - 4.0 * s, (x + dash * 0.6).min(px + w), py - 4.0 * s, 1.5, Color::from_rgba(240, 232, 200, 220));
                x += dash;
            }
        }
        SiteState::Building => {
            // tout reste dans la zone : les planches recouvrent la fondation de la gauche vers la droite
            let inner = (px + b, py + b, w - 2.0 * b, h - 2.0 * b);
            let wall_w = inner.2 * p;
            draw_rectangle(inner.0, inner.1, wall_w, inner.3, with_alpha_c(PLANK, 0.95));
            for i in 1..(wall_w / (6.0 * s)) as i32 {
                let x = inner.0 + i as f32 * 6.0 * s;
                draw_line(x, inner.1, x, inner.1 + inner.3, 1.0, PLANK_DARK);
            }
            // l'arête de travail : une poutre à la limite des planches posées
            if p < 1.0 {
                draw_rectangle(inner.0 + wall_w - 1.5 * s, inner.1, 3.0 * s, inner.3, theme::GOLD_LIGHT);
            }
            if p > 0.85 {
                // le toit de chaume se pose en dernier : une bande sur le faîte
                let roof_h = (inner.3 * 0.3).max(4.0 * s);
                let y = inner.1 + inner.3 / 2.0 - roof_h / 2.0;
                draw_rectangle(inner.0, y, wall_w, roof_h, Color::from_rgba(196, 164, 84, 255));
                draw_rectangle(inner.0, y, wall_w, 2.5 * s, Color::from_rgba(224, 196, 112, 255));
            }
        }
        SiteState::Done => {}
    }
    let _ = game;
}

fn with_alpha_c(c: Color, a: f32) -> Color {
    Color::new(c.r, c.g, c.b, a)
}

/// L'enseigne du chantier : son nom, son état, et une barre d'avancement pendant la construction.
fn draw_site_label(game: &Game, view: &View, site: &Site) {
    let s = view.tile_px / 32.0;
    let (px, py) = view.tile_origin(TilePos::new(site.x, site.y));
    let w = site.w as f32 * view.tile_px;
    // ce qui manque, en pictogrammes : un « ! » puis chaque ressource et la quantité
    let missing: Vec<(Resource, u32, bool)> = if site.state == SiteState::Waiting { missing(game, site) } else { Vec::new() };
    let state = match site.state {
        SiteState::Waiting if missing.is_empty() => "En attente : libre de poser le tas".to_string(),
        SiteState::Waiting => String::new(),
        SiteState::Delivering => "Livraison des materiaux".to_string(),
        SiteState::Building => format!("Construction {:.0}%", site.progress * 100.0),
        SiteState::Done => String::new(),
    };
    let title = plan_name(site.kind);
    let size = (15.0 * s).clamp(12.0, 22.0);
    let missing_w = if missing.is_empty() { 0.0 } else { 24.0 + crate::hud_icons::cost_width(&missing, size) };
    let tw = measure_text(&state, None, size as u16, 1.0).width.max(measure_text(title, None, size as u16, 1.0).width).max(missing_w);
    let (bw, bh) = (tw + 16.0, if missing.is_empty() { size * 2.4 } else { size + 32.0 });
    let bx = px + w / 2.0 - bw / 2.0;
    let by = py - bh - 20.0 * s;
    draw_rectangle(bx, by, bw, bh, theme::WOOD_DARK);
    draw_rectangle(bx + 2.0, by + 2.0, bw - 4.0, bh - 4.0, theme::PARCHMENT);
    draw_rectangle_lines(bx + 2.0, by + 2.0, bw - 4.0, bh - 4.0, 1.0, theme::GOLD);
    theme::ink(title, bx + 8.0, by + size + 1.0, size);
    let color = if site.state == SiteState::Waiting { Color::new(0.62, 0.16, 0.12, 1.0) } else { theme::INK_SOFT };
    draw_text(&state, bx + 8.0, by + size * 2.0 + 1.0, size, color);
    if !missing.is_empty() {
        let center = by + size + 1.0 + 4.0 + 10.0;
        crate::hud_icons::draw(crate::hud_icons::Icon::Bang, bx + 8.0, center - 10.0, 2.0, 1.0);
        crate::hud_icons::draw_cost(bx + 8.0 + 24.0, center, &missing, size, color);
    }
    if site.state == SiteState::Building {
        let (barw, barh) = (bw - 16.0, 5.0 * s.max(1.0));
        draw_rectangle(bx + 8.0, by + bh + 2.0, barw, barh, theme::INK);
        draw_rectangle(bx + 9.0, by + bh + 3.0, (barw - 2.0) * site.progress.clamp(0.0, 1.0), barh - 2.0, theme::GOLD_LIGHT);
    }
}

/// Ce qui manque à un plan qui attend : le bois et la pierre qu'on n'a pas (le booléen de `draw_cost` reste faux : toute
/// la ligne est déjà un manque).
fn missing(game: &Game, site: &Site) -> Vec<(Resource, u32, bool)> {
    [Resource::Wood, Resource::Stone]
        .into_iter()
        .filter_map(|resource| {
            let (need, have) = (site.cost.get(resource), game.available(resource));
            (have < need).then(|| (resource, need - have, false))
        })
        .collect()
}
