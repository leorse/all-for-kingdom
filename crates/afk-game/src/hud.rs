//! L'interface en jeu : des pictogrammes et des nombres, presque pas de mots (voir `hud_icons`). Seule l'aide, ouverte
//! avec H ou F1, est écrite. Comme `render`, ce fichier lit l'état de `afk-core` sans jamais le modifier.

use crate::character_view::Sheets;
use crate::hud_icons::{self as icons, Icon, GRID};
use crate::render::{fitness_color, role_color};
use crate::theme::{self, with_alpha};
use afk_core::{Character, Game, PlayerId, Resource, Role, WorkKind};
use macroquad::prelude::*;

const MARGIN: f32 = 8.0;
/// Taille d'un pixel de pictogramme à l'écran : un pictogramme fait 30 px.
const PX: f32 = 3.0;
const ICON: f32 = GRID as f32 * PX;
const NUM: f32 = 26.0;
const CHIP_H: f32 = 40.0;

const CARD_Y: f32 = MARGIN + CHIP_H + MARGIN;
const CARD_H: f32 = 72.0;
const PORTRAIT: f32 = 56.0;

const CELL_W: f32 = 104.0;
const CELL_H: f32 = 36.0;

pub fn draw(game: &Game, sheets: &Sheets, me: PlayerId, speed: f32) {
    draw_kingdom(game);
    draw_hero(game, sheets, game.possessed_by(me).and_then(|id| game.character(id)));
    draw_clock(game, speed);
    draw_stock(game);
}

/// L'aide (touche H ou F1) : fermée, une simple touche discrète en bas à droite ; ouverte, le panneau des commandes.
pub fn draw_help(open: bool) {
    if !open {
        let size = 28.0;
        theme::keycap(screen_width() - MARGIN - size, screen_height() - MARGIN - size, size, "H");
        return;
    }
    const LINES: [(&str, &str); 11] = [
        ("ZQSD / fleches", "bouger"),
        ("E", "travailler, dormir, deposer, parler"),
        ("L", "liste des employes"),
        ("Tab / 1-9 / clic", "incarner quelqu'un"),
        ("Espace", "mode AFK"),
        ("T", "vitesse du temps"),
        ("Clic droit + glisser", "deplacer la vue"),
        ("Molette", "zoom"),
        ("C", "vider l'entrepot central"),
        ("H / F1", "cette aide"),
        ("Echap", "quitter"),
    ];
    let (w, h) = (560.0, 78.0 + LINES.len() as f32 * 28.0);
    let (x, y) = ((screen_width() - w) / 2.0, ((screen_height() - h) / 2.0).max(40.0));
    theme::panel(x, y, w, h);
    theme::ribbon(x + 22.0, y - 14.0, w - 44.0, "Commandes");
    for (i, (keys, what)) in LINES.iter().enumerate() {
        let line_y = y + 66.0 + i as f32 * 28.0;
        theme::ink(keys, x + 28.0, line_y, 22.0);
        draw_text(what, x + 262.0, line_y, 22.0, theme::INK_SOFT);
    }
}

// ---------------------------------------------------------------- pastilles

fn chip_width(label: &str) -> f32 {
    10.0 + ICON + 8.0 + measure_text(label, None, NUM as u16, 1.0).width + 12.0
}

/// Une pastille de bois : un pictogramme et un nombre. Renvoie sa largeur.
fn chip(x: f32, y: f32, icon: Icon, label: &str) -> f32 {
    let w = chip_width(label);
    theme::plank(x, y, w, CHIP_H);
    icons::draw(icon, x + 10.0, y + (CHIP_H - ICON) / 2.0, PX, 1.0);
    theme::gilt(label, x + 10.0 + ICON + 8.0, y + CHIP_H / 2.0 + NUM * 0.3, NUM);
    w
}

/// Le royaume : le nombre d'habitants (les vagabonds de passage n'en sont pas) et, au camp, les lits libres.
fn draw_kingdom(game: &Game) {
    let people = game.characters().iter().filter(|c| !c.gone && c.role != Role::Vagabond).count();
    let w = chip(MARGIN, MARGIN, Icon::Crown, &people.to_string());
    if game.camp().is_some() {
        chip(MARGIN + w + MARGIN, MARGIN, Icon::Bed, &game.free_places().to_string());
    }
}

/// L'heure (soleil ou lune selon le jour) en haut à droite, et la vitesse du temps quand elle n'est pas normale.
fn draw_clock(game: &Game, speed: f32) {
    let clock = game.clock();
    let hour = clock.hour();
    let label = format!("J{} {:02}:{:02}", clock.day(), hour.floor() as u32, (hour.fract() * 60.0).floor() as u32);
    let w = chip_width(&label);
    let x = screen_width() - MARGIN - w;
    chip(x, MARGIN, if clock.is_night() { Icon::Moon } else { Icon::Sun }, &label);
    if (speed - 1.0).abs() > 0.01 {
        let label = format!("x{}", speed);
        chip(x - MARGIN - chip_width(&label), MARGIN, Icon::Fast, &label);
    }
}

/// Les ressources du royaume : une grille de deux colonnes, un pictogramme et un total par case.
fn draw_stock(game: &Game) {
    const COLS: usize = 2;
    let rows = Resource::ALL.len().div_ceil(COLS);
    let (w, h) = (COLS as f32 * CELL_W + 20.0, rows as f32 * CELL_H + 20.0);
    let (x, y) = (screen_width() - MARGIN - w, CARD_Y);
    theme::plank(x, y, w, h);
    for (i, resource) in Resource::ALL.into_iter().enumerate() {
        let (cx, cy) = (x + 10.0 + (i % COLS) as f32 * CELL_W, y + 10.0 + (i / COLS) as f32 * CELL_H);
        let amount = game.stock().get(resource);
        let alpha = if amount == 0 { 0.4 } else { 1.0 };
        icons::draw(Icon::Res(resource), cx, cy + (CELL_H - ICON) / 2.0, PX, alpha);
        let label = amount.to_string();
        let (tx, ty) = (cx + ICON + 6.0, cy + CELL_H / 2.0 + NUM * 0.3);
        draw_text(&label, tx + 1.0, ty + 2.0, NUM, with_alpha(BLACK, 0.8 * alpha));
        draw_text(&label, tx, ty, NUM, with_alpha(theme::GOLD_LIGHT, alpha));
    }
}

// ---------------------------------------------------------------- le personnage incarné

/// Le personnage incarné : sa tête, sa forme, ce qu'il peut faire (touche E) et ce qu'il porte. Sans personnage
/// incarné (mode AFK), une pause.
fn draw_hero(game: &Game, sheets: &Sheets, hero: Option<&Character>) {
    let Some(c) = hero else {
        theme::plank(MARGIN, CARD_Y, 72.0, CARD_H);
        icons::draw(Icon::Pause, MARGIN + 16.0, CARD_Y + 16.0, 4.0, 0.9);
        return;
    };
    let x0 = MARGIN + 8.0 + PORTRAIT + 10.0;
    let cargo_w = if c.cargo.is_some() { 76.0 } else { 0.0 };
    theme::plank(MARGIN, CARD_Y, x0 - MARGIN + 104.0 + cargo_w + 8.0, CARD_H);
    portrait(MARGIN + 8.0, CARD_Y + 8.0, c, sheets);

    fitness_bar(x0, CARD_Y + 12.0, 104.0, 14.0, c.fitness());
    if let Some(h) = hint(game, c) {
        draw_hint(x0, CARD_Y + 32.0, h);
    }
    if let Some((resource, amount)) = c.cargo {
        let cx = x0 + 104.0 + 4.0;
        icons::draw(Icon::Res(resource), cx, CARD_Y + 12.0, PX, 1.0);
        theme::gilt(&amount.to_string(), cx + ICON + 4.0, CARD_Y + 12.0 + ICON * 0.8, NUM);
    }
}

/// Jauge de forme : pleine quand le personnage est en forme, elle se vide avec la fatigue.
fn fitness_bar(x: f32, y: f32, w: f32, h: f32, fitness: f32) {
    draw_rectangle(x, y, w, h, Color::from_rgba(20, 20, 20, 255));
    draw_rectangle(x + 1.0, y + 1.0, (w - 2.0) * (fitness / 100.0).clamp(0.0, 1.0), h - 2.0, fitness_color(fitness));
}

/// La tête du personnage : celle de sa planche quand son métier est dessiné, sinon (le vagabond) un visage simple aux
/// couleurs de son métier. Endormi, un « z ».
fn portrait(x: f32, y: f32, c: &Character, sheets: &Sheets) {
    if let Some(sheet) = sheets.for_role(c.role) {
        draw_rectangle(x - 2.0, y - 2.0, PORTRAIT + 4.0, PORTRAIT + 4.0, theme::WOOD_DARK);
        draw_rectangle(x, y, PORTRAIT, PORTRAIT, with_alpha(role_color(c.role), 0.45));
        sheet.draw_portrait(x, y);
        if c.asleep {
            theme::gilt("z", x + PORTRAIT - 16.0, y + 22.0, 24.0);
        }
        draw_rectangle_lines(x, y, PORTRAIT, PORTRAIT, 2.0, theme::GOLD);
        return;
    }
    let role = role_color(c.role);
    let dark = Color::new(role.r * 0.7, role.g * 0.7, role.b * 0.7, 1.0);
    draw_rectangle(x - 2.0, y - 2.0, PORTRAIT + 4.0, PORTRAIT + 4.0, theme::WOOD_DARK);
    draw_rectangle(x, y, PORTRAIT, PORTRAIT, with_alpha(role, 0.45));
    draw_rectangle(x + 6.0, y + 40.0, 44.0, 16.0, role); // les épaules
    draw_rectangle(x + 16.0, y + 12.0, 24.0, 28.0, Color::from_rgba(240, 200, 160, 255)); // le visage
    match c.role {
        Role::King => {
            let gold = Color::from_rgba(228, 188, 52, 255);
            draw_rectangle(x + 15.0, y + 8.0, 26.0, 7.0, gold);
            for i in 0..3 {
                draw_rectangle(x + 15.0 + i as f32 * 10.5, y + 2.0, 5.0, 7.0, gold);
            }
        }
        Role::Guard => draw_rectangle(x + 14.0, y + 5.0, 28.0, 13.0, Color::from_rgba(160, 166, 184, 255)),
        _ => draw_rectangle(x + 14.0, y + 7.0, 28.0, 11.0, dark),
    }
    let eye = Color::from_rgba(44, 30, 22, 255);
    if c.asleep {
        draw_rectangle(x + 19.0, y + 28.0, 7.0, 2.0, eye);
        draw_rectangle(x + 30.0, y + 28.0, 7.0, 2.0, eye);
        theme::gilt("z", x + PORTRAIT - 16.0, y + 22.0, 24.0);
    } else {
        draw_rectangle(x + 20.0, y + 25.0, 4.0, 6.0, eye);
        draw_rectangle(x + 32.0, y + 25.0, 4.0, 6.0, eye);
    }
    draw_rectangle_lines(x, y, PORTRAIT, PORTRAIT, 2.0, theme::GOLD);
}

// ---------------------------------------------------------------- l'action possible

/// Ce qu'une pression sur E ferait, ou pourquoi le personnage ne peut pas travailler.
enum Hint {
    WakeUp,
    Sleep,
    Cancel,
    Work(WorkKind),
    Deposit,
    Load,
    Blocked(Blocked),
}

/// Pourquoi un producteur est à l'arrêt.
enum Blocked {
    /// Il manque cette ressource dans le coffre.
    Missing(Resource),
    /// Le coffre est plein de cette ressource.
    ChestFull(Resource),
    /// Ses mains sont pleines de cette ressource.
    HandsFull(Resource),
}

fn hint(game: &Game, c: &Character) -> Option<Hint> {
    if c.asleep {
        Some(Hint::WakeUp)
    } else if game.can_lie_down(c.id) {
        Some(Hint::Sleep)
    } else if c.work.is_some() {
        Some(Hint::Cancel)
    } else if let Some((kind, _)) = game.available_work(c.id) {
        Some(Hint::Work(kind))
    } else if let Some(why) = blocked(game, c) {
        Some(Hint::Blocked(why))
    } else if facing_crate(game, c) && c.cargo.is_some() {
        Some(Hint::Deposit)
    } else if facing_crate(game, c) && c.role == Role::Transporter {
        Some(Hint::Load)
    } else {
        None
    }
}

/// Le personnage regarde-t-il un coffre ?
fn facing_crate(game: &Game, c: &Character) -> bool {
    game.containers().iter().any(|k| k.tile == c.facing_tile())
}

fn blocked(game: &Game, c: &Character) -> Option<Blocked> {
    let container = c.container.and_then(|i| game.containers().get(i));
    match c.role {
        Role::Blacksmith => {
            let forge = container?;
            if forge.get(Resource::Iron) < game.config().iron_per_tool {
                Some(Blocked::Missing(Resource::Iron))
            } else if forge.room(Resource::Tools) < game.config().tools_per_forge {
                Some(Blocked::ChestFull(Resource::Tools))
            } else {
                None
            }
        }
        Role::Miner => {
            let mine = container?;
            let product = mine.product()?;
            (mine.room(product) == 0).then_some(Blocked::ChestFull(product))
        }
        Role::Farmer => {
            let farm = container?;
            let carried = c.cargo.map_or(0, |(_, n)| n);
            if carried + game.config().wheat_per_harvest > game.config().farmer_carry {
                Some(if farm.room(Resource::Wheat) == 0 { Blocked::ChestFull(Resource::Wheat) } else { Blocked::HandsFull(Resource::Wheat) })
            } else {
                None
            }
        }
        Role::Lumberjack => {
            let carried = c.cargo.map_or(0, |(_, n)| n);
            (carried + game.config().logs_per_tree > game.config().lumberjack_carry).then_some(Blocked::HandsFull(Resource::Wood))
        }
        _ => None,
    }
}

/// Les pictogrammes de l'action, de gauche à droite à partir de `x`. Un blocage commence par un panneau « ! » ; le reste
/// s'adresse à la touche E, dessinée devant.
fn draw_hint(x: f32, y: f32, hint: Hint) {
    let is_blocked = matches!(hint, Hint::Blocked(_));
    let list: Vec<Icon> = match hint {
        Hint::Blocked(why) => match why {
            Blocked::Missing(r) => vec![Icon::Bang, Icon::Res(r), Icon::Cross],
            Blocked::ChestFull(r) => vec![Icon::Bang, Icon::Chest, Icon::Res(r)],
            Blocked::HandsFull(r) => vec![Icon::Bang, Icon::Sack, Icon::Res(r)],
        },
        Hint::WakeUp => vec![Icon::Bed, Icon::Up],
        Hint::Sleep => vec![Icon::Bed],
        Hint::Cancel => vec![Icon::Cross],
        Hint::Work(kind) => vec![Icon::Work(kind)],
        Hint::Deposit => vec![Icon::Down, Icon::Chest],
        Hint::Load => vec![Icon::Up, Icon::Chest],
    };
    let mut cx = x;
    if !is_blocked {
        theme::keycap(x, y + 2.0, 26.0, "E");
        cx += 32.0;
    }
    for icon in list {
        icons::draw(icon, cx, y, PX, 1.0);
        cx += ICON + 4.0;
    }
}
