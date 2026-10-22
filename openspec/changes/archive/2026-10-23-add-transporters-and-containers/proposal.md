# Proposal

## Why

Aujourd'hui, une ressource apparaît dans un compteur global dès que le travail est fini : aucun lieu de stockage, aucun transport, aucune limite. Les mineurs, le bûcheron, le paysan et le forgeron produisent sans fin et le forgeron puise dans un compteur magique. Le gameplay décidé (`docs/gameplay/transporteur.md`, `ressources-et-stocks.md`) est tout autre : chaque métier a un stock physique limité, un producteur s'arrête quand il est plein, et ce sont les transporteurs qui font circuler la matière. C'est le gros morceau qui transforme le royaume en vraie chaîne logistique.

## What Changes

- **Un contenant par lieu de travail** : le champ du paysan, chaque mine, chaque cabane de bûcheron, la forge, et un nouvel **entrepôt central**. Un contenant est un coffre posé sur la carte, **toujours à l'extérieur** du bâtiment qu'il dessert, avec un **compteur visible** par ressource et une **limite de 200 par ressource**.
- **Un producteur s'arrête quand son contenant est plein** : le mineur ne mine plus, le paysan ne récolte plus, le forgeron ne forge plus, tant que quelqu'un n'a pas libéré de la place. Le mineur, le paysan et le forgeron n'ont jamais à aller au coffre : ils y déposent à distance, et le forgeron y prélève son fer sans bouger ; seuls les porteurs marchent jusqu'à un coffre. Il reprend tout seul ensuite.
- **Trois transporteurs** (nouveau métier), qui vivent à l'entrepôt central et portent **10 ressources par voyage** d'un contenant à l'autre, de leur propre initiative :
  - le fer des mines va **à la forge** en priorité, car elle en consomme ;
  - tout ce que personne ne consomme encore (blé, bois, pierre, or, cuivre, outils) va **à l'entrepôt central** ;
  - un contenant plein ne reçoit plus rien.
- Les transporteurs **courent** : ils se déplacent plus vite que les autres, joueur comme IA.
- **Le bûcheron porte lui-même son bois** jusqu'au contenant de sa cabane (il ne peut en porter qu'un peu, et s'arrête si le contenant est plein) ; les bûches ne tombent plus directement dans le compteur.
- Le **fer de départ** est dans le contenant de la forge. Les **compteurs de ressources du HUD** deviennent le total de tous les contenants.
- Le joueur qui incarne un transporteur charge et décharge à la main (touche d'action) ; celui qui incarne un bûcheron décharge son bois de la même façon.
- **Le paysan aussi porte lui-même son blé** (20 au plus, soit cinq récoltes) jusqu'au coffre du champ, comme le bûcheron son bois.
- **Touche C** : vide l'entrepôt central (toutes ses ressources disparaissent), pour relancer les producteurs bloqués.
- Hors périmètre : priorités du roi, chariots, routes plus rapides, consommateurs de bois et de pierre (artisan), lingots, amélioration des entrepôts, bandits.

## Capabilities

### New Capabilities
- `containers`: le contenant de chaque lieu de travail, son compteur, la limite de 200 par ressource et l'arrêt des producteurs quand il est plein.
- `transport`: le métier de transporteur : voyages de 10 ressources, choix autonome des trajets, vitesse de course, entrepôt central, joueur qui charge et décharge.

### Modified Capabilities
- `lumberjack`: le bois est porté à la main jusqu'à la cabane au lieu de rejoindre le compteur global.
- `mining`: le minerai va dans le contenant de la mine et le mineur s'arrête quand il est plein.
- `mine-house`: le minerai rejoint le contenant de la mine (plus le compteur global) ; le forgeron consomme le fer de son propre contenant.
- `kingdom-layout`: le royaume de départ contient les contenants, l'entrepôt central et les trois transporteurs.

## Impact

- `crates/afk-core/src/map.rs` : tuile solide de coffre (`Tile::Crate`).
- Nouveau module `afk-core` des contenants (stock par ressource, capacité, qui produit/consomme quoi) ; `game.rs` (le compteur global devient un total calculé, dépôts en fin de travail, chargements et déchargements, réservations des transporteurs, porte-bois du bûcheron) ; `work.rs`, `character.rs` (`Role::Transporter`, cargaison portée, routines IA du transporteur et du bûcheron livreur, vitesse de course, arrêt des producteurs), `farm.rs` (la récolte attend de la place), `config.rs` (capacité 200, charge 10, vitesse, charge du bûcheron).
- `scenario.rs` : coffres contre les bâtiments, entrepôt central avec trois lits, trois transporteurs ; test de connectivité étendu.
- `crates/afk-game/src/render.rs` et `input.rs` : dessin des coffres et de leurs compteurs, la ressource portée visible sur les transporteurs et les bûcherons (bloc coloré et quantité), HUD du porteur incarné.
- De nombreux tests de `game.rs` et l'outil `balance` supposent le compteur global : à réécrire autour des contenants.
- `docs/gameplay/*.md` (transporteur, ressources-et-stocks, mineur, bûcheron, forgeron, paysan, habitat) et `README.md`.
