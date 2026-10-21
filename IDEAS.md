# Idées

Vrac d'idées pour plus tard. Rien ici n'est décidé ni planifié : quand une idée mûrit, elle devient un changement OpenSpec.

## Bandits
- Ils circulent dans le royaume, peuvent voler et attaquer.
- Seuls les gardes peuvent se défendre.
- À définir : ce qu'ils volent, d'où ils viennent, ce que ça coûte au royaume.

## Le roi trace les plans
- Le roi place lui-même les bâtiments : champs, maisons, etc.
- Remplace le placement codé en dur dans `scenario.rs` (voir `kingdom-layout`, qui précise que le scénario est une donnée pure).
- Fait partie du gameplay du rôle de roi.

## Nouveaux arrivants
- De temps en temps, des gens arrivent dans le royaume. Ils sont là mais ne servent à rien tant qu'ils n'ont pas de rôle.
- Le roi les affecte à un rôle (paysan, forgeron, etc.).

## Mineurs
- Ils travaillent dans la mine et ramènent du minerai au forgeron (aujourd'hui le fer est un stock de départ).
- Il faut donc pouvoir placer des mines sur la carte.

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
