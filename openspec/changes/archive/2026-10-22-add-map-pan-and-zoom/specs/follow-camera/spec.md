## RENAMED Requirements
- FROM: `### Requirement: Zoom fixe`
- TO: `### Requirement: Zoom de départ constant`

## MODIFIED Requirements

### Requirement: Zoom de départ constant
Le jeu SHALL démarrer avec une taille de tuile constante en pixels, indépendante des dimensions de la carte. Cette taille ne change qu'à la demande du joueur (voir la capacité `map-navigation`) ; ni le redimensionnement de la fenêtre ni le suivi du personnage ne la modifient. Seule la partie de la carte qui tient dans la fenêtre est visible.

#### Scenario: Carte plus grande que la fenêtre
- **WHEN** la carte est plus grande que la fenêtre
- **THEN** une partie seulement de la carte est visible et la taille des tuiles ne change pas

#### Scenario: Redimensionnement de la fenêtre
- **WHEN** la fenêtre est redimensionnée
- **THEN** la taille des tuiles reste identique et la zone visible s'agrandit ou se réduit

#### Scenario: Aucun zoom demandé
- **WHEN** le joueur ne touche pas à la molette
- **THEN** la taille des tuiles reste celle du départ, quel que soit le personnage suivi

### Requirement: Suivi du personnage
La caméra SHALL suivre le personnage incarné quand il se déplace, de façon qu'il reste visible à tout moment. La carte ne défile que lorsque le personnage s'approche du bord de l'écran. Ce suivi est suspendu pendant la vue libre (voir la capacité `map-navigation`) et reprend dès que le joueur commande de nouveau le personnage.

#### Scenario: Marche vers le bord de l'écran
- **WHEN** le personnage incarné s'approche du bord de l'écran
- **THEN** la carte défile pour le garder à l'écran

#### Scenario: Déplacement au centre
- **WHEN** le personnage incarné se déplace dans la zone centrale de l'écran
- **THEN** la carte ne bouge pas

#### Scenario: Personnage en mode AFK
- **WHEN** le personnage incarné est piloté par l'IA (mode AFK)
- **THEN** la caméra continue de le suivre

#### Scenario: Vue libre
- **WHEN** le joueur a déplacé la vue avec le bouton droit
- **THEN** la caméra ne suit plus le personnage jusqu'à ce que le joueur le commande de nouveau
