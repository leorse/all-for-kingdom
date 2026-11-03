# Proposal

## Why

Le changement `hire-and-assign` a donné au roi le geste d'embaucher, mais seuls l'artisan, le transporteur, le tavernier et le forgeron ont des postes (fournis par des bâtiments). Le créateur veut que, **dès le début**, le roi puisse faire un **bûcheron**, un **mineur** ou un **paysan**, sans bâtiment : un **coffre** suffit. Sans ces métiers, bois et pierre ne se renouvellent pas et la partie s'arrête avec le pécule. Cette spec complète l'embauche et précise le métier de paysan ; elle est amenée à être **réajustée** à l'usage.

## What Changes

- **Un poste de production n'a pas de bâtiment.** Un bûcheron, un mineur ou un paysan travaille autour d'un **coffre** que le roi pose : la zone de travail est ce qui entoure le coffre (arbres de la forêt, filon, champ). Un coffre = un poste, un seul titulaire.
- **Poser un coffre-poste** : trois cartes dans la liste des plans du roi (« Poste de bucheron », « Poste de mineur », « Poste de paysan »), de la taille d'une tuile, posées près de la bonne matière (forêt, filon, champ). Le coût est payé tout de suite dans le coffre du camp ; pas de chantier.
- **Le mineur** : un poste de mineur touche un **filon de surface** et son coffre ne reçoit que le minerai de ce filon. Le mineur pourra **aussi** miner sous terre (mine souterraine, voir les idées) : **pas encore implémenté**, rien n'est proposé dans ce sens, mais le poste ne doit pas supposer qu'on mine toujours en surface.
- **Le champ appartient au paysan.** La **surface cultivable** est définie par le **roi** ou par le **paysan** (quand on l'incarne), avec le même geste (un rectangle de terre à cultiver). Chaque parcelle dépend du coffre-poste de paysan le plus proche à portée ; son titulaire la laboure, la sème, la récolte.
- **La culture est choisie par le paysan seul**, quand le royaume a **plusieurs graines différentes** (le roi ne choisit pas). Il n'y a aujourd'hui qu'une graine (le blé) : le choix n'apparaît que lorsqu'une seconde existera.
- **Postes dans la mini-carte** : les nouveaux postes apparaissent comme les autres (vert libre, rouge occupé) ; un coffre sans titulaire est un poste libre.
- Hors périmètre : la **mine souterraine**, de nouvelles cultures (autres graines que le blé), l'**annulation** ou le déplacement d'un coffre, les **compétences**, un **coût d'embauche**, l'**IA du roi** qui poserait seule, l'**équilibrage** des coûts et du pécule, le **mineur qui mine à la profondeur**.

## Capabilities

### New Capabilities
- `production-posts`: le coffre-poste du bûcheron, du mineur et du paysan, sa pose par le roi, sa zone de travail, son titulaire.
- `farm-surface`: la surface cultivable (définie par le roi ou le paysan), son rattachement à un coffre-poste de paysan.
- `crop-choice`: le choix de la culture par le seul paysan, quand plusieurs graines existent.

### Modified Capabilities
- `posts-and-roles`: de nouveaux postes (coffres de production) pour trois métiers de plus.
- `assignment-interface`: les cartes de pose du coffre-poste, et la lecture des nouveaux postes sur la mini-carte.

## Impact

- `crates/afk-core/src/character.rs` : `Role::has_posts` s'étend à bûcheron, mineur et paysan ; leurs routines ne travaillent qu'avec un poste (au camp).
- `crates/afk-core/src/game.rs` : `Command::PlaceChest` (et définition de surface), `posts()` inclut les coffres-poste, affectation d'un bûcheron, mineur ou paysan à un coffre ; liaison coffre → forêt, filon, parcelles.
- `crates/afk-core/src/container.rs`, `farm.rs`, `forest.rs`, `mine.rs` : un coffre posé par le roi (nouvelle origine de `ContainerKind::Farm/Lodge/Mine`) ; parcelles ajoutées en cours de partie ; zone de travail autour d'un coffre.
- `crates/afk-core/src/config.rs` : portées et coût du coffre-poste, portée de rattachement des parcelles.
- `crates/afk-game/src/plan_ui.rs`, `assign_ui.rs`, `render.rs` : cartes, gestes de surface, repères des postes.
- `knowledge/` : `metiers/paysan.md`, `bucheron.md`, `mineur.md`, `regles/propriete-et-affectation.md`, `regles/construction.md`, `regles/depart-et-camp.md`, `log.md`.
- Le village de départ (`--village`) garde ses bûcherons, mineurs et paysans fixes et leurs coffres : rien n'y change.
