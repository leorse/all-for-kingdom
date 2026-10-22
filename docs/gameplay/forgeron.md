# Le forgeron

Il transforme les matières premières en objets. Aujourd'hui il ne sait faire que des outils ; demain il aura toute une liste d'objets.

## Forger — en place

- Il travaille à une **enclume**, dans la forge. L'enclume est un obstacle : on forge depuis une case voisine, en la regardant.
- Chaque coup de forge dure 8 secondes (4 pour le joueur), consomme **2 de fer** et produit **1 outil**.
- Si le fer vient à manquer pendant un coup, le coup est perdu.

## Le joueur — en place

Il se place à côté de l'enclume en la regardant, puis appuie sur la touche d'action. Il forge 2 fois plus vite que l'IA.

## L'IA — en place

- Elle va se placer à côté d'une enclume libre et forge tant qu'il reste du fer.
- Quand il n'y a plus de fer, elle traîne dans la forge en attendant.
- Plusieurs forgerons ne se disputent pas la même enclume : celle qui est prise est évitée.

## Le fer — en place

Le fer vient d'un **stock de départ** (60) et du [mineur de fer](mineur.md), qui l'ajoute au même compteur. Le forgeron puise dans ce compteur. Pas encore de stock local ni de lingots.

## Décidé : le forgeron fabrique une liste d'objets

Le forgeron aura une **liste d'objets qu'il peut faire**. Pour l'instant, on a :

- **lingots** (à partir du fer, de l'or…) ;
- **outils** ;
- **armes** ;
- **pièces d'or**.

Cette liste s'agrandira. Chaque objet a sa recette : de quoi il est fait, combien de temps ça prend.

### Les priorités du roi

- Le **roi donne des priorités** au forgeron : par exemple « d'abord des armes, ensuite des outils ».
- Le forgeron **fabrique en fonction de son stock** : il ne peut faire que ce pour quoi il a la matière, et il suit l'ordre de priorité du roi.
- Quand le joueur incarne le forgeron, il choisit lui-même ce qu'il fabrique.

### Lingots

Le fer et l'or arrivent bruts de la mine. Le forgeron en fait des **lingots**, qui sont la matière de la plupart des autres objets. C'est une étape de plus dans la chaîne : minerai → lingot → objet.

### Un stock limité

Le forgeron a **une limite de stockage**, comme les autres métiers : une fois ses réserves pleines, il cesse de forger. Voir [ressources-et-stocks.md](ressources-et-stocks.md).

## À définir

- Les **recettes** : combien de minerai par lingot, combien de lingots par outil, arme, pièce ?
- Le **cuivre** : le forgeron en fait-il quelque chose ? Voir [mineur.md](mineur.md).
- Quelles **armes**, pour qui (les gardes, le chef de guerre) ?
- Comment le roi donne-t-il ses priorités : une liste ordonnée, des quantités à atteindre ?
- Une forge peut-elle avoir plusieurs enclumes, et un atelier plus grand ?
