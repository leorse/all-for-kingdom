# Tasks

## 1. Modèle d'habitat dans le cœur

- [x] 1.1 Ajouter `Tile::Table` et `Tile::Trough` (solides) dans `map.rs` et le module des habitats : `HabitatClass` (5×4/1, 7×4/3, 7×6/6, `next()`), `HabitatSpec` et `Habitat` ; vérifier par des tests unitaires (dimensions et places de chaque classe, classe suivante, la grande n'a pas de suivante, le mobilier bloque `find_path`)
- [x] 1.2 Écrire `build_habitat` et `validate_habitat` (dimensions, porte sur un mur hors angle, 1 à `max_places` lits, une table et un bac dans l'intérieur sur des cases distinctes, tous les lits joignables depuis l'entrée) ; vérifier par des tests unitaires de chaque règle (lit emmuré refusé, deux lits en petite maison refusés, meuble sur un mur refusé, maison moyenne à trois lits acceptée)
- [x] 1.3 Écrire `can_expand` (emprise de la classe suivante ancrée au même coin, cases nouvelles en herbe ou chemin dans la carte) ; vérifier par des tests unitaires (dégagé accepté, arbre refusé, eau refusée, bord de carte refusé, coffre refusé, grande maison refusée)

## 2. Scénario aux normes

- [x] 2.1 Ajouter `HabitatSpec` au `Scenario` et reprendre en habitats la caserne et l'entrepôt central (moyennes), les maisons du paysan, du forgeron et du tavernier, les mines et les cabanes (petites), avec table et bac, aux positions du design ; vérifier qu'aucune tuile existante n'est écrasée, que `validate_habitat` accepte chacun et que chaque lit de personnage appartient à un habitat
- [x] 2.2 Faire grandir la maison du paysan dans `village_with_farmers` (petite, moyenne ou grande selon le nombre de paysans, ancrage et lits du design) ; vérifier qu'elle est conforme de 1 à 6 paysans et ne chevauche rien
- [x] 2.3 Déplacer les postes de travail et les rondes qui tombent sur du mobilier ou hors du nouvel intérieur (caserne, entrepôt central, tavernier, forgeron ; le château n'est pas touché) ; vérifier avec le test de connectivité du scénario étendu aux habitats (chaque lit joignable depuis le château, aucun poste sur une tuile solide) pour `village()` et `village_with_farmers(6)`
- [x] 2.4 Mettre à jour les tests de `game.rs` et l'outil `balance` touchés par les positions déplacées ; vérifier que `cargo test` passe et que la simulation (`balance`) tourne comme avant

## 3. Places, embauche et lecture dans `Game`

- [x] 3.1 Exposer les habitats et les places dans `Game` (`habitats()`, places libres par habitat et pour le royaume) et la règle `can_hire()` ; vérifier par des tests (aucune place libre au départ, un lit sans occupant donne une place libre, `can_hire` suit `free_places`, la maison moyenne à moitié vide)

## 4. Rendu

- [ ] 4.1 Dessiner la table et le bac d'eau ; vérifier à l'écran que les intérieurs sont lisibles à plusieurs paliers de zoom et que les personnages se déplacent bien autour des meubles

## 5. Documentation et vérification finale

- [x] 5.1 Mettre à jour `docs/gameplay/habitat.md` (tailles, mobilier, places, règle d'embauche, condition d'agrandissement, ce qui reste décidé : le roi qui place, la taverne), `royaume.md` si besoin et `README.md` ; vérifier la cohérence des liens
- [x] 5.2 Intégration : `cargo test` complet puis une partie `--speed 20` sur plusieurs jours ; vérifier que chacun rejoint son lit, dort et se lève normalement dans les maisons meublées

## 6. Pièces plus grandes, lits et tables en 2×1, étagères

- [x] 6.1 Passer les classes à 7×5 / 9×5 / 9×7, ajouter les tuiles `BedFoot`, `TableEnd`, `Shelf`, `ShelfEnd`, le mobilier de 2×1 (lit, table, étagère) et du bac de 1×1 dans `HabitatSpec` (champ `shelf`), la validation (tuiles dans l'intérieur, sans chevauchement, 3 cases libres, lits joignables) et `build` ; vérifier par des tests unitaires (tailles et places, un lit couvre deux tuiles dont le pied est praticable, table/étagère/bac solides, chevauchement de meubles refusé, lit emmuré refusé, agrandissement avec les nouvelles tailles)
- [x] 6.2 Remettre le scénario aux normes avec les nouvelles tailles et l'étagère : positions, portes, chemins, lits (`home` = tuile de tête), coffres de l'entrepôt central et de la cabane sud, postes du bûcheron sud, du chef de guerre et des transporteurs, maison du paysan de 1 à 6 paysans ; vérifier par les tests de conformité (habitats valides et intacts sur la carte, non chevauchement, tous les lits joignables depuis le château, coffres à deux cases au plus de l'extraction) pour `village()` et `village_with_farmers(1..=6)`
- [x] 6.3 Mettre à jour les tests de `game.rs` et l'outil `balance` qui citent des positions déplacées ; vérifier que `cargo test` passe et que la simulation tourne comme avant (chacun rejoint son lit, la production continue)
- [ ] 6.4 Dessiner les lits à deux tuiles, la table et l'étagère à deux tuiles et le bac ; vérifier à l'écran à plusieurs zooms que les pièces sont lisibles et que les personnages circulent autour des meubles
- [x] 6.5 Mettre à jour `docs/gameplay/habitat.md` et `README.md` (nouvelles tailles, étagère, tailles des meubles) ; vérifier la cohérence des textes
