# Design

## Context

Tout le rendu (`render.rs`) est fait de rectangles de couleur calculés à chaque image à partir de `view.tile_px / 32` ; il n'y a ni sprite, ni texture, ni palette. Les paliers de zoom sont `[16, 20, 24, 32, 40, 48, 64]` (`camera.rs`). Un habitat est décrit dans le cœur par `HabitatSpec` (lits, table, étagère et bac posés par une tuile de gauche, meubles de 2×1 horizontaux) et la carte en porte des tuiles par moitié (`Bed`/`BedFoot`, `Table`/`TableEnd`, `Shelf`/`ShelfEnd`, `Trough`). Le cœur ne connaît pas l'apparence ; il fournit l'état que le rendu traduit. Voir `proposal.md` et l'image `docs/images/image_maison.jpg`.

## Goals / Non-Goals

**Goals:**
- Un pipeline de sprites en pixel art net réutilisable pour tout le jeu, avec une palette commune vérifiée.
- Les habitats entièrement redessinés d'après l'image, vus en coupe.
- Des meubles qui se posent dans quatre orientations, avec les règles de logement appliquées à l'emprise tournée.

**Non-Goals:**
- Redessiner autre chose que les habitats (terrain, arbres, champs, filons, coffres, forge, taverne, personnages, interface, château) ; l'usage gameplay des décors ; l'animation des personnages ; la création de fichiers image externes.

## Decisions

- **Sprites écrits en texte, textures créées au démarrage.** Un sprite est une grille de caractères (une lettre = une couleur de la palette, `.` = transparent) rangée dans un module `sprites.rs` de `afk-game`. Au lancement, chaque grille devient une `Texture2D` (macroquad) avec `FilterMode::Nearest`, et un petit utilitaire la dessine à une taille entière. Alternative : des fichiers PNG dans `assets/` — rejetée pour l'instant : je ne peux pas produire d'images, le texte est relisible et testable, et le même utilitaire pourra charger des PNG plus tard sans toucher au reste. Le code de dessin ne dépend que de l'API « dessiner le sprite N à (x, y) à l'échelle s ».
- **Tuile de base de 16 px, échelle entière.** `échelle = tile_px / 16` (1 à 4). Les positions écran des sprites sont arrondies à l'entier. Les paliers de zoom deviennent `[16, 32, 48, 64]`, départ 32 (indice 1). Le test « la carte 64×40 tient dans 1024×640 au palier minimal » reste vrai (16 px).
- **Palette commune dans `sprites.rs`**, tirée de l'image : bois (clair `#d9a557`, moyen `#b27a3c`, foncé `#7a4a22`, ombre `#4a2c14`), chaume (`#d8b04a`, `#b88a2e`), pierre (`#9a9a9c`, `#727276`, `#4a4a50`), rouge de couverture (`#c8323c`, `#8e1f2a`), blanc cassé `#f0ead8`, vert `#4c9a3c`, eau (`#4a86c8`, `#8cc0ec`), métal `#8a8f9a`, lueur de fenêtre `#ffe9a0`, flammes (`#ff9a2a`, `#ffd23c`) et contour `#2a1a12`. Un test vérifie que chaque lettre d'un sprite existe dans la palette et que chaque ligne a la bonne largeur.
- **Rotations dans le cœur.** Un type `Rotation { Deg0, Deg90, Deg180, Deg270 }` (quarts de tour dans le sens des aiguilles d'une montre depuis la pose de base) et un `Furniture { anchor, rotation }` (ancre = tuile en haut à gauche de l'empreinte). La pose de base est horizontale (2×1), tête de lit à gauche, façade d'étagère vers le bas ; aux quarts de tour impairs l'empreinte d'un meuble de 2 tuiles devient 1×2 ; le bac fait toujours 1×1. La **tête d'un lit** est à gauche à 0°, en haut à 90°, à droite à 180°, en bas à 270°, et c'est elle que désigne `home`. Le lit vertical de l'image est donc tourné de 90°. La façade d'une étagère regarde vers le bas, la gauche, le haut, la droite. Une même `Rotation` servira à tout futur meuble ou bâtiment.
- **`HabitatSpec` avec orientations.** `beds: Vec<Furniture>`, `table`, `shelf`, `trough` : `Furniture`. La validation calcule les emprises tournées, exige qu'elles tiennent dans l'intérieur sans se chevaucher, et que toute tuile de lit soit joignable depuis l'entrée. Le gabarit des règles de logement (maximum de places, trois cases libres, agrandissement) est inchangé.
- **Tuiles de meuble simplifiées.** Avec quatre orientations, des moitiés nommées (`BedFoot`, `TableEnd`…) ne suffisent plus. La carte porte seulement `Tile::Bed` (praticable, toute tuile de lit) et `Tile::Furniture` (solide, toute tuile de table, d'étagère ou de bac) ; c'est le **rendu qui retrouve le meuble et son orientation** dans la description de l'habitat. Le lit du château (une tuile, hors habitat) reste un `Tile::Bed` dessiné comme avant.
- **Les habitats se dessinent par une passe dédiée**, après le terrain : `draw_habitats` parcourt `game.habitats()` et dessine, pour chacun, sol, murs, toit, décors, meubles, paillasson ; la passe de tuiles ignore les tuiles couvertes par un habitat. Les autres bâtiments (château, forge, taverne) gardent la passe actuelle.
- **Coupe d'un habitat.** Mur nord : une rangée de tuiles de rondins dont la partie haute porte la bande de **chaume** (le toit est dessiné dans la hauteur de la tuile du mur, pour ne rien recouvrir au nord) ; fenêtres, cheminée et paillasson s'y posent. Murs ouest et est : planches de bois verticales. Mur sud : bas, avec la porte. Les coins sont raccordés. Le conduit de cheminée peut dépasser d'une demi-tuile au-dessus du toit. Les décors du mur nord ne couvrent aucune tuile de sol, donc ne changent pas les règles de logement. Alternative : un mur nord de deux tuiles de haut comme sur l'image — rejetée pour l'instant : elle demande de prendre une rangée de plus au nord de chaque habitat, qui collisionne avec le décor (filon de fer au nord de la mine). Question ouverte.
- **Décor du mur nord calculé par une fonction pure** (`wall_decor(habitat)` dans `afk-game`) : la cheminée à une position fixe sur le mur ; une, deux ou trois fenêtres (petite, moyenne, grande) réparties sur les tuiles restantes ; jamais sur la porte ni sur la cheminée. Testée sans fenêtre graphique.
- **Sol en planches.** Une tuile de sol en quelques variantes (planches de longueurs différentes, joints), choisie par un hachage déterministe de la position pour éviter la répétition visible.
- **Lumière.** La nuit les fenêtres sont dessinées allumées (lueur sur le sol dessous) ; la cheminée projette une lueur ambrée qui pulse légèrement, dessinée avant la passe de lumière du jour pour rester visible la nuit.
- **Animation.** Les flammes alternent trois images selon `game.time()` ; aucun état supplémentaire.
- **Personnage couché.** Le dessin du dormeur dépend de l'orientation de son lit : tête sur la tuile de tête, corps vers le pied, horizontal ou vertical.
- **Scénario réorganisé.** Les dix habitats reprennent leur emprise, leurs portes et leurs chemins actuels ; seule la disposition intérieure change pour profiter des orientations, sur le modèle de l'image : étagère contre le mur nord regardant vers le sud, lit vertical contre un mur latéral (tête en haut), table au centre, baquet près du mur sud. Les cases `home` changent avec la tête de lit ; les positions de postes de travail qui tombent sur un meuble sont ajustées. Le château n'est pas touché.

## Risks / Trade-offs

- [L'effort de dessin est important et je ne peux pas juger le rendu final à l'œil] → le pipeline est testable (palette, dimensions) ; la ressemblance à l'image est vérifiée par vous à l'écran, tâche par tâche, et les sprites se retouchent dans un seul fichier texte.
- [Un meuble haut tourné (étagère de profil) demande des dessins par orientation] → quatre vues pour l'étagère (face, dos, deux profils), des vues pivotées pour les meubles plats (lit, table, baquet, tapis).
- [Zoom à quatre paliers : moins de finesse qu'avant] → c'est le prix de pixels nets ; le palier ×2 reste le départ.
- [Un toit dans la hauteur de la tuile du mur est plus bas que sur l'image] → décision réversible, notée en question ouverte.
- [Beaucoup de tests de `habitat.rs`, `scenario.rs` et `game.rs` citent les tuiles `BedFoot`/`TableEnd` ou des positions de lits] → mises à jour ciblées avec la nouvelle API.

## Open Questions

- Faut-il un mur nord de deux tuiles de haut, comme sur l'image, quitte à décaler les filons voisins ?
- Quelle orientation exacte pour chaque meuble dans chaque classe ? Proposée dans le scénario ; à retoucher à l'œil.
- Chaises, coffre intérieur, bouclier : décors de l'image non couverts ici, à ajouter plus tard comme meubles tournables.
