# assignment-interface Specification (delta)

## ADDED Requirements

### Requirement: Poser un coffre et un champ depuis la liste des plans
La liste des plans du roi SHALL proposer deux cartes de plus : « Coffre », de la taille d'une tuile, avec le choix de sa ressource (bois, pierre, fer, or, cuivre, blé), et « Champ », un carré de 3×3 gratuit. Chaque carte SHALL suivre le code couleur des plans (bleu, rouge, gris) et dire pourquoi une pose est impossible. Un coffre posé là où il fournit un poste SHALL le dire (« poste de bucheron »).

#### Scenario: Le coffre fournit un poste
- **WHEN** le roi place un coffre de bois près d'une forêt
- **THEN** la carte annonce « poste de bucheron »

#### Scenario: Le coffre sans poste
- **WHEN** le roi place un coffre de bois loin de toute forêt
- **THEN** la zone est valide (bleue) mais la carte annonce qu'il ne fournira aucun poste

#### Scenario: Le champ est gratuit
- **WHEN** le roi place un champ
- **THEN** aucun coût n'est affiché et le carré de 3×3 suit la souris

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
