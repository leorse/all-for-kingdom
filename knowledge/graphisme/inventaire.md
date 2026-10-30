---
type: Inventaire
title: Inventaire graphique
description: Ce qui est déjà dans le nouveau style pixel art et ce qui est encore dessiné en rectangles de couleur, avec les couleurs actuelles.
tags: [graphisme, inventaire, redessin]
sources:
  - resource: crates/afk-game/src/render.rs
    title: dessin de l'ancien style
  - resource: crates/afk-game/src/habitat_view.rs
    title: habitats
  - resource: crates/afk-game/src/character_view.rs
    title: planches de personnages
---

# Inventaire graphique d'AFK

État relevé dans le code (`crates/afk-game/src/render.rs`, `sprites.rs`, `habitat_view.rs`, `character_view.rs`). À revérifier avant de s'y fier : le code prime. Toutes les coordonnées « ancien style » sont en pixels d'une tuile de 32 (`s = tile_px / 32`).

## Éléments déjà dans le nouveau style

| Élément | Où | Remarques |
|---|---|---|
| Habitats : maisons, mines, cabanes | `sprites.rs` (dessins 16 px), `habitat_view.rs` (assemblage) | Coupe : toit de chaume + mur de rondins au nord, murs de bois ouest/est, mur sud bas avec porte, sol en planches. Encore sur 16 px, affichés x2. Fenêtres éclairées la nuit, cheminée animée, paillasson. Meubles : lit, table, étagère, baquet, avec 4 rotations (`Rotation`, `Furniture` dans `afk-core/src/habitat.rs`). |
| Forgeron | `assets/characters/forgeron.svg/.png`, `character_sprites.rs` | Planche complète, voir SKILL.md. |
| Enclume | extra `enclume` de la planche forgeron | Affichée via `draw_extra`. |
| Étincelles de forge | `draw_forge_sparks` (`render.rs`) | Gerbe de `SPARKS_SECS = 0.35` s à partir de l'image d'impact. Procédural, pas dans la planche. |

## Éléments encore en ancien style (rectangles de couleur)

| Élément | Où | Aspect actuel |
|---|---|---|
| Terrain | `tile_color` | Herbe `(88+v,140+v,70)`, chemin `(176+v,150+v,108)`, eau `(52,96+v,168)`, champ `(130+v,96,58)`, mur `(96,90,100)`, sol/lit/enclume `(150+v,120+v,90)`, arbre `(40,98+v,52)`. `v` = petite variation par tuile. |
| Murs | `draw_map` | Rectangle `(96,90,100)` + liseré haut de 4 px `(120,114,124)`. |
| Arbres | `draw_map` | Carré 24x24 `(30,78,40)` centré dans la tuile. |
| Pousses (Sapling) | `draw_map` | Tige 3x12 verte + deux feuilles 6x5. |
| Coffres (Crate) | `draw_map` | Caisse 24x17 `(120,84,48)`, couvercle 4 px `(160,116,70)`, planche, sangle grise, ombre portée. Étiquette `ressource n/capacité` au-dessus (fond noir 55 %, texte blanc, rouge `(255,110,100)` si plein). **Candidat n°1 au redessin** (le placement des coffres vient d'être amélioré : dernier commit). |
| Lits (hors habitat) | `draw_map` | Cadre `(104,68,40)`, couverture `(196,74,74)`, oreiller `(236,234,222)`. |
| Filons | `draw_veins`, `ore_color` | Pastilles par minerai : fer `(132,138,150)`, cuivre `(188,108,56)`, or `(228,188,52)`, pierre `(196,190,170)`. |
| Champs | `draw_farm` | Terre `(92,62,38)`, sillons `(68,44,26)` ; plant vert `(80,175,70)` qui grandit, blé mûr `(214,178,62)` / `(240,206,90)`. Parcelle visée entourée de blanc (`draw_target`). |
| Personnages (hors forgeron) | `draw_characters` | Silhouette en rectangles : ombre, corps teinté par métier, tête `(240,200,160)`, casque garde `(160,166,184)`. Marqueur jaune `(255,230,40)` sur le personnage incarné, contour sombre, barre de progression au-dessus, bulle de texte. |
| Couleurs par métier | `role_color` | Paysan `(214,186,76)`, forgeron `(220,110,50)`, roi `(150,60,190)`, chef de guerre `(200,50,50)`, tavernier `(70,170,190)`, garde `(112,126,170)`, mineur `(150,104,66)`, bûcheron `(70,150,90)`, transporteur `(60,190,190)`. |
| Couleurs de ressources | `resource_color` | Blé `(232,200,70)`, bois `(140,92,50)`, fer `(132,138,150)`, cuivre `(188,108,56)`, or `(228,188,52)`, pierre `(196,190,170)`, outils `(110,150,200)`. |
| Jauge de forme | `fitness_color`, `fitness_bar` | Vert `(90,200,90)` / orange `(235,165,50)` / rouge `(215,60,60)` selon le niveau. |
| HUD | `draw_hud` | Texte et barres sur fonds sombres `(20,20,20)`, icône jour/nuit (soleil `(255,215,70)`, lune `(190,200,255)`), liste des personnages avec bordure blanche pour l'incarné. Police par défaut de macroquad. |
| Fond hors carte | `draw` | `(20,24,30)`. |

## Non touché volontairement

Le **château** : il aura son propre fonctionnement. Ne pas le redessiner sans demande explicite.

## Métiers existants (`Role`) et planches

Paysan, forgeron, roi, chef de guerre, tavernier, garde, mineur, bûcheron, transporteur. **Seule la planche forgeron existe** (`SHEETS: [SheetSpec; 1]`). Tout autre métier est dessiné en rectangles.

## Idées déjà évoquées touchant au visuel ([idées](/idees.md) et [habitat](/regles/habitat.md))

- Mobilier personnalisable dans les maisons (le joueur change les meubles en mode possession) : demandera un catalogue de meubles dans le style des habitats.
- Taverne améliorable (plus de lits), maisons améliorables (confort) : variantes visuelles par niveau.
- Le roi placera lui-même les bâtiments : il faudra des sprites de « plan » / chantier.
- Nouveaux arrivants (vagabonds) sans métier : un personnage sans rôle.
- Bandits, gardes, ennemis, pièces d'or : sprites et effets à venir.
Rien de tout cela n'est décidé : ne pas le dessiner sans demande.
