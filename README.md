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
| Tab, 1 à 9, clic | incarner un personnage (la caméra glisse jusqu'à lui) |
| E | travailler la parcelle visée (paysan) / forger (forgeron) / miner le filon visé (mineur) / abattre l'arbre visé (bûcheron) / se coucher près de son lit / se lever / annuler le travail en cours |
| C | vider l'entrepôt central (toutes ses ressources disparaissent) |
| Espace | mode AFK on/off (en vue libre, reprend aussi le dernier personnage) |
| Bouton droit maintenu + souris | déplacer la vue : on tire la carte |
| Molette | zoomer / dézoomer autour du curseur |
| T | vitesse de simulation (debug) |
| Échap | quitter |

## Le royaume et la caméra

Le royaume (64×40 tuiles) est plus grand que l'écran : les bâtiments publics sont au centre, les maisons
et les champs sont loin, vers les bords. Le zoom part de 32 px par tuile. La caméra suit le personnage
incarné : elle glisse rapidement vers lui quand on change de personnage, puis ne défile que lorsqu'il
s'approche du bord de l'écran. En mode AFK, elle continue de suivre le dernier personnage incarné.

**À la souris** : en maintenant le bouton droit, on tire la carte (elle reste bornée à ses bords) ; la molette
change de palier de zoom (16, 20, 24, 32, 40, 48 ou 64 px par tuile) autour du curseur, et à 16 px le royaume
entier tient dans la fenêtre par défaut. Déplacer la vue ainsi la rend **libre** : la caméra ne suit plus le
personnage, même s'il sort de l'écran. Elle le reprend (par un glissement) dès qu'on le déplace avec le
clavier, qu'on incarne un autre personnage (Tab, 1-9, clic) ou qu'on reprend la main avec Espace. Zoomer ne
rend pas la vue libre.

La disposition est définie dans `crates/afk-core/src/scenario.rs` ; le réglage de la caméra (durée du
glissement, marge de suivi, paliers de zoom) est dans `crates/afk-game/src/camera.rs`.

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
  Le blé récolté est porté à la main jusqu'au coffre du champ (voir « Les coffres et les transporteurs »).
- Les durées de labour, semis et récolte sont en secondes réelles ; la pousse est en heures de jeu.
  Tout se règle dans `crates/afk-core/src/config.rs`.
- Plusieurs paysans se répartissent les parcelles : chacun réserve celle qu'il vise, et les autres l'évitent
  (sans ça, tous se ruaient sur la même et en avoir 4 ne rapportait pas plus qu'en avoir 1).

**Le forgeron**
- L'enclume est un obstacle : on forge depuis une case voisine en la regardant (bouger vers elle suffit à la
  regarder), puis `E`. Chaque coup de forge dure 8 s (4 s pour le joueur), consomme 2 de fer et produit 1 outil.
- L'IA va seule à côté de l'enclume et forge tant qu'il reste du fer, puis traîne dans sa forge.
- Le forgeron prend son fer dans le coffre de la forge et y range ses outils sans quitter l'enclume. Le fer de départ (60) y est complété par celui que les transporteurs apportent des mines. Coffre sans fer ou plein d'outils : il attend.
  Réglable avec `starting_iron`, `iron_per_tool`, `forge_secs` dans `config.rs`.

**Le mineur**
- Un filon est un bloc de 2×2 tuiles solide, infini. Sa couleur dit sa nature (gris : fer, doré : or, beige clair :
  pierre ; le cuivre existe mais n'est pas encore posé) et ses points disent sa qualité : plus il y en a, plus la roche est
  impure et plus il faut de coups de pioche (3, 5 ou 8, d'une seconde chacun, `strikes_per_quality` et `strike_secs`).
- On mine depuis une case voisine en regardant le filon, puis `E` (2 fois plus vite que l'IA). Une unité terminée entre
  dans le coffre de la mine, sans que le mineur ait à y aller ; coffre plein (200), il attend.
- Chaque mineur a sa mine (sa maison, accolée au filon) et n'exploite que ce filon. Trois mines au départ : fer, or, pierre.
  L'IA se place sur la case libre la plus proche du filon et mine le jour ; la nuit elle rentre dormir.
- Le fer extrait est porté à la forge par les transporteurs : le fer n'est plus fini.

**Le bûcheron**
- Deux forêts (zones d'arbres avec des couloirs), chacune avec sa cabane et son bûcheron. On abat un arbre depuis une case voisine en le regardant, puis `E` : 6 s pour l'IA, 3 s pour le joueur (`chop_secs`), 3 bûches par arbre (`logs_per_tree`), que le bûcheron porte lui-même (9 au plus) jusqu'au coffre de sa cabane.
- Un arbre abattu laisse une pousse, qui redevient un arbre après 24 h de jeu (`tree_regrow_hours`). Les forêts ne s'étendent pas : on peut les vider en coupant plus vite que la repousse.
- Plusieurs bûcherons peuvent abattre le même arbre : leurs coups s'additionnent, mais seul celui qui porte le dernier coup reçoit le bois. L'IA coupe l'arbre le plus proche de sa forêt. Les arbres isolés hors forêt restent du décor.

**Les coffres et les transporteurs**
- Chaque lieu de travail a un **coffre** posé **dehors** : le champ du paysan, chaque mine, chaque cabane, la forge, et l'entrepôt central. Il affiche son contenu (« Fer 12/200 ») ; **200 au plus de chaque ressource**. Un producteur dont le coffre est plein s'arrête, et reprend dès qu'une place se libère.
- Trois **transporteurs** vivent à l'entrepôt central, près du château. Ils portent **10 ressources** par voyage, décident seuls (le fer des mines va à la forge d'abord, tout le reste à l'entrepôt central ; jamais dans un coffre plein) et **courent** (1,5 fois plus vite). On voit sur eux, comme sur le bûcheron, un bloc de la couleur de la ressource et la quantité portée.
- Incarné, un transporteur charge et décharge avec `E` devant un coffre ; le bûcheron dépose son bois et le paysan son blé de la même façon. `C` vide l'entrepôt central.
- Le paysan porte lui-même son blé (20 au plus, `farmer_carry`) jusqu'au coffre du champ, comme le bûcheron son bois.
- Les compteurs en haut à droite sont le total de tous les coffres. Réglages dans `config.rs` : `container_capacity`, `transport_load`, `transporter_speed_factor`, `lumberjack_carry`, `farmer_carry`.

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
- [x] Jalon 4 : filons de minerai, mineurs et mines
- [x] Jalon 5 : forêts, arbres qui repoussent et bûcherons
- [x] Jalon 6 : coffres limités à 200, trois transporteurs qui courent, bûcheron porteur
- À venir : priorités du roi pour les transporteurs, décrets du roi qui orientent les priorités de l'IA
  (que forger, combien de paysans), roi / chef de guerre / tavernier avec un vrai travail
- Plus tard : sprites en atlas, vraie lumière (carte de lumière), coop réseau
