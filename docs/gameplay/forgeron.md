# Le forgeron

Il transforme les matières premières en objets. Aujourd'hui il ne sait faire que des outils ; demain il aura toute une liste d'objets.

## Forger — en place

- Il travaille à une **enclume**, dans la forge. Elle n'occupe que la moitié basse de sa case : son socle bloque, la moitié haute se traverse à pied. On forge **uniquement depuis son poste**, dans cette moitié libre, en regardant vers le bas. Le poste est un point précis, centré sur l'enclume, les pieds contre le socle. L'IA, elle, ne passe pas par cette case pour circuler. On le voit ainsi de face lever et abattre le marteau, à la même hauteur qu'il soit debout ou en train de forger.
- Chaque coup de forge dure 8 secondes (4 pour le joueur), consomme **2 de fer** et produit **1 outil**.
- Si le fer vient à manquer pendant un coup, le coup est perdu.

## Le joueur — en place

Il se place à côté de l'enclume en la regardant, de n'importe quel côté, puis appuie sur la touche d'action (un appui suffit). S'il n'est pas exactement au poste de forge, le forgeron y marche seul, se tourne vers l'enclume et forge. Cela vaut aussi s'il est déjà sur la case au nord ou dans la moitié libre, mais décalé : il se recentre d'abord. Poussé vers le sud et centré, il s'arrête pile au poste. Un déplacement ou un nouvel appui pendant le trajet l'annulent.

Un appui lance une **série** : il enchaîne les outils sans s'arrêter, tant qu'il reste du fer et de la place. Un déplacement ou un nouvel appui arrêtent la série. Il forge 2 fois plus vite que l'IA.

## L'IA — en place

- Elle va se placer au poste de forge d'une enclume libre (au nord, contre son socle) et forge tant qu'il reste du fer, en enchaînant les outils sans pause.
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
