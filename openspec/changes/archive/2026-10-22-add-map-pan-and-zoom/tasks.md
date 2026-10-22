# Tasks

## 1. Caméra : vue libre et zoom (logique pure)

- [x] 1.1 Ajouter à `camera.rs` la vue libre : `Camera::pan` (déplacement borné, annule le glissement, suspend le suivi) et sa sortie par `recenter` ; vérifier par des tests unitaires (pan déplace le centre du bon delta, le suivi est suspendu même si la cible sort de la vue, `recenter` relance le glissement jusqu'à la cible, le bord de la carte borne le pan)
- [x] 1.2 Ajouter les paliers de zoom (`16, 20, 24, 32, 40, 48, 64`, départ 32) et le calcul du nouveau centre pour zoomer autour d'un point de l'écran ; vérifier par des tests unitaires (un cran monte ou descend d'un palier, butées aux deux bouts, le point sous le curseur reste au même pixel quand la carte le permet, résultat borné près des bords, la carte 64×40 tient dans 1024×640 au palier minimal)

## 2. Vue et entrées

- [x] 2.1 Rendre `tile_px` variable dans `View` et ajouter `pan_pixels`, `zoom` et `is_free` ; vérifier par un test que `to_screen` et `to_world` restent des inverses à chaque palier, et que `cargo test` passe
- [ ] 2.2 Lire le bouton droit, la molette et la souris dans `input.rs` (champs `moving`, `right_down`, `mouse`, `wheel`) et les brancher dans `main.rs` : déplacement mesuré entre deux images (nul à la première), un palier par image au plus, reprise du suivi quand la vue est libre et que le joueur bouge ; vérifier en jeu : tirer la carte, zoomer et dézoomer, marcher pour reprendre le suivi, Tab pour changer de personnage en vue libre
- [ ] 2.3 Vérifier la cohérence à tous les paliers : clic gauche pour incarner, cible de travail, parcelles, jauges, lumière nocturne, pas de couture entre tuiles ; corriger dans `render.rs` ce qui ne s'adapte pas

## 3. HUD et documentation

- [ ] 3.1 Ajouter le rappel des gestes dans la ligne d'aide (sur deux lignes si elle déborde) ; vérifier à l'écran aux largeurs de fenêtre courantes
- [x] 3.2 Décrire dans `README.md` les contrôles (bouton droit, molette), la vue libre et la reprise du suivi ; vérifier que le texte correspond au comportement constaté
- [ ] 3.3 Intégration : `cargo test` complet puis une partie `--speed 20` avec va-et-vient entre vue libre et suivi, y compris en mode AFK ; vérifier l'absence de saut de caméra et le retour au suivi
