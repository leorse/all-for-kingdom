## ADDED Requirements

### Requirement: Tous les logements du royaume de départ sont des habitats conformes
Le royaume de départ SHALL loger chacun de ses personnages dans un habitat conforme à la capacité `housing` : la caserne et l'entrepôt central en maison moyenne, les maisons du paysan, du forgeron et du tavernier, les mines et les cabanes en petite maison. La maison du paysan SHALL grandir avec le nombre de paysans (petite, moyenne ou grande maison), sans chevaucher les autres éléments du scénario. Les postes de travail et les rondes SHALL rester sur des cases praticables.

#### Scenario: Conformité du village
- **WHEN** la partie démarre
- **THEN** chaque lit appartient à un habitat de classe valide, avec sa table et son bac d'eau, et chaque lit est joignable depuis la porte

#### Scenario: Six paysans
- **WHEN** le scénario est construit avec six paysans
- **THEN** la maison du paysan est une grande maison avec six lits

#### Scenario: Pas de place libre au départ
- **WHEN** la partie démarre
- **THEN** chaque lit a son occupant et le royaume n'a aucune place libre
