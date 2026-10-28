# Tasks

## 1. Échelle : tuile de 32 px

- [x] 1.1 Passer `camera.rs` à `BASE_TILE_PX = 32`, `ZOOM_LEVELS = [32, 64, 96, 128]`, départ à l'index 1 (64 px) ; remplacer le test « la carte entière tient au plus petit palier » par « au moins un quart de la carte est visible » ; vérifier que `cargo test -p afk-game` passe
- [x] 1.2 S'assurer que les dessins de 16 px (habitats) sont affichés à `tile_px / 16` et restent nets à chaque palier ; vérifier en jeu à 32, 64, 96 et 128 px qu'une maison occupe toujours ses tuiles, sans flou ni décalage
- [x] 1.3 Mettre à jour `docs/art-direction.md` : tuile de 32 px, paliers, coexistence temporaire avec les dessins de 16 px affichés en double, et lien vers `docs/images/forgeron_sample.jpg` et les planches de comparaison ; garder dans `docs/images/samples/` les deux planches `comparaison_*.png` et supprimer les autres fichiers d'échantillon ; vérifier que les liens du guide s'ouvrent

## 2. Gabarit et palette

- [x] 2.1 Créer `crates/afk-game/src/character_sprites.rs` avec la cellule (64), l'ancre (32, 58), la table des animations (lignes, nombre d'images, images/s) et `frame_for(activity, facing, work, temps, id)`, sans dépendance à macroquad ; vérifier par des tests : marche → bonne ligne selon la direction, inactif regardant vers le bas → repos, inactif regardant vers le haut → debout de dos, l'index d'image boucle dans les bornes, deux personnages ne sont pas synchronisés
- [x] 2.2 Ajouter à `PALETTE` les couleurs des personnages (peau, cheveux et barbe, cuir, tissu, ombre de la chemise) ; vérifier que le test de palette existant passe toujours et que chaque nouvelle lettre est unique

## 3. Outil SVG → PNG

- [x] 3.1 Ajouter la fonctionnalité `spritegen` (dépendances optionnelles `resvg` et `png`) et le binaire `spritegen` (`required-features`), plus `sprites.bat` ; vérifier que `build.bat` compile toujours sans resvg et que `sprites.bat` compile l'outil avec la toolchain GNU
- [x] 3.2 Écrire la génération : lecture de chaque `assets/characters/*.svg`, contrôle des groupes exigés par le gabarit, rendu sans lissage, miroirs gauche/droite (sauf groupes gauche fournis), contour extérieur `k`, contrôle palette et transparence, écriture du PNG et d'un aperçu ×4 annoté dans `target/sprites-preview/` ; vérifier par des tests sur de petits SVG de test : groupe manquant → erreur qui le nomme, couleur hors palette → erreur avec la position, miroir exact, contour d'un pixel sans débordement de la cellule
- [x] 3.3 Ajouter le test de concordance (fonctionnalité `spritegen` ; `sprites.bat --check`) et le test toujours actif sur le PNG embarqué (dimensions, palette, aucune cellule exigée vide) ; vérifier que modifier le SVG sans régénérer fait échouer le premier test

## 4. Le forgeron dessiné

- [x] 4.1 Dessiner dans `assets/characters/forgeron.svg` les poses debout (bas, haut, droite) et le repos : un homme de ~1,7 tuile, barbe courte, tablier de cuir, manches retroussées, d'après la référence ; vérifier sur l'aperçu que le visage est symétrique et identique d'une image de face à l'autre, à part le clignement
- [x] 4.2 Dessiner les marches bas, haut et droite en 4 images (contact gauche, croisé, contact droit, croisé), avec les bras opposés aux jambes ; vérifier sur l'aperçu que les images 1 et 3 avancent des jambes différentes et que les pieds restent sur l'ancre
- [x] 4.3 Dessiner le sommeil (2 images, tête en haut, couverture rouge), la forge (6 images : marteau bas, lever, haut, descente, impact, rebond, avec la pince dans l'autre main) et l'enclume avec le métal chaud ; vérifier sur l'aperçu que le marteau passe au-dessus de la tête et touche le métal à l'image d'impact
- [ ] 4.4 Générer `forgeron.png` avec `sprites.bat` et commiter le SVG et le PNG ; vérifier que `cargo test` passe, test de concordance compris

## 5. Forger par le nord de l'enclume (cœur)

- [x] 5.1 Ajouter `forge_spot(anvil)` (la case au nord) ; faire viser à `forge_routine` uniquement cette case, avec un regard vers le bas à l'arrivée ; vérifier par des tests : l'IA forge depuis le nord, face en bas, jamais depuis un autre côté ; un second forgeron ne peut pas prendre l'enclume
- [x] 5.2 Ajouter l'approche du joueur : `Interact` à côté de l'enclume, hors poste, pose une `Approach` avec le chemin vers `forge_spot` ; le personnage incarné la suit au lieu du `Move` nul ; à l'arrivée, regard vers le bas puis lancement de `Forge` ; un `Move` non nul ou un nouvel `Interact` annulent l'approche ; vérifier par des tests : un seul `Interact` depuis l'ouest suffit pour qu'il contourne l'enclume et forge depuis le nord ; un déplacement ou un second `Interact` en route annulent sans forger ; déjà au poste, un appui forge aussitôt ; sans fer ou avec le poste pris, rien n'est proposé et il ne bouge pas
- [x] 5.3 Vérifier en jeu qu'un appui sur `E` à côté de l'enclume amène le forgeron au poste puis le fait forger, et que `input.rs` n'a pas changé
- [x] 5.4 Adapter les tests existants du forgeron, qui partaient de (38,15) face à droite, et ajouter un test de scénario « la case nord de chaque enclume est praticable » ; vérifier que `cargo test -p afk-core` passe
- [x] 5.5 Mettre à jour `docs/gameplay/forgeron.md` : on ne forge que depuis la case au nord de l'enclume ; l'IA s'y place seule ; le joueur, depuis n'importe quel côté, appuie sur `E` et le forgeron y marche seul puis forge ; vérifier que la doc et les tests disent la même chose

## 6. Affichage dans le jeu

- [x] 6.1 Charger la planche du forgeron (`include_bytes!`, `FilterMode::Nearest`) et dessiner le forgeron par `frame_for`, ancré aux pieds, avec l'ombre au sol, le marqueur d'incarnation au-dessus de la tête et la barre de progression sous les pieds ; les autres rôles gardent leurs rectangles ; vérifier en jeu : marche dans les 4 directions, repos, debout de dos, à chaque palier de zoom
- [x] 6.2 Dessiner le forgeron endormi avec la cellule de sommeil tournée selon son lit ; vérifier en jeu la nuit que la tête est sur l'oreiller
- [x] 6.3 Sortir l'enclume de la passe des tuiles, la dessiner avec la planche dans la passe triée par profondeur, devant le forgeron ; caler les étincelles sur l'image d'impact ; vérifier en jeu que l'enclume cache le bas du forgeron et que les étincelles jaillissent quand le marteau frappe

## 7. Vérification d'ensemble

- [x] 7.1 Lancer `build.bat` puis `run.bat` : partie complète d'une journée en mode AFK puis en incarnant le forgeron (marcher jusqu'à l'enclume, forger, dormir) ; vérifier que tout s'affiche net à chaque palier et que `cargo test` passe intégralement
