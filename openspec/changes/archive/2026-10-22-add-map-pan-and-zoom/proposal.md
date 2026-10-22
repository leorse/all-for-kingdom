# Proposal

## Why

Le royaume (64×40 tuiles) dépasse l'écran et la caméra ne sait que suivre le personnage incarné : pour regarder une forêt, une mine ou un champ éloignés, il faut y envoyer quelqu'un ou changer de personnage. Le joueur veut parcourir la carte à la souris, et zoomer pour voir tout le royaume ou un détail, comme dans un jeu de gestion.

## What Changes

- **Déplacer la vue avec le bouton droit** : tant que le bouton droit est maintenu, la carte suit le mouvement de la souris (on la « saisit » et on la tire). La vue reste bornée aux bords de la carte.
- **Zoom à la molette** : molette vers l'avant pour zoomer, vers l'arrière pour dézoomer, par paliers de taille de tuile, autour du point sous le curseur. Le zoom est borné (du royaume entier à un gros plan).
- **Vue libre** : dès qu'on déplace la vue à la souris, la caméra cesse de suivre le personnage. Elle ne le suit de nouveau qu'à la prochaine action du joueur sur ce personnage : il bouge (ZQSD, flèches) ou il incarne quelqu'un (Tab, 1-9, clic, Espace), auquel cas la caméra glisse jusqu'à lui comme aujourd'hui.
- Le clic gauche pour incarner et tous les dessins tiennent compte du zoom.
- Le HUD rappelle les deux gestes.
- Hors périmètre : défilement aux bords de l'écran, déplacement au clavier de la vue, minimap, zoom animé, mémorisation du zoom entre deux parties, bouton du milieu.

## Capabilities

### New Capabilities
- `map-navigation`: déplacer la vue à la souris (bouton droit maintenu), zoomer à la molette, et la vue libre qui en découle.

### Modified Capabilities
- `follow-camera`: le zoom n'est plus fixe (il l'est par défaut), et le suivi du personnage est suspendu pendant la vue libre.

## Impact

- `crates/afk-game/src/camera.rs` : vue libre (déplacement, reprise du suivi), niveaux de zoom et zoom autour d'un point, avec tests unitaires (logique pure, sans macroquad).
- `crates/afk-game/src/render.rs` : `View` porte le zoom courant et applique déplacement et zoom.
- `crates/afk-game/src/input.rs` et `main.rs` : lecture du bouton droit, de la molette et de la position de la souris ; reprise du suivi quand le joueur bouge ou change de personnage.
- `crates/afk-game/src/render.rs` (HUD) : rappel des gestes.
- `README.md` : contrôles et caméra. Aucun changement dans `afk-core`.
