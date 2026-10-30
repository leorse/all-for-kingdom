---
type: Module
title: Rendu, caméra et entrées
description: Organisation du crate afk-game, ordre de dessin d'une image, caméra à zoom entier, traduction des entrées.
tags: [moteur, rendu, camera]
sources:
  - resource: crates/afk-game/src/render.rs
    title: View, draw, HUD
  - resource: crates/afk-game/src/camera.rs
    title: caméra et zoom
  - resource: crates/afk-game/src/input.rs
    title: entrées vers Command
generated: { by: claude-code/claude-sonnet-5-5, at: 2026-10-30T12:00:00Z }
---

# Rendu, caméra et entrées

Crate `afk-game` (macroquad). Aucune règle de jeu ici.

- **Modules** : `main` (fenêtre 1024x640, boucle, options `--day-seconds` et `--speed`), `input` (touches et souris vers `Command`), `camera` (suivi, glissement, zoom), `render` (`View`, dessin du monde et HUD), `sprites` (palette et dessins 16 px), `habitat_view` (maisons en coupe, lumière), `character_sprites` (gabarit des personnages), `character_view` (planches de personnages), `spritegen` (SVG vers PNG, feature `spritegen`).
- **Ordre de dessin** (`render::draw`) : fond, carte, filons, champs, habitats, cible, personnages, coffres et leurs compteurs, étincelles de forge, voile de lumière du jour, lumières des habitats, HUD.
- **Vue** (`View`) : `tile_px` (palier de zoom), conversion monde vers écran, déplacement à la souris (la vue devient libre), glissement vers le personnage incarné (`SLIDE_SECS`), marge de suivi (`FOLLOW_MARGIN`).
- **Zoom** : paliers entiers (voir [la direction artistique](/graphisme/direction-artistique.md)) pour que le pixel art reste net ; le zoom garde le point sous le curseur.
- Les commandes passent par `afk_core::Command` (voir [l'architecture](/moteur/architecture.md)). La vitesse de simulation en jeu (touche `T`) agit sur le frontend, pas sur le cœur.
