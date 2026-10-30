## ADDED Requirements

### Requirement: L'enclume occupe la moitié basse de sa case
L'enclume SHALL être dessinée avec son socle sur la moitié basse de sa case, le bas du socle au bas de la case. La moitié haute de la case SHALL être praticable par tous les personnages. Un personnage dont les pieds sont plus bas que le haut du socle SHALL être dessiné devant l'enclume, et les autres derrière elle. La recherche de chemin SHALL continuer de traiter la case de l'enclume comme un obstacle. Un personnage qui se trouve dans la moitié libre SHALL pouvoir en repartir à pied, sans traverser le socle.

#### Scenario: Passer à côté, plus bas que le haut du socle
- **WHEN** un personnage se tient à côté de l'enclume, les pieds plus bas que le haut du socle
- **THEN** il est dessiné devant l'enclume, et la corne ne le recouvre pas

#### Scenario: Passer derrière
- **WHEN** un personnage se tient à côté de l'enclume, les pieds plus hauts que le haut du socle
- **THEN** il est dessiné derrière l'enclume

#### Scenario: Se coller au socle par le nord
- **WHEN** le joueur pousse un personnage vers le sud, contre l'enclume, depuis la case au nord
- **THEN** le personnage entre dans la moitié haute de la case et s'arrête, les pieds contre le haut du socle

#### Scenario: Le socle bloque
- **WHEN** le joueur pousse un personnage contre le socle, depuis le sud, l'est ou l'ouest
- **THEN** le personnage ne traverse pas le socle

#### Scenario: L'IA garde la case comme obstacle
- **WHEN** un personnage IA traverse le royaume sans viser cette enclume
- **THEN** son chemin ne passe pas par la case de l'enclume

#### Scenario: Repartir de la moitié libre
- **WHEN** un personnage IA doit partir alors qu'il se trouve dans la moitié libre, vers une destination au sud de l'enclume
- **THEN** il repart par le nord et contourne l'enclume, sans jamais rester bloqué contre le socle

## MODIFIED Requirements

### Requirement: On forge par le haut de l'enclume
Le forgeron SHALL forger uniquement depuis son **poste de forge** : un point de la moitié libre de la case de l'enclume, **centré à l'horizontale sur l'enclume**, **les pieds contre le socle**. Il y SHALL regarder vers le bas, vers l'enclume. Il ne SHALL jamais forger depuis un autre point, même dans la moitié libre. Cette règle SHALL valoir pour le joueur comme pour l'IA. Chaque enclume du royaume SHALL avoir sa case nord praticable, par laquelle on rejoint le poste.

#### Scenario: L'IA se place au nord
- **WHEN** le forgeron IA a du fer à forger
- **THEN** il marche jusqu'au poste de forge d'une enclume libre, se tourne vers le bas et forge

#### Scenario: Jamais depuis le côté
- **WHEN** un forgeron, joueur ou IA, forge
- **THEN** il se tient au poste de forge, au nord du socle, et regarde vers le bas

#### Scenario: Centré sur l'enclume
- **WHEN** un forgeron, joueur ou IA, forge
- **THEN** il est centré à l'horizontale sur l'enclume, les pieds contre le socle, et non décalé vers un côté

#### Scenario: Collé au socle et centré
- **WHEN** le joueur incarne le forgeron, centré et collé au socle, avec du fer, et appuie sur la touche d'action
- **THEN** la forge commence aussitôt, sans qu'il bouge

#### Scenario: Décalé dans la moitié libre
- **WHEN** le joueur incarne le forgeron dans la moitié libre de la case, décalé vers un côté, avec du fer, et appuie sur la touche d'action
- **THEN** il marche seul jusqu'au poste de forge, sans traverser le socle, puis forge

### Requirement: Le geste de forge vu de face
L'animation de travail du forgeron SHALL le montrer face à nous, derrière l'enclume : le bras qui tient le marteau se lève au-dessus de la tête puis frappe le métal sur l'enclume, tandis que l'autre main tient la pièce avec une pince. Elle SHALL compter 6 images (marteau bas, lever, en haut, descente, impact, rebond), jouées en boucle tant qu'il forge. L'enclume SHALL être dessinée devant lui. Ses images de forge SHALL contenir son corps entier, jambes et bottes comprises, à l'identique de l'image debout. Les étincelles et l'éclat du métal chaud SHALL jaillir à l'image d'impact, sur le dessus de l'enclume.

Au poste de forge, le forgeron SHALL être dessiné **à la même hauteur** debout et en train de forger : passer de l'un à l'autre ne SHALL pas le faire sauter vers le bas ou vers le haut, ni donner l'impression qu'il s'assoit. Quand il enchaîne plusieurs outils, incarné ou non, il SHALL rester dans son geste de forge d'un outil au suivant, sans aucune image debout entre les deux.

#### Scenario: Le bras qui frappe
- **WHEN** le forgeron forge
- **THEN** on voit le marteau monter au-dessus de sa tête puis retomber sur le dessus de l'enclume, en boucle

#### Scenario: Étincelles au bon moment
- **WHEN** le marteau touche le métal
- **THEN** des étincelles jaillissent du dessus de l'enclume à cet instant, et pas pendant que le marteau est levé

#### Scenario: L'enclume devant
- **WHEN** le forgeron forge
- **THEN** l'enclume est dessinée devant lui et masque le bas de ses jambes, pieds compris, car ils sont contre le socle

#### Scenario: Un corps entier
- **WHEN** on regarde une image de forge seule
- **THEN** le forgeron y est entier, ses bottes à la même hauteur que sur l'image debout

#### Scenario: Pas de sursaut en commençant
- **WHEN** le forgeron se tient debout au poste de forge, puis commence à forger
- **THEN** sa tête, son torse et ses pieds restent à la même hauteur à l'écran ; seuls ses bras et son marteau changent

#### Scenario: Pas de clignotement entre deux outils
- **WHEN** le forgeron, IA ou incarné, termine un outil et qu'il reste du fer et de la place pour le suivant
- **THEN** il commence le suivant dans le même pas de simulation, et il n'est jamais dessiné debout entre les deux
