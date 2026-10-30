---
type: Module
title: Configuration et unités
description: Config::default, le seul endroit des réglages d'équilibrage, et la distinction heures de jeu / secondes réelles.
tags: [moteur, equilibrage, config]
sources:
  - resource: crates/afk-core/src/config.rs
    title: Config et ses valeurs par défaut
  - resource: crates/afk-core/src/clock.rs
    title: horloge
  - resource: crates/afk-core/examples/balance.rs
    title: simulation d'équilibrage
generated: { by: claude-code/claude-sonnet-5-5, at: 2026-10-30T12:00:00Z }
---

# Configuration et unités

`Config` (`crates/afk-core/src/config.rs`) regroupe **tout ce qui s'équilibre**, commenté champ par champ. **Les valeurs ne sont volontairement pas recopiées ici** : les lire dans `Config::default()`, c'est la seule source qui ne se périme pas.

## Groupes de réglages

Cycle (durée de la journée, heure de départ, aube, crépuscule) ; fatigue (gain actif, gain à l'arrêt, récupération en dormant, seuils d'épuisement et de réveil, portée du lit) ; travail (facteur du joueur, durées de labour, semis, récolte, pousse, gain de blé) ; forge ; mine (coups par qualité, durée d'un coup) ; forêt (durée d'abattage, bûches, repousse) ; contenants et transport (capacité, charge, vitesse des transporteurs, portées à la main) ; déplacements (`grass_cost`).

## Les unités : le piège principal

- **Heures de jeu** : fatigue, sommeil, pousse des cultures, repousse des arbres. Elles suivent la longueur de la journée.
- **Secondes réelles** : déplacements et durées de travail. Elles ne suivent pas la longueur de la journée.
- Avec `day_length_secs = 600` (défaut), **1 h de jeu = 25 s réelles**. Raccourcir la journée rend les trajets proportionnellement plus longs par rapport au cycle (`--day-seconds`). La touche `T` accélère la simulation (x1, x10, x60) sans changer ces rapports.

## Simulation d'équilibrage

`cargo run --release -p afk-core --example balance -- <jours> <secondes_par_jour> <paysans>` fait tourner N jours avec M paysans IA et affiche, par jour, l'état des parcelles, le blé, et le temps de marche, de travail, d'attente et de sommeil par paysan. Pour d'autres métiers, écrire un exemple voisin dans `crates/afk-core/examples/`.

Voir aussi [le temps et le repos](/regles/temps-et-repos.md) pour les règles côté joueur.
