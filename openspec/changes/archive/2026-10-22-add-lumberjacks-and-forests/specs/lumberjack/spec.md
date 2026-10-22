## Purpose
Définit le métier de bûcheron : abattre les arbres de sa forêt, produire du bois, et la cabane qui lui sert de maison.

## ADDED Requirements

### Requirement: Le bûcheron est un métier
Le jeu SHALL comporter le métier de bûcheron, incarnable comme les autres personnages. Seul un bûcheron SHALL pouvoir abattre un arbre.

#### Scenario: Un autre métier ne coupe pas
- **WHEN** un paysan incarné regarde un arbre de forêt depuis une case voisine et appuie sur la touche d'action
- **THEN** aucun travail ne démarre

### Requirement: Abattre un arbre
Un bûcheron SHALL abattre un arbre de sa forêt depuis une case praticable voisine, en le regardant. Le travail dure un temps fixé par la configuration, avec barre de progression ; le joueur le lance avec la touche d'action, et un mouvement ou une seconde pression l'annule sans rien produire.

#### Scenario: Le joueur abat un arbre
- **WHEN** le bûcheron incarné se place contre un arbre de sa forêt en le regardant et appuie sur la touche d'action
- **THEN** une barre de progression s'affiche et, à la fin, l'arbre est abattu et du bois est produit

#### Scenario: Annuler
- **WHEN** le joueur bouge pendant qu'il coupe
- **THEN** le travail est annulé, l'arbre reste debout et aucun bois n'est produit

### Requirement: Un arbre donne trois bûches
Chaque arbre abattu SHALL produire 3 bûches, ajoutées au compteur global de bois (voir l'attribution au bûcheron qui achève l'arbre), sans stock local ni limite de capacité. Le compteur SHALL être affiché au HUD.

#### Scenario: Compteur de bois
- **WHEN** un bûcheron termine un arbre alors que le compteur de bois vaut 0
- **THEN** le compteur de bois vaut 3

### Requirement: Plusieurs bûcherons sur le même arbre
Plusieurs bûcherons SHALL pouvoir abattre le même arbre en même temps, chacun depuis une case voisine différente. Leurs coups s'additionnent : l'arbre tombe quand le total atteint la durée d'abattage, donc d'autant plus vite qu'ils sont nombreux. La progression de l'arbre SHALL être la même pour tous et SHALL se conserver si un bûcheron arrête avant la chute.

#### Scenario: À deux, l'arbre tombe plus vite
- **WHEN** deux bûcherons en pleine forme abattent ensemble le même arbre
- **THEN** il tombe en deux fois moins de temps qu'avec un seul

#### Scenario: Deux cases différentes
- **WHEN** deux bûcherons IA s'attaquent au même arbre
- **THEN** ils se tiennent sur deux cases voisines différentes

#### Scenario: Progression conservée
- **WHEN** un bûcheron arrête à mi-chemin et qu'un autre vient finir le même arbre
- **THEN** l'arbre tombe au bout du temps restant seulement

### Requirement: Le bois revient à celui qui abat le dernier coup
Quand un arbre tombe, ses 3 bûches SHALL appartenir à un seul bûcheron : celui dont le coup achève l'arbre. Les autres n'obtiennent rien et SHALL passer aussitôt à un autre arbre. Tant qu'il n'existe pas d'entrepôt, les bûches de ce bûcheron rejoignent immédiatement le compteur global ; plus tard, c'est lui qui devra les porter à l'entrepôt.

#### Scenario: Un seul propriétaire
- **WHEN** deux bûcherons abattent ensemble un arbre qui donne 3 bûches
- **THEN** le compteur de bois n'augmente que de 3 et un seul des deux est crédité

#### Scenario: Les autres repartent
- **WHEN** l'arbre tombe sous les coups de plusieurs bûcherons
- **THEN** ceux qui n'ont pas porté le dernier coup interrompent leur travail et visent un autre arbre

### Requirement: Vitesse du joueur et fatigue
Le joueur SHALL couper deux fois plus vite que l'IA. La fatigue SHALL ralentir un bûcheron comme les autres personnages.

#### Scenario: Joueur et IA
- **WHEN** un bûcheron incarné et un bûcheron IA, en pleine forme, abattent chacun un arbre
- **THEN** le joueur met la moitié du temps de l'IA

### Requirement: Le bûcheron IA travaille le jour dans sa forêt
Un bûcheron IA SHALL, dans son horaire de travail, aller à la case libre la plus proche d'un arbre abattable de sa forêt attitrée (même si un autre bûcheron y travaille déjà, tant qu'une case voisine est libre), le regarder et l'abattre, puis passer au suivant. S'il n'y a plus d'arbre abattable, il SHALL attendre dans les environs de sa forêt jusqu'à la repousse. La nuit il SHALL rentrer dormir dans sa cabane.

#### Scenario: Journée sans joueur
- **WHEN** la simulation tourne un jour entier sans personnage incarné
- **THEN** chaque bûcheron abat des arbres de sa forêt le jour et dort dans sa cabane la nuit

#### Scenario: Forêt vide
- **WHEN** tous les arbres de sa forêt sont abattus
- **THEN** le bûcheron IA attend sans erreur, puis reprend dès qu'un arbre a repoussé

### Requirement: La cabane est la maison du bûcheron
Chaque bûcheron SHALL avoir une cabane, bâtiment avec un lit et une porte, proche de sa forêt. Il y dort et y rentre chaque soir.

#### Scenario: Nuit du bûcheron
- **WHEN** la nuit tombe
- **THEN** le bûcheron IA rejoint le lit de sa cabane et s'y couche
