# Spec Delta

## Purpose

Définit le forgeron : depuis où et comment il forge à l'enclume, et son apparence animée, premier personnage dessiné selon le gabarit commun `character-sprites`.

## ADDED Requirements

### Requirement: On forge par le haut de l'enclume
Le forgeron SHALL forger uniquement depuis la case au nord de l'enclume (son poste de forge), en regardant vers le bas, vers l'enclume. Il ne SHALL jamais forger depuis une autre case. Cette règle SHALL valoir pour le joueur comme pour l'IA. Chaque enclume du royaume SHALL avoir sa case nord praticable.

#### Scenario: L'IA se place au nord
- **WHEN** le forgeron IA a du fer à forger
- **THEN** il marche jusqu'à la case au nord d'une enclume libre, se tourne vers le bas et forge

#### Scenario: Jamais depuis le côté
- **WHEN** un forgeron, joueur ou IA, forge
- **THEN** il se tient sur la case au nord de l'enclume et regarde vers le bas

### Requirement: Le joueur est conduit à son poste
Quand le joueur incarne le forgeron et se tient à côté de l'enclume en la regardant, depuis n'importe quel côté, l'action « forger » SHALL lui être proposée, comme aujourd'hui. La touche d'action SHALL garder son fonctionnement actuel : un appui lance l'action, et la maintenir enfoncée ne change rien. Si le forgeron n'est pas au poste de forge, cet appui SHALL le faire marcher seul jusqu'au poste par le chemin le plus court, se tourner vers l'enclume et commencer à forger, sans autre intervention du joueur. Pendant ce trajet, une touche de déplacement ou un nouvel appui sur la touche d'action SHALL annuler le trajet, comme ils annulent un travail en cours. Une fois commencée, la forge SHALL se poursuivre et s'annuler comme tout autre travail.

#### Scenario: Forger depuis le côté
- **WHEN** le joueur incarne le forgeron, se tient à l'ouest de l'enclume en la regardant, avec du fer dans la forge, et appuie une fois sur la touche d'action
- **THEN** le forgeron contourne seul l'enclume jusqu'à la case au nord, se tourne vers le bas et forge

#### Scenario: Maintenir la touche ne change rien
- **WHEN** le joueur garde la touche d'action enfoncée au lieu de l'appuyer une fois
- **THEN** le forgeron fait exactement la même chose qu'avec un seul appui

#### Scenario: Reprendre la main en route
- **WHEN** le joueur appuie sur une touche de déplacement, ou de nouveau sur la touche d'action, pendant que le forgeron marche vers son poste
- **THEN** le trajet est annulé, le forgeron ne forge pas et obéit de nouveau au joueur

#### Scenario: Déjà au poste
- **WHEN** le joueur incarne le forgeron sur la case au nord de l'enclume, face en bas, avec du fer, et appuie sur la touche d'action
- **THEN** la forge commence aussitôt

#### Scenario: Rien à forger
- **WHEN** le joueur se tient à côté de l'enclume mais qu'il n'y a plus de fer dans la forge
- **THEN** aucune action « forger » n'est proposée et la touche d'action ne le fait pas bouger

#### Scenario: Poste occupé
- **WHEN** un autre forgeron forge déjà à cette enclume
- **THEN** aucune action « forger » n'est proposée au joueur

#### Scenario: Enclume occupée
- **WHEN** un forgeron forge déjà à l'unique enclume
- **THEN** un second forgeron ne peut pas y forger et ne la vise pas

### Requirement: Un homme forgeron
Le forgeron SHALL être dessiné selon le gabarit `character-sprites`, en s'inspirant de `docs/images/forgeron_sample.jpg`, mais comme un **homme** aux proportions d'adulte et non comme un nain : barbe courte, tablier de cuir sur une chemise aux manches retroussées, bottes. Il SHALL fournir toutes les animations du gabarit.

#### Scenario: Toutes les animations
- **WHEN** on génère la planche du forgeron
- **THEN** elle contient debout et marche dans les 4 directions, repos, sommeil et forge, sans image manquante

### Requirement: Le geste de forge vu de face
L'animation de travail du forgeron SHALL le montrer face à nous, derrière l'enclume : le bras qui tient le marteau se lève au-dessus de la tête puis frappe le métal sur l'enclume, tandis que l'autre main tient la pièce avec une pince. Elle SHALL compter 6 images (marteau bas, lever, en haut, descente, impact, rebond), jouées en boucle tant qu'il forge. L'enclume SHALL être dessinée devant lui et cacher le bas de son corps. Les étincelles et l'éclat du métal chaud SHALL jaillir à l'image d'impact.

#### Scenario: Le bras qui frappe
- **WHEN** le forgeron forge
- **THEN** on voit le marteau monter au-dessus de sa tête puis retomber sur l'enclume, en boucle

#### Scenario: Étincelles au bon moment
- **WHEN** le marteau touche le métal
- **THEN** des étincelles jaillissent de l'enclume à cet instant, et pas pendant que le marteau est levé

#### Scenario: L'enclume devant
- **WHEN** le forgeron forge
- **THEN** l'enclume masque ses jambes et le bas de son tablier

### Requirement: Le forgeron dort dans son lit
Quand il dort, le forgeron SHALL être dessiné couché dans son lit, tête sur l'oreiller et corps sous la couverture, dans l'orientation du lit, avec son animation de respiration.

#### Scenario: Lit tourné
- **WHEN** le forgeron dort dans un lit orienté tête à gauche
- **THEN** il est dessiné couché tête à gauche, sur l'oreiller
