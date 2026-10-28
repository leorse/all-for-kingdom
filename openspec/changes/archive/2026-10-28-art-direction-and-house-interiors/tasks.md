# Tasks

## 1. Direction artistique et pipeline de sprites

- [x] 1.1 Écrire `docs/art-direction.md` (image de référence, tuile de base de 16 px, échelle entière, palette avec ses valeurs, contours sombres, lumière chaude, vue en trois-quarts, règle « un élément à la fois, pas de bâtiment mixte ») ; vérifier que le guide cite l'image `docs/images/image_maison.jpg` et que la palette du document est celle du code
- [x] 1.2 Créer le module `sprites.rs` d'`afk-game` : palette commune, sprite en grille de caractères, vérification (couleurs connues, lignes de même largeur), création des textures sans lissage et dessin à une échelle entière ; vérifier par des tests unitaires (une couleur hors palette est refusée, une grille irrégulière est refusée, les positions écran sont arrondies à l'entier) et `cargo test -p afk-game`

## 2. Zoom entier

- [x] 2.1 Passer les paliers de zoom à `[16, 32, 48, 64]` avec départ 32, dans `camera.rs` et `View` ; vérifier par les tests de caméra et de vue mis à jour (un cran monte ou descend d'un palier, butées, la carte tient dans 1024×640 à 16 px, la taille d'une tuile ne vaut jamais une valeur intermédiaire) et mettre à jour le `README.md`

## 3. Orientations dans le cœur

- [x] 3.1 Ajouter `Rotation` et `Furniture` (ancre, orientation) avec l'emprise tournée (2×1 ↔ 1×2, bac 1×1) et la tête de lit selon l'orientation ; vérifier par des tests unitaires (les quatre orientations d'un meuble de 2 tuiles, la tête d'un lit pour chaque orientation, la tuile de tête désigne la place)
- [x] 3.2 Faire porter `HabitatSpec` par des `Furniture` (lits, table, étagère, bac), valider les emprises tournées (dans l'intérieur, sans chevauchement, tuiles de lit joignables depuis l'entrée) et simplifier les tuiles de meuble en `Tile::Bed` (praticable) et `Tile::Furniture` (solide) ; vérifier par des tests unitaires (lit vertical accepté, meuble tourné qui déborde refusé, lit emmuré par des meubles tournés refusé, les tuiles de la carte correspondent aux emprises) et que `cargo test -p afk-core` compile

## 4. Scénario réorganisé

- [x] 4.1 Réorganiser l'intérieur des dix habitats avec les orientations (étagère contre le mur nord regardant vers le sud, lit vertical tête en haut contre un mur latéral, table au centre, baquet près du mur sud), maison du paysan de 1 à 6 paysans comprise ; mettre à jour les `home`, les postes de travail et les tests de scénario et de `game.rs` ; vérifier que les habitats sont valides et intacts sur la carte, que tous les lits sont joignables depuis le château et que `cargo test` passe

## 5. Rendu des habitats

- [x] 5.1 Écrire la fonction pure `wall_decor(habitat)` (cheminée, fenêtres selon la classe, jamais sur la porte ni l'un sur l'autre, dans le mur nord, aucune tuile de sol prise) ; vérifier par des tests unitaires pour les trois classes et pour une porte au nord
- [ ] 5.2 Dessiner la coupe d'un habitat : sol en planches (variantes déterministes), mur nord de rondins avec bande de chaume, murs ouest et est, mur sud avec la porte, paillasson, raccords des coins ; passe `draw_habitats` qui remplace la passe de tuiles pour les habitats ; vérifier à l'écran aux quatre zooms que les pixels restent nets et que château, forge et taverne n'ont pas changé
- [ ] 5.3 Dessiner fenêtres, cheminée de pierre au feu animé et leur lueur (fenêtres allumées la nuit, sol éclairé près du feu) ; vérifier à l'écran de jour et de nuit
- [ ] 5.4 Dessiner les meubles d'après l'image dans leurs quatre orientations (lit à oreiller et couverture rouge, table à assiette et tasse, étagère à livres et flacons avec ses vues de face, de dos et de profil, baquet cerclé avec sa louche) ; vérifier à l'écran sur les dix habitats, avec des meubles tournés dans plusieurs sens
- [ ] 5.5 Dessiner le personnage couché dans le sens de son lit (horizontal ou vertical, tête sur l'oreiller) ; vérifier à l'écran de nuit dans un lit vertical et dans un lit horizontal

## 6. Documentation et vérification finale

- [x] 6.1 Mettre à jour `docs/gameplay/habitat.md` et `README.md` (direction artistique, zoom entier, meubles tournables, aspect des maisons) ; vérifier la cohérence des textes
- [ ] 6.2 Intégration : `cargo test` complet puis une partie `--speed 20` de jour et de nuit ; vérifier que les maisons ressemblent à l'image de référence, que les personnages entrent, circulent et dorment normalement, et que tout le reste du jeu est inchangé
