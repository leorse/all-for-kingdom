## ADDED Requirements

### Requirement: Forêts, cabanes et bûcherons dans le royaume de départ
Le royaume de départ SHALL contenir deux forêts, chacune avec sa cabane proche et exactement un bûcheron. Les forêts, les cabanes et les postes de travail SHALL être définis par le scénario, sans chevaucher routes, champs, filons ni bâtiments.

#### Scenario: Deux forêts présentes
- **WHEN** la partie démarre
- **THEN** la carte contient deux forêts dont les arbres sont abattables, et deux bûcherons

#### Scenario: Forêts et cabanes accessibles
- **WHEN** on cherche un chemin depuis le château jusqu'à chaque lit de cabane et jusqu'à la lisière de chaque forêt
- **THEN** chacun est joignable à pied
