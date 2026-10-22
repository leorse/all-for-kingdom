# mining Specification

## Purpose
Définit le métier de mineur : comment il extrait du minerai d'un filon, à quelle vitesse, et comment le joueur et l'IA le jouent.

## Requirements

### Requirement: Le mineur est un métier
Le jeu SHALL comporter le métier de mineur, incarnable comme les autres personnages (Tab, chiffres, clic). Seul un mineur SHALL pouvoir miner.

#### Scenario: Un autre métier ne mine pas
- **WHEN** un paysan incarné regarde un filon depuis une case voisine et appuie sur la touche d'action
- **THEN** aucun travail ne démarre

### Requirement: Miner depuis une case voisine
Un mineur SHALL donner ses coups de pioche sur le filon de sa mine depuis une case praticable voisine, en le regardant. Le joueur SHALL lancer le travail avec la touche d'action ; un mouvement ou une seconde pression l'annule, comme pour les autres métiers.

#### Scenario: Le joueur mine
- **WHEN** le mineur incarné se place contre son filon en le regardant et appuie sur la touche d'action
- **THEN** une barre de progression s'affiche et, à la fin, une unité de minerai de la nature du filon est ajoutée au compteur global de cette nature

#### Scenario: Annuler
- **WHEN** le joueur bouge pendant qu'il mine
- **THEN** le travail est annulé et rien n'est déposé

### Requirement: La qualité fixe la durée
Chaque qualité de filon SHALL correspondre à un nombre de coups de pioche par unité de minerai ; la durée d'une unité est ce nombre multiplié par la durée d'un coup. Une roche pure SHALL demander moins de coups qu'une roche impure. Ces valeurs SHALL être des réglages de la configuration.

#### Scenario: Roche pure contre roche impure
- **WHEN** deux mineurs de même forme travaillent l'un sur un filon pur, l'autre sur un filon impur
- **THEN** le premier produit une unité plus vite que le second

### Requirement: Vitesse du joueur et fatigue
Le joueur SHALL miner deux fois plus vite que l'IA. La fatigue SHALL ralentir un mineur comme les autres personnages.

#### Scenario: Joueur et IA
- **WHEN** un mineur incarné et un mineur IA, en pleine forme, extraient une unité du même type de filon
- **THEN** le joueur met la moitié du temps de l'IA

### Requirement: Le mineur IA travaille le jour
Un mineur IA SHALL, dans son horaire de travail, se placer contre son filon, le regarder et miner sans fin ; la nuit il SHALL rentrer dormir dans sa mine.

#### Scenario: Journée sans joueur
- **WHEN** la simulation tourne un jour entier sans personnage incarné
- **THEN** chaque mineur produit du minerai de jour et dort dans sa mine la nuit

### Requirement: Un poste par case voisine
Jusqu'à huit mineurs SHALL pouvoir miner le même filon en même temps, un par case voisine. Deux mineurs ne SHALL pas viser la même case.

#### Scenario: Deux mineurs, un filon
- **WHEN** deux mineurs IA exploitent le même filon
- **THEN** ils se placent sur deux cases différentes et travaillent en même temps
