# Tasks

## 1. Le cœur : plans, coûts, chantiers

- [x] 1.1 Module `building` : types de plan, taille minimale, `plan_cost`, `plan_build_secs`, `check_plan` ; tests (la surface et le type changent coût et durée, tailles refusées, terrain bloqué, personnage dans l'emprise)
- [x] 1.2 Maison de taille libre (`HabitatSpec::house`, `max_house_beds`, étagère facultative) ; tests (valide du minimum au maximum, lits accessibles, trop petite refusée)
- [x] 1.3 Contenants : coffre du camp, tas de chantier (`wants`, `needs`, `retire`, `become_depot`) ; transport adapté ; `Resource::Seeds`
- [x] 1.4 `Game` : `PlacePlan`, états du chantier, livraison, construction partagée, fin, route vers le camp ; tests (fondations solides, attente sans ressources, matériel promis à un seul chantier, deux artisans deux fois plus vite, route à la fin, entrepôt, taverne)
- [x] 1.5 Rôles artisan et vagabond ; routines ; apparition et départ des vagabonds ; tests

## 2. Le scénario de départ

- [x] 2.1 `Scenario::camp` : nature, camp (feu, trois lits, coffre), roi et deux artisans, pécule ; tests (aucun bâtiment, trois lits, tout joignable)
- [x] 2.2 `--village` garde le village bâti ; le jeu démarre par défaut au camp

## 3. L'interface

- [x] 3.1 Thème médiéval (`theme`) ; panneau des plans ; pose à la souris (déplacer, agrandir, valider), bleu, rouge, gris, barre d'information
- [x] 3.2 Dessin du camp (feu, couchages), des fondations, de l'échafaudage, des tas de matériaux, de l'entrepôt et de la taverne

## 4. Documentation et vérification

- [x] 4.1 `knowledge/` (règles, métiers, moteur, graphisme, journal), `README.md`
- [x] 4.2 `cargo test` complet ; une partie réelle de la pose à la maison finie (captures d'écran sous écran virtuel)
