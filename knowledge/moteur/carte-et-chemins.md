---
type: Module
title: Carte et chemins
description: Types de tuiles, recherche de chemin sur routes ou en direct, coût de l'herbe.
tags: [moteur, carte, pathfinding]
sources:
  - resource: crates/afk-core/src/map.rs
    title: TileMap, Tile, PathMode, find_path
  - resource: crates/afk-core/src/scenario.rs
    title: disposition de départ
generated: { by: claude-code/claude-sonnet-5-5, at: 2026-10-30T12:00:00Z }
---

# Carte et chemins

Source : `crates/afk-core/src/map.rs`. Le royaume fait 64x40 tuiles (voir [le royaume](/regles/royaume.md)).

- **Tuiles** (`Tile`) : Grass, Path, Water, Field, Wall, Floor, Tree, Sapling, Bed, Anvil, Ore, Crate, Furniture. Certaines sont solides (mur, arbre, filon, coffre, enclume, meuble) ; le lit se traverse.
- **Chemins** : A* à 8 voisins, diagonales à coût 1,414 et ordre de voisins fixe pour départager de façon déterministe (`NEIGHBORS`).
- **Deux modes** (`PathMode`) : `Roads(grass_cost)` pour les trajets ordinaires (route, sol intérieur et lit coûtent 1, l'herbe `grass_cost`, 3 par défaut) ; `Direct` (tout coûte 1) pour la fuite d'un voleur et ses poursuivants (pas encore en jeu, voir [le garde](/metiers/garde.md)).
- `find_path_cost` renvoie le chemin **et** son coût : c'est la façon de mesurer une distance réelle entre deux lieux.
- Les coordonnées : `TilePos` (entières) et `Vec2` (flottantes, en tuiles), dans `math.rs`.
