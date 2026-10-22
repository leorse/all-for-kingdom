## Purpose
Définit comment le joueur parcourt la carte à la souris : déplacer la vue en maintenant le bouton droit, zoomer à la molette, et la vue libre qui en résulte.

## ADDED Requirements

### Requirement: Déplacer la vue avec le bouton droit
Tant que le bouton droit de la souris est maintenu, la carte SHALL suivre le mouvement du curseur : le point de la carte saisi reste sous le curseur. À la relâche, la vue SHALL rester où elle est. La vue SHALL rester bornée aux bords de la carte.

#### Scenario: Tirer la carte
- **WHEN** le joueur maintient le bouton droit et déplace le curseur de 100 pixels vers la gauche
- **THEN** la carte se déplace de 100 pixels vers la gauche à l'écran, et le point saisi reste sous le curseur

#### Scenario: Relâcher
- **WHEN** le joueur relâche le bouton droit
- **THEN** la vue s'arrête et ne bouge plus

#### Scenario: Bord de la carte
- **WHEN** le joueur tire la carte au-delà de son bord
- **THEN** la vue s'arrête au bord de la carte et n'affiche aucune zone hors de la carte

### Requirement: Zoom à la molette
La molette SHALL zoomer d'un palier par cran : vers l'avant pour agrandir les tuiles, vers l'arrière pour les réduire. Les paliers SHALL être des tailles de tuile entières, bornées de façon que le royaume entier tienne à l'écran au plus petit et qu'une tuile reste bien lisible au plus grand. Le zoom SHALL se faire autour du point de la carte situé sous le curseur, qui reste sous le curseur tant que la carte le permet.

#### Scenario: Zoomer
- **WHEN** le joueur tourne la molette d'un cran vers l'avant
- **THEN** les tuiles passent au palier supérieur et le point sous le curseur n'a pas bougé à l'écran

#### Scenario: Dézoomer
- **WHEN** le joueur tourne la molette d'un cran vers l'arrière
- **THEN** les tuiles passent au palier inférieur et une plus grande partie de la carte est visible

#### Scenario: Limites du zoom
- **WHEN** le zoom est déjà au palier maximal et le joueur continue de zoomer, ou au minimal et continue de dézoomer
- **THEN** rien ne change

#### Scenario: Taille de la carte au zoom minimal
- **WHEN** le zoom est au palier minimal dans la fenêtre par défaut
- **THEN** la carte entière est visible, centrée si elle est plus petite que la fenêtre

### Requirement: Vue libre
Dès que le joueur déplace la vue avec le bouton droit, la caméra SHALL cesser de suivre le personnage incarné : elle reste où le joueur l'a mise, même si le personnage bouge ou sort de l'écran. Le suivi SHALL reprendre dès que le joueur commande ce personnage : il le déplace (ZQSD, flèches) ou il incarne un personnage (Tab, 1 à 9, clic, Espace). Zoomer SHALL ne PAS déclencher la vue libre.

#### Scenario: Le personnage sort de la vue
- **WHEN** le joueur a tiré la carte et le personnage incarné marche hors de l'écran
- **THEN** la caméra ne bouge pas

#### Scenario: Reprise en marchant
- **WHEN** le joueur est en vue libre et appuie sur une touche de déplacement
- **THEN** la caméra glisse jusqu'au personnage incarné puis le suit de nouveau

#### Scenario: Reprise en changeant de personnage
- **WHEN** le joueur est en vue libre et incarne un autre personnage (Tab, chiffre, clic ou Espace)
- **THEN** la caméra glisse jusqu'à ce personnage, comme sans vue libre

#### Scenario: Zoomer en suivant
- **WHEN** le joueur zoome à la molette sans avoir déplacé la vue
- **THEN** la caméra continue de suivre le personnage, qui reste visible

### Requirement: Interactions cohérentes avec le zoom
Le clic gauche pour incarner un personnage, la cible de travail et tous les dessins de la carte (parcelles, personnages, jauges, lumière) SHALL correspondre à l'affichage quel que soit le palier de zoom et la position de la vue.

#### Scenario: Clic après zoom
- **WHEN** le joueur a zoomé et clique sur un personnage visible
- **THEN** ce personnage est incarné

#### Scenario: Dessin à tous les paliers
- **WHEN** le joueur parcourt tous les paliers de zoom
- **THEN** la carte, les bâtiments, les personnages et les jauges restent proportionnés, sans trou ni décalage entre les tuiles

### Requirement: Rappel des gestes
Le HUD SHALL indiquer les deux gestes : bouton droit maintenu pour déplacer la vue, molette pour zoomer.

#### Scenario: Lecture du HUD
- **WHEN** la partie est à l'écran
- **THEN** la ligne d'aide en bas de l'écran mentionne le bouton droit et la molette
