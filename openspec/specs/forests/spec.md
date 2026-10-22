# forests Specification

## Purpose
Définit les forêts : des zones d'arbres que l'on peut abattre, qui repoussent avec le temps et peuvent donc s'épuiser si on les exploite trop vite.

## Requirements

### Requirement: Les forêts sont des zones d'arbres abattables
Le royaume SHALL contenir des forêts, zones définies par le scénario. Les arbres situés dans une forêt SHALL pouvoir être abattus ; les arbres isolés hors forêt SHALL rester du décor infranchissable, impossible à couper.

#### Scenario: Arbre de forêt
- **WHEN** un bûcheron regarde un arbre à l'intérieur de sa forêt depuis une case voisine
- **THEN** il peut l'abattre

#### Scenario: Arbre isolé
- **WHEN** un bûcheron regarde un arbre situé hors de toute forêt
- **THEN** aucun travail ne peut démarrer

### Requirement: Une forêt reste praticable
Une forêt SHALL laisser des passages : chaque arbre d'une forêt du royaume de départ SHALL toucher, par un côté, une case praticable ou un autre arbre dont une case voisine l'est, de façon que les bûcherons puissent atteindre la lisière de la forêt à pied.

#### Scenario: Atteindre la forêt
- **WHEN** on cherche un chemin depuis le château jusqu'à la lisière de chaque forêt
- **THEN** il existe et passe par des couloirs sans arbre

### Requirement: Un arbre coupé laisse une pousse
Un arbre abattu SHALL disparaître et laisser à sa place une pousse, praticable, qui ne peut pas être coupée.

#### Scenario: Après l'abattage
- **WHEN** un arbre vient d'être abattu
- **THEN** sa case est une pousse sur laquelle on peut marcher et qu'aucun bûcheron ne peut couper

### Requirement: Une pousse redevient un arbre
Une pousse SHALL redevenir un arbre une fois écoulé un délai exprimé en heures de jeu (réglage de la configuration), quelle que soit la vitesse de la simulation. Si un personnage se tient sur la case à ce moment-là, l'arbre SHALL attendre qu'elle soit libre.

#### Scenario: Repousse
- **WHEN** le délai de repousse s'est écoulé après l'abattage
- **THEN** la case redevient un arbre et peut de nouveau être abattue

#### Scenario: Case occupée
- **WHEN** le délai est écoulé alors qu'un personnage se tient sur la pousse
- **THEN** l'arbre ne repousse que lorsque la case est libre, sans jamais emprisonner le personnage

### Requirement: Une forêt peut s'épuiser
Une forêt exploitée plus vite que ses arbres ne repoussent SHALL se vider : il n'y a pas d'arbre créé ailleurs ni de repousse accélérée.

#### Scenario: Forêt sur-exploitée
- **WHEN** des bûcherons abattent tous les arbres d'une forêt avant que la première pousse ne repousse
- **THEN** la forêt n'a plus aucun arbre abattable jusqu'à la repousse suivante
