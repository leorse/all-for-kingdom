# Spec Delta

## Purpose

Définit la partie du royaume affichée à l'écran : zoom fixe, centrage sur le personnage incarné et défilement de la carte quand il se déplace.

## ADDED Requirements

### Requirement: Zoom fixe
Le jeu SHALL afficher la carte avec une taille de tuile constante en pixels, indépendante des dimensions de la carte. Seule la partie de la carte qui tient dans la fenêtre est visible.

#### Scenario: Carte plus grande que la fenêtre
- **WHEN** la carte est plus grande que la fenêtre
- **THEN** une partie seulement de la carte est visible et la taille des tuiles ne change pas

#### Scenario: Redimensionnement de la fenêtre
- **WHEN** la fenêtre est redimensionnée
- **THEN** la taille des tuiles reste identique et la zone visible s'agrandit ou se réduit

### Requirement: Centrage animé à l'incarnation
Quand le joueur incarne un personnage (Tab, touches 1 à 9, clic), la caméra SHALL glisser rapidement (de l'ordre d'une demi-seconde au plus) de sa position actuelle vers le nouveau personnage, pour que le joueur voie d'où il part et où il arrive.

#### Scenario: Changement de personnage
- **WHEN** le joueur passe d'un personnage à un autre dont la maison est hors écran
- **THEN** la carte défile de façon visible jusqu'au nouveau personnage, qui finit au centre de l'écran, sauf si le bord de la carte l'en empêche

#### Scenario: Changement pendant une animation
- **WHEN** le joueur change de personnage alors qu'une animation est en cours
- **THEN** la nouvelle animation part de la position actuelle de la caméra, sans saut

#### Scenario: Cible qui bouge pendant l'animation
- **WHEN** le personnage incarné se déplace pendant l'animation
- **THEN** la caméra termine sur sa position réelle et non sur sa position de départ

#### Scenario: Démarrage de la partie
- **WHEN** la partie démarre avec un personnage incarné
- **THEN** la caméra est déjà centrée sur lui dès la première image, sans animation

#### Scenario: Vitesse de simulation
- **WHEN** la vitesse de simulation est changée (touche T)
- **THEN** la durée de l'animation reste la même en temps réel

### Requirement: Suivi du personnage
La caméra SHALL suivre le personnage incarné quand il se déplace, de façon qu'il reste visible à tout moment. La carte ne défile que lorsque le personnage s'approche du bord de l'écran.

#### Scenario: Marche vers le bord de l'écran
- **WHEN** le personnage incarné s'approche du bord de l'écran
- **THEN** la carte défile pour le garder à l'écran

#### Scenario: Déplacement au centre
- **WHEN** le personnage incarné se déplace dans la zone centrale de l'écran
- **THEN** la carte ne bouge pas

#### Scenario: Personnage en mode AFK
- **WHEN** le personnage incarné est piloté par l'IA (mode AFK)
- **THEN** la caméra continue de le suivre

### Requirement: Caméra bornée par la carte
La caméra SHALL NOT afficher de zone située au-delà des bords de la carte. Si la carte est plus petite que la fenêtre dans une dimension, elle est centrée dans cette dimension.

#### Scenario: Personnage près d'un coin de la carte
- **WHEN** le personnage incarné est près d'un coin de la carte
- **THEN** la caméra s'arrête au bord de la carte et le personnage est décentré à l'écran

### Requirement: Cohérence des interactions avec la vue
Le clic sur un personnage et les indications dessinées sur la carte (cible de parcelle, noms, jauges) SHALL correspondre à ce qui est affiché, quel que soit le décalage de la caméra.

#### Scenario: Clic après défilement
- **WHEN** la carte a défilé et le joueur clique sur un personnage visible
- **THEN** ce personnage est incarné et la caméra se recentre sur lui

#### Scenario: Personnage hors écran
- **WHEN** un personnage est hors de la zone visible
- **THEN** il n'est pas cliquable et aucun dessin parasite n'apparaît à l'écran
