# Tasks

## 1. La géométrie de l'enclume et le poste (afk-core)

- [x] 1.1 Déplacer `forge_post` à `anvil.y + 12/32` et faire démarrer le blocage de l'enclume à `12/32 + BODY_HALF`, calculé plutôt qu'écrit en dur. Tests :
  - le poste est dans la case de l'enclume, centré en x ;
  - `is_box_blocked(post, BODY_HALF)` est faux, et un pas de 0,02 de plus vers le sud est bloqué ;
  - en poussant vers le sud depuis la case au nord, centré, un personnage s'arrête au poste (`at_forge_post`).
- [x] 1.2 Dans `find_path_cost`, faire partir un trajet commencé sur une case d'enclume depuis sa case au nord. Tests :
  - un trajet depuis le poste vers une destination au sud contourne l'enclume, et le personnage l'atteint sans jamais être bloqué ;
  - un trajet qui ne vise pas l'enclume ne passe jamais par sa case.
- [x] 1.3 Dans `drive`, `forge_routine` et `Interact`, accepter la case de l'enclume comme la case nord pour jouer seulement le dernier pas. Tests :
  - décalé dans la moitié libre, un appui le recentre puis il forge ;
  - centré et collé au socle, un appui forge sans qu'il bouge ;
  - l'IA forge au poste, et enchaîne toujours les outils sans pause ;
  - lâché par le joueur au poste, l'IA repart normalement.
  Vérification : `cargo +stable-x86_64-pc-windows-gnu test -p afk-core`, tests existants mis à jour vers le nouveau poste.

## 2. Le dessin (afk-game)

- [x] 2.1 Dans `draw_characters`, dessiner l'enclume 14 px plus bas et passer sa clé de tri à `anvil.y + 0.5`. Vérification : `cargo +stable-x86_64-pc-windows-gnu test -p afk-game`, et sur capture, le bas du socle est au bas de la case.
- [x] 2.2 Dans le script de génération, placer les jambes de forge à leur hauteur finale, sans passer par le double décalage, puis régénérer `forgeron.svg` et `forgeron.png` avec `sprites.bat`. Vérification : `sprites.bat --check` réussit, et les images `travail_*` ont leurs bottes aux mêmes lignes que `debout_bas`.
- [x] 2.3 Dans `draw_character`, afficher aussi l'ombre pendant le travail. Vérification : l'ombre se voit sur la capture de 3.1.

## 3. Vérification en jeu et documentation

- [x] 3.1 Vérifier en jeu, avec des captures, les cas 3, 4 et 6 de l'image de l'utilisateur :
  - à côté de l'enclume, les pieds plus bas que le haut du socle : il passe devant, et la corne ne le recouvre plus ;
  - derrière l'enclume : il passe derrière ;
  - au poste : il forge sans sursaut, entier dans son dessin. Ses bottes restent cachées par le pied de l'enclume, ce que l'utilisateur a validé ;
  - possédé, il entre dans la moitié libre et en ressort par le nord et les côtés.
- [x] 3.2 Mettre à jour `docs/art-direction.md` (enclume sur la moitié basse de sa case, forgeron entier) et `docs/gameplay/forgeron.md` (position du poste, moitié libre praticable). Vérification : relecture, les deux documents décrivent le nouveau comportement.
