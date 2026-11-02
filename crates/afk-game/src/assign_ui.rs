//! L'interface du roi pour parler à quelqu'un et l'affecter : l'invite « E parler », le menu, la mini-carte (liste des
//! métiers à gauche, postes ou lits en vert/rouge), et la liste des employés (touche L). Aucune règle ici : les clics
//! deviennent des `Command` (`AssignPost`, `AssignBed`, `EndTalk`).

use crate::hud_icons::{self, Icon};
use crate::render::{tile_color, View};
use crate::theme;
use afk_core::{CharacterId, Command, Game, PlayerId, Role, TilePos};
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Step {
    Menu,
    Posts(Option<Role>),
    Beds,
}

pub struct AssignUi {
    step: Step,
    who: Option<CharacterId>,
    employees_open: bool,
    escape_used: bool,
    captured: bool,
}

/// Le nom d'un métier, tel qu'on le lit dans l'interface.
pub fn role_label(role: Role) -> &'static str {
    match role {
        Role::Farmer => "Fermier",
        Role::Blacksmith => "Forgeron",
        Role::King => "Roi",
        Role::Warlord => "Chef de guerre",
        Role::Innkeeper => "Tavernier",
        Role::Guard => "Garde",
        Role::Miner => "Mineur",
        Role::Lumberjack => "Bucheron",
        Role::Transporter => "Transporteur",
        Role::Artisan => "Artisan",
        Role::Vagabond => "Vagabond",
        Role::Villager => "Habitant",
    }
}

fn inside(p: (f32, f32), x: f32, y: f32, w: f32, h: f32) -> bool {
    p.0 >= x && p.0 < x + w && p.1 >= y && p.1 < y + h
}

const GREEN: Color = Color::new(0.20, 0.78, 0.30, 1.0);
const RED: Color = Color::new(0.86, 0.22, 0.20, 1.0);
const LIST_W: f32 = 250.0;
const ROW_H: f32 = 34.0;

/// Les métiers qui ont des postes, avec (libres, total) : ceux qui ont des postes libres d'abord.
fn roles_with_posts(game: &Game) -> Vec<(Role, u32, u32)> {
    let mut rows: Vec<(Role, u32, u32)> = Vec::new();
    for post in game.posts() {
        let free = post.unlimited || game.post_holders(post.tile).is_empty();
        match rows.iter_mut().find(|r| r.0 == post.role) {
            Some(r) => {
                r.2 += 1;
                r.1 += free as u32;
            }
            None => rows.push((post.role, free as u32, 1)),
        }
    }
    rows.sort_by_key(|r| r.1 == 0); // tri stable : les métiers sans poste libre passent après
    rows
}

/// Le rectangle de la mini-carte et l'échelle (pixels par tuile).
fn map_rect(game: &Game) -> (f32, f32, f32, f32, f32) {
    let (sw, sh) = (screen_width(), screen_height());
    let (x, y) = (LIST_W + 40.0, 70.0);
    let (aw, ah) = (sw - x - 24.0, sh - y - 40.0);
    let scale = (aw / game.map().width() as f32).min(ah / game.map().height() as f32).max(1.0);
    (x, y, game.map().width() as f32 * scale, game.map().height() as f32 * scale, scale)
}

impl AssignUi {
    pub fn new() -> AssignUi {
        AssignUi { step: Step::Menu, who: None, employees_open: false, escape_used: false, captured: false }
    }

    /// Le roi est en conversation : l'interface prend le clavier (plus de déplacement) et la souris.
    pub fn is_open(&self) -> bool {
        self.who.is_some()
    }

    pub fn captures_mouse(&self) -> bool {
        self.captured || self.who.is_some()
    }

    pub fn consumed_escape(&self) -> bool {
        self.escape_used
    }

    /// À appeler une fois par image, avant les autres entrées.
    pub fn update(&mut self, game: &Game, me: PlayerId) -> Vec<Command> {
        self.escape_used = false;
        self.captured = false;
        let mut commands = Vec::new();
        let talking = game.talking_to(me);
        if talking != self.who {
            self.who = talking;
            self.step = Step::Menu;
        }
        if is_key_pressed(KeyCode::L) && talking.is_none() {
            self.employees_open = !self.employees_open;
        }
        if self.employees_open {
            let (x, y, w, h) = employees_rect(game);
            if inside(mouse_position(), x, y, w, h) {
                self.captured = true;
            }
        }
        let Some(who) = talking else { return commands };

        let mouse = mouse_position();
        let click = is_mouse_button_pressed(MouseButton::Left);
        let back = is_key_pressed(KeyCode::Escape) || is_mouse_button_pressed(MouseButton::Right);
        if is_key_pressed(KeyCode::Escape) {
            self.escape_used = true;
        }
        match self.step {
            Step::Menu => {
                if back {
                    commands.push(Command::EndTalk { player: me });
                }
                for (i, step) in [Step::Posts(None), Step::Beds].into_iter().enumerate() {
                    let (x, y, w, h) = menu_button(i);
                    if click && inside(mouse, x, y, w, h) {
                        self.step = step;
                    }
                }
                let (x, y, w, h) = menu_button(2);
                if click && inside(mouse, x, y, w, h) {
                    commands.push(Command::EndTalk { player: me });
                }
            }
            Step::Posts(selected) => {
                if back {
                    self.step = Step::Menu;
                    return commands;
                }
                let roles = roles_with_posts(game);
                if click {
                    for (i, (role, _, _)) in roles.iter().enumerate() {
                        let (x, y, w, h) = role_row(i);
                        if inside(mouse, x, y, w, h) {
                            self.step = Step::Posts(Some(*role));
                            return commands;
                        }
                    }
                    if let Some(role) = selected {
                        let spots: Vec<TilePos> = game.posts().into_iter().filter(|p| p.role == role).map(|p| p.tile).collect();
                        if let Some(tile) = pick(game, &spots, mouse) {
                            commands.push(Command::AssignPost { player: me, who, post: tile });
                        }
                    }
                }
            }
            Step::Beds => {
                if back {
                    self.step = Step::Menu;
                    return commands;
                }
                if click {
                    let beds = game.beds();
                    if let Some(tile) = pick(game, &beds, mouse) {
                        commands.push(Command::AssignBed { player: me, who, bed: tile });
                    }
                }
            }
        }
        commands
    }

    /// Dessin du monde : l'invite « E parler » au-dessus de la personne qu'on peut aborder.
    pub fn draw_world(&self, game: &Game, view: &View, me: PlayerId) {
        let Some(id) = game.talkable(me) else { return };
        let Some(c) = game.character(id) else { return };
        let (x, y) = view.to_screen(c.pos);
        let lift = view.tile_px * 1.7;
        let label = "parler";
        let w = 30.0 + measure_text(label, None, 20, 1.0).width;
        theme::plank(x - w / 2.0 - 4.0, y - lift - 6.0, w + 8.0, 34.0);
        theme::keycap(x - w / 2.0 + 4.0, y - lift - 1.0, 22.0, "E");
        theme::gilt(label, x - w / 2.0 + 32.0, y - lift + 16.0, 20.0);
    }

    pub fn draw(&self, game: &Game, view: &View, me: PlayerId) {
        if self.employees_open && self.who.is_none() {
            draw_employees(game);
        }
        let Some(who) = self.who else { return };
        let Some(person) = game.character(who) else { return };
        // la personne à qui on parle : un cercle pulsant
        let (px, py) = view.to_screen(person.pos);
        let pulse = 0.6 + 0.4 * (game.time() * 5.0).sin().abs();
        draw_circle_lines(px, py, view.tile_px * 0.7, 3.0, theme::with_alpha(theme::GOLD_LIGHT, pulse));
        match self.step {
            Step::Menu => draw_menu(person.name.as_str(), person.role),
            Step::Posts(selected) => draw_minimap(game, person.name.as_str(), selected, false, who),
            Step::Beds => draw_minimap(game, person.name.as_str(), None, true, who),
        }
        let _ = me;
    }
}

fn menu_rect() -> (f32, f32, f32, f32) {
    let (w, h) = (330.0, 200.0);
    ((screen_width() - w) / 2.0, screen_height() - h - 24.0, w, h)
}

fn menu_button(i: usize) -> (f32, f32, f32, f32) {
    let (x, y, w, _) = menu_rect();
    (x + 24.0, y + 62.0 + i as f32 * 38.0, w - 48.0, 32.0)
}

fn draw_menu(name: &str, role: Role) {
    let (x, y, w, h) = menu_rect();
    theme::panel(x, y, w, h);
    theme::ribbon(x + 22.0, y - 14.0, w - 44.0, "Parler");
    theme::ink(&format!("{} ({})", name, role_label(role)), x + 24.0, y + 54.0, 20.0);
    let mouse = mouse_position();
    for (i, label) in ["Affecter a un poste", "Donner un lit", "Rien"].into_iter().enumerate() {
        let (bx, by, bw, bh) = menu_button(i);
        theme::button(bx, by, bw, bh, inside(mouse, bx, by, bw, bh), false);
        theme::ink(label, bx + 12.0, by + 22.0, 20.0);
    }
}

fn role_row(i: usize) -> (f32, f32, f32, f32) {
    (24.0, 96.0 + i as f32 * (ROW_H + 4.0), LIST_W, ROW_H)
}

/// Le marqueur le plus proche du clic, s'il est assez près.
fn pick(game: &Game, spots: &[TilePos], mouse: (f32, f32)) -> Option<TilePos> {
    let (mx, my, _, _, scale) = map_rect(game);
    let reach = (scale * 1.2).max(10.0);
    spots
        .iter()
        .map(|&t| {
            let (cx, cy) = (mx + (t.x as f32 + 0.5) * scale, my + (t.y as f32 + 0.5) * scale);
            (t, ((cx - mouse.0).powi(2) + (cy - mouse.1).powi(2)).sqrt())
        })
        .filter(|&(_, d)| d <= reach)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(t, _)| t)
}

fn draw_minimap(game: &Game, name: &str, selected: Option<Role>, beds: bool, who: CharacterId) {
    let (sw, sh) = (screen_width(), screen_height());
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.62));
    let title = if beds { format!("Un lit pour {}", name) } else { format!("Un poste pour {}", name) };
    theme::ribbon(sw / 2.0 - 220.0, 16.0, 440.0, &title);
    let mouse = mouse_position();

    // la carte
    let (mx, my, mw, mh, scale) = map_rect(game);
    theme::panel(mx - 12.0, my - 12.0, mw + 24.0, mh + 24.0);
    let map = game.map();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let tile = map.get(TilePos::new(x, y));
            draw_rectangle(mx + x as f32 * scale, my + y as f32 * scale, scale + 0.5, scale + 0.5, tile_color(tile, x, y));
        }
    }
    for c in game.characters().iter().filter(|c| !c.gone) {
        let col = if c.id == who { theme::GOLD_LIGHT } else { WHITE };
        draw_circle(mx + c.pos.x * scale, my + c.pos.y * scale, (scale * 0.45).max(2.0), col);
    }

    // les marqueurs : vert libre, rouge occupé
    let spots: Vec<(TilePos, bool)> = if beds {
        game.beds().into_iter().map(|b| (b, game.bed_owner(b).is_none_or(|o| o == who))).collect()
    } else if let Some(role) = selected {
        game.posts()
            .into_iter()
            .filter(|p| p.role == role)
            .map(|p| (p.tile, p.unlimited || game.post_holders(p.tile).iter().all(|&h| h == who)))
            .collect()
    } else {
        Vec::new()
    };
    let near = pick(game, &spots.iter().map(|s| s.0).collect::<Vec<_>>(), mouse);
    for (tile, free) in &spots {
        let size = (scale * 1.5).max(12.0) * if near == Some(*tile) { 1.3 } else { 1.0 };
        let (cx, cy) = (mx + (tile.x as f32 + 0.5) * scale, my + (tile.y as f32 + 0.5) * scale);
        let col = if *free { GREEN } else { RED };
        draw_rectangle(cx - size / 2.0, cy - size / 2.0, size, size, theme::with_alpha(col, 0.85));
        draw_rectangle_lines(cx - size / 2.0, cy - size / 2.0, size, size, 2.0, WHITE);
    }
    if beds {
        theme::gilt("Vert : lit libre. Rouge : pris (on le lui reprend).", 24.0, 70.0, 20.0);
    } else if spots.is_empty() {
        theme::gilt("Choisissez un metier.", 24.0, 70.0, 20.0);
    }

    // la liste des métiers
    if !beds {
        let roles = roles_with_posts(game);
        theme::gilt("Metiers", 24.0, 86.0, 22.0);
        if roles.is_empty() {
            draw_text("Aucun poste : batissez une", 24.0, 130.0, 20.0, WHITE);
            draw_text("forge, un entrepot, une taverne.", 24.0, 154.0, 20.0, WHITE);
        }
        for (i, (role, free, total)) in roles.iter().enumerate() {
            let (x, y, w, h) = role_row(i);
            theme::button(x, y, w, h, inside(mouse, x, y, w, h), selected == Some(*role));
            theme::ink(role_label(*role), x + 10.0, y + 23.0, 20.0);
            let count = format!("{}/{}", free, total);
            let cw = measure_text(&count, None, 20, 1.0).width;
            draw_text(&count, x + w - cw - 10.0, y + 23.0, 20.0, if *free > 0 { Color::new(0.1, 0.45, 0.15, 1.0) } else { RED });
        }
    }
    draw_text("Clic : affecter    Echap / clic droit : retour", mx, sh - 14.0, 20.0, WHITE);
}

fn employees(game: &Game) -> Vec<&afk_core::Character> {
    game.characters().iter().filter(|c| !c.gone && !matches!(c.role, Role::King | Role::Vagabond)).collect()
}

fn employees_rect(game: &Game) -> (f32, f32, f32, f32) {
    let rows = employees(game).len().max(1) as f32;
    let w = 400.0;
    (screen_width() - w - 14.0, 70.0, w, 70.0 + rows * 28.0)
}

fn draw_employees(game: &Game) {
    let (x, y, w, h) = employees_rect(game);
    theme::panel(x, y, w, h);
    theme::ribbon(x + 22.0, y - 14.0, w - 44.0, "Employes");
    let list = employees(game);
    if list.is_empty() {
        theme::ink("Personne pour l'instant.", x + 24.0, y + 56.0, 20.0);
    }
    for (i, c) in list.iter().enumerate() {
        let ly = y + 52.0 + i as f32 * 28.0;
        theme::ink(&c.name, x + 24.0, ly, 20.0);
        draw_text(role_label(c.role), x + 150.0, ly, 20.0, theme::INK_SOFT);
        let post = if c.post.is_some() { "poste" } else { "sans poste" };
        draw_text(post, x + 262.0, ly, 20.0, if c.post.is_some() { theme::INK_SOFT } else { RED });
        let (icon, alpha) = if c.has_bed { (Icon::Bed, 1.0) } else { (Icon::Cross, 0.9) };
        hud_icons::draw(icon, x + w - 54.0, ly - 16.0, 2.0, alpha);
    }
}
