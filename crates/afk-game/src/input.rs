//! Traduction des touches et de la souris en `Command` pour le cœur de jeu.
//! Le cœur ne voit jamais une touche : seulement des intentions ("bouger vers", "incarner").

use crate::render::View;
use afk_core::{CharacterId, Command, Game, PlayerId, Vec2};
use macroquad::prelude::*;

pub struct Input {
    pub commands: Vec<Command>,
    pub quit: bool,
    /// Touche de debug : passer à la vitesse de simulation suivante (affecte le frontend, pas le cœur).
    pub cycle_speed: bool,
    /// Une touche de déplacement est tenue : si la vue était libre, la caméra reprend le personnage.
    pub moving: bool,
    /// Bouton droit maintenu : la carte suit la souris (voir `View::pan_pixels`).
    pub right_down: bool,
    /// Position de la souris, en pixels écran.
    pub mouse: (f32, f32),
    /// Sens de la molette : positif pour zoomer, négatif pour dézoomer, 0 sinon. Seul le signe compte.
    pub wheel: i32,
}

pub fn poll(player: PlayerId, view: &View, game: &Game) -> Input {
    let mut commands = Vec::new();

    // Déplacement : flèches, WASD (QWERTY) ou ZQSD (AZERTY).
    let mut dir = Vec2::ZERO;
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) || is_key_down(KeyCode::Q) {
        dir.x -= 1.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        dir.x += 1.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::Z) {
        dir.y -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        dir.y += 1.0;
    }
    commands.push(Command::Move { player, dir });
    let moving = dir != Vec2::ZERO;

    if is_key_pressed(KeyCode::Tab) {
        commands.push(Command::PossessNext { player });
    }
    if is_key_pressed(KeyCode::Space) {
        commands.push(Command::ToggleIdle { player });
    }
    if is_key_pressed(KeyCode::E) {
        commands.push(Command::Interact { player });
    }
    if is_key_pressed(KeyCode::C) {
        commands.push(Command::EmptyDepot { player }); // vide les coffres de l'entrepôt central
    }

    // Choix direct : rangée de chiffres ou pavé numérique (les 9 premiers personnages).
    const DIGITS: [(KeyCode, KeyCode); 9] = [
        (KeyCode::Key1, KeyCode::Kp1),
        (KeyCode::Key2, KeyCode::Kp2),
        (KeyCode::Key3, KeyCode::Kp3),
        (KeyCode::Key4, KeyCode::Kp4),
        (KeyCode::Key5, KeyCode::Kp5),
        (KeyCode::Key6, KeyCode::Kp6),
        (KeyCode::Key7, KeyCode::Kp7),
        (KeyCode::Key8, KeyCode::Kp8),
        (KeyCode::Key9, KeyCode::Kp9),
    ];
    for (i, (key, keypad)) in DIGITS.iter().enumerate() {
        if is_key_pressed(*key) || is_key_pressed(*keypad) {
            commands.push(Command::Possess { player, character: CharacterId(i) });
        }
    }

    // Clic sur un personnage.
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(character) = game.character_at(view.to_world(mouse_position())) {
            commands.push(Command::Possess { player, character });
        }
    }

    // un cran de molette = un palier ; seul le sens compte (les unités varient selon la plate-forme)
    let wheel_y = mouse_wheel().1;
    let wheel = if wheel_y > 0.0 { 1 } else if wheel_y < 0.0 { -1 } else { 0 };

    Input {
        commands,
        quit: is_key_pressed(KeyCode::Escape),
        cycle_speed: is_key_pressed(KeyCode::T),
        moving,
        right_down: is_mouse_button_down(MouseButton::Right),
        mouse: mouse_position(),
        wheel,
    }
}
