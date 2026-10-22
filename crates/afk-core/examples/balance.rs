//! Outil d'équilibrage : simule la cité en mode AFK et affiche des statistiques.
//!
//!   cargo run --release -p afk-core --example balance                 # 6 jours, journée de 600 s, 1 paysan
//!   cargo run --release -p afk-core --example balance -- 8 600 3      # 8 jours, journée de 600 s, 3 paysans
//!
//! Sert à répondre à des questions chiffrées ("le paysan laisse-t-il des parcelles vides ?")
//! plutôt que de deviner en regardant l'écran.

use afk_core::{Activity, Config, Game, PlotState, Resource, Role, Scenario, WorkKind, TICK};

fn main() {
    let args: Vec<f32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let days = args.first().copied().unwrap_or(6.0) as u32;
    let day_secs = args.get(1).copied().unwrap_or(600.0);
    let farmers = args.get(2).copied().unwrap_or(1.0).max(1.0) as usize;

    let mut game = Game::new(
        Scenario::village_with_farmers(farmers),
        1,
        Config { day_length_secs: day_secs, ..Config::default() },
    );
    let farmer_ids: Vec<usize> = game
        .characters()
        .iter()
        .enumerate()
        .filter(|(_, c)| c.role == Role::Farmer)
        .map(|(i, _)| i)
        .collect();
    let n = farmer_ids.len() as f32;
    let total_plots = game.farm().plots().len();

    println!("{days} jours, journée de {day_secs} s, {total_plots} parcelles, {farmers} paysan(s) (IA)\n");
    println!("jour | friche labouree plantee mure | ble  | par paysan (h de jeu) : marche travaille attend dort");

    let mut wheat_after_day2 = 0;
    for day in 1..=days {
        let (mut walk, mut work, mut idle, mut sleep) = (0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32);
        let mut work_by_kind = [0.0_f32; 3];
        let mut empty_sum = 0u64;
        let mut samples = 0u64;

        while game.clock().day() == day {
            game.step(TICK);
            let dt_h = game.clock().hours_per_second() * TICK;
            for &i in &farmer_ids {
                let c = &game.characters()[i];
                match c.activity {
                    Activity::Walking => walk += dt_h,
                    Activity::Working => work += dt_h,
                    Activity::Idle => idle += dt_h,
                    Activity::Sleeping => sleep += dt_h,
                }
                if let Some(w) = c.work {
                    let k = match w.kind {
                        WorkKind::Till => 0,
                        WorkKind::Plant => 1,
                        _ => 2,
                    };
                    work_by_kind[k] += dt_h;
                }
            }
            if !game.clock().is_night() {
                empty_sum += game
                    .farm()
                    .plots()
                    .iter()
                    .filter(|p| matches!(p.state, PlotState::Fallow | PlotState::Tilled))
                    .count() as u64;
                samples += 1;
            }
        }

        let mut counts = [0; 4];
        for p in game.farm().plots() {
            match p.state {
                PlotState::Fallow => counts[0] += 1,
                PlotState::Tilled => counts[1] += 1,
                PlotState::Planted { .. } => counts[2] += 1,
                PlotState::Ripe => counts[3] += 1,
            }
        }
        let wheat = game.stock().get(Resource::Wheat) + game.in_hands(Resource::Wheat);
        if day == 2 {
            wheat_after_day2 = wheat;
        }
        println!(
            "{day:>4} | {:>5} {:>8} {:>7} {:>4} | {:>4} | {:>6.1} {:>9.1} {:>6.1} {:>4.1}   (labour {:.1}, semis {:.1}, recolte {:.1})  parcelles vides le jour : {:.1}",
            counts[0], counts[1], counts[2], counts[3], wheat,
            walk / n, work / n, idle / n, sleep / n,
            work_by_kind[0] / n, work_by_kind[1] / n, work_by_kind[2] / n,
            empty_sum as f64 / samples.max(1) as f64,
        );
        let trees: Vec<String> = game
            .forests()
            .iter()
            .map(|f| format!("{}/{}", f.trees(game.map()).len(), f.w * f.h - ((f.w + 2) / 3) * f.h))
            .collect();
        println!(
            "     bois {:>3} (arbres debout par foret : {})  fer {}  or {}  pierre {}",
            game.stock().get(Resource::Wood),
            trees.join(", "),
            game.stock().get(Resource::Iron),
            game.stock().get(Resource::Gold),
            game.stock().get(Resource::Stone),
        );
    }

    if days > 2 {
        let final_wheat = game.stock().get(Resource::Wheat) + game.in_hands(Resource::Wheat);
        println!(
            "\nrécolte en régime établi (jours 3 à {days}) : {:.0} blé par jour",
            (final_wheat - wheat_after_day2) as f32 / (days - 2) as f32
        );
    }
}
