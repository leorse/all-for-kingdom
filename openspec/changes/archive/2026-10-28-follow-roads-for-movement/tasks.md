# Tasks

## 1. Recherche de chemin à coûts et en 8 directions

- [x] 1.1 Dans `map.rs`, remplacer le BFS par un A\* à 8 directions, sans coin coupé, avec un paramètre de mode `Roads`/`Direct`, un coût d'herbe tiré de `config.rs` et un départage déterministe ; vérifier par tests unitaires : diagonale en terrain libre, pas de coin coupé entre deux obstacles, but inaccessible renvoie un chemin vide, résultat identique à chaque appel.
- [x] 1.2 Tests de `map.rs` pour les modes : en `Roads` le trajet longe une route parallèle proche, traverse l'herbe si le détour est trop long et quitte la route pour un but hors route ; en `Direct` il prend le plus court chemin sans regarder la route.
- [x] 1.3 Ajouter une fonction renvoyant chemin et coût, et vérifier par test que le coût d'un pas diagonal vaut √2.

## 2. Brancher les personnages sur le nouveau chemin

- [x] 2.1 Adapter les ≈25 appels à `find_path` (`character.rs`, `transport.rs` le cas échéant) au mode `Roads` et comparer les candidats par coût ; vérifier que `cargo test -p afk-core` compile et que les routines (mineur, bûcheron, forgeron, paysan, transporteur, garde) atteignent leur but.
- [x] 2.2 `go_to_bed` emprunte les routes ; vérifier par test qu'un habitant éloigné de son lit rejoint la route puis sa maison.
- [x] 2.3 Vérifier par test que `follow_path` ne reste pas bloqué sur une diagonale et ne traverse pas le coin d'un obstacle.
- [x] 2.3b Lisser la trajectoire dans `follow_path` (cap qui tourne progressivement, coins coupés si le segment est libre, arrivée exacte sur le but) ; vérifier par test sur un trajet « route horizontale puis montée vers un coffre » que l'angle entre pas successifs reste sous un seuil, qu'aucun point n'est dans un obstacle et que le personnage finit sur la case but ; vérifier à l'œil dans le jeu (`run.bat`) que le tracé ressemble à la courbe voulue.
- [x] 2.4 Mettre à jour les tests de `game.rs`, `character.rs` et `transport.rs` dont les durées ou positions changent ; vérifier que `cargo test --workspace` passe et que les journées complètes de l'IA restent faisables sur le scénario de départ.

## 3. Mode direct pour le voleur et les gardes

- [x] 3.1 Exposer le mode `Direct` pour la fuite vers la sortie du château et la poursuite des gardes (point d'entrée pour le futur comportement des voleurs), sans créer de voleur ; vérifier par test qu'un trajet `Direct` coupe l'herbe là où `Roads` suit la route, et que le garde reprend `Roads` ensuite.

## 4. Documentation

- [x] 4.1 Mettre à jour `docs/gameplay/royaume.md`, `garde.md`, `transporteur.md`, `temps-et-repos.md` et la section « Bandits » de `IDEAS.md` (routes, diagonales, fuite, poursuite) ; vérifier que chaque règle de la spec `road-movement` y figure.
