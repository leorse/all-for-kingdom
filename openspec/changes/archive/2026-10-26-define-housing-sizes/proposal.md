# Proposal

## Why

Les maisons du royaume sont des rectangles posés au hasard du scénario : un lit au milieu d'une pièce vide, des tailles qui varient sans règle (4×4, 5×4, 6×5…), des lits ajoutés à la main pour les paysans supplémentaires. Rien ne dit combien de personnes un bâtiment peut loger, ni ce qu'une maison doit contenir. Or le gameplay décidé (`docs/gameplay/habitat.md`, `roi.md`, `tavernier.md`) repose sur le logement : chaque habitant a sa maison, le roi posera de nouveaux habitats pour loger les arrivants, et on n'embauchera un vagabond de la taverne que s'il reste une place. Il faut d'abord un modèle de logement net.

## What Changes

- Des **tailles fixes** pour les habitats, en trois classes : **petite maison** 7×5 (1 place), **maison moyenne** 9×5 (jusqu'à 3 places), **grande maison** 9×7 (jusqu'à 6 places). Toute pièce où l'on dort est un habitat de l'une de ces classes.
- Un habitat contient toujours **au moins un lit, une table, une étagère et un bac d'eau**. Le **lit, la table et l'étagère font 2×1 tuiles, le bac 1×1**. Une « place » est un lit ; les places d'un habitat à plusieurs places se **partagent table, étagère et bac** (pièce commune). La table, l'étagère et le bac sont infranchissables ; les lits sont praticables ; tous les lits restent accessibles depuis la porte.
- **Chaque personne a son lit**, jamais deux personnes dans le même. Le jeu sait compter les **places libres** de chaque habitat et du royaume.
- **Embauche conditionnée** : une règle du cœur dit qu'un vagabond ne peut être embauché que s'il reste une place libre quelque part.
- **Agrandissement** : un habitat ne peut passer à la classe supérieure que si l'espace autour de lui est libre. Le cœur sait vérifier cette place ; l'action d'agrandir elle-même viendra plus tard.
- Le **scénario de départ** est remis aux normes : caserne et entrepôt central, maisons du paysan, du forgeron et du tavernier, mines et cabanes. La maison du paysan grandit avec le nombre de paysans.
- Tables, étagères, lits à deux tuiles et bacs sont dessinés.
- **Le château n'est pas touché** : il aura son propre fonctionnement et reste exempté de ces classes, sans table ni bac pour l'instant.
- Hors périmètre : le roi qui pose des habitats, l'arrivée de vagabonds à la taverne, l'action d'agrandir, l'usage de la table et du bac (repas, soif, confort), les améliorations de maison, le coût de construction.

## Capabilities

### New Capabilities
- `housing`: classes de taille des habitats, mobilier obligatoire, places et occupation, places libres, règle d'embauche et condition d'agrandissement.

### Modified Capabilities
- `kingdom-layout`: tous les logements du royaume de départ sont des habitats conformes.

## Impact

- `crates/afk-core/src/map.rs` : tuiles de lit (tête et pied), de table, d'étagère (deux moitiés chacune) et de bac d'eau.
- Nouveau module `afk-core` des habitats (classes, mobilier, places, vérifications) ; `game.rs` (habitats en lecture, places libres, règle d'embauche) ; `lib.rs`.
- `scenario.rs` : description des habitats (`HabitatSpec`), reprise des bâtiments existants (caserne, maisons, mines, cabanes, entrepôt central) et de `village_with_farmers` ; déplacement de postes de travail et de rondes qui tombent sur du mobilier ; tests de conformité et de connectivité.
- `crates/afk-game/src/render.rs` : sprites des lits à deux tuiles, de la table, de l'étagère et du bac.
- `docs/gameplay/habitat.md`, `README.md`.
