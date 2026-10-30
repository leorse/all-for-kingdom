---
type: Module
title: Architecture du moteur
description: Deux crates (afk-core pour les règles, afk-game pour l'affichage), contrat par Command et pas fixe, déterminisme.
tags: [moteur, architecture]
sources:
  - resource: crates/afk-core/src/game.rs
    title: Game, Command, TICK
  - resource: crates/afk-core/src/lib.rs
    title: liste des modules
  - resource: crates/afk-game/src/main.rs
    title: boucle de jeu et options de lancement
generated: { by: claude-code/claude-sonnet-5-5, at: 2026-10-30T12:00:00Z }
---

# Architecture du moteur

Deux crates Rust :

- **`afk-core`** (`crates/afk-core`) : toutes les **règles du jeu**, sans aucune dépendance graphique. Modules : `character`, `clock`, `config`, `container`, `farm`, `forest`, `game`, `habitat`, `map`, `math`, `mine`, `resources`, `rng`, `scenario`, `transport`, `work`. Voir [la configuration](/moteur/configuration.md), [les personnages](/moteur/personnages-et-deplacement.md), [la carte](/moteur/carte-et-chemins.md) et [les coffres et le transport](/moteur/coffres-et-transport.md).
- **`afk-game`** (`crates/afk-game`) : le **frontend macroquad** (fenêtre, boucle, entrées, rendu). Seul crate qui connaît macroquad ; **aucune règle de jeu ici**. Voir [le rendu et la caméra](/moteur/rendu-et-camera.md).

## Contrat entre les deux

- Les entrées arrivent au cœur sous forme de `Command` (`Move`, `Possess`, `PossessNext`, `ToggleIdle`, `Interact`, `EmptyDepot`), jamais de touches ni de clics bruts. `input.rs` fait la traduction.
- Le temps avance par **pas fixes** : `Game::step(dt)`, avec `TICK = 1/60 s`.
- L'état se lit par des accesseurs en lecture seule (`map()`, `characters()`, `containers()`, `clock()`, `config()`, `veins()`, `forests()`, `habitats()`...).
- Plusieurs joueurs sont déjà prévus (`PlayerId`, chaque joueur possède au plus un personnage). Aucun joueur qui possède quelqu'un = **mode AFK**.
- **Déterminisme** : `Rng` est un xorshift64 à graine fixe ; même graine, même partie. C'est ce qui rend les tests et la simulation d'équilibrage fiables.
- Le **scénario** (`Scenario`) est une donnée pure : la disposition de départ du royaume, décrite en dur (voir [le royaume](/regles/royaume.md)).

## Outils

- `cargo run`, options `--day-seconds N` et `--speed X` ; `cargo test` ; simulation d'équilibrage `cargo run --release -p afk-core --example balance -- <jours> <s_par_jour> <paysans>` (voir [la configuration](/moteur/configuration.md)).
- `sprites.bat` : conversion SVG vers PNG des personnages (voir [la direction artistique](/graphisme/direction-artistique.md)).
