# lumberjack Specification

## Purpose
Définit le métier de bûcheron : abattre les arbres de sa forêt, produire du bois, et la cabane qui lui sert de maison.

## Requirements

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
Chaque arbre abattu SHALL produire 3 bûches, remises au bûcheron qui l'a achevé (voir l'attribution plus bas). Les bûches ne SHALL pas rejoindre un compteur ou un contenant d'elles-mêmes : le bûcheron les porte jusqu'à son contenant.

#### Scenario: Compteur de bois
- **WHEN** un bûcheron achève un arbre alors qu'il ne portait rien
- **THEN** il porte 3 bûches et aucun contenant ni compteur de bois n'a changé

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
Quand un arbre tombe, ses 3 bûches SHALL appartenir à un seul bûcheron : celui dont le coup achève l'arbre, qui les prend en main. Les autres n'obtiennent rien et SHALL passer aussitôt à un autre arbre.

#### Scenario: Un seul propriétaire
- **WHEN** deux bûcherons abattent ensemble un arbre qui donne 3 bûches
- **THEN** un seul des deux porte 3 bûches et l'autre n'en porte pas plus qu'avant

#### Scenario: Les autres repartent
- **WHEN** l'arbre tombe sous les coups de plusieurs bûcherons
- **THEN** ceux qui n'ont pas porté le dernier coup interrompent leur travail et visent un autre arbre

### Requirement: Vitesse du joueur et fatigue
Le joueur SHALL couper deux fois plus vite que l'IA. La fatigue SHALL ralentir un bûcheron comme les autres personnages.

#### Scenario: Joueur et IA
- **WHEN** un bûcheron incarné et un bûcheron IA, en pleine forme, abattent chacun un arbre
- **THEN** le joueur met la moitié du temps de l'IA

### Requirement: Le bûcheron IA travaille le jour dans sa forêt
Un bûcheron IA SHALL, dans son horaire de travail, aller à la case libre la plus proche d'un arbre abattable de sa forêt attitrée (même si un autre bûcheron y travaille déjà, tant qu'une case voisine est libre), le regarder et l'abattre, puis passer au suivant, tant que ses mains peuvent recevoir les bûches d'un arbre de plus. Quand elles ne le peuvent plus, ou qu'il n'y a plus d'arbre abattable, il SHALL aller livrer son bois à son contenant. S'il n'y a plus d'arbre et plus rien à livrer, il SHALL attendre dans les environs de sa forêt jusqu'à la repousse. La nuit il SHALL rentrer dormir dans sa cabane.

#### Scenario: Journée sans joueur
- **WHEN** la simulation tourne un jour entier sans personnage incarné
- **THEN** chaque bûcheron abat des arbres de sa forêt, livre son bois à sa cabane le jour, et dort dans sa cabane la nuit

#### Scenario: Forêt vide
- **WHEN** tous les arbres de sa forêt sont abattus et qu'il ne porte rien
- **THEN** le bûcheron IA attend sans erreur, puis reprend dès qu'un arbre a repoussé

### Requirement: La cabane est la maison du bûcheron
Chaque bûcheron SHALL avoir une cabane, bâtiment avec un lit et une porte, proche de sa forêt. Il y dort et y rentre chaque soir.

#### Scenario: Nuit du bûcheron
- **WHEN** la nuit tombe
- **THEN** le bûcheron IA rejoint le lit de sa cabane et s'y couche

### Requirement: Le bûcheron porte lui-même son bois
Un bûcheron SHALL porter son bois en main, jusqu'à 9 bûches (réglage de la configuration), et le livrer en le déposant dans le contenant de sa cabane. Il dépose depuis une case voisine du contenant, instantanément. Il ne SHALL pas lancer l'abattage d'un arbre si ses mains ne peuvent pas recevoir les 3 bûches de plus.

#### Scenario: Trois arbres de suite
- **WHEN** un bûcheron IA a abattu trois arbres sans livrer
- **THEN** il porte 9 bûches, ne coupe plus et va à sa cabane

#### Scenario: Mains pleines
- **WHEN** le joueur, bûcheron, porte 8 bûches et appuie sur la touche d'action devant un arbre
- **THEN** aucun travail ne démarre

#### Scenario: Livraison
- **WHEN** un bûcheron portant 9 bûches arrive à côté du contenant de sa cabane qui a de la place
- **THEN** il dépose ses bûches, qui s'ajoutent au contenant, et ses mains sont vides

### Requirement: Contenant plein, le bûcheron attend
Si le contenant de la cabane ne peut pas recevoir toutes les bûches, le bûcheron SHALL en déposer ce qui rentre et garder le reste, puis attendre à côté du contenant qu'une place se libère, sans abattre d'autre arbre. Le joueur bûcheron SHALL pouvoir déposer ce qui rentre avec la touche d'action.

#### Scenario: Cabane pleine
- **WHEN** le contenant de la cabane contient 200 bûches et qu'un bûcheron IA arrive avec 9 bûches
- **THEN** il ne dépose rien et attend à côté du contenant avec ses 9 bûches

#### Scenario: Place libérée
- **WHEN** un transporteur emporte 10 bûches de ce contenant
- **THEN** le bûcheron dépose ses bûches et reprend son travail

#### Scenario: Le joueur dépose
- **WHEN** le joueur, bûcheron portant 5 bûches, appuie sur la touche d'action devant le contenant de sa cabane qui n'a de la place que pour 2
- **THEN** il dépose 2 bûches et en garde 3

### Requirement: La cargaison du bûcheron reste en main la nuit
Un bûcheron qui va dormir en portant du bois SHALL le garder et le livrer à son réveil, avant de reprendre l'abattage.

#### Scenario: La nuit tombe en forêt
- **WHEN** la nuit tombe alors qu'un bûcheron porte 6 bûches
- **THEN** il va dormir en les gardant, et les livre le lendemain

### Requirement: On voit le bois que porte le bûcheron
Le bois porté par un bûcheron SHALL être visible sur lui : un bloc de couleur bois dans ses bras, avec le nombre de bûches en chiffre au-dessus de sa tête. Rien n'est dessiné quand il ne porte rien.

#### Scenario: Bois en main
- **WHEN** un bûcheron porte 6 bûches
- **THEN** un bloc brun est dessiné dans ses bras et le chiffre 6 s'affiche au-dessus de sa tête

#### Scenario: Après la livraison
- **WHEN** le bûcheron a déposé tout son bois
- **THEN** le bloc et le chiffre disparaissent
