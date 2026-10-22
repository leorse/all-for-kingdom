# Tasks

## 1. Forêts et repousse

- [x] 1.1 Ajouter `Tile::Sapling` (praticable) dans `map.rs`, la ressource `Wood` dans `resources.rs` et les réglages `chop_secs`, `logs_per_tree` (3), `tree_regrow_hours` dans `Config` ; vérifier que `cargo test -p afk-core` compile et passe, avec un test d'ajout/retrait de bois
- [x] 1.2 Créer le module des forêts : type `Forest` (rectangle, `contains`) et liste des pousses avec leur délai en heures de jeu ; vérifier par des tests unitaires (le décompte aboutit à la repousse, une case occupée retarde la repousse sans bloquer, un arbre hors forêt n'est pas abattable)

## 2. Bûcheron dans le cœur

- [x] 2.1 Ajouter `Role::Lumberjack`, `WorkKind::Chop` et le rattachement bûcheron → forêt par le lit (`LodgeSpec`, `Character.forest`) ; vérifier que les tests existants passent sans modification de comportement
- [x] 2.2 Brancher `available_work` : seul un bûcheron peut lancer un `Chop` sur un arbre de sa forêt qu'il regarde depuis une case voisine ; la contribution de chaque bûcheron s'additionne à la progression de l'arbre tenue par `Game` et affichée sur la barre de tous ; vérifier par des tests (autre métier refusé, arbre isolé refusé, autre forêt refusée, annulation sans chute, progression conservée après abandon, double vitesse du joueur)
- [x] 2.3 Chute de l'arbre : quand le total atteint 1, l'arbre devient une pousse, +3 bûches au compteur au nom du bûcheron qui porte le dernier coup, travail des autres interrompu ; vérifier par des tests (à deux, l'arbre tombe en deux fois moins de temps, le compteur ne monte que de 3, un seul propriétaire, l'autre n'a plus de travail en cours)
- [x] 2.4 Faire repousser les pousses dans `Game::step` en heures de jeu ; vérifier par des tests (repousse après le délai, indépendante de la vitesse de simulation, ne piège pas un personnage)
- [x] 2.5 Écrire la routine IA du bûcheron (arbre le plus proche de sa forêt même déjà attaqué, case de poste réservée, attente sans arbre, nuit au lit de sa cabane) ; vérifier par des tests : une journée IA produit du bois, deux bûcherons IA peuvent s'attaquer au même arbre depuis deux cases différentes et jamais depuis la même, une forêt vidée ne fait pas planter l'IA et reprend à la repousse

## 3. Scénario

- [x] 3.1 Étendre `Scenario` avec forêts et cabanes, et ajouter dans `village()` deux forêts avec couloirs, deux cabanes, leurs chemins et deux bûcherons (à la fin de la liste des personnages) ; vérifier qu'aucune tuile existante n'est écrasée et que tous les arbres de forêt sont abattables
- [x] 3.2 Étendre le test de connectivité aux lits de cabane et aux lisières de forêt ; vérifier qu'il passe pour `village()` et `village_with_farmers(6)`
- [x] 3.3 Mettre à jour les tests de `game.rs` et `examples/balance.rs` touchés par les nouveaux personnages, puis régler `chop_secs` et `tree_regrow_hours` avec l'outil `balance` (arbres restants par forêt par jour) ; vérifier que `cargo test` passe et que les forêts tiennent plusieurs jours

## 4. Rendu et jeu

- [ ] 4.1 Dessiner la pousse, le bûcheron et le compteur Bois au HUD, et le message « E : abattre » ; vérifier visuellement qu'une pousse se distingue d'un arbre et d'un sol nu
- [ ] 4.2 Jouer un bûcheron : l'incarner, abattre un arbre, annuler en bougeant, voir le bois monter de 3 ; vérifier que la caméra et le clic restent corrects dans une forêt

## 5. Documentation et vérification finale

- [x] 5.1 Mettre à jour `docs/gameplay/bucheron.md`, `royaume.md`, `ressources-et-stocks.md`, le tableau de `docs/gameplay/README.md` et `README.md` : ce qui est en place (forêts, repousse, 3 bûches par arbre, compteur de bois) et ce qui reste repoussé (entrepôt, outils, replantation) ; vérifier la cohérence des liens
- [x] 5.2 Intégration : `cargo test` complet, puis une partie en mode AFK (`--speed 20`) sur plusieurs jours ; vérifier que les bûcherons produisent du bois et que les forêts ne se vident pas définitivement
