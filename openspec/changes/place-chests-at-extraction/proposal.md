# Proposal

## Why

Les coffres ont été posés à côté des bâtiments (maison du paysan, bâtiment de la mine, cabane), loin de l'endroit où la ressource est produite : le coffre du paysan est à vingt cases de son champ, ceux des mines contre le bâtiment et pas contre le filon. Le sens du jeu est inverse : le stock doit se trouver **au lieu d'extraction**, là où l'on produit. Cela raccourcit les trajets du paysan et du bûcheron, qui portent leur récolte à la main, et donne un sens visuel clair aux coffres.

## What Changes

- Le coffre du **paysan** est posé **à côté du champ**, contre ses parcelles.
- Les coffres des **mines** sont posés **au pied des filons**, là où les mineurs travaillent.
- Les coffres des **bûcherons** sont posés **en lisière de leur forêt**.
- Le coffre de la **forge** reste contre la forge ; celui de l'**entrepôt central** reste à côté de sa porte.
- Tous les coffres restent **dehors**, infranchissables, avec une case voisine praticable.
- Le scénario de départ déplace donc cinq coffres (champ, trois mines, deux forêts) ; les trajets des porteurs et des transporteurs s'en trouvent raccourcis.
- Hors périmètre : le roi qui place des zones et leurs coffres, toute autre règle de contenant.

## Capabilities

### New Capabilities

### Modified Capabilities
- `containers`: l'emplacement des coffres suit le lieu d'extraction.
- `kingdom-layout`: les positions des coffres du royaume de départ.

## Impact

- `crates/afk-core/src/scenario.rs` : coordonnées des coffres du paysan, des mines et des cabanes.
- Tests de `game.rs` qui citent ces coordonnées (chargement par le joueur, dépôt du paysan et du bûcheron).
- `docs/gameplay/*.md` et `README.md`. Aucun changement de comportement du cœur.
