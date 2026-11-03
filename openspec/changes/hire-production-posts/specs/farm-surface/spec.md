# farm-surface Specification (delta)

## ADDED Requirements

### Requirement: Définir une surface cultivable
Le roi SHALL pouvoir définir une surface cultivable : un rectangle de terre à cultiver, gratuit et immédiat. Un paysan que le joueur incarne SHALL pouvoir faire de même, avec le même geste. Seules les tuiles d'herbe libres du rectangle deviennent des parcelles, en friche.

#### Scenario: Le roi définit
- **WHEN** le roi trace un rectangle de 4×3 sur de l'herbe libre
- **THEN** douze parcelles en friche apparaissent

#### Scenario: Un paysan définit
- **WHEN** le joueur incarne un paysan et trace un rectangle
- **THEN** les parcelles apparaissent de la même façon

#### Scenario: Terrain occupé
- **WHEN** le rectangle recouvre de l'eau, un arbre, un filon, un coffre ou une fondation
- **THEN** ces tuiles ne deviennent pas des parcelles

### Requirement: Une parcelle dépend d'un coffre de paysan
Chaque parcelle SHALL dépendre du coffre-poste de paysan le plus proche, à moins de la portée de rattachement (en cas d'égalité, le coffre posé en premier). Une parcelle sans coffre à portée SHALL rester visible mais n'être travaillée par personne, jusqu'à ce qu'un coffre soit posé à portée.

#### Scenario: Un coffre arrive après le champ
- **WHEN** le roi pose un coffre de paysan à portée d'un champ déjà tracé
- **THEN** ces parcelles dépendent de ce coffre

#### Scenario: Deux coffres
- **WHEN** une parcelle est à portée de deux coffres de paysan
- **THEN** elle dépend du plus proche

### Requirement: Le paysan ne travaille que ses parcelles
Le titulaire d'un coffre-poste de paysan SHALL ne labourer, semer et récolter que les parcelles qui dépendent de son coffre, et y déposer son blé. Le roi ne travaille pas la terre.

#### Scenario: Deux paysans
- **WHEN** deux paysans tiennent deux coffres voisins
- **THEN** chacun travaille son champ sans piétiner l'autre
