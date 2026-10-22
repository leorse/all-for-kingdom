# Design

## Context

La caméra (`camera.rs`) est de la logique pure en tuiles : `Camera::update(target, view, map, recenter, dt)` suit le personnage avec une marge, glisse à l'incarnation et borne le centre par `clamp_center`. `View` (`render.rs`) fait le lien monde ↔ écran avec un `tile_px` fixe (32) et l'applique à la conversion `to_screen`/`to_world`, que le rendu et le clic gauche utilisent déjà : tous les dessins se mettent à l'échelle avec `view.tile_px / 32`. `input::poll` produit des `Command` pour le cœur ; la caméra n'est pas une commande du jeu, c'est un état du frontend. Voir `openspec/specs/follow-camera` et `proposal.md`.

## Goals / Non-Goals

**Goals:**
- Tout reste dans `afk-game` ; aucun changement du cœur.
- La logique (déplacement, vue libre, paliers, zoom autour d'un point) est pure et testable dans `camera.rs`, sans macroquad.
- Un seul endroit de conversion monde ↔ écran (`View`), pour que clic et dessin restent cohérents à tout zoom.

**Non-Goals:**
- Défilement aux bords de l'écran, déplacement au clavier de la vue, minimap, zoom animé, mémorisation du zoom, bouton du milieu.

## Decisions

- **Déplacer = saisir la carte.** Pendant que le bouton droit est maintenu, le centre de la caméra bouge de `-(déplacement du curseur en pixels) / taille de tuile`, de sorte que le point saisi reste sous le curseur. Alternative : défilement continu dans la direction du curseur par rapport au centre de l'écran — rejetée, moins précise et fatigante pour regarder un endroit précis. Le déplacement est mesuré entre deux images ; à la première image du bouton enfoncé il est nul, pour éviter un saut.
- **Vue libre = un drapeau de la caméra.** `Camera::pan` met `free` à vrai, annule un éventuel glissement et déplace le centre (borné par `clamp_center`). Tant que `free`, `update` ne suit plus la cible et ne fait que re-borner le centre (la fenêtre ou le zoom peuvent avoir changé). Le drapeau tombe quand `update` reçoit `recenter` : le glissement existant s'enclenche alors sans autre code.
- **Qui déclenche `recenter` : `main.rs`.** Aujourd'hui `recenter` vaut « l'identifiant incarné vient de changer ». On y ajoute « la vue est libre et le joueur appuie sur une touche de déplacement », fourni par un nouveau champ de `Input`. Ça couvre Tab, chiffres, clic et Espace (changement d'identifiant) et ZQSD/flèches (nouveau) sans toucher au cœur. En mode AFK (rien d'incarné), seul Espace ou Tab reprend, ce qui est voulu.
- **Zoom par paliers entiers.** Les tailles de tuile sont `16, 20, 24, 32, 40, 48, 64` px ; le départ est 32 (valeur actuelle). Des entiers évitent les coutures entre rectangles que donneraient des tailles fractionnaires (le rendu dessine des rectangles à `s = tile_px / 32`). Alternative : zoom continu — rejetée pour cette raison. Le minimum 16 px fait tenir la carte 64×40 dans la fenêtre par défaut 1024×640 ; le maximum 64 donne un gros plan. Un cran de molette = un palier ; seul le signe de la molette compte (les unités varient selon la plate-forme).
- **Zoom autour du curseur.** Le point du monde sous le curseur avant le zoom est recalculé sous le même pixel après : `nouveau_centre = p − (curseur − centre_écran) / nouvelle_taille`, puis borné. Zoomer ne déclenche pas la vue libre : si la caméra suivait, la zone morte du suivi remet le personnage dans la zone centrale au prochain `update` ; si elle était libre, elle le reste. Alternative : zoomer autour du centre de l'écran — plus simple mais on perd le plaisir de viser un endroit.
- **`View` porte le zoom** (`tile_px` n'est plus constant) et expose `pan_pixels(delta)`, `zoom(cran, curseur)`, `is_free()` ; tout le reste du rendu lit déjà `view.tile_px`. La constante `TILE_PX` devient la taille de départ.
- **Entrées.** `Input` gagne `moving` (une direction est tenue), `right_down`, `mouse` et `wheel`. La molette et le bouton droit ne génèrent pas de `Command`. Le clic gauche pour incarner continue d'utiliser `view.to_world`.
- **HUD.** La ligne d'aide en bas ajoute « Clic droit maintenu: deplacer la vue | Molette: zoom ». Le texte étant ASCII (limite de la police), pas d'accents.

## Risks / Trade-offs

- [La ligne d'aide devient longue et déborde de la fenêtre étroite] → la passer sur deux lignes si elle dépasse la largeur de l'écran (`measure_text`).
- [La vue libre peut dérouter : le personnage disparaît et on ne sait plus où il est] → le moindre appui sur ZQSD ou un changement de personnage ramène la caméra par un glissement visible. Pas de touche dédiée pour l'instant.
- [Lumière nocturne dessinée sur toute la carte à chaque zoom] → déjà bornée à l'écran par `to_screen` ; vérifier à 16 px que rien ne dépasse.
- [Un cran de molette trop rapide enchaîne plusieurs paliers] → un palier par image au plus (le signe de la molette, pas son amplitude).

## Open Questions

- Une touche pour recentrer sur le personnage sans bouger serait pratique ; non demandée, laissée de côté.
