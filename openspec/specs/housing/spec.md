# housing Specification

## Purpose
Définit les habitats : des bâtiments de taille fixe où l'on dort, leur mobilier obligatoire, leurs places, et les règles qui en découlent pour l'embauche et l'agrandissement.

## Requirements

### Requirement: Trois tailles fixes d'habitat
Tout bâtiment où quelqu'un dort SHALL être un habitat de l'une de ces classes, de dimensions fixes (murs compris), à l'exception du château, qui aura son propre fonctionnement et n'en fait pas partie : petite maison 7×5 (1 place), maison moyenne 9×5 (jusqu'à 3 places), grande maison 9×7 (jusqu'à 6 places). Aucune autre dimension n'est permise pour un habitat.

#### Scenario: Petite maison
- **WHEN** un habitat est de la classe petite maison
- **THEN** il fait 7×5 tuiles, dont un intérieur de 5×3, et offre une place

#### Scenario: Maison moyenne
- **WHEN** un habitat est de la classe maison moyenne
- **THEN** il fait 9×5 tuiles, dont un intérieur de 7×3, et offre jusqu'à trois places

#### Scenario: Grande maison
- **WHEN** un habitat est de la classe grande maison
- **THEN** il fait 9×7 tuiles, dont un intérieur de 7×5, et offre jusqu'à six places

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

### Requirement: Un lit, une table, une étagère, un bac d'eau
Chaque habitat SHALL contenir au moins un lit, exactement une table, exactement une étagère et exactement un bac d'eau, entièrement dans son intérieur, sans que deux meubles se chevauchent. La table, l'étagère et le bac SHALL être infranchissables ; les lits SHALL rester praticables.

#### Scenario: Petite maison meublée
- **WHEN** on regarde l'intérieur d'une petite maison
- **THEN** on y trouve un lit, une table, une étagère et un bac d'eau, et au moins trois cases libres

#### Scenario: Meubles dans l'intérieur
- **WHEN** on regarde un habitat
- **THEN** aucun meuble n'est sur un mur ni sur la porte, ni hors du bâtiment, et aucun ne déborde sur un autre

#### Scenario: Meubles infranchissables
- **WHEN** un personnage tente de traverser la table, l'étagère ou le bac
- **THEN** il est arrêté comme par un mur

#### Scenario: Les lits se traversent
- **WHEN** un personnage marche sur un lit
- **THEN** il n'est pas arrêté

### Requirement: Un lit par personne
Chaque personne qui dort SHALL avoir son propre lit dans un habitat ; deux personnes ne SHALL jamais partager un lit. Un habitat SHALL avoir autant de lits que de places occupées ou libres qu'il offre, sans dépasser le maximum de sa classe.

#### Scenario: Chacun son lit
- **WHEN** on liste les lits de tous les personnages du royaume de départ, le roi excepté
- **THEN** tous sont différents et chacun se trouve dans un habitat

#### Scenario: Plafond de la classe
- **WHEN** un habitat de classe petite maison est décrit avec deux lits
- **THEN** il est refusé comme non conforme

### Requirement: Pièce commune
Dans un habitat à plusieurs places, les lits SHALL se partager la même table, la même étagère et le même bac d'eau.

#### Scenario: Trois lits, un seul de chaque meuble
- **WHEN** une maison moyenne loge trois personnes
- **THEN** elle contient trois lits, une table, une étagère et un bac d'eau

### Requirement: Tout reste accessible
Dans un habitat, chaque tuile de chaque lit SHALL être joignable à pied depuis la porte, sans traverser de meuble infranchissable.

#### Scenario: Lit derrière les meubles
- **WHEN** un habitat est décrit avec un lit entouré de meubles et de murs
- **THEN** il est refusé comme non conforme

#### Scenario: Entrer et se coucher
- **WHEN** un personnage entre par la porte d'une maison meublée
- **THEN** il peut rejoindre son lit

### Requirement: Places libres
Le jeu SHALL savoir, pour chaque habitat et pour le royaume entier, combien de places sont libres : un lit que personne n'occupe comme logement.

#### Scenario: Maison du forgeron
- **WHEN** le forgeron habite seul sa petite maison
- **THEN** cet habitat n'a aucune place libre

#### Scenario: Maison moyenne à moitié vide
- **WHEN** une maison moyenne a trois lits et deux habitants
- **THEN** elle a une place libre et le royaume en compte une de plus

### Requirement: On n'embauche que s'il y a de la place
Un vagabond de la taverne ne SHALL pouvoir être embauché que s'il reste au moins une place libre dans un habitat du royaume. Sans place libre nulle part, l'embauche est refusée.

#### Scenario: Tout est plein
- **WHEN** aucune place n'est libre dans le royaume
- **THEN** l'embauche d'un vagabond est refusée

#### Scenario: Une place existe
- **WHEN** au moins une place est libre dans un habitat quelconque
- **THEN** l'embauche est permise

### Requirement: Agrandir demande de la place autour
Un habitat SHALL pouvoir passer à la classe supérieure seulement si l'emprise de cette classe, ancrée au même coin haut-gauche, ne couvre que des cases libres (herbe ou chemin) dans la carte. Un obstacle sur le chemin (arbre, eau, mur, coffre, champ, filon, autre bâtiment) l'interdit. Une grande maison ne peut pas être agrandie.

#### Scenario: Espace dégagé
- **WHEN** toutes les cases de l'emprise de la classe supérieure sont de l'herbe ou un chemin
- **THEN** l'agrandissement est permis

#### Scenario: Un arbre gêne
- **WHEN** un arbre se trouve dans l'emprise de la classe supérieure
- **THEN** l'agrandissement est refusé

#### Scenario: Déjà au maximum
- **WHEN** l'habitat est une grande maison
- **THEN** l'agrandissement est refusé
