# game-start Specification

## Purpose
Définit le départ d'une vraie partie : une carte sans bâtiment, un camp de base, un pécule, le roi et deux artisans, et les vagabonds qui passent.

## ADDED Requirements

### Requirement: Une carte sans bâtiment mais avec sa nature
Le départ de partie SHALL proposer une carte sans aucun bâtiment (ni mur, ni habitat), dont la nature est déjà là : de l'eau et des lacs, des forêts, des filons, des arbres.

#### Scenario: Départ
- **WHEN** une partie démarre
- **THEN** la carte ne contient ni habitat ni mur, et contient de l'eau, des arbres, au moins deux forêts et des filons

### Requirement: Le camp de base
Le camp de base SHALL être une place au milieu de la carte avec un feu, trois lits et un coffre. Le coffre SHALL pouvoir contenir toutes les ressources, jusqu'à 100 de chacune. Il n'y a pas d'entrepôt au départ.

#### Scenario: Le camp
- **WHEN** une partie démarre
- **THEN** il y a un feu, trois lits et un seul coffre ; ce coffre accepte n'importe quelle ressource, avec une limite de 100 par ressource

### Requirement: Le pécule de départ
Le coffre du camp SHALL contenir au départ un pécule de bois, de pierre et de graines.

#### Scenario: Le pécule
- **WHEN** une partie démarre
- **THEN** le coffre du camp contient du bois, de la pierre et des graines, et nulle part ailleurs

### Requirement: Le roi et deux artisans
La partie SHALL commencer avec le roi et deux artisans, qui dorment chacun dans l'un des trois lits du camp.

#### Scenario: Les habitants
- **WHEN** une partie démarre
- **THEN** il y a un roi et deux artisans, chacun avec son lit au camp

### Requirement: Les vagabonds passent par là
Des vagabonds SHALL arriver de temps en temps par le bord de la carte, sans qu'il faille de taverne, se promener autour du camp, puis repartir. Ils n'ont ni lit ni heure de sommeil. Il n'y en a jamais plus qu'une limite à la fois. Une taverne les retient plus longtemps.

#### Scenario: Un vagabond arrive et repart
- **WHEN** assez de temps s'est écoulé
- **THEN** un vagabond apparaît sur un bord praticable, se déplace, puis finit par quitter le royaume

#### Scenario: La limite
- **WHEN** des vagabonds arrivent plus vite qu'ils ne repartent
- **THEN** il n'y en a jamais plus que la limite en même temps

### Requirement: Le village bâti reste le banc d'essai
Le village préconstruit SHALL rester disponible (`--village`), sans camp, sans vagabonds et sans changement de comportement.

#### Scenario: Le village
- **WHEN** on lance une partie dans le village
- **THEN** il n'y a ni camp ni vagabond, et les règles d'avant s'appliquent
