## MODIFIED Requirements

### Requirement: Zoom à la molette
La molette SHALL zoomer d'un palier par cran : vers l'avant pour agrandir les tuiles, vers l'arrière pour les réduire. Les paliers SHALL être les multiples entiers de la tuile de base de 16 pixels : ×1, ×2, ×3 et ×4, soit 16, 32, 48 et 64 pixels par tuile, de façon que le dessin reste net (voir la capacité `art-direction`). Le palier de départ est ×2. Le royaume entier SHALL tenir à l'écran au plus petit palier, et une tuile rester bien lisible au plus grand. Le zoom SHALL se faire autour du point de la carte situé sous le curseur, qui reste sous le curseur tant que la carte le permet.

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

#### Scenario: Seulement des échelles entières
- **WHEN** le joueur parcourt tous les paliers de zoom
- **THEN** la taille d'une tuile vaut successivement 16, 32, 48 puis 64 pixels, jamais une valeur intermédiaire
