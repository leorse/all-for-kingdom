# Idées

Vrac d'idées pour plus tard. Rien ici n'est décidé ni planifié : quand une idée mûrit, elle est décrite dans `docs/gameplay/` puis devient un changement OpenSpec.

## Bandits
- Ils circulent dans le royaume, peuvent voler et attaquer.
- Seuls les gardes peuvent se défendre.
- Un voleur qui s'enfuit prend le plus court chemin vers la sortie du château, sans suivre les routes ; les gardes le poursuivent de même (voir `docs/gameplay/garde.md`).
- À définir : ce qu'ils volent, d'où ils viennent, ce que ça coûte au royaume.

## Le roi trace les plans
- Le roi place lui-même les bâtiments : champs, maisons, etc. Un artisan vient les construire, avec du bois et de la pierre.
- Remplace le placement codé en dur dans `scenario.rs` (voir `kingdom-layout`, qui précise que le scénario est une donnée pure).
- Fait partie du gameplay du rôle de roi.
- À terme la carte est vide : le roi place des zones et les routes se tracent seules (inspiration *The Settlers*).
- Détails dans `docs/gameplay/artisan.md`, `docs/gameplay/roi.md` et `docs/gameplay/royaume.md`.

## Bûcheron, transporteurs
- Le bûcheron coupe des arbres et produit du bois pour la construction.
- Les transporteurs portent les ressources d'un endroit à un autre : les stocks deviennent des lieux.
- Détails dans `docs/gameplay/bucheron.md` et `docs/gameplay/transporteur.md`.

## Nouveaux arrivants
- De temps en temps, des vagabonds arrivent dans le royaume et sont accueillis à la taverne. Ils ne servent à rien tant qu'ils n'ont pas de rôle.
- Le roi les affecte à un rôle (paysan, forgeron, etc.). Taverne pleine : plus de vagabonds.
- Les arrivants ont des compétences : bons paysans, bons forgerons, chevaliers… Une mauvaise affectation est contre-productive.
- Améliorer la taverne donne plus de lits.
- Détails dans `docs/gameplay/tavernier.md` et `docs/gameplay/competences.md`.

## Mineurs
Décidé dans les grandes lignes : voir `docs/gameplay/mineur.md`. Reste à trancher le nombre de niveaux de qualité, l'usage du cuivre, la capacité des entrepôts.

## Forgeron : liste d'objets et priorités du roi
- Le forgeron a une liste d'objets qu'il sait faire : lingots, outils, armes, pièces d'or, et d'autres plus tard.
- Le roi lui donne des priorités ; il fabrique selon ces priorités et selon son stock.
- Détails dans `docs/gameplay/forgeron.md`.

## Gestion des stocks
- Chaque métier a un stock limité (le mineur dans sa mine, le paysan, le forgeron...). Stock plein : il arrête de produire.
- Implique d'équilibrer le nombre de travailleurs entre les métiers.
- Détails dans `docs/gameplay/ressources-et-stocks.md`.

## Amélioration des maisons
- Une maison s'améliore : confort, sommeil plus rapide, travailleur plus rapide, entrepôt plus grand.
- Probablement lié à l'idée « Mobilier » ci-dessous.
- Détails dans `docs/gameplay/habitat.md`.

## Région et royaume ennemi
- Le royaume est dans une région du monde, avec un ennemi qui a les mêmes besoins et les mêmes villageois.
- Il peut envoyer des vagues d'ennemis qui attaquent le royaume.
- Implique une carte plus grande que le royaume et une notion de région.

## Économie : pièces d'or et taxes
- Une monnaie (pièce d'or) et des taxes levées sur les habitants.
- Entretenir un guerrier ou un garde coûte de l'argent.
- À définir : qui perçoit (le roi ?), ce que ça finance d'autre, ce qui se passe quand les caisses sont vides.

## Mobilier et personnalisation de l'habitat
- Chaque personnage a une maison. En mode IA, il ne fait que son travail et dormir : il ne s'occupe pas de son logement.
- Le joueur ne peut changer le mobilier que lorsqu'il possède le personnage (mode possession).
- Le mobilier affecte les performances du personnage (par exemple le repos, donc la forme, et le rendement au travail).
- Chaque meuble coûte de l'argent (voir Économie) : il faut arbitrer entre confort et dépenses.
- À définir : d'où vient l'argent (le personnage gagne-t-il un salaire ?), la liste des meubles et leurs effets, et si on peut revendre un meuble.
