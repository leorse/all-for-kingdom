# Tasks

## 1. Scénario : royaume agrandi

- [x] 1.1 Passer `village()` à 64×40 : bordure d'eau, routes, bâtiments publics regroupés au centre, maisons et champs repoussés vers les bords ; vérifier que `cargo test -p afk-core` compile et passe
- [x] 1.2 Mettre à jour lits, postes de travail, rondes des gardes, arbres et `EXTRA_BEDS` pour la nouvelle disposition ; vérifier qu'aucun `home` ni `work_spot` n'est sur une tuile solide
- [x] 1.3 Ajouter un test de scénario : chaque lit et chaque poste est joignable par `find_path` depuis la porte de chaque bâtiment, pour `village()` et `village_with_farmers(6)` ; vérifier qu'il passe
- [x] 1.4 Lancer des journées sans joueur (`cargo test`, puis `cargo run --release -p afk-core --example balance -- 8 600 2`) et ajuster tests et réglages impactés par les trajets plus longs ; vérifier que les tests passent et que la sortie de `balance` reste plausible

## 2. Caméra et rendu

- [x] 2.1 Rendre `View` persistante, à taille de tuile fixe, avec centre de caméra, bornage aux bords de la carte et centrage de la carte quand elle est plus petite que la fenêtre ; vérifier par des tests unitaires de `to_screen`/`to_world` (aller-retour) et de bornage
- [x] 2.2 Ajouter le suivi à marge ; vérifier par des tests unitaires (déplacement au centre : caméra immobile ; sortie de marge : caméra suit)
- [x] 2.3 Ajouter le glissement animé (~0,35 s, atténuation en fin de course, temps réel) : démarrage depuis la position courante, cible recalculée à chaque image, suivi à marge suspendu pendant l'animation ; vérifier par des tests unitaires (arrivée exacte sur la cible bornée en fin de durée, redémarrage sans saut, cible mobile)
- [x] 2.4 Brancher la caméra dans `main.rs` : mise à jour chaque image avec `get_frame_time` (indépendant de la vitesse T), animation quand l'identifiant incarné change, pas d'animation à la première image ; vérifier en lançant le jeu et en passant de personnage en personnage (Tab, 1-9, clic), y compris à x10 et x60
- [x] 2.5 Limiter `draw_map` aux tuiles visibles et ignorer les éléments hors écran dans parcelles, personnages, étincelles et lumière ; vérifier visuellement l'absence d'artefacts au bord et que le HUD n'est pas décalé
- [x] 2.6 Vérifier que le clic d'incarnation et la cible de parcelle (E) correspondent à l'affichage après défilement ; test manuel après avoir marché jusqu'à faire défiler la carte

## 3. Documentation et vérification finale

- [x] 3.1 Décrire la caméra et le nouveau royaume dans `README.md` (zoom fixe, suivi, recentrage) ; vérifier que les commandes documentées fonctionnent
- [x] 3.2 Intégration : `cargo test` complet, puis une partie en mode AFK avec `--speed 20` ; vérifier que tous les personnages mènent leur journée et que maisons et champs sont hors écran au démarrage
