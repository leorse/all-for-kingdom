# Tasks

## 1. Filons sur la carte

- [x] 1.1 Ajouter `Tile::Ore` (solide) dans `map.rs`, et un type `Vein` (origine, nature fer/cuivre/or/pierre, qualité à 3 niveaux) avec « filon contenant cette tuile » et « cases voisines praticables » ; vérifier par des tests unitaires (le filon bloque `find_path`, 8 cases voisines en terrain dégagé)
- [x] 1.2 Ajouter les ressources `Copper`, `Gold`, `Stone` dans `resources.rs` ; vérifier par un test d'ajout/retrait de chaque ressource
- [x] 1.3 Ajouter à `Config` les réglages de minage (coups par qualité, durée d'un coup) ; vérifier que `cargo test -p afk-core` compile et passe

## 2. Mine et minage dans le cœur

- [x] 2.1 Ajouter `Role::Miner`, `WorkKind::Mine` et la durée portée par le travail (coups × durée d'un coup, selon la qualité du filon visé), sans changer paysan ni forgeron ; vérifier par des tests de durée (pur plus rapide qu'impur, joueur 2× plus rapide) et par les tests existants inchangés
- [x] 2.2 Brancher `available_work` et la fin de travail : le mineur ne mine que le filon de sa mine depuis une case voisine en le regardant ; à la fin, +1 au compteur global de la nature ; vérifier par des tests (autre métier refusé, compteur +1 à la fin, annulation sans gain)
- [x] 2.3 Réserver la case de poste du mineur (`claimed_tile`) et écrire la routine IA du mineur (case voisine libre la plus proche du filon, nuit au lit de la mine) ; vérifier par des tests : une journée IA produit du minerai, deux mineurs sur un même filon (scénario de test) se placent sur deux cases, la nuit tous dorment
- [x] 2.4 Vérifier que le forgeron, inchangé, continue de forger après épuisement du fer de départ quand un mineur de fer travaille ; test de bout en bout sur plusieurs jours

## 3. Scénario

- [x] 3.1 Étendre `Scenario` avec filons et mines (`MineSpec`) et ajouter, dans `village()`, trois filons (fer, or, pierre) avec leurs mines accolées et un mineur chacun, hors routes, champs et bâtiments ; vérifier qu'aucune tuile n'est écrasée et que les trois natures sont présentes
- [x] 3.2 Étendre le test de connectivité (`assert_connected`) aux lits, portes et cases voisines de filon ; vérifier qu'il passe pour `village()` et `village_with_farmers(6)`
- [x] 3.3 Mettre à jour les tests de `game.rs` et `examples/balance.rs` touchés par les nouveaux personnages et le fer renouvelable, puis régler les coups par qualité avec `cargo run --release -p afk-core --example balance` ; vérifier que `cargo test` passe

## 4. Rendu et jeu

- [ ] 4.1 Dessiner les filons (couleur par nature, points selon la qualité) dans `render.rs` ; vérifier visuellement que les natures et les trois qualités se distinguent au premier coup d'œil
- [ ] 4.2 Dessiner la mine (bâtiment-maison) et le mineur, ajouter les compteurs Cuivre/Or/Pierre au HUD ; vérifier en jeu
- [ ] 4.3 Jouer un mineur : l'incarner, miner avec la touche d'action, annuler en bougeant, constater la hausse du compteur ; vérifier que la caméra et le clic restent corrects près des filons

## 5. Documentation et vérification finale

- [x] 5.1 Mettre à jour `docs/gameplay/mineur.md`, `ressources-et-stocks.md`, `forgeron.md` et le tableau du `README.md` de `docs/gameplay` : ce qui est en place (filons, mineur, mine-maison, minerai dans les compteurs) et ce qui reste décidé mais repoussé (entrepôt limité, stock local du forgeron) ; vérifier la cohérence des liens
- [x] 5.2 Décrire dans `README.md` le mineur et les filons ; vérifier que les commandes documentées fonctionnent
- [ ] 5.3 Intégration : `cargo test` complet, puis une partie en mode AFK (`--speed 20`) sur plusieurs jours ; vérifier que les trois mineurs produisent et que le forgeron forge sans interruption
