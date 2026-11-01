//! L'interface du roi : choisir un plan dans une liste, puis le placer sur la carte.
//!
//! Quand on incarne le roi, un panneau de parchemin liste les plans (maison, entrepôt, taverne). Un clic sur une carte
//! choisit le plan : un rectangle en surbrillance suit la souris (déplacer), un clic le fixe par son coin haut-gauche,
//! puis la souris l'agrandit (le coût en bois et en pierre se recalcule à chaque tuile) et un second clic valide. La
//! zone est bleue quand le royaume a les ressources, rouge sinon, grise quand le terrain ne le permet pas. On peut poser
//! un plan rouge : le chantier attendra les matériaux. Le temps ne s'arrête jamais pendant qu'on place.
//!
//! Ce module traduit la souris en `Command::PlacePlan` : il ne décide d'aucune règle (le cœur valide tout).

use crate::construction_view::plan_name;
use crate::render::View;
use crate::theme::{self, with_alpha};
use afk_core::{Command, Game, HabitatSpec, PlanError, PlanKind, PlayerId, Resource, TilePos};
use macroquad::prelude::*;

const PANEL_X: f32 = 8.0;
const PANEL_Y: f32 = 150.0;
/// Hauteur du panneau réduit : juste l'en-tête et son bouton.
const COLLAPSED_H: f32 = 46.0;
const PANEL_W: f32 = 276.0;
const CARD_H: f32 = 80.0;
const CARD_GAP: f32 = 6.0;
const HEADER_H: f32 = 50.0;
const BAR_W: f32 = 760.0;
const BAR_H: f32 = 122.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    /// Le panneau est ouvert, aucun plan n'est choisi.
    Browse,
    /// Un plan est choisi, le rectangle suit la souris.
    Move { kind: PlanKind, beds: u32 },
    /// Le coin haut-gauche est fixé, la souris agrandit.
    Size { kind: PlanKind, anchor: (i32, i32), beds: u32 },
}

pub struct PlanUi {
    mode: Mode,
    /// Un message passager (pourquoi un plan est refusé, plan posé...) et le temps qu'il lui reste.
    message: Option<(String, f32)>,
    escape_used: bool,
    captured: bool,
    /// Le coin haut-gauche du panneau : on peut le déplacer en tirant son en-tête.
    panel: (f32, f32),
    /// Le panneau est réduit à son en-tête.
    collapsed: bool,
    /// Le panneau est tenu : écart entre la souris et son coin.
    drag: Option<(f32, f32)>,
    /// Où le bouton droit a été enfoncé (un clic bref annule, un glissé tire la carte).
    right_press: Option<(f32, f32)>,
}

/// La forme du plan en cours, d'après le mode et la souris : coin haut-gauche et taille.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Shape {
    kind: PlanKind,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    beds: u32,
    sizing: bool,
}

fn inside(p: (f32, f32), x: f32, y: f32, w: f32, h: f32) -> bool {
    p.0 >= x && p.0 <= x + w && p.1 >= y && p.1 <= y + h
}

fn panel_height(collapsed: bool) -> f32 {
    if collapsed {
        COLLAPSED_H
    } else {
        HEADER_H + PlanKind::ALL.len() as f32 * (CARD_H + CARD_GAP) + 34.0
    }
}

fn card_rect(origin: (f32, f32), i: usize) -> (f32, f32, f32, f32) {
    (origin.0 + 16.0, origin.1 + HEADER_H + i as f32 * (CARD_H + CARD_GAP), PANEL_W - 32.0, CARD_H)
}

/// Le bouton qui réduit ou rouvre le panneau, à droite de l'en-tête.
fn toggle_rect(origin: (f32, f32)) -> (f32, f32, f32, f32) {
    (origin.0 + PANEL_W - 40.0, origin.1 + 18.0, 24.0, 24.0)
}

/// L'en-tête, par où l'on tire le panneau : le ruban et la bande dessous.
fn header_rect(origin: (f32, f32)) -> (f32, f32, f32, f32) {
    (origin.0, origin.1 - 14.0, PANEL_W, HEADER_H + 14.0)
}

fn bar_origin() -> (f32, f32) {
    ((screen_width() - BAR_W) / 2.0, screen_height() - BAR_H - 56.0)
}

/// Les boutons « - » et « + » du nombre de lits, dans la barre.
fn bed_buttons() -> ((f32, f32, f32, f32), (f32, f32, f32, f32)) {
    let (bx, by) = bar_origin();
    ((bx + BAR_W - 214.0, by + 22.0, 28.0, 28.0), (bx + BAR_W - 70.0, by + 22.0, 28.0, 28.0))
}

fn card_at(origin: (f32, f32), mouse: (f32, f32)) -> Option<PlanKind> {
    PlanKind::ALL.iter().enumerate().find(|&(i, _)| {
        let (x, y, w, h) = card_rect(origin, i);
        inside(mouse, x, y, w, h)
    }).map(|(_, &k)| k)
}

fn error_text(e: PlanError) -> &'static str {
    match e {
        PlanError::TooSmall => "Trop petit pour ce batiment",
        PlanError::TooLarge => "Trop grand",
        PlanError::BadBeds => "Ce nombre de lits ne tient pas",
        PlanError::Blocked => "Terrain bloque : eau, arbre, filon, coffre ou bord de la carte",
        PlanError::Occupied => "Quelqu'un se tient sur l'emplacement",
    }
}

impl PlanUi {
    pub fn new() -> PlanUi {
        PlanUi {
            mode: Mode::Browse,
            message: None,
            escape_used: false,
            captured: false,
            panel: (PANEL_X, PANEL_Y),
            collapsed: false,
            drag: None,
            right_press: None,
        }
    }

    /// La souris est-elle prise par l'interface (panneau, barre, ou un plan en cours de placement) ? Alors un clic ne
    /// doit pas incarner quelqu'un.
    pub fn captures_mouse(&self) -> bool {
        self.captured
    }

    /// Échap vient de servir à annuler un plan : il ne doit pas quitter le jeu.
    pub fn consumed_escape(&self) -> bool {
        self.escape_used
    }

    fn tell(&mut self, text: impl Into<String>) {
        self.message = Some((text.into(), 3.5));
    }

    /// La forme du plan en cours, sous la souris.
    fn shape(&self, view: &View) -> Option<Shape> {
        let cursor = view.to_world(mouse_position()).tile();
        match self.mode {
            Mode::Browse => None,
            Mode::Move { kind, beds } => {
                let (w, h) = kind.min_size();
                Some(Shape { kind, x: cursor.x - w / 2, y: cursor.y - h / 2, w, h, beds, sizing: false })
            }
            Mode::Size { kind, anchor, beds } => {
                let (min_w, min_h) = kind.min_size();
                let (max_w, max_h) = kind.max_size();
                let w = (cursor.x - anchor.0 + 1).clamp(min_w, max_w);
                let h = (cursor.y - anchor.1 + 1).clamp(min_h, max_h);
                Some(Shape { kind, x: anchor.0, y: anchor.1, w, h, beds, sizing: true })
            }
        }
    }

    /// Le nombre de lits d'une maison, ramené à ce que la taille permet.
    fn fit_beds(shape: &mut Shape) {
        if shape.kind == PlanKind::House {
            let max = HabitatSpec::max_house_beds(shape.w, shape.h).unwrap_or(1);
            shape.beds = shape.beds.clamp(1, max);
        } else {
            shape.beds = 0;
        }
    }

    /// À appeler une fois par image, avant la lecture des autres entrées. Renvoie les commandes à envoyer au cœur.
    pub fn update(&mut self, game: &Game, view: &View, me: PlayerId, dt: f32) -> Vec<Command> {
        self.escape_used = false;
        self.captured = false;
        if let Some((_, left)) = self.message.as_mut() {
            *left -= dt;
        }
        if self.message.as_ref().map_or(false, |(_, left)| *left <= 0.0) {
            self.message = None;
        }
        let mut commands = Vec::new();
        if !game.is_king(me) {
            self.mode = Mode::Browse; // on a lâché le roi : plus de plan en cours
            return commands;
        }

        let mouse = mouse_position();
        let (bx, by) = bar_origin();
        let clicked = is_mouse_button_pressed(MouseButton::Left);
        let esc = is_key_pressed(KeyCode::Escape);

        // le panneau : réduire, rouvrir, tirer par l'en-tête
        let toggle = toggle_rect(self.panel);
        let header = header_rect(self.panel);
        if clicked && inside(mouse, toggle.0, toggle.1, toggle.2, toggle.3) {
            self.collapsed = !self.collapsed;
        } else if clicked && inside(mouse, header.0, header.1, header.2, header.3) {
            self.drag = Some((mouse.0 - self.panel.0, mouse.1 - self.panel.1));
        }
        if !is_mouse_button_down(MouseButton::Left) {
            self.drag = None;
        }
        if let Some((dx, dy)) = self.drag {
            let h = panel_height(self.collapsed);
            self.panel = (
                (mouse.0 - dx).clamp(0.0, (screen_width() - PANEL_W).max(0.0)),
                (mouse.1 - dy).clamp(14.0, (screen_height() - h).max(14.0)),
            );
        }
        let dragging = self.drag.is_some();
        let over_panel = dragging || inside(mouse, self.panel.0, self.panel.1 - 14.0, PANEL_W, panel_height(self.collapsed) + 14.0);
        let placing = self.mode != Mode::Browse;
        let over_bar = placing && inside(mouse, bx, by, BAR_W, BAR_H);
        self.captured = over_panel || placing;

        // bouton droit : un clic bref annule (un plan en cours de taille revient au déplacement, puis au panneau) ;
        // en glissant, il tire la carte et n'annule rien
        let mut cancel = false;
        if is_mouse_button_pressed(MouseButton::Right) {
            self.right_press = Some(mouse);
        }
        if is_mouse_button_released(MouseButton::Right) {
            if let Some(start) = self.right_press.take() {
                cancel = (mouse.0 - start.0).abs() < 6.0 && (mouse.1 - start.1).abs() < 6.0;
            }
        }
        let cards_open = !self.collapsed && !dragging;

        // les boutons de lits de la barre
        let mut bed_delta = 0;
        if clicked && over_bar {
            let (minus, plus) = bed_buttons();
            if inside(mouse, minus.0, minus.1, minus.2, minus.3) {
                bed_delta = -1;
            } else if inside(mouse, plus.0, plus.1, plus.2, plus.3) {
                bed_delta = 1;
            }
        }

        match self.mode {
            Mode::Browse => {
                if clicked && over_panel && cards_open {
                    if let Some(kind) = card_at(self.panel, mouse) {
                        self.mode = Mode::Move { kind, beds: kind.min_beds() };
                    }
                }
            }
            Mode::Move { kind, beds } => {
                if esc || cancel {
                    self.mode = Mode::Browse;
                    self.escape_used = esc;
                } else if bed_delta != 0 {
                    self.mode = Mode::Move { kind, beds: (beds as i32 + bed_delta).max(0) as u32 };
                } else if clicked && over_panel {
                    if let Some(k) = card_at(self.panel, mouse).filter(|_| cards_open) {
                        self.mode = Mode::Move { kind: k, beds: k.min_beds() };
                    }
                } else if clicked && !over_bar {
                    if let Some(shape) = self.shape(view) {
                        self.mode = Mode::Size { kind, anchor: (shape.x, shape.y), beds };
                    }
                }
            }
            Mode::Size { kind, anchor, beds } => {
                if esc || cancel || is_key_pressed(KeyCode::Backspace) {
                    self.mode = Mode::Move { kind, beds };
                    self.escape_used = esc;
                } else if bed_delta != 0 {
                    self.mode = Mode::Size { kind, anchor, beds: (beds as i32 + bed_delta).max(0) as u32 };
                } else if (clicked && !over_bar && !over_panel) || is_key_pressed(KeyCode::Enter) {
                    if let Some(mut shape) = self.shape(view) {
                        Self::fit_beds(&mut shape);
                        match game.check_plan(shape.kind, shape.x, shape.y, shape.w, shape.h, shape.beds) {
                            Ok(()) => {
                                commands.push(Command::PlacePlan {
                                    player: me,
                                    kind: shape.kind,
                                    x: shape.x,
                                    y: shape.y,
                                    w: shape.w,
                                    h: shape.h,
                                    beds: shape.beds,
                                });
                                let cost = game.plan_cost(shape.kind, shape.w, shape.h, shape.beds);
                                self.tell(if game.can_afford(cost) {
                                    format!("{} pose : les materiaux arrivent", plan_name(shape.kind))
                                } else {
                                    format!("{} pose : le chantier attend les materiaux", plan_name(shape.kind))
                                });
                                self.mode = Mode::Move { kind, beds };
                            }
                            Err(e) => self.tell(error_text(e)),
                        }
                    }
                }
            }
        }
        commands
    }

    /// Le rectangle en surbrillance, sur la carte : à dessiner avec le monde, sous l'interface.
    pub fn draw_world(&self, game: &Game, view: &View, me: PlayerId) {
        if !game.is_king(me) {
            return;
        }
        if let Some(mut shape) = self.shape(view) {
            Self::fit_beds(&mut shape);
            draw_ghost(game, view, &shape);
        }
    }

    /// Le panneau des plans, la barre d'information du plan en cours et les messages : par-dessus tout.
    pub fn draw(&self, game: &Game, view: &View, me: PlayerId) {
        if !game.is_king(me) {
            self.draw_message();
            return;
        }
        let hover = if self.collapsed { None } else { card_at(self.panel, mouse_position()) };
        let selected = match self.mode {
            Mode::Browse => None,
            Mode::Move { kind, .. } | Mode::Size { kind, .. } => Some(kind),
        };
        draw_panel(game, self.panel, self.collapsed, hover, selected);
        if let Some(mut shape) = self.shape(view) {
            Self::fit_beds(&mut shape);
            draw_bar(game, &shape);
        }
        self.draw_message();
    }

    fn draw_message(&self) {
        let Some((text, left)) = &self.message else { return };
        let size = 22.0;
        let w = measure_text(text, None, size as u16, 1.0).width + 36.0;
        let (x, y) = ((screen_width() - w) / 2.0, 64.0);
        let alpha = (left / 0.6).clamp(0.0, 1.0);
        draw_rectangle(x, y, w, 36.0, with_alpha(theme::WOOD_DARK, alpha));
        draw_rectangle(x + 2.0, y + 2.0, w - 4.0, 32.0, with_alpha(theme::PARCHMENT, alpha));
        draw_rectangle_lines(x + 2.0, y + 2.0, w - 4.0, 32.0, 1.0, with_alpha(theme::GOLD, alpha));
        draw_text(text, x + 18.0, y + 25.0, size, with_alpha(theme::INK, alpha));
    }
}

/// Le coût d'un plan, en pictogrammes (les mêmes que le HUD) ; `y` est la ligne de base des nombres.
fn cost_tokens(x: f32, y: f32, wood: u32, stone: u32, size: f32, wood_short: bool, stone_short: bool) -> f32 {
    let items = [(Resource::Wood, wood, wood_short), (Resource::Stone, stone, stone_short)];
    crate::hud_icons::draw_cost(x, y - size * 0.33, &items, size, theme::INK)
}

fn draw_panel(game: &Game, origin: (f32, f32), collapsed: bool, hover: Option<PlanKind>, selected: Option<PlanKind>) {
    let (x, y, w, h) = (origin.0, origin.1, PANEL_W, panel_height(collapsed));
    theme::panel(x, y, w, h);
    theme::ribbon(x + 22.0, y - 14.0, w - 44.0, "Plans du Roi");
    let (tx, ty, tw, th) = toggle_rect(origin);
    theme::button(tx, ty, tw, th, inside(mouse_position(), tx, ty, tw, th), false);
    theme::ink(if collapsed { "+" } else { "-" }, tx + 7.0, ty + 19.0, 24.0);
    if collapsed {
        return;
    }
    for (i, &kind) in PlanKind::ALL.iter().enumerate() {
        let (cx, cy, cw, ch) = card_rect(origin, i);
        let is_hover = hover == Some(kind);
        let is_selected = selected == Some(kind);
        theme::button(cx, cy, cw, ch, is_hover, is_selected);
        draw_icon(kind, cx + 8.0, cy + 10.0, 56.0);
        let (min_w, min_h) = kind.min_size();
        theme::ink(plan_name(kind), cx + 74.0, cy + 24.0, 24.0);
        draw_text(&format!("Min {}x{}", min_w, min_h), cx + 74.0, cy + 40.0, 17.0, theme::INK_SOFT);
        let cost = game.plan_cost(kind, min_w, min_h, kind.min_beds());
        cost_tokens(cx + 74.0, cy + 62.0, cost.wood, cost.stone, 17.0, false, false);
        let blurb = match kind {
            PlanKind::House => "lits, table, bac d'eau",
            PlanKind::Warehouse => "stocke beaucoup plus",
            PlanKind::Tavern => "retient les vagabonds",
        };
        draw_text(blurb, cx + 74.0, cy + 75.0, 13.0, with_alpha(theme::INK_SOFT, 0.9));
    }
    let hint = "Clic : choisir | Droit : annuler";
    draw_text(hint, x + 24.0, y + h - 14.0, 16.0, theme::INK_SOFT);
}

/// Un petit pictogramme du bâtiment, dans un carré de `s` pixels.
fn draw_icon(kind: PlanKind, x: f32, y: f32, s: f32) {
    let k = s / 56.0;
    let wall = Color::from_rgba(150, 104, 62, 255);
    let wall_dark = Color::from_rgba(104, 70, 40, 255);
    let thatch = Color::from_rgba(214, 180, 96, 255);
    let thatch_dark = Color::from_rgba(170, 138, 70, 255);
    let door = Color::from_rgba(70, 44, 24, 255);
    match kind {
        PlanKind::House => {
            draw_rectangle(x + 8.0 * k, y + 26.0 * k, 40.0 * k, 26.0 * k, wall);
            draw_rectangle(x + 8.0 * k, y + 26.0 * k, 40.0 * k, 4.0 * k, wall_dark);
            draw_triangle(vec2(x + 2.0 * k, y + 28.0 * k), vec2(x + 54.0 * k, y + 28.0 * k), vec2(x + 28.0 * k, y + 6.0 * k), thatch);
            draw_triangle(vec2(x + 28.0 * k, y + 6.0 * k), vec2(x + 54.0 * k, y + 28.0 * k), vec2(x + 28.0 * k, y + 28.0 * k), thatch_dark);
            draw_rectangle(x + 22.0 * k, y + 36.0 * k, 12.0 * k, 16.0 * k, door);
            draw_rectangle(x + 38.0 * k, y + 34.0 * k, 7.0 * k, 7.0 * k, Color::from_rgba(240, 214, 120, 255));
        }
        PlanKind::Warehouse => {
            draw_rectangle(x + 4.0 * k, y + 20.0 * k, 48.0 * k, 32.0 * k, wall);
            draw_rectangle(x + 2.0 * k, y + 14.0 * k, 52.0 * k, 9.0 * k, Color::from_rgba(120, 118, 124, 255));
            draw_rectangle(x + 2.0 * k, y + 14.0 * k, 52.0 * k, 3.0 * k, Color::from_rgba(160, 158, 164, 255));
            draw_rectangle(x + 10.0 * k, y + 30.0 * k, 36.0 * k, 22.0 * k, door);
            for i in 0..3 {
                draw_line(x + (18.0 + 10.0 * i as f32) * k, y + 30.0 * k, x + (18.0 + 10.0 * i as f32) * k, y + 52.0 * k, 1.5, wall_dark);
            }
            draw_rectangle(x + 10.0 * k, y + 38.0 * k, 36.0 * k, 3.0 * k, wall_dark);
        }
        PlanKind::Tavern => {
            draw_rectangle(x + 4.0 * k, y + 22.0 * k, 48.0 * k, 30.0 * k, wall);
            draw_rectangle(x + 4.0 * k, y + 22.0 * k, 48.0 * k, 4.0 * k, wall_dark);
            draw_triangle(vec2(x + 0.0, y + 24.0 * k), vec2(x + 56.0 * k, y + 24.0 * k), vec2(x + 28.0 * k, y + 4.0 * k), Color::from_rgba(150, 70, 56, 255));
            draw_rectangle(x + 10.0 * k, y + 34.0 * k, 11.0 * k, 18.0 * k, door);
            // une enseigne : une chope
            draw_line(x + 34.0 * k, y + 28.0 * k, x + 50.0 * k, y + 28.0 * k, 2.0, wall_dark);
            draw_rectangle(x + 38.0 * k, y + 31.0 * k, 9.0 * k, 11.0 * k, Color::from_rgba(240, 196, 80, 255));
            draw_rectangle(x + 38.0 * k, y + 31.0 * k, 9.0 * k, 3.0 * k, Color::from_rgba(250, 250, 240, 255));
        }
    }
}

/// Le rectangle en surbrillance sur la carte : bleu (payable), rouge (pas assez de ressources), gris (impossible).
fn draw_ghost(game: &Game, view: &View, shape: &Shape) {
    let t = view.tile_px;
    let valid = game.check_plan(shape.kind, shape.x, shape.y, shape.w, shape.h, shape.beds).is_ok();
    let cost = game.plan_cost(shape.kind, shape.w, shape.h, shape.beds);
    let color = if !valid {
        theme::PLAN_BLOCKED
    } else if game.can_afford(cost) {
        theme::PLAN_OK
    } else {
        theme::PLAN_SHORT
    };
    let (px, py) = view.tile_origin(TilePos::new(shape.x, shape.y));
    let (w, h) = (shape.w as f32 * t, shape.h as f32 * t);
    draw_rectangle(px, py, w, h, with_alpha(color, 0.36));
    // le quadrillage des tuiles
    for i in 1..shape.w {
        draw_line(px + i as f32 * t, py, px + i as f32 * t, py + h, 1.0, with_alpha(color, 0.35));
    }
    for j in 1..shape.h {
        draw_line(px, py + j as f32 * t, px + w, py + j as f32 * t, 1.0, with_alpha(color, 0.35));
    }
    if !valid {
        // des hachures : on ne peut pas poser ici
        let step = 14.0;
        let mut d = -h;
        while d < w {
            let (x0, y0) = (px + d.max(0.0), py + (-d).max(0.0));
            let len = (w - d.max(0.0)).min(h - (-d).max(0.0));
            if len > 0.0 {
                draw_line(x0, y0, x0 + len, y0 + len, 2.0, with_alpha(color, 0.55));
            }
            d += step;
        }
    }
    draw_rectangle_lines(px, py, w, h, 3.0, with_alpha(color, 0.95));
    // les marqueurs : la porte (au milieu du mur sud), le tas de matériaux (devant, à gauche)
    let door = TilePos::new(shape.x + shape.w / 2, shape.y + shape.h - 1);
    let (dx, dy) = view.tile_origin(door);
    draw_rectangle(dx + t * 0.18, dy + t * 0.1, t * 0.64, t * 0.9, with_alpha(theme::WOOD_DARK, 0.75));
    let (mx, my) = view.tile_origin(TilePos::new(shape.x + 1, shape.y + shape.h));
    draw_rectangle_lines(mx + t * 0.1, my + t * 0.1, t * 0.8, t * 0.8, 2.0, with_alpha(theme::GOLD_LIGHT, 0.9));
    // les poignées d'angle quand on agrandit
    if shape.sizing {
        for &(cx, cy) in &[(px + w, py + h), (px + w, py), (px, py + h)] {
            draw_rectangle(cx - 5.0, cy - 5.0, 10.0, 10.0, with_alpha(theme::PARCHMENT_LIGHT, 0.95));
            draw_rectangle_lines(cx - 5.0, cy - 5.0, 10.0, 10.0, 2.0, color);
        }
    }
    // la taille au centre
    let label = format!("{}x{}", shape.w, shape.h);
    let size = (22.0 * (t / 32.0)).clamp(16.0, 34.0);
    let tw = measure_text(&label, None, size as u16, 1.0).width;
    draw_text(&label, px + w / 2.0 - tw / 2.0 + 1.0, py + h / 2.0 + size / 3.0 + 1.0, size, with_alpha(BLACK, 0.6));
    draw_text(&label, px + w / 2.0 - tw / 2.0, py + h / 2.0 + size / 3.0, size, WHITE);
}

/// La barre d'information d'un plan en cours : son coût, sa durée, ses lits, et les gestes.
fn draw_bar(game: &Game, shape: &Shape) {
    let (x, y) = bar_origin();
    theme::panel(x, y, BAR_W, BAR_H);
    let valid = game.check_plan(shape.kind, shape.x, shape.y, shape.w, shape.h, shape.beds);
    let cost = game.plan_cost(shape.kind, shape.w, shape.h, shape.beds);
    let secs = game.plan_secs(shape.kind, shape.w, shape.h, shape.beds);
    let (wood_have, stone_have) = (game.available(Resource::Wood), game.available(Resource::Stone));
    let (wood_short, stone_short) = (wood_have < cost.wood, stone_have < cost.stone);

    theme::ink(plan_name(shape.kind), x + 22.0, y + 40.0, 26.0);
    draw_text(&format!("{} x {}", shape.w, shape.h), x + 22.0, y + 60.0, 18.0, theme::INK_SOFT);
    let cost_end = cost_tokens(x + 170.0, y + 38.0, cost.wood, cost.stone, 20.0, wood_short, stone_short);
    // ce que le royaume a : un coffre, puis ses réserves
    let (chest_x, center) = (cost_end + 16.0, y + 38.0 - 20.0 * 0.33);
    crate::hud_icons::draw(crate::hud_icons::Icon::Chest, chest_x, center - 10.0, 2.0, 1.0);
    let stock = [(Resource::Wood, wood_have, false), (Resource::Stone, stone_have, false)];
    crate::hud_icons::draw_cost(chest_x + 26.0, center, &stock, 20.0, theme::INK_SOFT);
    draw_text(
        &format!("Construction : {:.0} s ({:.0} s a deux artisans)", secs, secs / 2.0),
        x + 170.0,
        y + 74.0,
        15.0,
        theme::INK_SOFT,
    );

    // le nombre de lits d'une maison
    if shape.kind == PlanKind::House {
        let (minus, plus) = bed_buttons();
        let mouse = mouse_position();
        let hot = |r: (f32, f32, f32, f32)| inside(mouse, r.0, r.1, r.2, r.3);
        theme::button(minus.0, minus.1, minus.2, minus.3, hot(minus), false);
        theme::button(plus.0, plus.1, plus.2, plus.3, hot(plus), false);
        theme::ink("-", minus.0 + 9.0, minus.1 + 22.0, 26.0);
        theme::ink("+", plus.0 + 8.0, plus.1 + 22.0, 26.0);
        theme::ink(&format!("Lits : {}", shape.beds), minus.0 + 38.0, minus.1 + 22.0, 22.0);
    }

    // l'état du placement, et les gestes
    let (status, ok) = match valid {
        Ok(()) if wood_short || stone_short => ("Il manque des materiaux : le chantier attendra", false),
        Ok(()) => ("Pret : les materiaux seront livres", true),
        Err(e) => (error_text(e), false),
    };
    let status_color = if ok { Color::new(0.12, 0.34, 0.16, 1.0) } else { Color::new(0.62, 0.14, 0.10, 1.0) };
    draw_text(status, x + 22.0, y + BAR_H - 36.0, 16.0, status_color);
    let gesture = if shape.sizing { "Clic : poser | Droit : deplacer | Echap : terminer" } else { "Clic : fixer le coin | Droit : terminer" };
    draw_text(gesture, x + 22.0, y + BAR_H - 20.0, 15.0, theme::INK_SOFT);
}
