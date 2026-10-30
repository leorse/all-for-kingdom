---
type: Module
title: Personnages et déplacement
description: Rôles, activités, vitesses du joueur et de l'IA, efficacité due à la fatigue, routines par métier.
tags: [moteur, personnages, deplacement]
sources:
  - resource: crates/afk-core/src/character.rs
    title: Character, Role, routines, vitesses
  - resource: crates/afk-core/src/work.rs
    title: WorkKind et durées
generated: { by: claude-code/claude-sonnet-5-5, at: 2026-10-30T12:00:00Z }
---

# Personnages et déplacement

Source : `crates/afk-core/src/character.rs`.

- **Rôles** (`Role`) : `Farmer`, `Blacksmith`, `King`, `Warlord`, `Innkeeper`, `Guard`, `Miner`, `Lumberjack`, `Transporter`. Chaque rôle a une routine d'IA (`farm_routine`, `forge_routine`, `mine_routine`, `chop_routine`, `transport_routine`, `patrol_routine`...). **Il n'y a pas de rôle artisan en jeu** (voir [l'artisan](/metiers/artisan.md)).
- **Activités** (`Activity`) : `Idle`, `Walking`, `Working`, `Sleeping`. Orientation : `Facing` (Down, Up, Left, Right).
- **Vitesses** : `PLAYER_SPEED` (joueur) et `AI_SPEED` (IA, deux fois plus lente), en tuiles par seconde ; les transporteurs sont multipliés par `transporter_speed_factor`. Valeurs : voir le fichier.
- **Efficacité** (`efficiency()`) : 1 jusqu'à 50 de fatigue, puis décroît linéairement jusqu'à 0,5 à 100. Elle multiplie la vitesse de marche **et** la vitesse de travail.
- **Le terrain ne ralentit pas** : route ou herbe, même vitesse. Il agit seulement sur le choix du chemin (voir [la carte et les chemins](/moteur/carte-et-chemins.md)). Temps de trajet = longueur réelle du chemin / (vitesse x facteur x efficacité).
- **Travail** : `WorkKind` (Till, Plant, Harvest, Forge, Mine, Chop...) et `work_secs` donnent les durées ; le joueur avance `player_work_speed` fois plus vite. Un travail progresse de `dt / durée x vitesse x efficacité`.
- **Collision** : boîte de demi-côté `BODY_HALF`. Les virages sont arrondis (`CORNER_RADIUS`).
- Le joueur incarné n'est jamais couché de force (voir [le temps et le repos](/regles/temps-et-repos.md)).

Le jeu appelle `Character::update(env, control, rng)` à chaque pas : `control` vaut `Some(direction)` pour un personnage incarné, `None` pour l'IA.
