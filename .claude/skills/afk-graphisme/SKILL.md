---
name: afk-graphisme
description: Méthode graphique d'All For Kingdom (AFK) et images de référence. Pointe vers la base OKF knowledge/graphisme (direction artistique, palette, tuile de 32 px, gabarit des personnages, inventaire du redessin) et donne la marche à suivre pour dessiner, animer, changer une taille, un thème ou ajouter une image. À charger pour toute tâche de sprite, d'animation, de palette, de zoom ou d'apparence.
---

# AFK - Graphisme

**La connaissance est dans la base OKF `knowledge/`**, pas ici : ce skill donne la méthode et les images.

## Lire d'abord

1. `knowledge/index.md` (section Le graphisme), puis :
2. `knowledge/graphisme/direction-artistique.md` : style, tuile de 32 px, zoom 32/64/96/128, palette, gabarit des personnages, animations, lumière, chaîne SVG vers PNG.
3. `knowledge/graphisme/inventaire.md` : ce qui est déjà redessiné et ce qui est encore en rectangles de couleur (coffres, arbres, champs...).
4. Le code prime sur les pages en cas de divergence : `crates/afk-game/src/sprites.rs` (palette), `camera.rs` (tuile, zoom), `character_sprites.rs` (gabarit).

## Images de référence (`samples/`, à regarder avec Read avant de dessiner)

| Fichier | Rôle |
|---|---|
| `samples/image_maison.jpg` | Référence du **décor** et de la **lumière** (intérieur de maison). Tout nouvel élément doit lui ressembler. |
| `samples/forgeron_sample.jpg` | Inspiration des **personnages** : les nôtres sont des hommes aux proportions d'adulte (pas des nains), à la marche mieux décomposée. |
| `samples/comparaison_32_vs_64.png` | Même forgeron sur tuile 32 px et 64 px à même taille d'écran : justifie le choix du 32 px. |
| `samples/comparaison_zoom_de_depart_64.png` | Même comparaison au zoom de départ (64 px par tuile). |
| `crates/afk-game/assets/characters/forgeron.png` (+ `.svg`) | Planche de référence, 6 colonnes x 9 lignes de cellules 64x64. **Le modèle à copier** pour un nouveau métier. |

Ces images sont des supports de travail : ne pas les citer dans le README ni dans le code.

## Méthode

- **Personnage** : copier `forgeron.svg`, adapter vêtements, couleurs et animation de travail ; déclarer la planche (`SheetSpec`, `SHEETS`) dans `character_sprites.rs` et la charger dans `character_view.rs` ; lancer `sprites.bat` ; relire l'aperçu `target/sprites-preview/<métier>.html` (visage, alternance des jambes, pieds sur l'ancre) ; `cargo test -p afk-game`.
- **Objet ou meuble d'habitat** : lettres de palette avec `Canvas` dans `sprites.rs`, rangé dans `all_pixel_sprites()`, affiché via `Sprite::draw` / `Turns` à échelle entière. Aperçu : `PREVIEW_OUT=maisons.bmp cargo test -p afk-game preview_houses -- --ignored`.
- **Élément en ancien style** (rectangles dans `render.rs`) : coordonnées en pixels d'une tuile de 32 (`s = tile_px / 32`) ; ne jamais mélanger les deux styles dans un même bâtiment ; le château n'est pas à redessiner.
- **Nouvelle couleur** : ajouter à `PALETTE` (lettre et couleur uniques), justifier, mettre à jour la page de palette de la base.
- **Vérifier à plusieurs zooms** : pixels carrés et nets, échelle entière.

## Maintenance (obligatoire)

Tout changement graphique met à jour `knowledge/graphisme/*` **dans le même changement** (état d'avancement dans `inventaire.md`, règles dans `direction-artistique.md`), ajoute une ligne à `knowledge/log.md` et vérifie les `sources` des pages touchées. Si une page diverge du code, le signaler et la corriger.
