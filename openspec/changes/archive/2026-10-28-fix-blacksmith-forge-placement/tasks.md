# Tasks

## 1. Le poste de forge devient un point (afk-core)

- [x] 1.1 Ajouter `forge_post(anvil) -> Vec2` et `at_forge_post(pos, anvil)` dans `character.rs`, avec le point centré sur l'enclume et collé contre elle (2 px au-dessus de sa case ; le haut de la case de l'enclume, `ANVIL_BACK`, ne bloque plus). Tests :
  - le point est dans la case nord ;
  - il est centré en x ;
  - `is_box_blocked(post, BODY_HALF)` est faux ;
  - il est plus bas que le centre de la case.
- [x] 1.2 Ajouter le dernier pas vers le point (`step_to_point`), joué après la fin du chemin de cases, et l'utiliser dans le trajet d'approche du joueur. `Outcome::Arrived` ne part qu'une fois le point atteint, et le champ est vidé partout où l'approche est annulée (`cancel_approach`, `release_to_ai`, `lie_down`). Vérification : `cargo test -p afk-core`.
- [x] 1.3 `Interact` et `arrive_at_forge` testent `at_forge_post` au lieu de la case. Sur la bonne case mais décalé, l'appui lance une approche réduite au dernier segment. Tests :
  - décalé sur la case nord, un appui le recentre puis il forge, à `forge_post` ;
  - exactement au poste, un appui forge sans bouger ;
  - les tests qui posaient `forge_spot(ANVIL).center()` sont mis à jour.
- [x] 1.4 `forge_routine` : l'IA marche jusqu'à `forge_post` (chemin de cases, puis dernier segment) et ne forge qu'à ce point. Test : l'IA forge avec `pos` égal à `forge_post(ANVIL)`, à une marge près.
- [x] 1.5 Dans `Game::finish_work` (Forge), relancer aussitôt la forge du forgeron, incarné ou non, toujours au poste, s'il reste du fer et de la place. Tests :
  - sur une suite de pas couvrant deux outils, `work` n'est jamais `None`, pour l'IA comme pour le joueur ;
  - sans fer après le premier outil, il s'arrête ;
  - incarné, un seul appui sur E produit plusieurs outils ;
  - incarné, un déplacement ou un nouvel appui sur E arrête la série.

## 2. Les images de forge à la hauteur de l'image debout (dessin)

- [x] 2.1 Dans `crates/afk-game/assets/characters/forgeron.svg`, remonter de 14 px les 6 images `travail_*` (le poste est 14 px sous le centre de la case) et tracer le corps 14 px plus bas au lieu de 16, pour que le corps soit à la même hauteur que `debout_bas`. Le marteau, la pince et le métal chaud restent sur le dessus de l'enclume. Régénérer avec `sprites.bat`. Vérification :
  - `sprites.bat --check` réussit ;
  - dans l'aperçu `target/sprites-preview/forgeron.html`, tête et pieds sont aux mêmes lignes que `debout_bas`.
- [x] 2.2 Déplacer l'origine des étincelles dans `draw_forge_sparks` (`render.rs`) sur le nouveau métal. Vérification : `cargo test -p afk-game`, et sur une capture d'impact, les étincelles partent du métal.
- [x] 2.3 Vérifier en jeu, avec des captures, les plans de l'utilisateur :
  - (1→2) décalé sur la case nord, un appui sur E le recentre puis il forge centré ;
  - (3→4) collé à l'enclume, un appui sur E ne le fait plus sauter ;
  - l'IA, puis le joueur avec un seul appui, enchaînent plusieurs outils sans clignoter.

## 3. Documentation

- [x] 3.1 Mettre à jour `docs/art-direction.md`, la ligne « dessiné 16 pixels plus bas » : le poste est collé contre l'enclume et les images de forge sont à la hauteur de l'image debout. Mettre aussi à jour `docs/gameplay/forgeron.md` : poste centré et collé contre l'enclume, recentrage du joueur, un appui lance une série d'outils. Vérification : relecture, les deux documents décrivent le nouveau comportement.
