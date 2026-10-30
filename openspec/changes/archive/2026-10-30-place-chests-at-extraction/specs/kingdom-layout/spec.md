## MODIFIED Requirements

### Requirement: Contenants, entrepôt central et transporteurs dans le royaume de départ
Le royaume de départ SHALL contenir un contenant à côté du champ du paysan, au pied de chaque filon, en lisière de chaque forêt et contre la forge, ainsi qu'un entrepôt central avec son contenant dehors à côté de sa porte, ses trois lits dedans et trois transporteurs. Ces éléments SHALL être définis par le scénario, sans chevaucher routes, champs, filons ni autres bâtiments. Chaque contenant SHALL avoir au moins une case voisine praticable, joignable à pied depuis le château.

#### Scenario: Les coffres sont présents
- **WHEN** la partie démarre
- **THEN** sept coffres de production et un coffre d'entrepôt central sont sur la carte

#### Scenario: Les coffres sont accessibles
- **WHEN** on cherche un chemin depuis le château jusqu'à une case voisine de chaque contenant et jusqu'à chaque lit de l'entrepôt central
- **THEN** chacun est joignable à pied

#### Scenario: Les coffres sont à l'extraction
- **WHEN** on place les coffres du scénario de départ
- **THEN** celui du paysan touche son champ à deux cases près, ceux des mines leur filon, ceux des cabanes leur forêt
