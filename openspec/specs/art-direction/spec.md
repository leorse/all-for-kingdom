# art-direction Specification

## Purpose
Définit le style visuel du jeu, tiré de l'image de référence `docs/images/image_maison.jpg` : un pixel art net, chaud et en bois, et la manière dont ce style s'étend peu à peu à tous les éléments.

## Requirements

### Requirement: Une référence et un guide de style
Le dépôt SHALL contenir l'image de référence `docs/images/image_maison.jpg` et un guide de style `docs/art-direction.md` qui en tire les règles : pixel art net, tuile de base de 16×16 pixels, palette chaude et limitée, contours sombres, lumière chaude, vue de dessus en trois-quarts. Tout nouvel élément dessiné SHALL suivre ce guide.

#### Scenario: Le guide existe
- **WHEN** on ouvre le dossier `docs`
- **THEN** on y trouve `art-direction.md` qui cite l'image de référence et décrit la palette, la taille de base et les règles de dessin

### Requirement: Tuile de base de 16 pixels, affichage net
Le dessin SHALL partir d'une tuile de base de 16×16 pixels. Il SHALL être affiché sans aucun lissage, à une échelle entière de cette base, de sorte que chaque pixel du dessin devienne un carré net de pixels d'écran.

#### Scenario: Pixels nets
- **WHEN** un sprite est affiché à n'importe quel palier de zoom
- **THEN** chaque pixel du dessin est un carré entier de pixels d'écran, sans flou ni demi-pixel

#### Scenario: Taille d'une tuile à l'écran
- **WHEN** le zoom est au palier ×2
- **THEN** une tuile fait 32 pixels d'écran de côté et chaque pixel du dessin en occupe 2×2

### Requirement: Une palette partagée
Tous les sprites dessinés dans ce style SHALL n'utiliser que les couleurs d'une palette commune, définie une fois : plusieurs tons de bois, le chaume, la pierre, le rouge de la couverture, le vert, le bleu de l'eau, les blancs cassés et les contours sombres. Un sprite ne SHALL pas introduire de couleur hors de la palette.

#### Scenario: Couleur hors palette
- **WHEN** un sprite référence une couleur absente de la palette
- **THEN** il est refusé par la vérification des sprites

#### Scenario: Même bois partout
- **WHEN** on compare le bois d'un lit, d'une table et d'un mur
- **THEN** leurs tons viennent de la même gamme de la palette

### Requirement: Une lumière chaude
La scène SHALL rester éclairée de façon chaude : l'intérieur des habitats SHALL paraître plus chaleureux que l'extérieur, et les sources de lumière visibles (fenêtres la nuit, feu de cheminée) SHALL éclairer leur voisinage d'une teinte ambrée.

#### Scenario: Le feu éclaire
- **WHEN** le feu d'une cheminée brûle
- **THEN** le sol à sa proximité est plus clair et plus chaud que le reste de la pièce

### Requirement: Un style qui s'étend élément par élément
Le style SHALL devenir celui de tout le jeu, élément par élément. Les habitats sont les premiers. Tant qu'un élément n'a pas été redessiné, il garde son apparence actuelle, et un même bâtiment ne SHALL pas mêler les deux styles.

#### Scenario: Les maisons d'abord
- **WHEN** la partie démarre après cette étape
- **THEN** tous les habitats sont dessinés dans le nouveau style, et le terrain, les arbres, les champs, les filons, les coffres, la forge, la taverne et les personnages gardent leur apparence d'avant

#### Scenario: Pas de bâtiment mixte
- **WHEN** on regarde un habitat
- **THEN** son toit, ses murs, son sol et ses meubles sont tous dans le nouveau style

### Requirement: Le château n'est pas touché
Le château SHALL garder son apparence et sa logique actuelles : il aura son propre fonctionnement.

#### Scenario: Château inchangé
- **WHEN** la partie démarre
- **THEN** le château est dessiné comme avant
