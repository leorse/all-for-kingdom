//! Tout l'affichage. Ce fichier lit l'état de `afk-core` et le dessine : il ne le modifie jamais.
//! C'est ici (et dans `input.rs`) que vivent les choix visuels : couleurs, sprites, tailles, lumières...
//! Changer de bibliothèque de rendu = réécrire ce fichier, pas la logique du jeu.

use crate::camera::{step_zoom, zoom_center, Camera, DEFAULT_ZOOM, ZOOM_LEVELS};
use crate::character_view::Sheets;
use afk_core::{Activity, Character, Game, OreKind, PlayerId, PlotState, Resource, Role, Tile, TilePos, Vec2, WorkKind};
use macroquad::prelude::*;

/// Passage coordonnées monde (tuiles) <-> écran (pixels). Le zoom est l'un des paliers de
/// `ZOOM_LEVELS` ; la caméra décide quelle partie de la carte est à l'écran.
pub struct View {
    pub tile_px: f32,
    zoom_level: usize,
    origin: (f32, f32),
    screen: (f32, f32),
    /// Taille de la carte en tuiles, connue dès la première mise à jour.
    map_size: Vec2,
    camera: Camera,
}

impl View {
    pub fn new() -> View {
        View {
            tile_px: ZOOM_LEVELS[DEFAULT_ZOOM],
            zoom_level: DEFAULT_ZOOM,
            origin: (0.0, 0.0),
            screen: (0.0, 0.0),
            map_size: Vec2::ZERO,
            camera: Camera::new(),
        }
    }

    /// À appeler une fois par image, avant de dessiner. `target` : le personnage suivi ;
    /// `recenter` : on vient d'en incarner un autre, la caméra glisse jusqu'à lui.
    pub fn update(&mut self, game: &Game, target: Option<Vec2>, recenter: bool, dt: f32) {
        self.screen = (screen_width(), screen_height());
        let map = Vec2::new(game.map().width() as f32, game.map().height() as f32);
        self.map_size = map;
        let view = self.view_size();
        self.camera.update(target.unwrap_or(self.camera.center()), view, map, recenter, dt);
        self.refresh_origin();
    }

    /// Recalcule la position à l'écran de l'origine de la carte depuis le centre de la caméra et le zoom.
    fn refresh_origin(&mut self) {
        let c = self.camera.center();
        self.origin = (
            (self.screen.0 / 2.0 - c.x * self.tile_px).floor(),
            (self.screen.1 / 2.0 - c.y * self.tile_px).floor(),
        );
    }

    /// La vue est libre : le joueur l'a déplacée à la souris, la caméra ne suit plus le personnage.
    pub fn is_free(&self) -> bool {
        self.camera.is_free()
    }

    /// Taille de la zone visible, en tuiles.
    fn view_size(&self) -> Vec2 {
        Vec2::new(self.screen.0 / self.tile_px, self.screen.1 / self.tile_px)
    }

    /// Le joueur a tiré la carte de `dx`, `dy` pixels (bouton droit maintenu) : la carte suit le curseur.
    pub fn pan_pixels(&mut self, dx: f32, dy: f32) {
        if self.map_size == Vec2::ZERO {
            return; // pas encore de première image
        }
        let delta = Vec2::new(-dx / self.tile_px, -dy / self.tile_px);
        self.camera.pan(delta, self.view_size(), self.map_size);
        self.refresh_origin();
    }

    /// Zoome de `steps` paliers (positif : on agrandit) autour du point de l'écran `cursor`, qui garde
    /// le même point de la carte dessous. Ne déclenche pas la vue libre.
    pub fn zoom(&mut self, steps: i32, cursor: (f32, f32)) {
        let level = step_zoom(self.zoom_level, steps);
        if level == self.zoom_level || self.map_size == Vec2::ZERO {
            return;
        }
        let offset = Vec2::new(cursor.0 - self.screen.0 / 2.0, cursor.1 - self.screen.1 / 2.0);
        let new_px = ZOOM_LEVELS[level];
        let center = zoom_center(self.camera.center(), offset, self.tile_px, new_px);
        self.zoom_level = level;
        self.tile_px = new_px;
        self.camera.set_center(center, self.view_size(), self.map_size);
        self.refresh_origin();
    }

    pub fn to_screen(&self, p: Vec2) -> (f32, f32) {
        (self.origin.0 + p.x * self.tile_px, self.origin.1 + p.y * self.tile_px)
    }

    pub fn to_world(&self, screen: (f32, f32)) -> Vec2 {
        Vec2::new(
            (screen.0 - self.origin.0) / self.tile_px,
            (screen.1 - self.origin.1) / self.tile_px,
        )
    }

    /// Coin haut-gauche d'une tuile, en pixels.
    pub(crate) fn tile_origin(&self, t: TilePos) -> (f32, f32) {
        self.to_screen(Vec2::new(t.x as f32, t.y as f32))
    }

    /// Plage de tuiles (x0..x1, y0..y1) qui touchent l'écran, bornée à la carte.
    fn visible_tiles(&self, map_w: i32, map_h: i32) -> (i32, i32, i32, i32) {
        let tl = self.to_world((0.0, 0.0));
        let br = self.to_world(self.screen);
        (
            (tl.x.floor() as i32).max(0),
            (br.x.ceil() as i32).min(map_w),
            (tl.y.floor() as i32).max(0),
            (br.y.ceil() as i32).min(map_h),
        )
    }

    /// Ce point du monde est-il (à peu près) à l'écran ? `margin` en tuiles, pour les éléments qui débordent.
    pub(crate) fn point_visible(&self, p: Vec2, margin: f32) -> bool {
        let (x, y) = self.to_screen(p);
        let m = margin * self.tile_px;
        x >= -m && y >= -m && x <= self.screen.0 + m && y <= self.screen.1 + m
    }
}

// ---------------------------------------------------------------- apparence

pub(crate) fn tile_color(tile: Tile, x: i32, y: i32) -> Color {
    // petite variation déterministe pour éviter l'aplat monotone
    let v = ((x * 7 + y * 13) % 5) as u8 * 3;
    match tile {
        Tile::Grass => Color::from_rgba(88 + v, 140 + v, 70, 255),
        Tile::Path => Color::from_rgba(176 + v, 150 + v, 108, 255),
        Tile::Water => Color::from_rgba(52, 96 + v, 168, 255),
        Tile::Field => Color::from_rgba(130 + v, 96, 58, 255),
        Tile::Wall => Color::from_rgba(96, 90, 100, 255),
        Tile::Floor | Tile::Bed | Tile::Anvil => Color::from_rgba(150 + v, 120 + v, 90, 255),
        Tile::Tree => Color::from_rgba(40, 98 + v, 52, 255),
        Tile::Ore => Color::from_rgba(120, 118, 124, 255), // recouvert par `draw_veins`, qui connaît la nature
        Tile::Sapling => Color::from_rgba(88 + v, 140 + v, 70, 255), // de l'herbe, la tige est dessinée par `draw_map`
        Tile::Crate => Color::from_rgba(88 + v, 140 + v, 70, 255), // de l'herbe, le coffre est dessiné par `draw_map`
        Tile::Campfire => Color::from_rgba(176 + v, 150 + v, 108, 255), // la terre battue du camp, le feu est dessiné à part
        Tile::Foundation => Color::from_rgba(122, 96, 66, 255), // la terre damée, dessinée à part par `construction_view`
        Tile::Furniture => Color::from_rgba(150 + v, 120 + v, 90, 255), // le sol ; les meubles des habitats sont dessinés par `habitat_view`
    }
}

pub(crate) fn role_color(role: Role) -> Color {
    match role {
        Role::Farmer => Color::from_rgba(214, 186, 76, 255),
        Role::Blacksmith => Color::from_rgba(220, 110, 50, 255),
        Role::King => Color::from_rgba(150, 60, 190, 255),
        Role::Warlord => Color::from_rgba(200, 50, 50, 255),
        Role::Innkeeper => Color::from_rgba(70, 170, 190, 255),
        Role::Guard => Color::from_rgba(112, 126, 170, 255),
        Role::Miner => Color::from_rgba(150, 104, 66, 255),
        Role::Lumberjack => Color::from_rgba(70, 150, 90, 255),
        Role::Transporter => Color::from_rgba(60, 190, 190, 255),
        Role::Artisan => Color::from_rgba(188, 134, 70, 255),
        Role::Vagabond => Color::from_rgba(132, 120, 108, 255),
        Role::Villager => Color::from_rgba(160, 150, 130, 255),
    }
}

/// La couleur d'une ressource : sur le bloc que porte un transporteur ou un bûcheron.
pub(crate) fn resource_color(resource: Resource) -> Color {
    match resource {
        Resource::Wheat => Color::from_rgba(232, 200, 70, 255),
        Resource::Seeds => Color::from_rgba(196, 160, 92, 255),
        Resource::Wood => Color::from_rgba(140, 92, 50, 255),
        Resource::Iron => Color::from_rgba(132, 138, 150, 255),
        Resource::Copper => Color::from_rgba(188, 108, 56, 255),
        Resource::Gold => Color::from_rgba(228, 188, 52, 255),
        Resource::Stone => Color::from_rgba(196, 190, 170, 255),
        Resource::Tools => Color::from_rgba(110, 150, 200, 255),
    }
}

/// Vert quand on est en forme, orange quand on commence à fatiguer, rouge quand on est à bout.
pub(crate) fn fitness_color(fitness: f32) -> Color {
    if fitness > 50.0 {
        Color::from_rgba(90, 200, 90, 255)
    } else if fitness > 15.0 {
        Color::from_rgba(235, 165, 50, 255)
    } else {
        Color::from_rgba(215, 60, 60, 255)
    }
}

// ---------------------------------------------------------------- dessin

pub fn draw(
    game: &Game,
    view: &View,
    art: &crate::habitat_view::Art,
    sheets: &Sheets,
    plan_ui: &crate::plan_ui::PlanUi,
    assign_ui: &crate::assign_ui::AssignUi,
    me: PlayerId,
    speed: f32,
    help_open: bool,
) {
    clear_background(Color::from_rgba(20, 24, 30, 255));
    draw_map(game, view);
    draw_veins(game, view);
    draw_farm(game, view);
    crate::habitat_view::draw_habitats(game, view, art);
    crate::habitat_view::draw_buildings(game, view, art);
    crate::construction_view::draw_sites(game, view);
    crate::construction_view::draw_campfire(game, view);
    draw_target(game, view, me);
    draw_characters(game, view, sheets, me);
    draw_containers(game, view);
    draw_forge_sparks(game, view);
    draw_daylight(game, view);
    crate::habitat_view::draw_habitat_lights(game, view);
    crate::construction_view::draw_campfire_light(game, view);
    plan_ui.draw_world(game, view, me);
    assign_ui.draw_world(game, view, me);
    crate::hud::draw(game, sheets, me, speed);
    plan_ui.draw(game, view, me);
    assign_ui.draw(game, view, me);
    crate::hud::draw_help(help_open);
}

fn draw_map(game: &Game, view: &View) {
    let map = game.map();
    let t = view.tile_px;
    let s = t / 32.0;
    let (x0, x1, y0, y1) = view.visible_tiles(map.width(), map.height());
    for y in y0..y1 {
        for x in x0..x1 {
            let tile = map.get(TilePos::new(x, y));
            if crate::habitat_view::is_habitat_tile(game, TilePos::new(x, y)) {
                continue; // les habitats se dessinent à part (voir `habitat_view`)
            }
            let (px, py) = view.tile_origin(TilePos::new(x, y));
            // un lit au camp est posé sur la terre battue, pas sur un plancher
            let ground = if tile == Tile::Bed && game.camp().is_some() { Tile::Path } else { tile };
            draw_rectangle(px, py, t, t, tile_color(ground, x, y));
            match tile {
                Tile::Wall => draw_rectangle(px, py, t, 4.0 * s, Color::from_rgba(120, 114, 124, 255)),
                Tile::Crate => {
                    draw_rectangle(px + 4.0 * s, py + 22.0 * s, 24.0 * s, 5.0 * s, Color::from_rgba(40, 60, 40, 255)); // ombre
                    draw_rectangle(px + 4.0 * s, py + 8.0 * s, 24.0 * s, 17.0 * s, Color::from_rgba(120, 84, 48, 255)); // caisse
                    draw_rectangle(px + 4.0 * s, py + 8.0 * s, 24.0 * s, 4.0 * s, Color::from_rgba(160, 116, 70, 255)); // couvercle
                    draw_rectangle(px + 4.0 * s, py + 15.0 * s, 24.0 * s, 2.0 * s, Color::from_rgba(80, 56, 34, 255)); // planche
                    draw_rectangle(px + 14.0 * s, py + 8.0 * s, 4.0 * s, 17.0 * s, Color::from_rgba(70, 70, 78, 255)); // sangle
                }
                Tile::Sapling => {
                    draw_rectangle(px + 15.0 * s, py + 14.0 * s, 3.0 * s, 12.0 * s, Color::from_rgba(70, 120, 50, 255)); // tige
                    draw_rectangle(px + 10.0 * s, py + 10.0 * s, 6.0 * s, 5.0 * s, Color::from_rgba(120, 190, 80, 255)); // feuille gauche
                    draw_rectangle(px + 17.0 * s, py + 7.0 * s, 6.0 * s, 5.0 * s, Color::from_rgba(120, 190, 80, 255)); // feuille droite
                }
                Tile::Tree => draw_rectangle(px + 4.0 * s, py + 4.0 * s, 24.0 * s, 24.0 * s, Color::from_rgba(30, 78, 40, 255)),
                Tile::Field if (x + y) % 2 == 0 => {
                    draw_rectangle(px, py + 14.0 * s, t, 4.0 * s, Color::from_rgba(110, 80, 48, 255))
                }
                // un couchage du camp de base
                Tile::Bed if game.camp().is_some() => crate::construction_view::draw_bedroll(view, TilePos::new(x, y)),
                // le lit d'une seule tuile (celui du château)
                Tile::Bed => {
                    draw_rectangle(px + 3.0 * s, py + 6.0 * s, 26.0 * s, 20.0 * s, Color::from_rgba(104, 68, 40, 255)); // cadre
                    draw_rectangle(px + 5.0 * s, py + 8.0 * s, 22.0 * s, 16.0 * s, Color::from_rgba(196, 74, 74, 255)); // couverture
                    draw_rectangle(px + 5.0 * s, py + 8.0 * s, 7.0 * s, 16.0 * s, Color::from_rgba(236, 234, 222, 255)); // oreiller
                }
                _ => {}
            }
        }
    }
}

/// La couleur d'une nature de minerai : gris le fer, cuivré le cuivre, doré l'or, beige clair la pierre.
fn ore_color(kind: OreKind) -> Color {
    match kind {
        OreKind::Iron => Color::from_rgba(132, 138, 150, 255),
        OreKind::Copper => Color::from_rgba(188, 108, 56, 255),
        OreKind::Gold => Color::from_rgba(228, 188, 52, 255),
        OreKind::Stone => Color::from_rgba(196, 190, 170, 255),
    }
}

/// Les filons : une couleur par nature, et des points sombres sur chaque tuile, d'autant plus nombreux
/// que la roche est impure.
fn draw_veins(game: &Game, view: &View) {
    let t = view.tile_px;
    let s = t / 32.0;
    // positions des points dans une tuile, dans l'ordre où on les ajoute
    const DOTS: [(f32, f32); 3] = [(8.0, 9.0), (21.0, 17.0), (11.0, 23.0)];
    for vein in game.veins() {
        let base = ore_color(vein.kind);
        let (r, g, b) = (base.r, base.g, base.b);
        let shade = Color::new(r * 0.55, g * 0.55, b * 0.55, 1.0);
        let light = Color::new((r + 0.2).min(1.0), (g + 0.2).min(1.0), (b + 0.2).min(1.0), 1.0);
        for tile in vein.tiles() {
            if !view.point_visible(tile.center(), 1.0) {
                continue;
            }
            let (px, py) = view.tile_origin(tile);
            draw_rectangle(px, py, t, t, base);
            draw_rectangle(px, py, t, 3.0 * s, light); // arête haute
            draw_rectangle(px, py + t - 3.0 * s, t, 3.0 * s, shade); // arête basse
            for &(dx, dy) in DOTS.iter().take(vein.quality.dots() as usize) {
                draw_rectangle(px + dx * s, py + dy * s, 4.0 * s, 4.0 * s, shade);
            }
        }
    }
}

/// Les parcelles : terre retournée, pousses qui grandissent, épis dorés quand c'est mûr.
fn draw_farm(game: &Game, view: &View) {
    let t = view.tile_px;
    let s = t / 32.0;
    let growth_hours = game.config().crop_growth_hours;
    let soil = Color::from_rgba(92, 62, 38, 255);
    let furrow = Color::from_rgba(68, 44, 26, 255);

    for plot in game.farm().plots() {
        if plot.state == PlotState::Fallow || !view.point_visible(plot.tile.center(), 1.0) {
            continue; // l'aspect de base du champ suffit, et rien à dessiner hors écran
        }
        let (px, py) = view.tile_origin(plot.tile);
        draw_rectangle(px, py, t, t, soil);
        for row in 0..3 {
            draw_rectangle(px, py + (6.0 + row as f32 * 9.0) * s, t, 3.0 * s, furrow);
        }

        match plot.state {
            PlotState::Planted { .. } => {
                let h = (3.0 + 13.0 * plot.state.growth_fraction(growth_hours)) * s;
                for i in 0..3 {
                    draw_rectangle(px + (6.0 + 9.0 * i as f32) * s, py + 25.0 * s - h, 4.0 * s, h, Color::from_rgba(80, 175, 70, 255));
                }
            }
            PlotState::Ripe => {
                for i in 0..3 {
                    let x = px + (6.0 + 9.0 * i as f32) * s;
                    draw_rectangle(x, py + 9.0 * s, 4.0 * s, 16.0 * s, Color::from_rgba(214, 178, 62, 255));
                    draw_rectangle(x - 1.0 * s, py + 6.0 * s, 6.0 * s, 6.0 * s, Color::from_rgba(240, 206, 90, 255));
                }
            }
            _ => {}
        }
    }
}

/// Entoure la parcelle visée par la touche E (celle devant le personnage incarné, ou sous ses pieds).
fn draw_target(game: &Game, view: &View, me: PlayerId) {
    let Some(id) = game.possessed_by(me) else { return };
    let Some((_, tile)) = game.available_work(id) else { return };
    let (px, py) = view.tile_origin(tile);
    let pulse = 0.55 + 0.45 * (game.time() * 6.0).sin().abs();
    draw_rectangle_lines(px, py, view.tile_px, view.tile_px, 3.0, Color::new(1.0, 1.0, 1.0, pulse));
}

/// Vers où s'étend un dormeur depuis sa tête : le pied de son lit (à droite pour un lit de base, en bas pour un lit
/// vertical tête en haut). Hors habitat (le château), le lit est horizontal.
fn sleep_direction(game: &Game, c: &Character) -> (f32, f32) {
    use afk_core::Rotation;
    for h in game.habitats() {
        if let Some(bed) = h.beds.iter().find(|b| b.head() == c.home) {
            return match bed.rotation {
                Rotation::Deg0 => (1.0, 0.0),
                Rotation::Deg90 => (0.0, 1.0),
                Rotation::Deg180 => (-1.0, 0.0),
                Rotation::Deg270 => (0.0, -1.0),
            };
        }
    }
    (1.0, 0.0)
}

fn draw_characters(game: &Game, view: &View, sheets: &Sheets, me: PlayerId) {
    let s = view.tile_px / 32.0;
    let t = game.time();
    let possessed = game.possessed_by(me);

    // personnages et enclumes, triés par profondeur : ce qui est plus bas à l'écran passe devant. Une enclume compte à
    // la ligne du haut de son socle : ses pieds plus bas, un personnage passe devant ; plus haut, derrière
    enum Body<'a> {
        Character(&'a Character),
        Anvil(TilePos),
    }
    let mut bodies: Vec<(f32, Body)> = game.characters().iter().filter(|c| !c.gone).map(|c| (c.pos.y, Body::Character(c))).collect();
    bodies.extend(game.map().find_tiles(Tile::Anvil).into_iter().map(|a| (a.y as f32 + 0.5, Body::Anvil(a))));
    bodies.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    for (_, body) in &bodies {
        let c = match body {
            Body::Anvil(anvil) => {
                if view.point_visible(anvil.center(), 2.0) {
                    // 14 px sous le centre de la case : le bas du socle tombe au bas de la case
                    let at = anvil.center() + afk_core::Vec2::new(0.0, 14.0 / 32.0);
                    sheets.blacksmith.draw_extra("enclume", at, view);
                }
                continue;
            }
            Body::Character(c) => *c,
        };
        if !view.point_visible(c.pos, 2.0) {
            continue;
        }
        if let Some(sheet) = sheets.for_role(c.role) {
            sheet.draw_character(c, view, t, sleep_direction(game, c));
            continue;
        }
        let (x, y) = view.to_screen(c.pos);

        if c.activity == Activity::Sleeping {
            // couché dans le sens de son lit : la tête sur l'oreiller, le corps vers le pied
            let (fx, fy) = sleep_direction(game, c);
            let (bw, bh) = if fx != 0.0 { (16.0, 10.0) } else { (10.0, 16.0) };
            let (bx, by) = (x + fx * 4.0 * s, y + fy * 4.0 * s);
            draw_rectangle(bx - bw * s / 2.0, by - bh * s / 2.0, bw * s, bh * s, role_color(c.role));
            let (hx, hy) = (x - fx * 6.0 * s, y - fy * 6.0 * s);
            draw_rectangle(hx - 4.0 * s, hy - 4.0 * s, 8.0 * s, 8.0 * s, Color::from_rgba(240, 200, 160, 255));
            continue;
        }

        let work_bob = if c.work.is_some() { (t * 8.0).sin() * 1.5 * s } else { 0.0 };
        draw_rectangle(x - 10.0 * s, y + 8.0 * s, 20.0 * s, 6.0 * s, Color::from_rgba(40, 60, 40, 255)); // ombre
        if Some(c.id) == possessed {
            draw_rectangle(x - 11.0 * s, y - 14.0 * s, 22.0 * s, 26.0 * s, WHITE); // contour
        }
        draw_rectangle(x - 9.0 * s, y - 12.0 * s + work_bob, 18.0 * s, 22.0 * s, role_color(c.role)); // corps
        draw_rectangle(x - 5.0 * s, y - 17.0 * s + work_bob, 10.0 * s, 8.0 * s, Color::from_rgba(240, 200, 160, 255)); // tête
        if c.role == Role::Guard {
            draw_rectangle(x - 6.0 * s, y - 20.0 * s + work_bob, 12.0 * s, 5.0 * s, Color::from_rgba(160, 166, 184, 255)); // casque
        }
    }

    // par-dessus tout le monde : le « z » des dormeurs, ce qu'on porte, le marqueur du personnage incarné et la barre
    // de travail
    for c in game.characters().iter().filter(|c| !c.gone) {
        if !view.point_visible(c.pos, 2.0) {
            continue;
        }
        let (x, y) = view.to_screen(c.pos);
        let drawn = sheets.for_role(c.role).is_some();
        // le haut de la tête et le bas des pieds, à l'écran
        let (head, feet) = if drawn { (y - 54.0 * s, y + 4.0 * s) } else { (y - 17.0 * s, y + 16.0 * s) };
        let active = Some(c.id) == possessed;

        if c.activity == Activity::Sleeping {
            let float = (t * 2.0).sin() * 2.0 * s;
            draw_text("z", x + 6.0 * s, y - 12.0 * s + float, 22.0 * s, WHITE);
            if active {
                draw_rectangle(x - 4.0 * s, y - 28.0 * s, 8.0 * s, 8.0 * s, Color::from_rgba(255, 230, 40, 255));
            }
            continue;
        }

        // ce qu'il porte : un bloc de la couleur de la ressource dans les bras, la quantité au-dessus de la tête
        if let Some((resource, amount)) = c.cargo {
            let chest = if drawn { y - 24.0 * s } else { y - 3.0 * s };
            draw_rectangle(x - 7.0 * s, chest, 14.0 * s, 10.0 * s, Color::from_rgba(30, 30, 30, 255)); // contour
            draw_rectangle(x - 6.0 * s, chest + 1.0 * s, 12.0 * s, 8.0 * s, resource_color(resource));
            let size = (16.0 * s).clamp(12.0, 24.0);
            let label = amount.to_string();
            let w = measure_text(&label, None, size as u16, 1.0).width;
            draw_text(&label, x - w / 2.0 + 1.0, head - 4.0 * s + 1.0, size, BLACK);
            draw_text(&label, x - w / 2.0, head - 4.0 * s, size, WHITE);
        }
        if active {
            let bob = (t * 6.0).sin() * 3.0 * s;
            draw_rectangle(x - 4.0 * s, head - 19.0 * s + bob, 8.0 * s, 8.0 * s, Color::from_rgba(255, 230, 40, 255)); // marqueur
        }

        // barre de progression du travail, sous les pieds
        if let Some(work) = c.work {
            let (bw, bh) = (26.0 * s, 5.0 * s);
            draw_rectangle(x - bw / 2.0, feet, bw, bh, Color::from_rgba(20, 20, 20, 255));
            draw_rectangle(
                x - bw / 2.0 + 1.0,
                feet + 1.0,
                (bw - 2.0) * work.progress.clamp(0.0, 1.0),
                bh - 2.0,
                Color::from_rgba(250, 220, 80, 255),
            );
        }
    }
}

/// Le compteur de chaque coffre : une ligne par ressource, son pictogramme et « 12/200 », au-dessus du coffre.
/// Rouge quand la ressource est au plafond (le producteur est à l'arrêt, le transporteur n'y apporte plus rien).
fn draw_containers(game: &Game, view: &View) {
    let s = view.tile_px / 32.0;
    let size = (15.0 * s).clamp(11.0, 22.0);
    let icon_px = if s >= 2.0 { 3.0 } else { 2.0 };
    let icon = crate::hud_icons::GRID as f32 * icon_px;
    let line = icon.max(size) + 2.0;
    for container in game.containers() {
        if !view.point_visible(container.tile.center(), 3.0) {
            continue;
        }
        let (px, py) = view.tile_origin(container.tile);
        // le tas d'un chantier montre ce qu'il lui faut : « 12/28 »
        let shown: Vec<Resource> = if container.is_site() { container.needs().keys().copied().collect() } else { container.shown() };
        for (i, resource) in shown.iter().enumerate() {
            let count = container.get(*resource);
            let cap = if container.is_site() { container.needs().get(resource).copied().unwrap_or(0) } else { container.capacity() };
            let label = format!("{}/{}", count, cap);
            let w = icon + 3.0 + measure_text(&label, None, size as u16, 1.0).width;
            let x = px + view.tile_px / 2.0 - w / 2.0;
            let top = py + 6.0 * s - (shown.len() - i) as f32 * line;
            let full = count >= cap;
            // un fond sombre pour lire sur n'importe quel sol
            draw_rectangle(x - 2.0, top, w + 4.0, line - 1.0, Color::new(0.0, 0.0, 0.0, 0.55));
            crate::hud_icons::draw(crate::hud_icons::Icon::Res(*resource), x, top + (line - 1.0 - icon) / 2.0, icon_px, 1.0);
            // un tas de chantier complet est une bonne nouvelle : vert ; un coffre au plafond, une mauvaise : rouge
            let color = if full && container.is_site() {
                Color::from_rgba(150, 240, 150, 255)
            } else if full {
                Color::from_rgba(255, 110, 100, 255)
            } else {
                WHITE
            };
            draw_text(&label, x + icon + 3.0, top + (line - 1.0) / 2.0 + size * 0.33, size, color);
        }
    }
}

/// Durée d'une gerbe d'étincelles, en secondes, à partir du coup de marteau.
const SPARKS_SECS: f32 = 0.35;

/// Les étincelles qui jaillissent du métal chaud quand le marteau le frappe (le métal lui-même est dans le dessin).
fn draw_forge_sparks(game: &Game, view: &View) {
    let s = crate::character_view::scale(view);
    for c in game.characters() {
        if !c.work.is_some_and(|w| w.kind == WorkKind::Forge) || !view.point_visible(c.pos, 2.0) {
            continue;
        }
        let since = crate::character_sprites::since_impact(&crate::character_sprites::BLACKSMITH, game.time(), c.id.0);
        if since > SPARKS_SECS {
            continue;
        }
        let phase = since / SPARKS_SECS;
        // le métal chaud sur l'enclume, dans son dessin : 16 px au-dessus de ses pieds (posés au poste de forge)
        let (x, y) = view.to_screen(c.pos);
        let (mx, my) = (x + 2.0 * s, y - 16.0 * s);
        for i in 0..6 {
            let spread = i as f32 - 2.5;
            let dx = spread * 5.0 * phase;
            let dy = -(10.0 + (i % 3) as f32 * 3.0) * phase * (1.2 - phase);
            draw_rectangle((mx + dx * s).round(), (my + dy * s).round(), s, s, Color::new(1.0, 0.85, 0.3, 1.0 - phase));
        }
    }
}

/// Voile coloré sur la carte : orangé au crépuscule et à l'aube, bleu nuit la nuit.
/// (Plus tard : une vraie carte de lumière, avec des sources lumineuses.)
fn draw_daylight(game: &Game, view: &View) {
    let day = game.clock().daylight();
    if day >= 0.999 {
        return;
    }
    let dark = 1.0 - day;
    let warm = (day * (1.0 - day) * 4.0).clamp(0.0, 1.0); // 1 en plein crépuscule, 0 aux extrêmes
    let (night_r, night_g, night_b) = (0.04, 0.07, 0.24);
    let (dusk_r, dusk_g, dusk_b) = (0.95, 0.45, 0.15);
    let mix = |a: f32, b: f32| a + (b - a) * warm;

    let (x, y) = view.to_screen(Vec2::new(0.0, 0.0));
    let w = game.map().width() as f32 * view.tile_px;
    let h = game.map().height() as f32 * view.tile_px;
    draw_rectangle(
        x,
        y,
        w,
        h,
        Color::new(mix(night_r, dusk_r), mix(night_g, dusk_g), mix(night_b, dusk_b), dark * 0.62),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Une vue sans fenêtre : écran 1024×640, carte 64×40, caméra posée sur `target`.
    fn view_on(target: Vec2) -> View {
        let mut v = View::new();
        v.screen = (1024.0, 640.0);
        v.map_size = Vec2::new(64.0, 40.0);
        let size = v.view_size();
        v.camera.update(target, size, v.map_size, false, 0.016);
        v.refresh_origin();
        v
    }

    #[test]
    fn world_and_screen_conversions_are_inverses_at_every_zoom_level() {
        for &px in &ZOOM_LEVELS {
            let mut v = view_on(Vec2::new(32.0, 20.0));
            v.tile_px = px;
            v.refresh_origin();
            for p in [Vec2::new(0.0, 0.0), Vec2::new(31.5, 19.25), Vec2::new(63.9, 39.9)] {
                let back = v.to_world(v.to_screen(p));
                assert!((back.x - p.x).abs() < 1e-3 && (back.y - p.y).abs() < 1e-3, "{px}px : {p:?} -> {back:?}");
            }
        }
    }

    #[test]
    fn zooming_keeps_the_world_point_under_the_cursor() {
        let mut v = view_on(Vec2::new(32.0, 20.0));
        let cursor = (700.0, 200.0);
        let before = v.to_world(cursor);
        v.zoom(1, cursor);
        assert_eq!(v.tile_px, 96.0);
        let after = v.to_world(cursor);
        // l'origine est arrondie au pixel : au plus un pixel d'écart
        assert!((before.x - after.x).abs() < 1.0 / 96.0 + 1e-3 && (before.y - after.y).abs() < 1.0 / 96.0 + 1e-3);
        assert!(!v.is_free(), "zoomer ne déclenche pas la vue libre");
    }

    #[test]
    fn zoom_stops_at_both_ends_and_a_quarter_of_the_map_shows_at_the_smallest() {
        let mut v = view_on(Vec2::new(32.0, 20.0));
        for _ in 0..20 {
            v.zoom(1, (512.0, 320.0));
        }
        assert_eq!(v.tile_px, 128.0);
        for _ in 0..20 {
            v.zoom(-1, (512.0, 320.0));
        }
        assert_eq!(v.tile_px, 32.0);
        let size = v.view_size();
        assert!(size.x * size.y >= 64.0 * 40.0 / 4.0, "{size:?}");
    }

    #[test]
    fn dragging_moves_the_map_with_the_cursor_and_frees_the_view() {
        let mut v = view_on(Vec2::new(32.0, 20.0));
        let point = Vec2::new(30.0, 18.0);
        let before = v.to_screen(point);
        v.pan_pixels(-96.0, 64.0); // le curseur va à gauche de 96 px et en bas de 64 px
        let after = v.to_screen(point);
        assert!(v.is_free());
        assert!((after.0 - (before.0 - 96.0)).abs() <= 1.0 && (after.1 - (before.1 + 64.0)).abs() <= 1.0, "{before:?} -> {after:?}");
    }
}
