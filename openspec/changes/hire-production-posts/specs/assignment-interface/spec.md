# assignment-interface Specification (delta)

## ADDED Requirements

### Requirement: Poser un coffre-poste depuis la liste des plans
La liste des plans du roi SHALL proposer trois cartes de la taille d'une tuile : « Poste de bucheron », « Poste de mineur » et « Poste de paysan », plus une carte « Champ » pour tracer une surface cultivable. Chaque carte SHALL suivre le code couleur des plans (bleu, rouge, gris) et dire pourquoi une pose est impossible.

#### Scenario: Gris loin de la matière
- **WHEN** le roi place la carte « Poste de bucheron » loin de toute forêt
- **THEN** la zone est grise et un message indique qu'aucune forêt n'est à portée

#### Scenario: Le champ est gratuit
- **WHEN** le roi trace un champ
- **THEN** aucun coût n'est affiché

### Requirement: Les coffres-poste sur la mini-carte
Les coffres-poste SHALL apparaître sur la mini-carte comme les autres postes (vert libre, rouge occupé), et leur métier SHALL figurer dans la liste des métiers avec ses postes libres.

#### Scenario: Liste des métiers
- **WHEN** deux coffres de bûcheron sont posés, dont un tenu
- **THEN** la liste affiche « Bucheron 1/2 »

### Requirement: Le choix de culture du paysan incarné
Quand le joueur incarne un paysan et que plusieurs graines existent, l'interface SHALL lui proposer le choix de la culture. Elle SHALL ne rien proposer au roi ni quand une seule graine existe.

#### Scenario: Une seule graine
- **WHEN** seule la graine de blé existe
- **THEN** aucune liste de cultures n'est affichée
