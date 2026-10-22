# kingdom-layout Specification

## Purpose
Définit l'étendue et la disposition du royaume de départ : des distances suffisantes entre habitations, champs et bâtiments publics pour que le royaume dépasse l'écran.

## Requirements

### Requirement: Royaume plus grand que l'écran
Le royaume de départ SHALL être plus grand que la zone visible d'une fenêtre par défaut (1024×640 pixels), dans les deux dimensions.

#### Scenario: Fenêtre par défaut
- **WHEN** la partie démarre dans la fenêtre par défaut
- **THEN** une partie du royaume est hors écran, à droite et en bas

### Requirement: Maisons et champs éloignés
Les maisons et les champs SHALL être situés loin du centre du village et les uns des autres, de sorte qu'au moins la maison et le champ du paysan ne tiennent pas ensemble à l'écran avec le château.

#### Scenario: Trajet du paysan
- **WHEN** le paysan rentre de son champ à sa maison
- **THEN** il parcourt un trajet nettement plus long qu'avant et franchit des zones hors de l'écran si la caméra est centrée ailleurs

#### Scenario: Maison hors écran au départ
- **WHEN** la caméra est centrée sur le personnage de départ
- **THEN** au moins une maison et un champ ne sont pas visibles

### Requirement: Royaume entièrement connecté
Chaque bâtiment, lit, champ et poste de travail du scénario SHALL rester accessible à pied depuis la porte de chaque bâtiment par des routes ou de l'herbe, et chaque rôle SHALL pouvoir mener sa journée complète.

#### Scenario: Journée complète de l'IA
- **WHEN** la simulation tourne plusieurs jours sans personnage incarné
- **THEN** chaque personnage atteint son lit et ses postes de travail et le cycle travail/sommeil continue

### Requirement: Disposition définie par le scénario
La disposition du royaume SHALL rester une donnée du scénario, indépendante du rendu et de la caméra. Le placement de bâtiments par le roi en cours de partie n'est pas couvert.

#### Scenario: Changement de scénario
- **WHEN** le scénario change de dimensions ou de coordonnées
- **THEN** le rendu et la caméra s'adaptent sans modification de la logique de jeu

### Requirement: Filons, mines et mineurs dans le royaume de départ
Le royaume de départ SHALL contenir trois filons (fer, or et pierre), chacun avec sa mine accolée et exactement un mineur. Les filons, les mines et leurs postes de travail SHALL être définis par le scénario, sans chevaucher routes, champs ni bâtiments.

#### Scenario: Trois natures présentes
- **WHEN** la partie démarre
- **THEN** la carte contient un filon de fer, un d'or et un de pierre, et trois mineurs

#### Scenario: Mines accessibles
- **WHEN** on cherche un chemin depuis le château jusqu'à chaque lit de mine et jusqu'à chaque case voisine d'un filon
- **THEN** chacun est joignable à pied

### Requirement: Forêts, cabanes et bûcherons dans le royaume de départ
Le royaume de départ SHALL contenir deux forêts, chacune avec sa cabane proche et exactement un bûcheron. Les forêts, les cabanes et les postes de travail SHALL être définis par le scénario, sans chevaucher routes, champs, filons ni bâtiments.

#### Scenario: Deux forêts présentes
- **WHEN** la partie démarre
- **THEN** la carte contient deux forêts dont les arbres sont abattables, et deux bûcherons

#### Scenario: Forêts et cabanes accessibles
- **WHEN** on cherche un chemin depuis le château jusqu'à chaque lit de cabane et jusqu'à la lisière de chaque forêt
- **THEN** chacun est joignable à pied
