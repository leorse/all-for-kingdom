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

mod input;
mod render;

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

    // Pas fixe pour la simulation : le comportement ne dépend pas de la vitesse d'affichage.
    let mut accumulator = 0.0_f32;

    loop {
        let view = render::View::fit(&game);

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

        accumulator = (accumulator + get_frame_time().min(0.1) * speed).min(10.0);
        while accumulator >= TICK {
            game.step(TICK);
            accumulator -= TICK;
        }

        render::draw(&game, &view, me, speed);
        next_frame().await;
    }
}
