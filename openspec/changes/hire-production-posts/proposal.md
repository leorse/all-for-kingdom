# Proposal

## Why

Le changement `hire-and-assign` a donné au roi le geste d'embaucher, mais seuls l'artisan, le transporteur, le tavernier et le forgeron ont des postes (fournis par des bâtiments). Le créateur veut que, **dès le début**, le roi puisse faire un **bûcheron**, un **mineur** ou un **paysan**, sans bâtiment : un **coffre** suffit. Sans ces métiers, bois et pierre ne se renouvellent pas et la partie s'arrête avec le pécule. Cette spec complète l'embauche, précise le métier de paysan et fixe le **modèle des coffres** qui en découle ; elle est amenée à être **réajustée** à l'usage.

## What Changes

- **Le coffre est un objet à part, posé volontairement** par le roi (jamais automatiquement avec un champ, une forêt ou un filon). Il est payé en bois au camp, posé tout de suite, sans chantier.
- **Un coffre, une seule ressource**, choisie à la pose, 200 au plus. Exceptions : le **coffre du camp** (tout, comme aujourd'hui) et l'**entrepôt**.
- **L'entrepôt n'a aucun coffre à l'extérieur** : il a de **grands coffres à l'intérieur**, chacun de **200 ressources de n'importe quel type, mélangés**. **On n'entre pas** : toucher la porte suffit, pour déposer comme pour prendre (simplification du gameplay).
- **Quand personne n'est incarné**, tout producteur (bûcheron, mineur, paysan) **porte lui-même** sa ressource au **coffre le plus proche qui l'accepte** ; le mineur aussi, qui ne dépose plus à distance au camp.
- **Un coffre posé près de sa matière est un poste** : coffre de bois près d'une forêt → bûcheron ; coffre d'un minerai près d'un filon de cette nature → mineur ; coffre de blé près d'un champ → paysan. Un coffre = un poste. Le mineur pourra **aussi** miner sous terre : **pas implémenté**, mais le poste ne suppose pas la surface.
- **Le champ est un carré de terre de 3×3, posé nu** (gratuit, sans coffre), par le roi ou par un paysan incarné. Le **paysan l'entretient** ; il dépend du coffre de blé le plus proche à portée.
- **La culture est choisie par le paysan seul**, quand le royaume a plusieurs graines (le roi ne choisit jamais). Une seule graine aujourd'hui (le blé) : aucun choix n'apparaît.
- **Interface** : deux cartes dans la liste des plans, « Coffre » (avec la ressource) et « Champ » ; les postes apparaissent sur la mini-carte comme les autres.
- Hors périmètre : la **mine souterraine**, de nouvelles cultures, l'**annulation** ou le déplacement d'un coffre, les **compétences**, un **coût d'embauche**, l'**IA du roi** qui poserait seule, l'**équilibrage** des coûts, du pécule et du nombre de grands coffres.

## Capabilities

### New Capabilities
- `production-posts`: un coffre posé près de sa matière fournit un poste de bûcheron, de mineur ou de paysan ; zone de travail, titulaire.
- `chests`: le coffre comme objet posé volontairement, une ressource par coffre, dépôt au plus proche, grands coffres de l'entrepôt, contact avec la porte.
- `farm-surface`: le champ de 3×3 posé nu, entretenu par le paysan, rattaché au coffre de blé le plus proche.
- `crop-choice`: le choix de la culture par le seul paysan, quand plusieurs graines existent.

### Modified Capabilities
- `posts-and-roles`: de nouveaux postes (coffres de production) pour trois métiers de plus.
- `assignment-interface`: les cartes « Coffre » et « Champ », la lecture des nouveaux postes sur la mini-carte.
- `containers`: limite par coffre (une ressource, ou 200 mélangées en grand coffre) et dépôt porté par le producteur au camp.

## Impact

- `crates/afk-core/src/character.rs` : `Role::has_posts` s'étend à bûcheron, mineur et paysan ; ils portent leur production jusqu'au coffre le plus proche.
- `crates/afk-core/src/container.rs`, `game.rs` : `Command::PlaceChest` et `PlaceField`, coffre à une ressource, grands coffres de l'entrepôt (200 mélangés), dépôt/prélèvement à la porte de l'entrepôt, choix du coffre le plus proche qui accepte une ressource ; `posts()` inclut les coffres-poste.
- `crates/afk-core/src/farm.rs`, `forest.rs`, `mine.rs` : champs de 3×3 ajoutés en cours de partie, zone de travail autour d'un coffre.
- `crates/afk-core/src/config.rs` : coût du coffre, portées (poste, travail, champ), nombre de grands coffres d'un entrepôt.
- `crates/afk-game/src/plan_ui.rs`, `assign_ui.rs`, `render.rs` : cartes, choix de ressource, dessin des grands coffres, repères des postes.
- `knowledge/` : `metiers/` (paysan, bûcheron, mineur, transporteur), `regles/ressources-et-stocks.md`, `propriete-et-affectation.md`, `construction.md`, `depart-et-camp.md`, `moteur/coffres-et-transport.md`, `log.md`.
- Le village de départ (`--village`) garde ses coffres, métiers et dépôts fixes : rien n'y change.
