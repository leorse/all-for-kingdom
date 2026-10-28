//! All For Kingdom (AFK) — frontend macroquad.
//! Rôle de ce crate : fenêtre, boucle de jeu, entrées, rendu. Aucune règle de jeu ici.
//!
//! Options de lancement :
//!   --day-seconds N   durée d'une journée de jeu en secondes réelles (défaut : 600 = 10 minutes)
//!   --speed X         accélère la simulation X fois (défaut : 1). La touche T change la vitesse en jeu.
//!
//! Exemples :
//!   cargo run -- --day-seconds 10     journée de 10 s (tout le rythme de vie suit)
//!   cargo run -- --speed 20           journée normale vue en accéléré (30 s)

mod camera;
mod habitat_view;
mod input;
mod render;
mod sprites;

use afk_core::{CharacterId, Command, Config, Game, PlayerId, Scenario, TICK};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "All For Kingdom (AFK)".to_string(),
        window_width: 1024,
        window_height: 640,
        ..Default::default()
    }
}

/// Valeur numérique qui suit `name` dans la ligne de commande.
fn arg_f32(name: &str) -> Option<f32> {
    let args: Vec<String> = std::env::args().collect();
    let i = args.iter().position(|a| a == name)?;
    args.get(i + 1)?.parse().ok()
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut config = Config::default();
    if let Some(secs) = arg_f32("--day-seconds") {
        config.day_length_secs = secs.max(1.0);
    }
    let mut speed = arg_f32("--speed").unwrap_or(1.0).clamp(0.1, 200.0);

    let me = PlayerId(0);
    let mut game = Game::new(Scenario::village(), 1, config);
    game.apply(Command::Possess { player: me, character: CharacterId(0) });

    let mut view = render::View::new();
    // les sprites des habitats : construits une fois, la fenêtre étant ouverte
    let art = habitat_view::Art::new();
    // Le personnage que la caméra suit : le dernier incarné (elle continue de le suivre en mode AFK).
    let mut followed: Option<CharacterId> = None;

    // Pas fixe pour la simulation : le comportement ne dépend pas de la vitesse d'affichage.
    let mut accumulator = 0.0_f32;

    // Position de la souris à l'image précédente tant que le bouton droit est tenu (pour mesurer le déplacement).
    let mut last_mouse: Option<(f32, f32)> = None;
    // Un personnage était-il incarné à l'image précédente ? (Espace en quitte puis en reprend un.)
    let mut was_possessed = true;

    loop {
        let input = input::poll(me, &view, &game);
        if input.quit {
            break;
        }
        if input.cycle_speed {
            speed = if speed < 5.0 {
                10.0
            } else if speed < 30.0 {
                60.0
            } else {
                1.0
            };
        }
        for command in input.commands {
            game.apply(command);
        }

        // la vue à la souris : bouton droit maintenu = on tire la carte ; molette = zoom autour du curseur
        if input.right_down {
            if let Some(last) = last_mouse {
                view.pan_pixels(input.mouse.0 - last.0, input.mouse.1 - last.1);
            }
            last_mouse = Some(input.mouse);
        } else {
            last_mouse = None;
        }
        if input.wheel != 0 {
            view.zoom(input.wheel, input.mouse);
        }

        accumulator = (accumulator + get_frame_time().min(0.1) * speed).min(10.0);
        while accumulator >= TICK {
            game.step(TICK);
            accumulator -= TICK;
        }

        // la caméra : glisse vers un nouveau personnage incarné, puis le suit
        let mut recenter = false;
        if let Some(id) = game.possessed_by(me) {
            recenter = followed.is_some() && followed != Some(id);
            followed = Some(id);
        }
        // vue libre : le joueur reprend son personnage en le déplaçant ou avec Espace (changer de personnage recentre déjà)
        let possessed = game.possessed_by(me).is_some();
        let moves_it = input.moving && !input.right_down && possessed;
        let takes_it_back = possessed && !was_possessed; // Espace : on reprend le dernier personnage
        if view.is_free() && (moves_it || takes_it_back) {
            recenter = true;
        }
        was_possessed = possessed;
        let target = followed.and_then(|id| game.character(id)).map(|c| c.pos);
        view.update(&game, target, recenter, get_frame_time());

        render::draw(&game, &view, &art, me, speed);
        next_frame().await;
    }
}
