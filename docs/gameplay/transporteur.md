# Le transporteur

Il **porte les ressources d'un endroit à un autre**, de lui-même. Ce métier est **en place** (version de départ : sans priorités du roi ni chariots).

## En place aujourd'hui

- **Trois transporteurs** vivent dans l'**entrepôt central**, un bâtiment près du château avec trois lits et son propre coffre dehors, à côté de la porte.
- Chaque lieu de travail a un **coffre** posé **à l'extérieur** : le champ du paysan, chaque mine, chaque cabane de bûcheron, la forge, et l'entrepôt central. Un compteur sur le coffre montre « Fer 12/200 ». **Chaque coffre contient au plus 200 de chaque ressource.**
- Un transporteur porte **10 ressources** par voyage. Il ne part que pour une charge complète, ou pour remplir exactement la place qui reste à destination.
- Il décide seul : le **fer** des mines va à la **forge** d'abord (elle le consomme), tout le reste va à l'**entrepôt central**. L'entrepôt central rend du fer à la forge si elle en manque. Un coffre plein ne reçoit plus rien, en tenant compte de ce que les autres transporteurs apportent déjà.
- Les transporteurs **courent** : 1,5 fois la vitesse des autres, joueur compris.
- Leur cargaison se voit sur eux (un bloc de la couleur de la ressource et la quantité) ; ils la gardent pour la nuit et la livrent à l'aube.
- **Incarné**, on charge (jusqu'à 10 de ce que le coffre donne et dont il y a le plus) et on décharge avec la touche d'action, devant un coffre.

## Le principe — décidé

Les ressources ne se déplacent pas toutes seules. Ce sont les transporteurs qui les envoient :

- du **mineur** vers le **forgeron** (minerai) ;
- du **bûcheron** et du **mineur de pierre** vers le **chantier** de l'[artisan](artisan.md) (bois, pierre) ;
- et plus généralement d'un stock à un autre.

### Ils sont autonomes — décidé

Personne ne leur dit quoi porter. **Les transporteurs décident eux-mêmes où il faut envoyer les ressources**, en fonction des besoins qu'ils voient : un entrepôt plein, un atelier qui manque de matière, un chantier en attente.

### Quand deux métiers veulent la même matière — décidé dans le principe

Il arrive que deux métiers aient besoin du même matériau (le fer pour les lingots et pour les armes, le bois pour deux chantiers…). C'est le **roi qui définit les priorités**. Le transporteur les respecte pour arbitrer. Comment le roi les exprime n'est pas encore défini : voir [roi.md](roi.md).

## Ce que ça change dans le jeu

Avant les transporteurs, une ressource apparaissait dans un compteur global dès que le travail était fini (voir [ressources-et-stocks.md](ressources-et-stocks.md)). Avec eux :

- chaque **stock** est un **lieu précis** (la mine du mineur, la forge, un chantier…) ;
- une ressource **voyage** à pied sur les routes, ce qui prend du temps ;
- sans transporteur, un entrepôt plein reste plein : le producteur s'arrête et le forgeron manque de matière même si les mines débordent.

Cela donne une vraie **logistique** : les routes, les distances et le nombre de transporteurs comptent. Comme le transporteur est autonome, le joueur n'a pas à micro-gérer chaque livraison : il règle les **priorités** et le **nombre** de transporteurs.

## À définir

- **Comment le roi exprime ses priorités** : une liste ordonnée, une pondération, des quantités à atteindre ?
- Combien un transporteur **porte** à chaque voyage ? A-t-il une charrette plus tard ?
- Les **routes** accélèrent-elles le transport ? (Le lien avec les routes qui se tracent seules : voir [royaume.md](royaume.md).)
- Le **compteur global** affiché à l'écran disparaît-il, ou reste-t-il comme total des stocks du royaume ?
- Où habitent les transporteurs, et ont-ils un lieu d'attache (un entrepôt central) ?
- Les transporteurs sont-ils vulnérables aux **bandits** (voir `IDEAS.md`) ?
- Si on incarne un transporteur, que décide-t-on à sa place ?
