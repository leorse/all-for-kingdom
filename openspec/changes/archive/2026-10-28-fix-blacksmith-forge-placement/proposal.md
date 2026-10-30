# Proposal

## Why

En jeu, le forgeron ne se place pas bien à l'enclume. Quatre défauts se voient, relevés sur des captures :

1. **Il forge décalé.** Le joueur le met sur la case au nord de l'enclume sans le centrer, puis appuie sur E : il forge là où il est, à côté du milieu de l'enclume, sans pieds visibles. Le jeu vérifie seulement la **case** où il se trouve, pas sa position exacte.
2. **Il fait un sursaut en commençant à forger.** Ses images de forge sont dessinées 16 px plus bas que sa position. Debout, collé contre l'enclume, il n'est que 7 px sous le centre de la case. En passant à la forge, il saute donc d'environ 9 px vers le bas et semble « assis dans l'enclume ».
3. **Il clignote entre deux outils (IA).** Quand l'IA finit un outil, le forgeron passe une image hors du travail avant de reprendre le suivant. Pendant cette image, il est dessiné debout, puis de nouveau en forge. Comme sa position dessinée change entre les deux, on voit un aller-retour à chaque outil.
4. **Incarné, il s'arrête après chaque outil.** Le joueur appuie sur E et le forgeron forge un outil, puis il s'arrête : il faut réappuyer sur E pour le suivant.

## What Changes

- Le **poste de forge** est désormais un **point précis** au lieu d'une case : centré sur l'enclume à l'horizontale, et collé contre elle, aussi près que les collisions le permettent.
- Le forgeron, joueur ou IA, ne commence à forger **qu'à ce point**. S'il est ailleurs, y compris sur la bonne case mais décalé, l'appui sur la touche d'action le fait d'abord **marcher jusqu'au point**, sans rien changer d'autre au comportement actuel. L'IA y va de la même façon.
- Les **images de forge** sont redessinées dans le SVG **à la même hauteur que l'image debout**. Il n'y a donc plus de décalage vertical : au poste, debout ou en train de forger, son corps est au même endroit. Le marteau et le métal chaud restent sur le dessus de l'enclume. Les étincelles suivent le métal.
- Le forgeron **enchaîne les outils**, qu'il soit incarné ou non. La forge suivante commence **dans le même pas de simulation** que la fin de la précédente, tant qu'il reste du fer et de la place pour l'outil. Il n'y a plus d'image hors travail entre deux outils. Incarné, un appui sur E lance donc une **série** d'outils. Elle s'arrête quand il n'y a plus de fer ou de place, ou quand le joueur se déplace ou réappuie sur E.
- La documentation est mise à jour : `docs/art-direction.md` (la mention « dessiné 16 pixels plus bas ») et `docs/gameplay/forgeron.md`.

## Capabilities

### New Capabilities

Aucune.

### Modified Capabilities

- `blacksmith` :
  - le poste de forge devient un point centré sur l'enclume et collé contre elle, et l'on n'y forge qu'une fois arrivé exactement à ce point ;
  - le joueur décalé sur la bonne case est d'abord recentré ;
  - le passage entre debout et forge se fait sans sursaut ;
  - le forgeron, IA ou incarné, enchaîne les outils sans clignoter ; incarné, un appui lance la série au lieu d'un seul outil.

## Impact

- `crates/afk-core/src/character.rs` :
  - le point de poste (`forge_spot` donne une case, il faut aussi un point) ;
  - la fin du trajet d'approche jusqu'au point, et non plus au centre de la case ;
  - `forge_routine` (l'arrivée au point, et le démarrage seulement à ce point).
- `crates/afk-core/src/game.rs` :
  - `Interact` et `arrive_at_forge` testent la position exacte et non plus la case ;
  - l'enchaînement immédiat de la forge, IA ou joueur, dans `finish_work` ;
  - les tests.
- `crates/afk-game/assets/characters/forgeron.svg` et `forgeron.png` : les 6 images de la ligne `travail`, régénérées avec `sprites.bat`.
- `crates/afk-game/src/render.rs` : la position des étincelles.
- Documentation : `docs/art-direction.md` et `docs/gameplay/forgeron.md`.
