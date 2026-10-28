# Proposal

## Why

Les personnages ne sont aujourd'hui que des rectangles colorés, et la tuile de base de 16 px est trop petite pour un personnage lisible : un visage de 3 px, un bras sur l'enclume à peine visible. On veut un vrai personnage animé, en commençant par le **forgeron**, et poser du même coup l'**échelle de tout le jeu** et un **squelette d'animation** que les autres métiers reprendront.

Deux échantillons ont été comparés (`docs/images/samples/comparaison_zoom_de_depart_64.png`) : une tuile dessinée à 32 px et une autre à 64 px, toutes deux affichées à 64 px à l'écran. La tuile de **32 px** est retenue : un vrai pixel art proche de la référence `docs/images/forgeron_sample.jpg`, avec des paliers de zoom nets en dessous et au-dessus de 64 px.

## What Changes

- **BREAKING** — **La tuile de base passe de 16 à 32 px de dessin.** Paliers de zoom : 32, **64** (départ), 96 et 128 px par tuile. Les dessins de 16 px déjà faits (habitats) restent nets, affichés à une échelle double, jusqu'à ce qu'on les redessine. Au plus petit palier, le royaume entier ne tient plus dans la fenêtre par défaut.
- **Un gabarit de personnage commun** : une cellule de 64×64 px (2×2 tuiles), avec un point d'ancrage aux pieds. On y définit une liste fixe d'animations : debout et marche dans les 4 directions, repos (*idle*), sommeil et travail. Chaque animation a un nombre d'images et une vitesse fixés.
- **Une chaîne SVG → PNG** : chaque personnage est dessiné dans un fichier SVG versionné, organisé en planche (une ligne par animation, une colonne par image). Une commande (`sprites.bat`) le convertit en planche PNG sans lissage, ajoute le contour sombre, crée la marche vers la gauche en miroir de la marche vers la droite, et refuse toute couleur hors palette. Le PNG est commité ; un test vérifie qu'il correspond bien au SVG.
- **Le forgeron dessiné** d'après la référence, mais en **homme** et non en nain : barbe courte, tablier de cuir, manches retroussées. Il a un visage soigné et **identique d'une image à l'autre**, et une marche dont les jambes **alternent strictement** (jamais la même jambe en avant sur deux pas consécutifs).
- **On forge uniquement par le haut de l'enclume** : le forgeron se tient sur la case au nord de l'enclume et regarde vers le bas. On le voit donc lever et abattre le marteau au-dessus de l'enclume, qui est dessinée devant lui et lui cache le bas du corps. L'IA s'y place d'elle-même. Le joueur reçoit l'option « forger » depuis n'importe quel côté de l'enclume, comme aujourd'hui : la touche d'action fonctionne comme maintenant (un appui), et si le forgeron n'est pas au poste, il y marche seul, puis forge. **BREAKING** pour le gameplay : on ne forge plus depuis le côté.
- L'**enclume** est redessinée dans le nouveau style, dans la même planche, et les étincelles sont calées sur l'image d'impact.
- Hors périmètre : les autres métiers (ils gardent leurs rectangles), le redessin du terrain, des habitats et des bâtiments à 32 px, les autres activités de l'image de référence (boire, porter, tremper la lame, montrer), une animation de transport, une vue d'ensemble de la carte (la carte deviendra bien plus grande ; une mini-carte est possible mais pas encore décidée).

## Capabilities

### New Capabilities
- `character-sprites` : le gabarit commun des personnages (cellule, ancrage, animations et leurs règles) et la chaîne SVG → PNG.
- `blacksmith` : le forgeron, c'est-à-dire où et comment il forge (par le nord de l'enclume, face vers le bas) et son apparence animée.

### Modified Capabilities
- `art-direction` : la tuile de base passe à 32 px ; la palette s'élargit aux personnages ; les anciens dessins de 16 px sont affichés en double.
- `map-navigation` : paliers de zoom de 32, 64, 96 et 128 px, départ à 64 ; le royaume ne tient plus entier au plus petit palier.

## Impact

- `crates/afk-game/src/camera.rs` : `BASE_TILE_PX`, `ZOOM_LEVELS`, `DEFAULT_ZOOM` et leurs tests.
- `crates/afk-game/src/sprites.rs` : palette élargie, échelle des dessins de 16 px.
- `crates/afk-game/src/habitat_view.rs` : échelle d'affichage des habitats.
- `crates/afk-game/src/render.rs` : dessin des personnages et de l'enclume, ordre d'affichage, étincelles.
- Nouveau : `crates/afk-game/src/character_sprites.rs` (gabarit, choix de l'image), un binaire `spritegen` avec la fonctionnalité optionnelle `spritegen` (dépendance `resvg`), `assets/characters/forgeron.svg` et `.png`, et `sprites.bat`.
- `crates/afk-core/src/character.rs` (`forge_routine`, approche du joueur) et `game.rs` (`Interact`, tests du forgeron) : forger seulement depuis le nord de l'enclume, avec une approche automatique du joueur.
- Docs : `docs/art-direction.md`, `docs/gameplay/forgeron.md`.
