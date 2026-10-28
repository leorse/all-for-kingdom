# Design

## Context

Les coffres sont des données du scénario (`ContainerSpec { kind, tile }` dans `scenario.rs`) : le cœur ne connaît que leur tuile. Les producteurs déposent à distance (mineur, forgeron) ou portent à pied (paysan, bûcheron) jusqu'à `Container::tile` ; les transporteurs vont à une case voisine. Changer l'emplacement est donc une pure affaire de scénario et de tests. Voir `proposal.md`.

## Goals / Non-Goals

**Goals:**
- Chaque coffre au lieu d'extraction, dehors, sans gêner les postes de travail ni couper un passage.
- Une vérification automatique de la proximité coffre ↔ lieu d'extraction, pour que le prochain scénario ne l'oublie pas.

**Non-Goals:**
- Toute autre règle de contenant (capacité, transport) ; le placement par le roi.

## Decisions

- **Nouvelles positions** (le coffre ne doit couvrir ni un poste de mineur/bûcheron, ni un chemin) :
  - paysan : (11,6), contre la lisière est du champ (les parcelles vont de x=3 à x=10, y=4 à y=9) ;
  - mine de fer : (48,4), au pied du filon (45..46, 3..4), contre le mur du bâtiment, à côté du poste (47,4) ;
  - mine d'or : (28,35), au pied du filon (25..26, 34..35), à côté du poste (27,35) ;
  - mine de pierre : (50,33), au pied du filon (47..48, 32..33), à côté du poste (49,33) ;
  - cabane nord : (22,9), sur la rangée libre sous la forêt (x=13..24, y=2..8), dans le couloir de la colonne 22 ;
  - cabane sud : (17,31), sur la rangée libre au-dessus de la forêt (x=8..19, y=32..37), contre le mur de la cabane ;
  - forge (42,15) et entrepôt central (30,17) inchangés.
- **Proximité mesurée sur la distance de Manhattan** entre la tuile du coffre et la tuile la plus proche de la zone d'extraction (parcelle, tuile de filon, arbre de la forêt au départ) : au plus 2. Le test de scénario applique la règle aux sept coffres de production.
- **Rien d'autre ne change** : un coffre reste une `Tile::Crate` dehors, avec ses compteurs, ses réservations de transport et ses règles de plein. Les postes de travail des mineurs et des bûcherons sont inchangés ; on vérifie qu'ils ne sont pas masqués par un coffre.

## Risks / Trade-offs

- [Un coffre posé tout près d'un poste de mineur peut gêner un autre mineur qui voudrait s'y tenir] → les trois filons n'ont qu'un mineur et la case du coffre n'est pas un des huit postes ; vérifié par le test de scénario.
- [Les transporteurs ont un peu plus de chemin à parcourir pour atteindre les mines] → sans importance : les voyages se font de coffre à coffre.
- [Le coffre sud de la forêt (17,31) touche le mur de la cabane et rapproche la zone de la maison du paysan quand elle est grande] → la grande maison du paysan est ancrée en (8,26), à gauche (x≤14) ; vérifié avec six paysans.

## Open Questions

- Aucune.
