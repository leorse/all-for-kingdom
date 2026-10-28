# Le forgeron

Il transforme les matières premières en objets. Aujourd'hui il ne sait faire que des outils ; demain il aura toute une liste d'objets.

## Forger — en place

- Il travaille à une **enclume**, dans la forge. L'enclume est un obstacle, et il n'y a **qu'une place pour forger : juste au-dessus d'elle, face à elle**, comme un forgeron derrière son enclume.
- Chaque coup de forge dure 8 secondes (4 pour le joueur), consomme **2 de fer** et produit **1 outil**.
- Si le fer vient à manquer pendant un coup, le coup est perdu.

## Le joueur — en place

Il s'approche de l'enclume et la regarde : le jeu lui propose alors de forger. En appuyant sur la touche d'action, il est **placé automatiquement à son poste**, au-dessus de l'enclume, et commence à forger. Il forge 2 fois plus vite que l'IA. Bouger interrompt le travail.

## L'IA — en place

- Elle va seule à son poste, au-dessus d'une enclume libre, et forge tant qu'il reste du fer.
- Quand il n'y a plus de fer, elle traîne dans la forge en attendant.
- Plusieurs forgerons ne se disputent pas la même enclume : celle qui est prise est évitée.

## Le fer — en place, à changer

Le fer est aujourd'hui un **stock de départ fini** (60). Il est transformé en outils en moins d'une journée. Aucune source n'en produit d'autre : c'est le [mineur](mineur.md) qui va l'alimenter.

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
- Une forge peut-elle avoir plusieurs enclumes, et un atelier plus grand ? (Chaque enclume a besoin d'une case libre au-dessus d'elle.)

## L'apparence — en place

Le forgeron est le premier personnage dessiné avec de vrais sprites : un nain roux à grande barbe, tablier de cuir, bandeaux aux poignets. Il a une animation de marche dans les quatre directions, une animation de forge (marteau levé puis abattu sur la pièce chauffée, tenue aux tenailles) et il dort dans un lit redessiné, la tête sur l'oreiller.
