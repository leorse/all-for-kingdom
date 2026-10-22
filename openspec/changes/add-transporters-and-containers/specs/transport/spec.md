## Purpose
Définit le transporteur : le métier qui fait circuler les ressources d'un contenant à l'autre, de lui-même, à pied et en courant, ainsi que l'entrepôt central où il habite.

## ADDED Requirements

### Requirement: Le transporteur est un métier
Le jeu SHALL comporter le métier de transporteur, incarnable comme les autres personnages. Le royaume de départ SHALL en compter trois.

#### Scenario: Trois transporteurs
- **WHEN** la partie démarre
- **THEN** trois personnages sont transporteurs et se trouvent à l'entrepôt central

### Requirement: Un voyage porte dix ressources
Un transporteur SHALL porter au plus 10 unités d'une seule ressource par voyage. Il prend la marchandise dans un contenant et la dépose dans un autre. Il ne part que si une charge de 10 est disponible à la source et peut être reçue à destination, ou si la charge, plus petite, remplit exactement la place restante de la destination.

#### Scenario: Charge complète
- **WHEN** une mine de fer contient 25 fer et que le contenant de la forge a de la place
- **THEN** un transporteur emporte 10 fer et en laisse 15 dans la mine

#### Scenario: Source presque vide
- **WHEN** une mine de fer ne contient que 4 fer et que la forge a de la place pour beaucoup plus
- **THEN** aucun transporteur n'y va

#### Scenario: Compléter une destination
- **WHEN** la forge n'a de la place que pour 4 fer et qu'une mine en contient 30
- **THEN** un transporteur emporte 4 fer

### Requirement: Ils décident seuls où porter quoi
Un transporteur IA SHALL choisir lui-même son prochain voyage, sans consigne. Une ressource peut sortir d'un contenant qui la produit, ou de l'entrepôt central si un consommateur en manque. Elle SHALL aller d'abord à un contenant qui la consomme et a de la place (la forge pour le fer), sinon à l'entrepôt central. Entre plusieurs voyages possibles, il SHALL préférer les plus gros, puis les plus proches.

#### Scenario: Le fer va à la forge d'abord
- **WHEN** la forge a de la place pour du fer et qu'une mine en a en réserve
- **THEN** le fer de cette mine est porté à la forge, pas à l'entrepôt central

#### Scenario: Ce que personne ne consomme
- **WHEN** une mine d'or contient 10 or ou plus
- **THEN** un transporteur l'emporte à l'entrepôt central

#### Scenario: La forge est pleine de fer
- **WHEN** le contenant de la forge contient 200 fer
- **THEN** le fer des mines est porté à l'entrepôt central

#### Scenario: L'entrepôt central rend à la forge
- **WHEN** la forge manque de fer et que l'entrepôt central en contient
- **THEN** un transporteur porte ce fer à la forge

#### Scenario: Pas de va-et-vient inutile
- **WHEN** l'entrepôt central contient du bois
- **THEN** aucun transporteur ne le reporte à une cabane

### Requirement: Un contenant plein ne reçoit rien
Un transporteur ne SHALL apporter aucune unité d'une ressource à un contenant qui contient déjà 200 unités de cette ressource, y compris en tenant compte de ce que les autres transporteurs sont en train d'y apporter.

#### Scenario: Destination pleine
- **WHEN** tous les contenants qui peuvent recevoir une ressource sont pleins pour elle
- **THEN** les transporteurs ne la portent pas et attendent

#### Scenario: Trois transporteurs, une seule place
- **WHEN** la forge n'a de la place que pour 10 fer et que trois transporteurs sont libres
- **THEN** un seul part avec du fer pour la forge, et le contenant ne dépasse jamais 200

### Requirement: Ils courent
Un transporteur SHALL se déplacer plus vite qu'un autre personnage : 1,5 fois la vitesse d'un autre personnage dans les mêmes conditions (réglage de la configuration), qu'il soit piloté par l'IA ou incarné. La fatigue le ralentit comme les autres.

#### Scenario: Plus vite que les autres
- **WHEN** un transporteur IA et un garde IA, tous deux en pleine forme, marchent la même distance
- **THEN** le transporteur arrive plus tôt

#### Scenario: Joueur
- **WHEN** le joueur incarne un transporteur
- **THEN** il se déplace plus vite qu'un autre personnage incarné

### Requirement: L'entrepôt central est leur maison
Le royaume SHALL avoir un entrepôt central : un bâtiment avec trois lits, une porte et un contenant, près du château et relié à la route. Les transporteurs y dorment, y attendent quand il n'y a rien à porter, et il reçoit ce que personne ne consomme.

#### Scenario: Nuit des transporteurs
- **WHEN** la nuit tombe
- **THEN** les transporteurs IA rejoignent leur lit à l'entrepôt central et s'y couchent

#### Scenario: Rien à porter
- **WHEN** aucun voyage n'est possible
- **THEN** le transporteur IA attend à l'entrepôt central

### Requirement: La cargaison reste en main
Un transporteur qui porte une cargaison SHALL la garder jusqu'à ce qu'il la dépose, même s'il doit aller dormir : il la livre à son réveil avant de choisir autre chose. La ressource portée SHALL être visible sur le personnage : un bloc de la couleur de la ressource dans ses bras, avec la quantité en chiffre au-dessus de sa tête.

#### Scenario: La nuit tombe en chemin
- **WHEN** la nuit tombe alors qu'un transporteur porte 10 fer
- **THEN** il va dormir en gardant les 10 fer, et les livre le lendemain matin

#### Scenario: Voir ce qu'il porte
- **WHEN** un transporteur porte 10 fer
- **THEN** un bloc gris est dessiné dans ses bras et le chiffre 10 s'affiche au-dessus de sa tête

#### Scenario: Mains vides
- **WHEN** un transporteur ne porte rien
- **THEN** aucun bloc ni chiffre n'est dessiné sur lui

### Requirement: Le joueur charge et décharge à la main
Un transporteur incarné SHALL charger et décharger avec la touche d'action, depuis une case voisine d'un contenant en le regardant. Les mains vides, il charge jusqu'à 10 unités de la ressource que ce contenant donne et dont il y a le plus. Chargé, il décharge dans ce contenant ce que celui-ci accepte et a la place de recevoir ; le reste reste en main.

#### Scenario: Charger
- **WHEN** le joueur, mains vides, appuie sur la touche d'action devant une mine de fer qui contient 30 fer
- **THEN** il porte 10 fer et la mine en contient 20

#### Scenario: Décharger
- **WHEN** le joueur chargé de 10 fer appuie sur la touche d'action devant la forge qui a de la place
- **THEN** il ne porte plus rien et le contenant de la forge a 10 fer de plus

#### Scenario: Contenant qui n'accepte pas
- **WHEN** le joueur chargé de 10 fer appuie sur la touche d'action devant la mine d'or
- **THEN** rien ne change

#### Scenario: Contenant plein
- **WHEN** le joueur chargé appuie sur la touche d'action devant un contenant plein de cette ressource
- **THEN** rien ne change
