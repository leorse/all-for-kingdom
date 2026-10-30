## MODIFIED Requirements

### Requirement: Un contenant par lieu de travail
Le royaume SHALL avoir un contenant à chacun de ces lieux, posé au niveau de l'extraction de la ressource : à côté du champ du paysan, au pied de chaque filon, en lisière de chaque forêt, contre la forge et à côté de la porte de l'entrepôt central. Un contenant SHALL être un coffre posé sur la carte, toujours à l'extérieur des bâtiments et jamais à l'intérieur, infranchissable, auquel on accède depuis une case voisine.

#### Scenario: Les contenants du royaume de départ
- **WHEN** la partie démarre
- **THEN** la carte porte un coffre à côté du champ du paysan, un au pied de chacun des trois filons, un en lisière de chacune des deux forêts, un contre la forge et un contre l'entrepôt central

#### Scenario: Un coffre est un obstacle
- **WHEN** un personnage tente de traverser un coffre
- **THEN** il est arrêté comme par un mur

#### Scenario: Les coffres sont dehors
- **WHEN** on regarde la carte au départ
- **THEN** aucun coffre ne se trouve à l'intérieur d'un bâtiment

#### Scenario: Le coffre du paysan est au champ
- **WHEN** on mesure la distance entre le coffre du paysan et la parcelle la plus proche
- **THEN** elle est d'au plus deux cases

#### Scenario: Le coffre d'une mine est au filon
- **WHEN** on mesure la distance entre le coffre d'une mine et son filon
- **THEN** elle est d'au plus deux cases

#### Scenario: Le coffre d'un bûcheron est à la forêt
- **WHEN** on mesure la distance entre le coffre d'un bûcheron et l'arbre le plus proche de sa forêt
- **THEN** elle est d'au plus deux cases
