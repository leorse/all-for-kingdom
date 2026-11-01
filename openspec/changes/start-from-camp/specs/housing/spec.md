# housing Specification (delta)

## MODIFIED Requirements

### Requirement: Trois tailles fixes d'habitat
Les habitats du village de départ SHALL garder les trois classes de taille fixe (petite maison 7×5, moyenne 9×5, grande 9×7). Une maison bâtie par le roi SHALL avoir une taille libre au-dessus d'un minimum de 6×5 (murs compris), avec autant de lits que sa taille le permet, au moins un.

#### Scenario: Maison bâtie
- **WHEN** le roi fait bâtir une maison de n'importe quelle taille au-dessus du minimum
- **THEN** c'est un habitat valide : un lit par place demandée, une table et un bac d'eau, et chaque lit accessible depuis la porte

### Requirement: Un lit, une table, une étagère, un bac d'eau
Chaque habitat du village SHALL contenir au moins un lit, une table, une étagère et un bac d'eau. Une maison bâtie par le roi SHALL contenir au moins un lit, une table et un bac d'eau ; l'étagère y est facultative.

#### Scenario: Maison bâtie sans étagère
- **WHEN** on regarde une maison bâtie par le roi
- **THEN** elle a un lit, une table et un bac d'eau, sans étagère obligatoire
