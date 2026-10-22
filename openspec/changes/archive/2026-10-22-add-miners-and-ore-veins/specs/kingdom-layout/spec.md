## ADDED Requirements

### Requirement: Filons, mines et mineurs dans le royaume de départ
Le royaume de départ SHALL contenir trois filons (fer, or et pierre), chacun avec sa mine accolée et exactement un mineur. Les filons, les mines et leurs postes de travail SHALL être définis par le scénario, sans chevaucher routes, champs ni bâtiments.

#### Scenario: Trois natures présentes
- **WHEN** la partie démarre
- **THEN** la carte contient un filon de fer, un d'or et un de pierre, et trois mineurs

#### Scenario: Mines accessibles
- **WHEN** on cherche un chemin depuis le château jusqu'à chaque lit de mine et jusqu'à chaque case voisine d'un filon
- **THEN** chacun est joignable à pied
