# farm-surface Specification (delta)

## ADDED Requirements

### Requirement: Un champ est un carré de terre de 3 sur 3
Un champ SHALL être un **carré de terre de 3×3 tuiles**, posé **nu** : sans coffre, sans bâtiment, gratuit et immédiat. Le roi SHALL pouvoir le poser ; un paysan que le joueur incarne SHALL pouvoir le poser de la même façon. La pose est refusée si l'une des neuf tuiles n'est pas de l'herbe libre (eau, arbre, filon, coffre, fondation, quelqu'un dessus). Chaque tuile est une parcelle, en friche.

#### Scenario: Le roi pose un champ
- **WHEN** le roi pose un champ sur 3×3 tuiles d'herbe libres
- **THEN** neuf parcelles en friche apparaissent, sans coffre

#### Scenario: Un paysan pose un champ
- **WHEN** le joueur incarne un paysan et pose un champ
- **THEN** il apparaît de la même façon

#### Scenario: Terrain occupé
- **WHEN** une des neuf tuiles est de l'eau, un arbre, un filon ou un coffre
- **THEN** le champ ne se pose pas

### Requirement: Le paysan entretient le champ
Un champ SHALL être entretenu par un paysan : il laboure, sème et récolte ses parcelles. Un champ dépend du **coffre de blé le plus proche** (à portée de rattachement) ; son titulaire d'un poste de paysan l'entretient. Un champ sans coffre à portée SHALL rester visible, mais n'être entretenu par personne, jusqu'à ce qu'un coffre soit posé à portée.

#### Scenario: Un coffre arrive après le champ
- **WHEN** le roi pose un coffre de blé à portée d'un champ déjà posé
- **THEN** ce champ dépend de ce coffre et son paysan l'entretient

#### Scenario: Deux coffres
- **WHEN** un champ est à portée de deux coffres de blé
- **THEN** il dépend du plus proche (en cas d'égalité, du premier posé)

### Requirement: Le paysan rapporte son blé au coffre
Le paysan SHALL récolter à la main puis porter son blé au coffre de blé le plus proche qui n'est pas plein (voir `chests`). Les champs du paysan sont ceux de son coffre ; deux paysans de deux coffres voisins ne se piétinent pas.

#### Scenario: Deux paysans
- **WHEN** deux paysans tiennent deux coffres voisins, chacun près de son champ
- **THEN** chacun entretient son champ et rapporte son blé à son coffre
