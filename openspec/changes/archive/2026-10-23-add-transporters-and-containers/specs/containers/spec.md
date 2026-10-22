## Purpose
Définit les contenants : le coffre de chaque lieu de travail, son compteur et sa limite, et l'arrêt de tout producteur dont le contenant est plein.

## ADDED Requirements

### Requirement: Un contenant par lieu de travail
Le royaume SHALL avoir un contenant à chacun de ces lieux : le champ du paysan, chaque mine, chaque cabane de bûcheron, la forge et l'entrepôt central. Un contenant SHALL être un coffre posé sur la carte, toujours à l'extérieur du bâtiment qu'il dessert et jamais à l'intérieur, infranchissable, auquel on accède depuis une case voisine.

#### Scenario: Les contenants du royaume de départ
- **WHEN** la partie démarre
- **THEN** la carte porte un coffre au champ du paysan, un contre chacune des trois mines, un contre chacune des deux cabanes, un contre la forge et un contre l'entrepôt central

#### Scenario: Un coffre est un obstacle
- **WHEN** un personnage tente de traverser un coffre
- **THEN** il est arrêté comme par un mur

#### Scenario: Les coffres sont dehors
- **WHEN** on regarde la carte au départ
- **THEN** aucun coffre ne se trouve à l'intérieur d'un bâtiment

### Requirement: Compteur visible sur chaque contenant
Chaque contenant SHALL afficher sur la carte la quantité qu'il contient pour chaque ressource, avec sa limite, sous la forme « Fer 12/200 ». Les ressources que le contenant produit ou consomme SHALL être affichées même à zéro.

#### Scenario: Lire un compteur
- **WHEN** une mine de fer contient 12 fer
- **THEN** son coffre affiche « Fer 12/200 »

#### Scenario: Contenant vide
- **WHEN** le coffre de la forge ne contient aucun outil
- **THEN** il affiche « Outils 0/200 »

### Requirement: Limite de 200 par ressource
Un contenant SHALL contenir au plus 200 unités de chaque ressource (réglage de la configuration). Aucun dépôt, de producteur ou de transporteur, ne SHALL dépasser cette limite.

#### Scenario: Contenant plein
- **WHEN** un contenant contient déjà 200 unités d'une ressource
- **THEN** plus aucune unité de cette ressource n'y est ajoutée

#### Scenario: Une autre ressource
- **WHEN** le coffre de la forge est plein de fer
- **THEN** il peut encore recevoir des outils

### Requirement: Le producteur s'arrête quand son contenant est plein
Un personnage qui produit une ressource SHALL cesser de la produire tant que son contenant ne peut pas la recevoir, et reprendre dès qu'il y a de la place. Un joueur incarné ne SHALL pas pouvoir lancer ce travail.

#### Scenario: Mineur bloqué
- **WHEN** le contenant d'une mine contient 200 minerai
- **THEN** son mineur ne mine plus, et reprend quand un transporteur en a emporté

#### Scenario: Paysan bloqué
- **WHEN** le contenant du champ est plein de blé et que le paysan porte déjà le blé de plusieurs récoltes
- **THEN** le paysan ne récolte plus (ses mains ne peuvent plus recevoir 4 blé de plus) ; les parcelles mûres restent mûres, et il continue de labourer et de semer ce qui peut l'être

#### Scenario: Forgeron bloqué
- **WHEN** le coffre de la forge contient 200 outils
- **THEN** le forgeron ne forge plus

#### Scenario: Reprise
- **WHEN** un transporteur emporte du contenu d'un contenant plein
- **THEN** le producteur reprend son travail sans autre intervention

### Requirement: Dépôt par le producteur
Le minerai extrait et les outils forgés SHALL être ajoutés au contenant du lieu de travail au moment où le travail se termine, sans que le producteur marche jusqu'au coffre, quelle que soit la distance qui l'en sépare. Le bois (capacité `lumberjack`) et le blé (voir plus bas) font exception : leurs producteurs les portent eux-mêmes.

#### Scenario: Minerai déposé
- **WHEN** un mineur termine une unité de fer
- **THEN** le contenant de sa mine contient une unité de fer de plus

#### Scenario: Le mineur ne quitte pas son poste
- **WHEN** un mineur termine une unité à son filon, à plusieurs tuiles du coffre
- **THEN** il reste à son poste et enchaîne le coup suivant, et le coffre a une unité de plus

### Requirement: Le forgeron travaille avec le contenant de la forge
Le forgeron SHALL prélever son fer dans le contenant de la forge et y déposer ses outils, sans jamais aller au coffre : le fer est toujours à sa portée, où qu'il se tienne. Il ne SHALL pas forger si ce contenant manque de fer ou ne peut pas recevoir d'outil. Le fer de départ du royaume SHALL se trouver dans ce contenant.

#### Scenario: Fer de départ
- **WHEN** la partie démarre
- **THEN** le coffre de la forge contient le fer de départ et aucun autre contenant ne contient de fer

#### Scenario: Le forgeron ne va pas au coffre
- **WHEN** le forgeron forge à l'enclume avec du fer dans le coffre de la forge
- **THEN** il forge sans quitter l'enclume et le fer du coffre diminue

#### Scenario: Plus de fer à la forge
- **WHEN** le coffre de la forge n'a plus assez de fer pour un outil
- **THEN** le forgeron attend, même si des mines débordent de fer

### Requirement: Compteurs du HUD
Les compteurs de ressources affichés à l'écran SHALL être le total de chaque ressource dans tous les contenants. Ce qu'un porteur tient en main n'y est pas compté.

#### Scenario: Total
- **WHEN** deux contenants contiennent 12 et 30 fer
- **THEN** le HUD affiche 42 fer

### Requirement: Le paysan porte lui-même son blé
Le blé récolté SHALL être pris en main par le paysan, jusqu\x27à 20 unités (réglage de la configuration), puis livré dans le contenant du champ, qu\x27il atteint à pied depuis une case voisine. Il ne SHALL pas récolter si ses mains ne peuvent pas recevoir les 4 blé de plus. L\x27IA livre quand ses mains ne peuvent plus recevoir une récolte, ou qu\x27il n\x27y a plus rien à cultiver, et à son réveil si elle s\x27est couchée avec du blé ; elle ne livre que si le contenant a de la place, et continue sinon de labourer et de semer. Le joueur dépose avec la touche d\x27action devant le coffre ce qui y rentre.

#### Scenario: Blé en main
- **WHEN** un paysan termine la récolte d\x27une parcelle alors qu\x27il ne portait rien
- **THEN** il porte 4 blé, le contenant du champ n\x27a pas changé, et un bloc jaune avec le chiffre 4 est dessiné sur lui

#### Scenario: Mains pleines
- **WHEN** le paysan porte 20 blé
- **THEN** il ne récolte plus et va livrer au contenant du champ

#### Scenario: Livraison
- **WHEN** le paysan arrive à côté du contenant du champ qui a de la place avec 20 blé
- **THEN** il dépose son blé et ses mains sont vides

#### Scenario: Coffre plein
- **WHEN** le contenant du champ est plein et que le paysan IA porte du blé
- **THEN** il garde son blé, ne va pas le livrer, et continue de labourer et de semer

#### Scenario: Nuit
- **WHEN** la nuit tombe alors que le paysan porte du blé
- **THEN** il va dormir en le gardant et le livre le lendemain avant de reprendre le travail

#### Scenario: Le joueur dépose
- **WHEN** le joueur, paysan portant 8 blé, appuie sur la touche d\x27action devant le contenant du champ qui n\x27a de place que pour 5
- **THEN** il dépose 5 blé et en garde 3

### Requirement: Vider l\x27entrepôt central
Le joueur SHALL pouvoir vider l\x27entrepôt central avec la touche C : toutes les ressources de son contenant disparaissent. Les autres contenants ne sont pas touchés. Ce qui est en main ou en route reste.

#### Scenario: Touche C
- **WHEN** l\x27entrepôt central contient 200 blé et 120 bois et que le joueur appuie sur C
- **THEN** son contenant est vide pour toutes les ressources et les autres contenants sont inchangés

#### Scenario: Les transporteurs reprennent
- **WHEN** l\x27entrepôt central était plein de blé, qu\x27on le vide et que le coffre du champ en contient 10 ou plus
- **THEN** un transporteur reprend le transport du blé vers l\x27entrepôt central

#### Scenario: Rappel
- **WHEN** la partie est à l\x27écran
- **THEN** la ligne d\x27aide du bas mentionne la touche C
