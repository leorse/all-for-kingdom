# Proposal

## Why

Le jeu est aujourd'hui dessiné avec des rectangles de couleur calculés à la volée : propre pour tester la logique, mais sans identité. L'image de référence `docs/images/image_maison.jpg` fixe la **direction artistique** du jeu : un pixel art net et chaleureux, vu de dessus en trois-quarts, fait de bois (sol en planches, murs en rondins, meubles), de chaume, de pierre et de touches de couleur franches (couverture rouge, flacons, flammes), avec une lumière chaude. Cette image montre l'intérieur d'une maison ; elle doit servir d'abord à refaire **toutes les maisons**, puis de modèle au reste du jeu.

## What Changes

- **Direction artistique écrite** : un guide de style (`docs/art-direction.md`) tiré de l'image : pixel art net, tuile de base de **16×16 pixels**, palette limitée chaude, contours sombres, lumière chaude, vue de dessus en trois-quarts avec murs vus de face au nord.
- **Pixels nets à l'écran** : le dessin est affiché sans lissage, à une échelle **entière**. Le zoom à la molette passe donc de sept à quatre paliers : **×1, ×2, ×3, ×4** (16, 32, 48 et 64 pixels par tuile) ; le départ reste ×2 (32 px).
- **Les maisons d'abord** : chaque habitat est redessiné comme sur l'image. Vue en coupe : bande de toit de chaume au nord, mur nord en rondins avec fenêtres et cheminée de pierre, murs ouest et est en bois, mur sud bas avec porte et paillasson, sol en planches. Meubles reconnaissables : lit à couverture rouge, table avec assiette et tasse, étagère à livres et flacons, baquet d'eau cerclé avec sa louche. Les fenêtres s'éclairent la nuit, le feu de la cheminée est animé.
- **Tout peut être tourné** : lits, tables, étagères, baquets, et tout meuble à venir, se posent dans **quatre rotations** (0°, 90°, 180°, 270°). Un meuble de 2×1 devient 1×2 aux quarts de tour impairs ; la tête d'un lit suit la rotation (le lit vertical tête en haut de l'image est un lit tourné de 90°). Les règles de logement (lits joignables, pas de chevauchement) s'appliquent à la pose choisie.
- **Le reste du jeu suit ensuite** : terrain, arbres, champs, filons, coffres, forge, taverne, personnages et interface gardent leur apparence actuelle tant qu'ils ne sont pas redessinés, un élément à la fois, dans ce style. Le château est hors périmètre pour l'instant.
- Hors périmètre : usage gameplay des nouveaux décors (chaises, coffre intérieur, bouclier), agrandissement réel des maisons, animation des personnages, musique et sons.

## Capabilities

### New Capabilities
- `art-direction`: le style visuel du jeu (pixel art net, tuile de 16 px, échelle entière, palette, lumière) et la façon dont il s'étend progressivement à tous les éléments.
- `house-appearance`: l'aspect des habitats : coupe de la maison, sol, murs, fenêtres, cheminée, mobilier, paillasson.

### Modified Capabilities
- `housing`: les meubles se posent dans quatre orientations ; la taille d'un meuble tourne avec lui.
- `map-navigation`: le zoom à la molette n'a plus que des paliers entiers ×1 à ×4.

## Impact

- `crates/afk-core` : un type `Rotation` (haut, droite, bas, gauche) ; le modèle des habitats (`habitat.rs`) pose chaque meuble avec son orientation (emprise tournée, tête de lit selon l'orientation) et le valide ; tuiles de meuble simplifiées (un lit praticable, un meuble solide), le dessin retrouvant l'orientation dans la description de l'habitat ; `scenario.rs` : habitats réorganisés (orientations, nouveaux décors muraux).
- `crates/afk-game/src/camera.rs` : paliers de zoom 16, 32, 48, 64.
- Nouveau module de sprites pour `afk-game` : définition des pixels d'un sprite, palette commune, création de textures sans lissage ; `render.rs` : dessin des habitats par sprites (sol, murs, toit, fenêtres, cheminée, meubles, paillasson), lueur des fenêtres la nuit, flammes animées, personnage couché dans le bon sens.
- `docs/art-direction.md` (nouveau), `docs/gameplay/habitat.md`, `README.md`.
