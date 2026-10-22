## ADDED Requirements

### Requirement: Contenants, entrepôt central et transporteurs dans le royaume de départ
Le royaume de départ SHALL contenir un contenant au champ du paysan, contre chaque mine, contre chaque cabane et contre la forge, ainsi qu'un entrepôt central avec son contenant dehors à côté de sa porte, ses trois lits dedans et trois transporteurs. Ces éléments SHALL être définis par le scénario, sans chevaucher routes, champs, filons ni autres bâtiments. Chaque contenant SHALL avoir au moins une case voisine praticable, joignable à pied depuis le château.

#### Scenario: Les coffres sont présents
- **WHEN** la partie démarre
- **THEN** sept coffres de production et un coffre d'entrepôt central sont sur la carte

#### Scenario: Les coffres sont accessibles
- **WHEN** on cherche un chemin depuis le château jusqu'à une case voisine de chaque contenant et jusqu'à chaque lit de l'entrepôt central
- **THEN** chacun est joignable à pied
