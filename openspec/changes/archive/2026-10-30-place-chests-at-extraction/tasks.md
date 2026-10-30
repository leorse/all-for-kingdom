# Tasks

## 1. Emplacements des coffres

- [x] 1.1 Déplacer dans `village()` les coffres du paysan (11,6), des mines de fer (48,4), d'or (28,35) et de pierre (50,33), des cabanes nord (22,9) et sud (17,31) ; vérifier qu'aucune tuile existante n'est écrasée, qu'aucun coffre n'est sur un poste de travail et que `cargo test -p afk-core` compile
- [x] 1.2 Ajouter au test de scénario la règle de proximité (chaque coffre de production à deux cases au plus de son lieu d'extraction : parcelle, filon, arbre de la forêt) et la conserver pour `village_with_farmers(6)` ; vérifier qu'il passe
- [x] 1.3 Mettre à jour les tests de `game.rs` qui citent les anciennes coordonnées (chargement du transporteur joueur, dépôt du paysan joueur, dépôt du bûcheron joueur) ; vérifier que `cargo test` passe

## 2. Documentation et vérification finale

- [x] 2.1 Mettre à jour `docs/gameplay/transporteur.md`, `habitat.md`, `paysan.md`, `bucheron.md`, `mineur.md`, `forgeron.md` et `README.md` : les coffres sont au lieu d'extraction ; vérifier la cohérence des textes
- [x] 2.2 Mesurer avec `balance` que le paysan récolte plus qu'avant (trajets plus courts) et que le bois et les minerais circulent toujours ; vérifier à l'écran la position des coffres
