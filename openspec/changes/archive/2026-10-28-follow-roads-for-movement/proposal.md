# Proposal

## Why

Aujourd'hui les habitants marchent en ligne droite sur l'herbe comme sur la route : la recherche de chemin ne regarde que si une case est praticable, en 4 directions seulement. Les routes du royaume ne servent donc à rien, les trajets forment des escaliers (horizontal puis vertical) et rien ne distingue un habitant qui rentre se coucher d'un voleur qui s'enfuit. Il faut que les routes aient un rôle, que les déplacements soient plus naturels, et préparer les poursuites entre voleurs et gardes (voir `IDEAS.md`, « Bandits »).

## What Changes

- Les habitants **suivent les routes** pour se déplacer : à distance égale (ou à peine plus longue), ils préfèrent la route à l'herbe. Ils ne quittent la route que pour atteindre un but qui n'y est pas.
- Un but ou un coffre **hors des routes** reste atteignable : les transporteurs (et tous les autres) quittent la route pour la dernière portion du trajet.
- Les déplacements ne sont plus limités à l'horizontal et au vertical : les **diagonales** sont permises, et le trajet choisi est le plus court au sens du coût (route peu coûteuse, herbe plus coûteuse).
- Un **voleur** qui a volé quelque chose rejoint la sortie du château par le **plus court chemin**, sans suivre les routes. Les **gardes** qui le poursuivent prennent eux aussi le plus court chemin, sans suivre les routes.
- Une personne qui **va se coucher** emprunte les routes.
- Hors périmètre : l'apparition des voleurs/bandits, ce qu'ils volent, les combats et la capture (idées dans `IDEAS.md`) ; cette capacité ne fixe que **comment ils se déplacent** quand ils existent. Le roi qui trace de nouvelles routes n'est pas couvert.

## Capabilities

### New Capabilities
- `road-movement`: modes de déplacement (suivre les routes / plus court chemin), diagonales, et quel mode s'applique à quelle situation (travail, coucher, fuite d'un voleur, poursuite par les gardes).

### Modified Capabilities

## Impact

- `crates/afk-core/src/map.rs` : recherche de chemin avec coûts et 8 directions (remplace le parcours en largeur 4 directions) ; mode de déplacement en paramètre.
- `crates/afk-core/src/character.rs` : les appels à `find_path` (≈25) choisissent un mode ; `go_to_bed` en mode routes ; suivi de chemin sans coin coupé à travers un obstacle.
- `crates/afk-core/src/config.rs` : coût de l'herbe par rapport à la route.
- Tests de `character.rs`, `game.rs`, `transport.rs` dont les durées de trajet changent.
- `docs/gameplay/royaume.md`, `garde.md`, `transporteur.md`, `temps-et-repos.md`, `IDEAS.md`.
