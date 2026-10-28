# Design

## Context

- Le rendu (`afk-game`, macroquad) dessine les personnages en rectangles colorés (`render.rs`, `draw_characters`). Leur taille dépend de `s = tile_px / 32`.
- Les habitats sont en pixel art de 16 px (`sprites.rs`), sous forme de grilles de lettres-couleurs. Ils sont affichés par `Sprite::draw(x, y, scale)` avec `scale = tile_px / 16`, et un test vérifie la palette.
- `camera.rs` fixe `BASE_TILE_PX = 16`, `ZOOM_LEVELS = [16, 32, 48, 64]` et `DEFAULT_ZOOM = 1`. La carte fait 64×40 tuiles et la fenêtre par défaut 1024×640 : au palier 16, la carte tient exactement.
- L'enclume est une tuile solide (`Tile::Anvil`, en (39,15) au milieu de la forge), dessinée dans la passe des tuiles, donc **sous** les personnages. Les étincelles (`draw_forge_sparks`) suivent une horloge libre.
- `forge_routine` va vers n'importe quelle case voisine praticable d'une enclume libre ; `available_work` accepte la case regardée ou la case occupée. Les tests du forgeron le placent à l'ouest, en (38,15), regardant à droite.
- Le cœur expose déjà `activity`, `facing`, `work` et `pos` : le rendu a tout ce qu'il faut pour choisir une image.
- On compile avec la toolchain GNU (`build.bat`) : les dépendances doivent être en Rust pur.
- Deux échantillons de taille ont été produits par un script Python jetable (des formes géométriques échantillonnées au centre des pixels, puis un contour). C'est ce qui a montré que la tuile de 32 px convient.

## Goals / Non-Goals

**Goals:**
- Un SVG lisible et modifiable à la main (Inkscape ou éditeur de texte) comme source unique ; les PNG en découlent par une seule commande.
- Un gabarit défini une seule fois, partagé par l'outil de génération et par le jeu.
- Le jeu se construit et se lance sans l'outil ni ses dépendances.

**Non-Goals:**
- Redessiner à 32 px les habitats, le terrain ou les autres bâtiments.
- Un éditeur d'animation, des atlas de textures ou du chargement à l'exécution depuis le disque.
- Caler la marche sur la distance réellement parcourue (voir Risques).

## Decisions

### Échelle
- `BASE_TILE_PX = 32`, `ZOOM_LEVELS = [32, 64, 96, 128]`, `DEFAULT_ZOOM = 1` (64 px). Le palier de départ garde l'index 1 : le reste du code qui l'utilise ne change pas.
- **Dessins de 16 px : `scale = tile_px / 16`**, comme aujourd'hui, ce qui donne toujours un entier (2, 4, 6 ou 8). Rien ne change dans `habitat_view.rs`, à part la vérification que ses tailles se déduisent de `tile_px` et non de `BASE_TILE_PX`. Dessins de 32 px : `scale = tile_px / 32` (1, 2, 3 ou 4).
  - *Alternative* : doubler les grilles de 16 px en 32 px une fois pour toutes. Rejeté : ça ne change rien à l'écran et ça complique le redessin futur.
- Le test « la carte tient au plus petit palier » devient : « au plus petit palier, au moins un quart de la carte est visible ».
  - *Alternative* : un palier ×½ de vue d'ensemble. Rejeté, il ne serait pas net ; une mini-carte pourra venir plus tard.

### Format du SVG source
- `crates/afk-game/assets/characters/forgeron.svg`. Le `viewBox` est en pixels de dessin, et `shape-rendering="crispEdges"` est posé sur la racine.
- La planche est une grille de cellules 64×64. Chaque image est un `<g id="<animation>_<n>" transform="translate(col*64, ligne*64)">`. Ouvert dans un navigateur ou dans Inkscape, on voit toute la planche d'un coup.
- **Décidé à la mise en œuvre** : le contenu des images est fait de **pièces nommées** (tête, torse, bras, jambes, marteau...). Chacune est un groupe de rectangles alignés sur la grille des pixels, fusionnés en bandes. Les pièces identiques d'une image à l'autre (les têtes, les torses) sont dans `<defs>` et réutilisées par `<use x y>`. Le visage est donc le même partout par construction, et modifier une pièce la modifie dans toutes les images.
  - *Pourquoi pas des formes libres (ellipses, chemins) rendues par resvg ?* À cette taille, le résultat d'une ellipse au pixel près dépend de l'échantillonnage. Des rectangles alignés donnent exactement les pixels voulus et restent modifiables à la main.
  - Le premier jet a été produit par un script d'aide, non commité. Il décrit les pièces par des formes, les pose image par image (la marche, le geste de forge) et les échantillonne au centre des pixels. Le SVG reste la seule source de vérité : on le retouche directement.
- Les ombres et les reflets sont **dessinés** comme des pixels de teinte plus sombre ou plus claire, sans calcul automatique. Seul le **contour extérieur** est ajouté par l'outil : un pixel `k` autour de toute la silhouette opaque de chaque cellule, sans déborder de la cellule. Le dessin garde donc une marge d'un pixel.
- Les images vers la gauche sont absentes du SVG : l'outil les crée en miroir horizontal des images vers la droite, par rapport à l'axe de l'ancre. Si un groupe `marche_gauche_n` ou `debout_gauche` existe, il est pris tel quel ; une asymétrie, comme le marteau à la ceinture, pourra ainsi être corrigée plus tard.

### Gabarit partagé (`character_sprites.rs`)
- Une table constante de `Anim { nom, ligne, images, ips }`, avec `CELL = 64` et `ANCHOR = (32, 58)`. L'ancre est l'endroit des pieds dans la cellule ; elle est posée sur `c.pos`.

  | Animation | Images | Images/s |
  |---|---|---|
  | `debout_{bas,haut,droite,gauche}` | 1 | — |
  | `marche_{bas,haut,droite,gauche}` | 4 (contact G, croisé, contact D, croisé) | 8 |
  | `repos` | 4 (respiration, clignement sur une image) | 3 |
  | `sommeil` | 2 | 1 |
  | `travail` | 6 pour le forgeron (marteau bas, lever, haut, descente, impact, rebond) | 8 |

- Le même module donne au rendu le choix de l'image, `frame_for(activity, facing, work, temps) -> (ligne, colonne)`. Les animations bouclent sur `game.time()`, avec un décalage par identifiant de personnage pour éviter que tout le monde soit synchronisé.
- Ce module n'a **aucune dépendance** sur macroquad ni sur resvg. Le jeu et l'outil l'utilisent tous les deux.

### Outil de génération
- C'est un binaire `spritegen` dans `afk-game`, avec `required-features = ["spritegen"]`. La fonctionnalité `spritegen` active les dépendances optionnelles `resvg` (qui contient usvg et tiny-skia, en Rust pur) et `png`. Le lancement se fait par `sprites.bat` (`cargo +stable-x86_64-pc-windows-gnu run --features spritegen --bin spritegen`).
  - *Alternative 1* : un `build.rs` qui génère à chaque compilation. Rejeté : resvg serait toujours compilé, et on voudrait aussi voir le PNG dans git.
  - *Alternative 2* : un script Python. Rejeté : une seconde chaîne d'outils, et le gabarit en double.
- Les étapes :
  1. Lire le SVG.
  2. Vérifier qu'il contient tous les groupes exigés par le gabarit (sinon, erreur qui nomme l'image).
  3. Le rendre à l'échelle 1, sans lissage (`crispEdges`).
  4. Créer les miroirs.
  5. Ajouter le contour.
  6. Vérifier la planche : chaque pixel a une transparence de 0 ou 255, et chaque pixel opaque est une couleur de `PALETTE` (sinon, erreur avec la position et la couleur).
  7. Écrire `assets/characters/forgeron.png`.
  8. Écrire un aperçu agrandi ×4 avec la grille et les noms d'images dans `target/sprites-preview/` : c'est là qu'on relit l'alternance des jambes et le visage.
- L'outil traite tous les `*.svg` de `assets/characters/` : un futur personnage n'a qu'à ajouter son fichier.

### Chargement dans le jeu
- La planche est embarquée par `include_bytes!`, puis chargée en `Texture2D` avec `FilterMode::Nearest`. Le jeu dessine la sous-image `(col*64, ligne*64, 64, 64)` avec `draw_texture_ex` à la position `snap(x - 32*scale, y - 58*scale)`, en taille `64*scale`.
- L'ombre au sol est une ellipse sombre semi-transparente, dessinée par le jeu sous les pieds.
- Les rôles qui n'ont pas de planche gardent le dessin actuel en rectangles. Un `match` sur `Role` renvoie `Option<&CharacterSheet>`.
- Le marqueur du personnage incarné, la charge portée et la barre de progression sont placés par rapport à la nouvelle hauteur : au-dessus de la tête pour le marqueur, sous les pieds pour la barre.

### Sommeil
- L'image `sommeil` est dessinée tête en haut, dans la colonne centrale de 32 px de la cellule. La tête est sur l'oreiller et le corps sous une couverture rouge (`R`/`r`), comme celle du lit.
- Elle est tournée selon l'orientation du lit avec le même procédé que les meubles. `sleep_direction` donne déjà l'orientation ; on en tire un nombre de quarts de tour, et on pré-calcule les 4 rotations de cette cellule au chargement. Le « z » flottant reste.

### Forge par le nord
- **Cœur** : une fonction `forge_spot(anvil) = anvil + (0, -1)`.
  - `forge_routine` ne cherche plus `walkable_neighbors` : elle prend `forge_spot` de chaque enclume libre. Elle fait `facing = Down` une fois arrivée.
  - La réservation (`claimed`) ne change pas : avec une seule case de forge, elle exclut déjà un second forgeron.
- **Joueur — proposer depuis n'importe quel côté, forger seulement au poste.** `available_work` garde sa règle actuelle pour l'enclume (on la regarde depuis une case voisine) et renvoie toujours `(Forge, enclume)`. C'est ce qui affiche « E : forger ». Le lancement, lui, dépend de la position :
  - **Au poste, face en bas** : `Interact` lance le travail tout de suite, comme aujourd'hui.
  - **Ailleurs** : `Interact` ne lance pas le travail. Il pose sur le personnage une **approche** (`Approach { anvil }`) avec le chemin vers `forge_spot` (`find_path`). Tant que l'approche existe, le pas de simulation du personnage incarné suit ce chemin (`follow_path`, comme l'IA), au lieu du `Move` du joueur. Un `Move` de direction nulle ne l'interrompt pas. À l'arrivée, on fait `facing = Down`, on revérifie `available_work` et on lance `Forge`.
  - **Fin de l'approche** : un `Move` non nul ou un nouvel `Interact` l'annulent, exactement comme ils annulent un travail en cours. Si le fer manque ou que l'enclume est prise pendant le trajet, l'approche s'annule à l'arrivée, sans forger.
  - **La touche d'action ne change pas** : `input.rs` envoie toujours `Interact` sur `is_key_pressed(E)` seulement. Il n'y a ni commande de relâchement ni notion de touche tenue.
  - *Alternative* : téléporter le forgeron au poste. Rejeté : on veut le voir marcher.
  - Le mécanisme est générique (`Approach` vers un poste de travail), mais on ne le branche ici que sur l'enclume.
- Un test de scénario vérifie que la case nord de chaque enclume est praticable. Les tests existants qui posent le forgeron en (38,15) face à droite deviennent des tests d'approche.
- **Ordre d'affichage** : l'enclume quitte la passe des tuiles. Elle est dessinée dans la passe triée par `y` avec les personnages. Son `y` de tri est le bas de sa case, donc plus grand que celui du forgeron (pieds au centre de la case nord) : elle passe devant lui.
- L'enclume est une cellule spéciale de la planche du forgeron, l'image `enclume` en 64×64, ancrée sur sa case comme un personnage, plutôt qu'un sprite de 16 px. Elle est posée sur un billot de bois, et le dessus de sa table est à 26 px dans sa cellule.
- **Décidé à la mise en œuvre** : dans ses images de forge, le forgeron est dessiné **16 px plus bas** que debout, comme penché sur l'enclume. Ses jambes sortent du bas de la cellule ; elles sont de toute façon cachées par l'enclume. Sans ce décalage, une case entière sépare ses pieds de l'enclume : elle ne cacherait rien, et le marteau ne pourrait pas l'atteindre dans une cellule de 64 px. Le métal chaud, posé sur l'enclume, fait partie des images de forge ; les étincelles sont dessinées par le jeu.
- **Étincelles** : `draw_forge_sparks` lit l'image de forge en cours avec le même `frame_for`. Elle ne déclenche une gerbe qu'au début de l'image d'impact (n° 4) ; le métal chaud est dessiné dans la planche, sur l'enclume, et non par le jeu.

### Palette
- On ajoute à `PALETTE` des lettres libres pour la peau (3 tons), les cheveux et la barbe (3), le cuir du tablier (3), le tissu du pantalon (3) et l'ombre de la chemise. Le bois, la pierre, le métal, les braises et les blancs cassés existants sont réutilisés.
- Le test de palette de `sprites.rs` couvre déjà les grilles. La vérification de la planche PNG se fait sur les valeurs RVB, avec la même `PALETTE`.

### Test de concordance
- Le test `cargo test --features spritegen` (lancé aussi par `sprites.bat --check`) régénère la planche en mémoire et la compare octet par octet au PNG commité.
- Sans la fonctionnalité, un test toujours actif décode le PNG embarqué et vérifie trois choses : ses dimensions correspondent au gabarit, ses pixels sont dans la palette, et aucune cellule exigée n'est vide.

## Risks / Trade-offs

- [Deux densités de pixels à l'écran : des habitats en gros pixels de 4×4 et un forgeron en 2×2] → C'est assumé et temporaire, puisqu'il faudra de toute façon tout repenser. On le note dans `docs/art-direction.md`.
- [resvg ne respecte pas `crispEdges` sur une forme et produit des pixels de couleur intermédiaire] → La vérification de palette fait échouer la génération en donnant la position. Au besoin, on aligne la forme sur la grille.
- [Le contour ajouté automatiquement épaissit les petites formes, comme le manche du marteau] → Une forme qui touche le bord de la silhouette prend un pixel de contour. Le manche fait donc au moins 2 px, ce qui sera vérifié sur l'aperçu.
- [La marche est cadencée dans le temps et pas sur la distance : les pieds peuvent glisser un peu si la vitesse change] → La vitesse de marche est fixe dans le jeu, et 8 images/s est réglé pour une case parcourue par cycle. On ne refera ce calage que si la vitesse varie un jour.
- [Le miroir de la marche droite donne une image gauche où le marteau change de côté de la ceinture] → C'est acceptable pour un essai, et le SVG peut fournir ses propres images gauches.
- [Au zoom de départ (64 px), on voit 16×10 tuiles au lieu de 32×20] → C'est voulu. Le palier de 32 px redonne la vue d'avant.

## Migration Plan

Il n'y a ni sauvegarde ni réseau à migrer. Le changement se livre en un bloc ; pour revenir en arrière, on annule le commit. Les échantillons de comparaison restent dans `docs/images/samples/` (seulement les deux planches de comparaison) comme trace de la décision de taille.
