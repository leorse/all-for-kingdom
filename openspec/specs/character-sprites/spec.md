# character-sprites Specification

## Purpose
Définit le gabarit commun des personnages dessinés (taille de cellule, point d'ancrage, liste des animations et leurs règles de qualité) et la chaîne qui produit leurs sprites PNG à partir d'un dessin SVG versionné et modifiable.

## Requirements

### Requirement: Une cellule commune de 64×64 ancrée aux pieds
Chaque image d'un personnage SHALL tenir dans une cellule de 64×64 pixels de dessin (2×2 tuiles). Le personnage SHALL y être placé de sorte qu'un point fixe de la cellule, le même pour toutes les images, corresponde à ses pieds : ce point est posé sur la position du personnage dans le monde. Un homme adulte debout SHALL mesurer environ 1,7 tuile de haut et une tuile de large au plus hors bras tendus, de sorte que sa tête dépasse sur la tuile au-dessus de celle où il se tient.

#### Scenario: Les pieds sur sa case
- **WHEN** un personnage se tient immobile au centre d'une case
- **THEN** ses pieds sont dessinés sur cette case et sa tête dépasse sur la case du dessus

#### Scenario: Pas de saut entre deux animations
- **WHEN** un personnage passe de la marche au repos sans bouger
- **THEN** ses pieds restent au même endroit à l'écran

### Requirement: Les animations d'un personnage
Un personnage dessiné SHALL fournir au moins les animations suivantes, chacune avec un nombre d'images et une vitesse fixés par le gabarit, identiques pour tous les personnages :
- **debout** dans les 4 directions (bas, haut, gauche, droite), une image chacune ;
- **marche** dans les 4 directions, 4 images chacune ;
- **repos** (*idle*), face vers le bas, 4 images en boucle : respiration et clignement d'yeux ;
- **sommeil** couché, tête en haut, 2 images de respiration ;
- **travail**, propre au métier.
Le jeu SHALL choisir l'animation d'après l'activité et la direction du personnage : la marche quand il marche, l'animation de travail quand il travaille, le sommeil quand il dort. Quand il est inactif, il SHALL être en repos s'il regarde vers le bas, et debout dans sa direction sinon.

#### Scenario: Marcher vers la gauche
- **WHEN** un personnage marche vers la gauche
- **THEN** l'animation de marche vers la gauche joue en boucle

#### Scenario: Attendre
- **WHEN** un personnage inactif regarde vers le bas
- **THEN** il respire et cligne des yeux de temps en temps

#### Scenario: S'arrêter face au mur
- **WHEN** un personnage inactif regarde vers le haut
- **THEN** il est dessiné debout, vu de dos

### Requirement: Une marche aux jambes bien alternées
Dans un cycle de marche, la première image SHALL montrer une jambe en avant, la troisième l'autre jambe en avant, et les deuxième et quatrième les jambes qui se croisent. Deux images consécutives ne SHALL jamais avancer la même jambe. Les bras SHALL se balancer à l'opposé des jambes.

#### Scenario: Deux pas, deux jambes
- **WHEN** on compare les images 1 et 3 d'une marche
- **THEN** ce n'est pas la même jambe qui est en avant, et le bras en avant est du côté opposé à la jambe en avant

### Requirement: Un visage soigné et constant
Vu de face, le visage d'un personnage SHALL montrer les yeux, les sourcils et la bouche (ou la barbe), symétriques et alignés. Il SHALL être identique sur toutes les images de face d'un même personnage, à l'exception du clignement d'yeux et des mouvements de tête voulus par l'animation.

#### Scenario: Même tête d'une image à l'autre
- **WHEN** on compare le visage du personnage au repos et en marche vers le bas
- **THEN** les pixels du visage sont les mêmes, à part les yeux pendant un clignement

### Requirement: Un dessin SVG versionné, des PNG générés
Chaque personnage dessiné SHALL avoir pour source un fichier SVG versionné, organisé en planche : une ligne par animation, une colonne par image, chaque image repérée par un identifiant (par exemple `marche_bas_2`). Une commande unique SHALL produire depuis ce SVG la planche PNG utilisée par le jeu, sans aucun lissage. La commande SHALL aussi ajouter le contour sombre extérieur et produire les animations vers la gauche par miroir de celles vers la droite, sauf si le SVG les dessine lui-même. Le PNG produit SHALL être versionné, pour que le jeu se construise sans la commande, et un test SHALL échouer si le PNG ne correspond plus à son SVG.

#### Scenario: Modifier le forgeron
- **WHEN** on modifie le SVG du forgeron et qu'on lance la commande de génération
- **THEN** la planche PNG est régénérée et le jeu affiche le forgeron modifié

#### Scenario: PNG oublié
- **WHEN** le SVG a été modifié sans régénérer le PNG
- **THEN** le test de concordance échoue en nommant le personnage concerné

#### Scenario: Image manquante
- **WHEN** il manque au SVG une image exigée par le gabarit
- **THEN** la génération échoue en nommant l'image manquante

### Requirement: Des pixels nets et dans la palette
Chaque pixel d'une planche de personnage SHALL être soit entièrement transparent, soit d'une couleur de la palette commune : aucune demi-transparence ni couleur intermédiaire née d'un lissage. L'ombre au sol ne fait pas partie du sprite ; le jeu la dessine.

#### Scenario: Lissage accidentel
- **WHEN** une forme du SVG produit des pixels de couleur intermédiaire
- **THEN** la génération échoue en donnant la position et la couleur du premier pixel fautif

### Requirement: Un gabarit réutilisable par les autres métiers
Le gabarit (cellule, ancrage, liste des animations, nombre d'images, vitesses et disposition de la planche) SHALL être défini une seule fois et partagé : un nouveau personnage SHALL pouvoir se faire en partant du SVG d'un autre, sans changer le code du jeu, à l'exception de son animation de travail. Tant qu'un métier n'a pas de planche, il SHALL garder son apparence actuelle.

#### Scenario: Les autres métiers inchangés
- **WHEN** la partie démarre après ce changement
- **THEN** seul le forgeron a son nouveau dessin ; les autres personnages gardent leur apparence d'avant
