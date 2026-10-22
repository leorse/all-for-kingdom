# Proposal

## Why

Le bûcheron, les forêts et le bois sont décidés (`docs/gameplay/bucheron.md`, `royaume.md`) mais absents du jeu : les arbres ne sont aujourd'hui que du décor qui bloque le passage. Le bois est la première matière du futur chantier de l'artisan ; il faut une production de bois, et une ressource qui peut manquer si on coupe plus vite qu'elle ne repousse, à la différence des filons infinis.

## What Changes

- Des **forêts** : zones de la carte (définies par le scénario) denses en arbres, dont les arbres se coupent. Les arbres isolés hors forêt restent du décor.
- Un arbre coupé **donne 3 bûches** et laisse une **pousse** (praticable), qui redevient un arbre après un délai en heures de jeu. Une forêt peut donc s'épuiser si on coupe trop vite.
- Un nouveau métier, le **bûcheron** : il abat un arbre depuis une case voisine en le regardant (touche d'action pour le joueur, 2× plus vite que l'IA, fatigue comme les autres). L'IA va couper l'arbre libre le plus proche **de sa forêt**, le jour, et dort la nuit.
- **Travail à plusieurs** : l'IA vise l'arbre le plus proche, même déjà attaqué, et plusieurs bûcherons peuvent abattre le même arbre depuis des cases voisines différentes. Leurs coups s'additionnent et font tomber l'arbre plus vite. Seul celui qui porte le **dernier coup** obtient les 3 bûches ; les autres repartent vers un autre arbre.
- Chaque bûcheron a sa **cabane** (sa maison, avec un lit) à côté de sa forêt, et une forêt attitrée.
- Les bûches du bûcheron qui achève l'arbre vont **directement dans un compteur global de bois**, affiché au HUD (même simplification que les mineurs : pas d'entrepôt local, pas de capacité). Plus tard, avec les entrepôts, ce sera ce bûcheron qui portera lui-même son bois jusqu'à l'entrepôt ; le propriétaire des bûches est donc déjà identifié.
- Le scénario de départ gagne **deux forêts, deux cabanes et deux bûcherons**, un par forêt.
- Hors périmètre : entrepôt limité de la cabane, transporteur, consommation du bois (artisan), outils pour couper, replantation à la main, désignation d'une forêt par le roi, forêts posées par le roi.

## Capabilities

### New Capabilities
- `forests`: zones de forêt, arbres abattables, repousse d'un arbre coupé après un délai.
- `lumberjack`: le métier de bûcheron : abattre un arbre, 3 bûches par arbre, compteur de bois, cabane, joueur et IA.

### Modified Capabilities
- `kingdom-layout`: le royaume de départ contient aussi deux forêts, deux cabanes et deux bûcherons, tous accessibles à pied.

## Impact

- `crates/afk-core/src/map.rs` : nouvelle tuile praticable de pousse (`Tile::Sapling`).
- `crates/afk-core/src/resources.rs`, `config.rs` : ressource `Wood` ; réglages (durée d'abattage, bûches par arbre, délai de repousse).
- Nouveau module `afk-core` pour les forêts et la repousse ; `work.rs` (`WorkKind::Chop`), `character.rs` (`Role::Lumberjack`, routine IA, forêt attitrée), `game.rs` (travail disponible, abattage en fin de travail, repousse à chaque pas).
- `scenario.rs` : forêts, cabanes, bûcherons ; test de connectivité étendu.
- `crates/afk-game/src/render.rs` : dessin de la pousse, du bûcheron, compteur de bois au HUD.
- Tests de `game.rs` : le décompte des personnages et des lits change.
- `docs/gameplay/*.md` (bûcheron, royaume, ressources, README) et `README.md` : état « en place » et ce qui reste repoussé.
