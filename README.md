# All For Kingdom (AFK)

Jeu de gestion/RPG en vue de dessus : on incarne tour à tour les habitants d'une cité médiévale
(paysan, forgeron, roi, chef de guerre...) pendant que l'IA fait tourner les autres.
Sans personne d'incarné, la cité tourne seule : c'est le mode AFK.

## Lancer

    cargo run                          # jeu, journée de 10 minutes
    cargo run -- --day-seconds 10      # journée de 10 secondes (tout le rythme de vie suit)
    cargo run -- --speed 20            # journée normale, vue en accéléré
    cargo test                         # tests de la logique
    cargo run --release -p afk-core --example balance -- 8 600 2   # outil d'équilibrage : 8 jours, 2 paysans

`--day-seconds N` change la durée d'une journée de jeu (défaut 600 s). Les durées de fatigue et de
sommeil sont exprimées en heures de jeu : elles suivent le cycle. Les déplacements, eux, restent en
secondes réelles, donc avec une journée très courte les gens n'ont plus le temps de rentrer chez eux ;
pour voir un cycle cohérent en accéléré, `--speed` est plus adapté. La touche `T` change la vitesse en jeu (x1, x10, x60).

Tous les autres réglages (heure de départ, aube, crépuscule, rythme de fatigue...) sont dans
`crates/afk-core/src/config.rs`.

## Contrôles

| Touche | Action |
|---|---|
| ZQSD / WASD / flèches | bouger |
| Tab, 1 à 9, clic | incarner un personnage |
| E | travailler la parcelle visée (paysan) / se coucher près de son lit / se lever / annuler le travail en cours |
| Espace | mode AFK on/off |
| T | vitesse de simulation (debug) |
| Échap | quitter |

## Règles actuelles

**Temps et repos**
- Cycle jour/nuit : aube à 6 h, crépuscule à 20 h. La nuit, l'IA rentre dormir dans son lit.
- La jauge de **forme** (pleine = en forme) se vide quand le personnage se fatigue en marchant ou en travaillant,
  et se remplit en dormant. Sous 50 de forme, il va plus lentement et travaille moins vite.
  À 15 de forme, l'IA va se coucher même en plein jour. (En interne, le jeu compte la fatigue, l'inverse de la forme.)
- Le personnage incarné n'est jamais couché de force : on peut veiller, au prix de la forme.
  On se couche soi-même avec `E` près de son lit (la nuit, ou si on est fatigué).
- Un dormeur continue de dormir quand on incarne quelqu'un d'autre.

**Le paysan**
- Chaque tuile de champ est une parcelle : `en friche -> labourée -> plantée -> (pousse) -> mûre -> récoltée`.
- Avec `E`, le joueur travaille la parcelle devant lui (sinon celle sous ses pieds) ; une barre de progression
  s'affiche, et bouger ou rappuyer sur `E` annule. La parcelle visée est entourée en blanc.
- Le joueur travaille **2 fois plus vite** que l'IA (`player_work_speed`). La fatigue ralentit tout le monde.
- La pousse (12 h de jeu) ne dépend que du temps : personne ne peut l'accélérer.
- L'IA du paysan travaille seule : récolte d'abord, puis semis, puis labour, parcelle par parcelle.
  Récolter augmente le stock de blé, affiché en haut à droite (simple compteur pour l'instant).
- Les durées de labour, semis et récolte sont en secondes réelles ; la pousse est en heures de jeu.
  Tout se règle dans `crates/afk-core/src/config.rs`.
- Plusieurs paysans se répartissent les parcelles : chacun réserve celle qu'il vise, et les autres l'évitent
  (sans ça, tous se ruaient sur la même et en avoir 4 ne rapportait pas plus qu'en avoir 1).

**Le forgeron**
- L'enclume est un obstacle : on forge depuis une case voisine en la regardant (bouger vers elle suffit à la
  regarder), puis `E`. Chaque coup de forge dure 8 s (4 s pour le joueur), consomme 2 de fer et produit 1 outil.
- L'IA va seule à côté de l'enclume et forge tant qu'il reste du fer, puis traîne dans sa forge.
- **Le fer est fini** (60 au départ, aucune mine pour l'instant) : l'IA le transforme en outils en moins d'un jour.
  Réglable avec `starting_iron`, `iron_per_tool`, `forge_secs` dans `config.rs`.

**Les gardes**
- Deux gardes dorment à la caserne et font des rondes dans l'ordre le long des routes, avec une pause à chaque
  point : un garde de jour (de 6 h à 20 h) et un garde de nuit (de 20 h à 6 h, il dort le jour).
- Le « quart de nuit » est une propriété de n'importe quel personnage (`night_shift`) : son heure de dormir
  est inversée. Le joueur qui l'incarne n'est jamais couché de force, comme les autres.

**Équilibrage mesuré** (outil `balance`, 48 parcelles, journées de 10 minutes) :

| Paysans | Blé par jour | Parcelles vides le jour | Attente par paysan |
|---|---|---|---|
| 1 | 116 | ~15 | 0,1 h |
| 2 | 202 | ~2 | 0,8 h |
| 3 | 215 | ~1 | 3,4 h |
| 4 | 221 | ~1 | 5,3 h |

Un seul paysan est saturé (13 h de travail par jour pour 29 cycles sur 48 parcelles) ; deux suffisent pour
cette surface ; au-delà c'est la terre qui plafonne (12 h de pousse dont une partie la nuit) et les paysans
en trop n'ont plus rien à faire, ce qui sera utile pour répartir les gens entre métiers.

## Architecture

    crates/
      afk-core/   logique pure : carte, personnages, IA, horloge, règles. AUCUNE dépendance.
      afk-game/   frontend macroquad : fenêtre, rendu, entrées. Seul endroit qui connaît macroquad.

Règles à garder :

1. `afk-core` ne dépend jamais d'une bibliothèque de rendu, d'entrées, de son ou de réseau.
2. Le monde est en **tuiles**, jamais en pixels. La taille d'affichage est l'affaire du rendu.
3. Le cœur ne contient ni couleur, ni sprite, ni nom de touche : seulement des états
   (`Role`, `Tile`, `Activity`, `Facing`, `fatigue`, `daylight`...) que le rendu traduit en images.
4. Les entrées arrivent au cœur sous forme de `Command` (intentions), jamais de touches brutes.
5. La simulation avance par pas fixes (`Game::step(TICK)`), indépendamment de la fréquence d'affichage.
6. Lecture de l'état par accesseurs en lecture seule ; seules `apply` et `step` le modifient.

Changer de bibliothèque de rendu = réécrire `render.rs`, `input.rs` et `main.rs` dans `afk-game`.
Rien dans `afk-core` ne bouge.

## Feuille de route

- [x] Jalon 1 : cycle jour/nuit, maisons et lits, fatigue, routine de l'IA (dormir/travailler)
- [x] Jalon 2 : le paysan (parcelles, labourer/planter/récolter), travail joueur 2x plus rapide, stock de blé affiché
- [x] Jalon 3 : forgeron (forger, consomme du fer) et gardes (patrouilles, quart de jour et de nuit)
- À venir : une source de fer (mine ou commerce), décrets du roi qui orientent les priorités de l'IA
  (que forger, combien de paysans), roi / chef de guerre / tavernier avec un vrai travail
- Plus tard : sprites en atlas, vraie lumière (carte de lumière), coop réseau
