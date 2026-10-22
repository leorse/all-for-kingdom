# Tasks

## 1. Contenants et coffres

- [x] 1.1 Ajouter `Tile::Crate` (solide), le module des contenants (`Container`, genres `Farm`/`Mine`/`Lodge`/`Forge`/`Depot`, stock par ressource, capacité, table « donne / consomme / accepte ») et les réglages (`container_capacity` 200, `transport_load` 10, `transporter_speed_factor` 1,5, `lumberjack_carry` 9) ; vérifier par des tests unitaires (dépôt plafonné à 200, retrait, une ressource pleine n'empêche pas une autre, chaque genre donne et accepte ce qu'il doit)
- [x] 1.2 Étendre `Scenario` avec les contenants (`ContainerSpec`) et poser les sept coffres de production dans `village()`, tous à l'extérieur des bâtiments (champ à côté de la porte du paysan, un contre chaque mine, chaque cabane et la forge) ; vérifier qu'aucune tuile existante n'est écrasée, qu'aucun coffre n'est dans un bâtiment et que chaque coffre a une case voisine praticable
- [x] 1.3 Remplacer `Game::stock` par les contenants : `Game::stock()` devient le total calculé, le fer de départ va au contenant de la forge, `Env` donne les contenants en lecture ; vérifier que `cargo test -p afk-core` compile, et réécrire les tests qui supposaient le compteur global (fer de départ, minerai, blé, bois) autour des contenants

## 2. Les producteurs déposent et s'arrêtent quand c'est plein

- [x] 2.1 Mineur : dépôt dans le contenant de sa mine à la fin du travail, démarrage refusé (joueur) ou attente (IA) si le contenant est plein de sa ressource ; vérifier par des tests (dépôt, plafond à 200, reprise quand on retire du minerai, mine d'un autre filon non concernée)
- [x] 2.2 Paysan : récolte déposée dans le coffre du champ, récolte exclue des cibles de l'IA et refusée au joueur quand il ne reste pas de place pour 4 blé, labour et semis continuent ; vérifier par des tests (parcelles mûres qui restent mûres, reprise automatique)
- [x] 2.3 Forgeron : fer pris et outils déposés dans le contenant de la forge, pas de forge sans fer dans ce contenant ni sans place pour un outil ; vérifier par des tests (le fer de départ est dans le coffre de la forge, mine pleine de fer mais forge à sec : le forgeron attend, arrêt à 200 outils)

## 3. Transporteurs

- [x] 3.1 Écrire le planificateur de voyages pur (`best_trip`) : voyages candidats selon la table, quantité `min(10, source, place)` avec réservations des autres, charge complète ou remplissage exact, priorité consommateur puis gros puis proche ; vérifier par des tests unitaires de toutes les scénarios de la spec (forge d'abord, dépôt central sinon, forge pleine, dépôt rend à la forge, pas de va-et-vient vers les cabanes, source à moins de 10, complétion de 4, trois transporteurs et une place)
- [x] 3.2 Ajouter `Role::Transporter`, la cargaison portée (`Character.cargo`, `Job`) et les événements de transfert (prendre, déposer) appliqués par `Game` après les travaux, avec revérification du stock et de la place ; vérifier par des tests de `Game` (prise de 10 à la source, dépôt plafonné, cargaison conservée)
- [x] 3.3 Routine IA du transporteur : planifier, marcher jusqu'à une case voisine du coffre source, prendre, marcher jusqu'à la destination, déposer, replanifier si la destination est devenue pleine, attendre à l'entrepôt central sinon, dormir la nuit en gardant la cargaison et livrer à l'aube ; vérifier par des tests (le fer d'une mine arrive à la forge, le fer ne dépasse jamais 200 à la forge, l'or d'une mine va au dépôt central, nuit avec cargaison, aucune perte ni création de ressource sur plusieurs jours)
- [x] 3.4 Vitesse de course : facteur 1,5 appliqué au déplacement du transporteur, IA et joueur ; vérifier par des tests (le transporteur parcourt la même distance plus vite qu'un garde, et le joueur transporteur plus vite qu'un autre joueur)
- [x] 3.5 Joueur transporteur : charger (jusqu'à 10 de la ressource dont le contenant a le plus et qu'il donne) et décharger avec la touche d'action devant un coffre ; vérifier par des tests (charger 10 sur 30, décharger à la forge, refus à la mine d'or, refus quand c'est plein, reste en main)

## 4. Bûcheron porteur

- [x] 4.1 Remettre les 3 bûches dans la cargaison du propriétaire de l'arbre au lieu du compteur ; refuser/éviter d'abattre quand la cargaison + 3 dépasse `lumberjack_carry` ; vérifier par des tests (bûches en main, mains pleines à 8, un seul propriétaire à deux sur un arbre)
- [x] 4.2 Routine IA du bûcheron livreur : livrer au coffre de la cabane quand les mains sont pleines ou qu'il n'y a plus d'arbre, attendre à côté si le coffre est plein, nuit avec du bois en main puis livraison à l'aube ; joueur : déposer ce qui rentre avec la touche d'action ; vérifier par des tests (trois arbres puis livraison, cabane pleine et attente, reprise quand un transporteur emporte du bois, dépôt partiel du joueur)

## 5. Scénario : entrepôt central et transporteurs

- [x] 5.1 Ajouter à `village()` l'entrepôt central (bâtiment, porte, chemin vers la route, trois lits dedans, coffre dehors à côté de la porte) et trois transporteurs à la fin de la liste des personnages ; vérifier qu'aucune tuile n'est écrasée
- [x] 5.2 Étendre le test de connectivité du scénario : chaque coffre a une case voisine joignable depuis le château, lits de l'entrepôt central joignables, pour `village()` et `village_with_farmers(6)` ; vérifier qu'il passe
- [x] 5.3 Mettre à jour les tests restants de `game.rs` et `examples/balance.rs` (stocks par contenant, fer du forgeron via transporteurs) puis mesurer avec `balance` sur 8 jours : le forgeron ne manque pas de fer en régime normal, les contenants non consommés se bloquent à 200 puis leur producteur s'arrête ; vérifier que `cargo test` passe

## 6. Rendu et jeu

- [ ] 6.1 Dessiner le coffre et son compteur par ressource (« Fer 12/200 »), la ressource portée visible sur les transporteurs et les bûcherons (bloc de la couleur de la ressource dans les bras et quantité au-dessus de la tête) et la ligne de HUD de la cargaison du personnage incarné ; vérifier à l'écran à plusieurs paliers de zoom, y compris que le bloc disparaît mains vides
- [ ] 6.2 Jouer un transporteur (charger à une mine, décharger à la forge, courir) et un bûcheron (déposer son bois) ; vérifier que les compteurs bougent comme annoncé et que la caméra et le clic restent corrects

## 7. Documentation et vérification finale

- [x] 7.1 Mettre à jour `docs/gameplay/transporteur.md`, `ressources-et-stocks.md`, `mineur.md`, `bucheron.md`, `forgeron.md`, `paysan.md`, `habitat.md`, le tableau de `docs/gameplay/README.md` et `README.md` : ce qui est en place (contenants, limite 200, transporteurs, entrepôt central, bûcheron porteur) et ce qui reste décidé (priorités du roi, chariots, routes) ; vérifier la cohérence des liens
- [x] 7.2 Intégration : `cargo test` complet puis une partie `--speed 20` sur plusieurs jours ; vérifier que les trois transporteurs circulent, que les compteurs des coffres montent et plafonnent à 200, que les producteurs pleins s'arrêtent et reprennent, et que la forge ne manque pas de fer

## 8. Compléments : le paysan porteur et la touche C

- [x] 8.1 Le paysan prend son blé en main (`farmer_carry` 20 dans la configuration), ne récolte que si ses mains peuvent recevoir 4 blé de plus, et livre au coffre du champ (IA : mains pleines, plus rien à cultiver ou réveil avec du blé, seulement si le coffre a de la place ; joueur : touche d'action) ; vérifier par des tests (blé en main et coffre inchangé, mains pleines, livraison, coffre plein sans livraison et sans arrêt du labour, nuit avec du blé, dépôt partiel du joueur) et réécrire les tests de blé qui lisaient le compteur
- [x] 8.2 Touche C : commande `Command::EmptyDepot` qui vide le contenant de l'entrepôt central, branchée dans `input.rs`, et rappel dans l'aide du bas ; vérifier par des tests (seul le dépôt est vidé, le blé du champ reprend ensuite le chemin du dépôt) et par la ligne d'aide à l'écran
- [x] 8.3 Mettre à jour `docs/gameplay/paysan.md`, `ressources-et-stocks.md` et `README.md` (paysan porteur, touche C, réglage `farmer_carry`) ; vérifier avec `cargo test` et l'outil `balance` que le blé récolté par jour reste plausible
