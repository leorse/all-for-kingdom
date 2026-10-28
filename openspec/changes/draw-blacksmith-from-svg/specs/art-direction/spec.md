## MODIFIED Requirements

### Requirement: Une référence et un guide de style
Le dépôt SHALL contenir l'image de référence `docs/images/image_maison.jpg`, l'image d'inspiration des personnages `docs/images/forgeron_sample.jpg` et un guide de style `docs/art-direction.md` qui en tire les règles : pixel art net, tuile de base de 32×32 pixels, personnages dans une cellule de 64×64, palette chaude et limitée, contours sombres, lumière chaude, vue de dessus en trois-quarts. Tout nouvel élément dessiné SHALL suivre ce guide.

#### Scenario: Le guide existe
- **WHEN** on ouvre le dossier `docs`
- **THEN** on y trouve `art-direction.md` qui cite les images de référence et décrit la palette, la taille de base, le gabarit des personnages et les règles de dessin

### Requirement: Tuile de base de 16 pixels, affichage net
Le dessin SHALL partir d'une tuile de base de 32×32 pixels. Il SHALL être affiché sans aucun lissage, à une échelle entière de cette base, de sorte que chaque pixel du dessin devienne un carré net de pixels d'écran. Les dessins faits sur l'ancienne tuile de 16 pixels SHALL être affichés à une échelle double de celle des dessins de 32 pixels, pour occuper la même place, et rester nets jusqu'à ce qu'ils soient redessinés.

#### Scenario: Pixels nets
- **WHEN** un sprite est affiché à n'importe quel palier de zoom
- **THEN** chaque pixel du dessin est un carré entier de pixels d'écran, sans flou ni demi-pixel

#### Scenario: Taille d'une tuile à l'écran
- **WHEN** le zoom est au palier de départ
- **THEN** une tuile fait 64 pixels d'écran de côté et chaque pixel d'un dessin de 32 pixels en occupe 2×2

#### Scenario: Un ancien dessin de 16 pixels
- **WHEN** le zoom est au palier de départ
- **THEN** un mur d'habitat dessiné sur 16 pixels occupe toujours une tuile de 64 pixels d'écran, chacun de ses pixels en occupant 4×4

### Requirement: Une palette partagée
Tous les sprites dessinés dans ce style SHALL n'utiliser que les couleurs d'une palette commune, définie une fois : plusieurs tons de bois, le chaume, la pierre, le rouge de la couverture, le vert, le bleu de l'eau, les blancs cassés, les contours sombres, et pour les personnages la peau, les cheveux, le cuir et le tissu des vêtements. Un sprite ne SHALL pas introduire de couleur hors de la palette.

#### Scenario: Couleur hors palette
- **WHEN** un sprite référence une couleur absente de la palette
- **THEN** il est refusé par la vérification des sprites

#### Scenario: Même bois partout
- **WHEN** on compare le bois d'un lit, d'une table et d'un mur
- **THEN** leurs tons viennent de la même gamme de la palette

### Requirement: Un style qui s'étend élément par élément
Le style SHALL devenir celui de tout le jeu, élément par élément. Les habitats ont été les premiers, puis le forgeron et l'enclume. Tant qu'un élément n'a pas été redessiné, il garde son apparence actuelle, et un même bâtiment ne SHALL pas mêler les deux styles.

#### Scenario: Les maisons d'abord
- **WHEN** la partie démarre après cette étape
- **THEN** tous les habitats, le forgeron et l'enclume sont dessinés dans le nouveau style ; le terrain, les arbres, les champs, les filons, les coffres, la taverne et les autres personnages gardent leur apparence d'avant

#### Scenario: Pas de bâtiment mixte
- **WHEN** on regarde un habitat
- **THEN** son toit, ses murs, son sol et ses meubles sont tous dans le nouveau style
