# road-movement Specification

## Purpose
Définit comment les personnages choisissent leur trajet : en suivant les routes ou par le plus court chemin, et dans quelles situations (travail, coucher, fuite d'un voleur, poursuite des gardes). Les déplacements ne se limitent pas à l'horizontal et au vertical.

## Requirements

### Requirement: Les habitants suivent les routes
Un habitant qui se rend quelque part par lui-même (travail, livraison, ronde, promenade) SHALL suivre les routes : entre deux trajets de même longueur, ou de longueurs proches, il SHALL préférer celui qui passe le plus par des routes. Le surcoût de l'herbe par rapport à la route SHALL être un réglage de la configuration. Un habitant ne SHALL pas faire de détour disproportionné pour rester sur la route.

#### Scenario: Route parallèle
- **WHEN** un habitant doit aller d'un point à un autre et qu'une route longe le trajet direct à quelques cases près
- **THEN** il rejoint la route, la suit, et la quitte seulement près de son but

#### Scenario: Détour trop long
- **WHEN** la route ne mène au but que par un détour plus long que le surcoût de l'herbe ne le justifie
- **THEN** il traverse l'herbe

### Requirement: Quitter la route pour atteindre un but
Un but hors des routes (coffre, champ, filon, arbre, poste de travail) SHALL rester atteignable : le personnage, transporteur compris, quitte la route pour la portion du trajet qui y mène. Il ne quitte pas la route plus tôt que nécessaire.

#### Scenario: Coffre au bord du champ
- **WHEN** un transporteur doit prendre une cargaison dans un coffre éloigné de toute route
- **THEN** il suit les routes jusqu'à leur point le plus proche, puis traverse l'herbe jusqu'à une case voisine du coffre

#### Scenario: But sur la route
- **WHEN** le but est sur une route
- **THEN** le trajet tient entièrement sur les routes

### Requirement: Déplacements en diagonale
Les personnages SHALL pouvoir se déplacer en diagonale : un trajet ne se limite pas à des segments horizontaux et verticaux. Un personnage ne SHALL pas couper le coin d'un obstacle (mur, eau, arbre, coffre, meuble) : une diagonale n'est possible que si les deux cases de côté sont praticables.

#### Scenario: Trajet en diagonale
- **WHEN** un personnage traverse de l'herbe libre vers un but situé à 5 cases à l'est et 5 cases au sud
- **THEN** il avance en diagonale et non en escalier

#### Scenario: Coin d'un mur
- **WHEN** une diagonale passerait entre deux obstacles qui se touchent par le coin, ou frôlerait le coin d'un mur
- **THEN** le personnage ne la prend pas et contourne l'obstacle

### Requirement: Trajectoires courbes
La trajectoire réellement parcourue SHALL être lissée : aux changements de direction, le personnage arrondit le virage au lieu de pivoter sur place sur le centre d'une case. Le lissage SHALL rester proche du trajet choisi (il suit la route, ne coupe pas à travers un obstacle) et SHALL se terminer exactement sur le but. Le personnage regarde dans le sens où il avance.

#### Scenario: Virage arrondi
- **WHEN** un transporteur suit une route horizontale puis doit monter vers un coffre
- **THEN** son tracé à l'écran est une courbe continue sans angle vif, et non un coude à angle droit ou à 45°

#### Scenario: Pas de coupe à travers un obstacle
- **WHEN** le virage arrondi passerait dans un mur, un coffre ou de l'eau
- **THEN** la courbe est resserrée jusqu'à rester sur des cases praticables

#### Scenario: Arrivée exacte
- **WHEN** le personnage termine son trajet
- **THEN** il s'arrête sur la case but, avec le même résultat qu'un trajet non lissé

### Requirement: Aller se coucher par les routes
Une personne qui part se coucher (heure du repos ou épuisement) SHALL emprunter les routes, comme pour un trajet ordinaire.

#### Scenario: Retour à la maison le soir
- **WHEN** la nuit tombe et qu'un habitant est loin de son lit
- **THEN** il rejoint la route puis la suit jusqu'à sa maison

### Requirement: Le voleur fuit par le plus court chemin
Un voleur qui a volé quelque chose SHALL rejoindre la sortie du château par le plus court chemin en distance, sans tenir compte des routes (herbe et route comptent de la même façon).

#### Scenario: Fuite en ligne
- **WHEN** un voleur chargé de son butin quitte le château et qu'une route ferait un détour
- **THEN** il coupe à travers l'herbe par le trajet le plus court

#### Scenario: Obstacles
- **WHEN** un mur ou de l'eau barre le trajet le plus direct
- **THEN** il passe par le plus court chemin praticable

### Requirement: Les gardes poursuivent hors des routes
Un garde qui poursuit un voleur SHALL prendre le plus court chemin vers lui, sans suivre les routes. Quand le voleur n'est plus poursuivi, le garde reprend ses déplacements ordinaires le long des routes.

#### Scenario: Poursuite
- **WHEN** un garde poursuit un voleur à travers un champ
- **THEN** il traverse le champ en ligne la plus courte, sans repasser par la route

#### Scenario: Retour à la ronde
- **WHEN** la poursuite se termine
- **THEN** le garde rejoint sa ronde en suivant de nouveau les routes

### Requirement: Un but inaccessible reste inaccessible
Un trajet SHALL toujours mener à un but praticable et joignable à pied ; si aucun trajet n'existe dans un mode, le personnage SHALL se comporter comme aujourd'hui pour un but inaccessible (il attend ou renonce).

#### Scenario: But isolé
- **WHEN** le but est entouré d'obstacles
- **THEN** aucun trajet n'est suivi, et le personnage attend ou choisit un autre but
