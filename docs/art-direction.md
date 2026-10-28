# Direction artistique

Ce guide fixe l'apparence du jeu. Il part d'une image de référence : [`images/image_maison.jpg`](images/image_maison.jpg), l'intérieur d'une maison. Tout nouvel élément dessiné doit lui ressembler.

![Référence : l'intérieur d'une maison](images/image_maison.jpg)

## Le style en une phrase

Un **pixel art net et chaleureux**, vu de dessus en trois-quarts, fait de **bois** (sol en planches, murs en rondins, meubles), de **chaume** et de **pierre**, avec quelques touches de couleur franches (couverture rouge, livres, flacons, flammes) et une **lumière chaude**.

## Les règles

- **Tuile de base : 16×16 pixels.** Tout est dessiné sur cette grille. Un meuble de deux tuiles fait 32×16 (ou 16×32 tourné).
- **Pixels nets.** Le dessin est affiché sans aucun lissage, à une **échelle entière** : chaque pixel du dessin devient un carré de 2×2, 3×3 ou 4×4 pixels d'écran. Le zoom à la molette a donc quatre paliers : ×1, ×2, ×3, ×4, soit 16, 32, 48 et 64 pixels par tuile (départ ×2).
- **Palette limitée et partagée.** Les couleurs sont celles du tableau ci-dessous, définies une seule fois dans `crates/afk-game/src/sprites.rs`. Un sprite n'en utilise pas d'autre ; un test le vérifie.
- **Un bois, trois ou quatre tons.** Clair pour les arêtes, moyen pour les surfaces, foncé pour les joints et les ombres. Le bois d'un lit, d'une table et d'un mur vient de la même gamme.
- **Contours sombres** là où un objet se détache d'un fond (le baquet, par exemple), ombres dures d'un pixel sous les meubles.
- **Lumière chaude.** Les intérieurs sont plus chaleureux que l'extérieur ; le feu de la cheminée et les fenêtres éclairées de nuit teintent le sol d'ambre.
- **Vue en coupe des maisons.** Bande de toit de chaume et mur de rondins au nord, murs de bois à l'ouest et à l'est, mur sud bas avec la porte, sol en planches.
- **Tout peut être tourné.** Les meubles se posent dans quatre orientations ; leur dessin suit.
- **Un élément à la fois.** Le style s'étend petit à petit. Un élément pas encore redessiné garde son ancien aspect, mais **un même bâtiment ne mélange jamais les deux styles**.

## La palette

| Lettre | Couleur | Usage |
|---|---|---|
| `k` | `#2a1a12` | contour sombre |
| `W` `w` `d` `D` | `#d9a557` `#b27a3c` `#7a4a22` `#4a2c14` | bois : clair, moyen, foncé, ombre |
| `T` `t` `h` | `#d8b04a` `#b88a2e` `#8a6420` | chaume : clair, moyen, foncé |
| `S` `s` `z` | `#9a9a9c` `#727276` `#4a4a50` | pierre : claire, moyenne, foncée |
| `R` `r` | `#c8323c` `#8e1f2a` | rouge de la couverture |
| `F` `f` | `#f0ead8` `#cbbfa3` | blanc cassé, crème (oreiller, assiette) |
| `G` `g` | `#4c9a3c` `#2f6b2a` | vert |
| `B` `b` | `#4a86c8` `#8cc0ec` | eau, eau claire |
| `M` `m` | `#8a8f9a` `#5a5f6a` | métal (cerclage, poignées) |
| `u` `Y` `y` | `#a9d2ec` `#ffe9a0` `#f5c25a` | vitre de jour, vitre éclairée, lueur ambrée |
| `O` `o` `e` | `#ff9a2a` `#ffd23c` `#d8541a` | flammes et braises |
| `L` `N` `P` `Q` | `#3a5fa8` `#a8403a` `#8a4ab0` `#4cb05a` | livres bleu et brun, flacons violet et vert |
| `C` `c` | `#8a8a3a` `#5e5e26` | tapis olive |

## Où est le code

- `crates/afk-game/src/sprites.rs` : la palette, les sprites (dessinés pixel par pixel avec des rectangles sur une grille de 16 pixels), leur rotation exacte et leur affichage net.
- `crates/afk-game/src/habitat_view.rs` : la coupe d'une maison (sol, murs, toit, fenêtres, cheminée, paillasson, meubles) et sa lumière.
- `crates/afk-game/src/camera.rs` : les paliers de zoom entiers.

## Ce qui est déjà dans ce style

Les **habitats** (toutes les maisons, mines et cabanes). Le terrain, les arbres, les champs, les filons, les coffres, la forge, la taverne, les personnages et l'interface gardent leur apparence d'avant, jusqu'à ce qu'on les redessine à leur tour. Le **château** n'est pas touché : il aura son propre fonctionnement.

## Pour ajouter un dessin

1. Dessiner le sprite dans `sprites.rs` avec des couleurs de la palette, sur une grille multiple de 16.
2. Le ranger dans `all_pixel_sprites` pour que le test de palette le vérifie.
3. L'afficher par `Sprite::draw` (échelle entière) ; pour un objet orientable, passer par `Turns`.
4. Regarder le résultat à plusieurs zooms : les pixels doivent rester carrés et nets.
