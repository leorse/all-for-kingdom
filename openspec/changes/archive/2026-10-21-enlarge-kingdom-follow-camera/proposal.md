# Proposal

## Why

Le royaume tient aujourd'hui entièrement dans la fenêtre (32×20 tuiles, la vue s'ajuste à la carte) : tout le monde est toujours visible et rien n'est « loin ». On veut un royaume plus vaste, dont seule une partie est visible à la fois, pour que les distances comptent (trajets des paysans entre maisons et champs) et préparer le gameplay où le roi placera lui-même les bâtiments.

## What Changes

- La carte du village devient nettement plus grande ; maisons et champs sont éloignés du centre et en grande partie hors écran au départ.
- Le zoom est fixe (taille de tuile en pixels constante) au lieu de s'ajuster à la carte entière.
- Une caméra suit le personnage incarné : elle glisse rapidement vers lui quand on en change (Tab, 1-9, clic), puis ne défile que lorsqu'il approche du bord de l'écran.
- La caméra ne montre jamais au-delà des bords de la carte.
- Le clic pour incarner et les marqueurs tiennent compte du décalage de la caméra.
- Le placement des bâtiments reste défini dans le scénario (donnée pure) ; le placement en jeu par le roi est hors périmètre.

## Capabilities

### New Capabilities
- `follow-camera`: vue partielle de la carte à zoom fixe, centrage sur le personnage incarné et défilement quand il se déplace.
- `kingdom-layout`: dimensions et disposition du royaume de départ (distances entre maisons, champs et bâtiments publics).

### Modified Capabilities

## Impact

- `crates/afk-core/src/scenario.rs` : nouvelles dimensions, nouvelles coordonnées (bâtiments, routes, champs, arbres, lits, postes de travail, rondes des gardes, `EXTRA_BEDS`).
- `crates/afk-game/src/render.rs` : `View` (origine mobile, zoom fixe), dessin limité aux tuiles visibles, HUD inchangé (coordonnées écran).
- `crates/afk-game/src/main.rs` et `input.rs` : mise à jour de la caméra à chaque image, conversion écran→monde pour le clic.
- `crates/afk-core/examples/balance.rs` et les tests de `game.rs` qui dépendent du scénario : à vérifier (durées de trajet plus longues).
- `README.md` : décrire la caméra.
