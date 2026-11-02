# forge-plan Specification

## Purpose
Ajoute la forge à la liste des plans du roi : un bâtiment avec son enclume, son coffre et son poste de forgeron.

## ADDED Requirements

### Requirement: La forge est un plan
La liste des plans SHALL proposer une forge, avec une taille minimale (murs compris) et une taille libre au-dessus, comme les autres types. Son coût en bois et en pierre et sa durée de construction suivent la surface et le type, et comprennent son enclume.

#### Scenario: Dans la liste
- **WHEN** le roi ouvre la liste des plans
- **THEN** la forge y figure avec sa taille minimale et son coût minimal

#### Scenario: Plus grande, plus chère
- **WHEN** on compare deux forges dont l'une est plus grande
- **THEN** la plus grande coûte plus de bois et de pierre et prend plus de temps à construire

### Requirement: Une enclume posée automatiquement
Une forge construite SHALL contenir au moins une enclume, posée automatiquement à l'intérieur, de façon que la case au nord de l'enclume soit libre et praticable : c'est le point où se tient le forgeron pour forger. L'enclume est un obstacle.

#### Scenario: Le point de forge
- **WHEN** une forge est terminée
- **THEN** chaque enclume est solide et la case juste au nord est libre, accessible depuis la porte

### Requirement: Un coffre de forge
Une forge terminée SHALL avoir un coffre à l'extérieur, contre ses murs, qui sert de réserve de fer et de rangement des outils (le comportement d'un coffre de forge existant). Les transporteurs lui apportent le fer qu'il consomme.

#### Scenario: Le coffre
- **WHEN** une forge est terminée
- **THEN** un coffre de forge, vide, se trouve contre le bâtiment et reçoit le fer que les transporteurs lui livrent

### Requirement: Un poste de forgeron par enclume
Chaque enclume SHALL être un poste de forgeron (voir `posts-and-roles`). Tant qu'aucun forgeron n'y est affecté, personne ne forge. Un forgeron affecté va se placer à son poste et forge comme avant (la durée, le fer consommé et les outils produits ne changent pas).

#### Scenario: Pas de forgeron
- **WHEN** une forge est terminée sans forgeron affecté
- **THEN** rien n'est forgé

#### Scenario: Un forgeron affecté
- **WHEN** le roi affecte quelqu'un au poste de forgeron d'une forge qui a du fer
- **THEN** il se place au nord de l'enclume et forge des outils

### Requirement: Sans fer, le forgeron attend
Le forgeron SHALL attendre à son poste quand le coffre de la forge n'a pas assez de fer ; ce comportement existant ne change pas.

#### Scenario: Pas de fer
- **WHEN** le coffre de la forge n'a pas de fer
- **THEN** le forgeron ne forge pas et ne quitte pas son poste
