## MODIFIED Requirements

### Requirement: Le mobilier a une taille
Dans un habitat, un lit SHALL occuper 2 tuiles, une table 2 tuiles, une étagère 2 tuiles et un bac d'eau 1 tuile. Chaque meuble SHALL pouvoir se poser dans quatre rotations (0°, 90°, 180° et 270°, quarts de tour dans le sens des aiguilles d'une montre depuis la pose de base) : un meuble de 2 tuiles fait 2×1 (largeur 2, hauteur 1) à 0° et 180°, et 1×2 à 90° et 270°. La pose de base a la tête de lit à gauche et la façade d'étagère vers le bas. Un lit a donc sa tête à gauche à 0°, en haut à 90°, à droite à 180° et en bas à 270° ; la tuile de sa tête désigne la place.

#### Scenario: Un lit de deux tuiles
- **WHEN** on regarde un lit d'habitat
- **THEN** il couvre deux tuiles côte à côte, la tête d'un côté et le pied de l'autre

#### Scenario: Table et étagère
- **WHEN** on regarde la table et l'étagère d'un habitat
- **THEN** chacune couvre deux tuiles côte à côte, dans le sens de son orientation

#### Scenario: Le bac reste petit
- **WHEN** on regarde le bac d'eau d'un habitat
- **THEN** il couvre une seule tuile, quelle que soit son orientation

#### Scenario: Meuble tourné
- **WHEN** un lit est tourné de 90°
- **THEN** il couvre deux tuiles l'une au-dessus de l'autre, la tête en haut, et la tuile de tête désigne la place

#### Scenario: Emprise tournée dans l'intérieur
- **WHEN** un meuble tourné déborde de l'intérieur de l'habitat ou chevauche un autre meuble
- **THEN** l'habitat est refusé comme non conforme

#### Scenario: Accès avec un meuble tourné
- **WHEN** un lit tourné est entouré de meubles de façon à n'être plus joignable depuis la porte
- **THEN** l'habitat est refusé comme non conforme
