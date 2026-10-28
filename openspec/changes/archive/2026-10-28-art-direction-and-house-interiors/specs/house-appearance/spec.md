## Purpose
Définit l'aspect d'un habitat : sa coupe, son sol, ses murs, ses fenêtres, sa cheminée, son mobilier et son paillasson, tels que sur l'image de référence.

## ADDED Requirements

### Requirement: Un habitat vu en coupe
Un habitat SHALL être dessiné en coupe, comme sur l'image de référence : une bande de toit de chaume sur son bord nord, un mur nord en rondins vu de face, des murs ouest et est en bois dont on voit l'épaisseur, un mur sud bas percé de la porte, et l'intérieur visible entre les murs. Cet aspect SHALL s'appliquer à tout habitat, de toute classe et de toute taille.

#### Scenario: Les quatre côtés
- **WHEN** on regarde une petite maison
- **THEN** on voit le toit de chaume au nord, le mur de rondins sous lui, un mur de bois à l'ouest et à l'est, et un mur sud bas avec la porte

#### Scenario: Toutes les tailles
- **WHEN** on regarde une maison moyenne puis une grande maison
- **THEN** elles ont les mêmes éléments, étirés à leur taille

#### Scenario: Porte au nord
- **WHEN** la porte d'un habitat est dans le mur nord
- **THEN** elle est dessinée dans le mur de rondins et la bande de toit reste continue

### Requirement: Un sol en planches
L'intérieur d'un habitat SHALL être un sol de planches de bois, avec des planches de longueurs variées et quelques joints, sur toute sa surface libre.

#### Scenario: Le sol
- **WHEN** on regarde l'intérieur d'une maison vide de meubles
- **THEN** on voit des planches de bois qui se prolongent d'un mur à l'autre

### Requirement: Des fenêtres
Le mur nord d'un habitat SHALL porter au moins une fenêtre, plus pour les classes plus larges, à des endroits qui ne recouvrent ni la porte ni la cheminée. La nuit, les fenêtres SHALL être éclairées d'une lumière chaude.

#### Scenario: Une fenêtre au minimum
- **WHEN** on regarde le mur nord d'une petite maison
- **THEN** il porte au moins une fenêtre

#### Scenario: Fenêtres la nuit
- **WHEN** la nuit tombe
- **THEN** les fenêtres des habitats s'illuminent d'ambre

### Requirement: Une cheminée de pierre
Chaque habitat SHALL avoir une cheminée de pierre contre son mur nord, avec un foyer où brûle un feu animé et un conduit qui traverse la bande de toit. La cheminée ne couvre aucune tuile de sol : c'est un décor du mur.

#### Scenario: Le feu brûle
- **WHEN** on regarde une maison pendant quelques secondes
- **THEN** les flammes de la cheminée bougent

#### Scenario: Aucune place prise
- **WHEN** on compte les cases libres de l'intérieur d'une maison
- **THEN** la cheminée n'en retire aucune

### Requirement: Un mobilier reconnaissable
Les meubles obligatoires SHALL se reconnaître au premier coup d'œil : le lit a un cadre de bois, un oreiller blanc et une couverture rouge ; la table est en bois avec une assiette et une tasse ; l'étagère est une bibliothèque de bois avec des livres, des flacons et deux portes basses ; le baquet est un tonneau cerclé de métal, plein d'eau, avec sa louche.

#### Scenario: Le lit
- **WHEN** on regarde un lit
- **THEN** on y voit un oreiller blanc à la tête et une couverture rouge

#### Scenario: La table
- **WHEN** on regarde une table
- **THEN** on y voit une assiette et une tasse

#### Scenario: L'étagère
- **WHEN** on regarde une étagère
- **THEN** on y voit des livres et des flacons de couleurs

#### Scenario: Le baquet
- **WHEN** on regarde un baquet
- **THEN** on y voit de l'eau et une louche sur un tonneau cerclé

### Requirement: Le mobilier tourne avec sa rotation
Un meuble dessiné SHALL montrer sa rotation : un lit tourné a sa tête du côté voulu par sa rotation, une étagère montre sa façade vers l'intérieur de la pièce, une table et un baquet sont dessinés selon leur sens. Le dessin SHALL couvrir exactement les tuiles de son emprise tournée.

#### Scenario: Lit vertical
- **WHEN** un lit est tourné de 90°
- **THEN** il est dessiné sur deux tuiles l'une au-dessus de l'autre, l'oreiller en haut

#### Scenario: Étagère contre le mur est
- **WHEN** une étagère est tournée de 90°, pour faire face à l'ouest
- **THEN** sa façade est dessinée vers l'ouest

### Requirement: Un paillasson à la porte
Chaque habitat SHALL avoir un paillasson dessiné devant sa porte, à l'intérieur, sans couvrir de meuble.

#### Scenario: Le paillasson
- **WHEN** on regarde la porte d'un habitat
- **THEN** un paillasson est posé juste derrière elle

### Requirement: Un personnage couché dans le bon sens
Un personnage qui dort SHALL être dessiné couché dans son lit, la tête sur l'oreiller, quelle que soit l'orientation du lit.

#### Scenario: Lit vertical
- **WHEN** un personnage dort dans un lit tourné de 90° (tête en haut)
- **THEN** il est dessiné couché verticalement, la tête vers le haut

### Requirement: Les décors ne gênent pas le jeu
Fenêtres, cheminée, paillasson et autres décors SHALL rester lisibles sans changer le gameplay : ils ne bloquent aucun déplacement ni aucun lit, et ne cachent aucun personnage.

#### Scenario: Marcher sur le paillasson
- **WHEN** un personnage entre dans une maison
- **THEN** il marche sur le paillasson sans être arrêté et reste visible
