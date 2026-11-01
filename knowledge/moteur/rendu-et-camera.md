---
type: Module
title: Rendu, caméra et entrées
description: Organisation du crate afk-game, ordre de dessin d'une image, caméra à zoom entier, traduction des entrées.
tags: [moteur, rendu, camera]
sources:
  - resource: crates/afk-game/src/render.rs
    title: View, draw
  - resource: crates/afk-game/src/hud.rs
    title: HUD en pictogrammes, aide
  - resource: crates/afk-game/src/hud_icons.rs
    title: pictogrammes 10x10
  - resource: crates/afk-game/src/camera.rs
    title: caméra et zoom
  - resource: crates/afk-game/src/input.rs
    title: entrées vers Command
generated: { by: claude-code/claude-sonnet-5-5, at: 2026-10-30T12:00:00Z }
---

# Rendu, caméra et entrées

Crate `afk-game` (macroquad). Aucune règle de jeu ici.

- **Nouveaux modules** : `plan_ui` (le panneau des plans et la pose d'un plan à la souris), `theme` (le thème médiéval : bois, parchemin, filet d'or, rivets), `construction_view` (le feu de camp, les couchages, les fondations, les planches qui recouvrent la zone, l'enseigne d'un chantier). `habitat_view` dessine aussi l'entrepôt et la taverne (`draw_buildings`).
- **Modules** : `main` (fenêtre 1024x640, boucle, options `--day-seconds` et `--speed`), `input` (touches et souris vers `Command`), `camera` (suivi, glissement, zoom), `render` (`View`, dessin du monde), `hud` (le HUD : royaume, personnage incarné, horloge, ressources, aide), `hud_icons` (les pictogrammes 10x10), `sprites` (palette et dessins 16 px), `habitat_view` (maisons en coupe, lumière), `character_sprites` (gabarit des personnages), `character_view` (planches de personnages), `spritegen` (SVG vers PNG, feature `spritegen`).
- **Ordre de dessin** (`render::draw`) : fond, carte, filons, champs, habitats, entrepôts et tavernes, chantiers, feu de camp, cible, personnages, coffres et leurs compteurs, étincelles de forge, voile de lumière du jour, lumières des habitats et du camp, rectangle du plan en cours, HUD, panneau des plans, aide (touche H ou F1).
- **L'interface du roi** : `main` appelle `PlanUi::update` avant `input::poll` : un clic pris par le panneau n'incarne personne (`captures_mouse`), et Échap qui annule un plan ne quitte pas le jeu (`consumed_escape`). Le frontend n'envoie que `Command::PlacePlan`. Hors du roi, aucun panneau n'est affiché. Le panneau est déplaçable et réductible (`PlanUi.panel`, `collapsed`) ; un clic droit bref annule l'étape du plan (un glissé tire toujours la carte).
- **Vue** (`View`) : `tile_px` (palier de zoom), conversion monde vers écran, déplacement à la souris (la vue devient libre), glissement vers le personnage incarné (`SLIDE_SECS`), marge de suivi (`FOLLOW_MARGIN`).
- **Zoom** : paliers entiers (voir [la direction artistique](/graphisme/direction-artistique.md)) pour que le pixel art reste net ; le zoom garde le point sous le curseur.
- Les commandes passent par `afk_core::Command` (voir [l'architecture](/moteur/architecture.md)). La vitesse de simulation en jeu (touche `T`) agit sur le frontend, pas sur le cœur.
