## Purpose
Définit les filons de minerai posés sur la carte : leur forme, leurs quatre natures et leur qualité, tous visibles sans avoir à miner.

## ADDED Requirements

### Requirement: Un filon est un bloc de 2×2 tuiles
Un filon SHALL occuper un bloc de 2×2 tuiles sur la carte. Ses tuiles SHALL être infranchissables à pied.

#### Scenario: Le filon bloque le passage
- **WHEN** un personnage tente de traverser une tuile de filon
- **THEN** il est arrêté comme par un mur et les chemins le contournent

#### Scenario: Huit cases d'accès
- **WHEN** un filon est posé en terrain dégagé
- **THEN** huit cases praticables le touchent par un côté, deux par face

### Requirement: Quatre natures de filon
Chaque filon SHALL être de l'une de quatre natures : fer, cuivre, or ou pierre. La nature SHALL se reconnaître à sa couleur : gris pour le fer, cuivré pour le cuivre, doré pour l'or, une teinte de pierre distincte du mur des bâtiments pour la pierre.

#### Scenario: Reconnaître une nature
- **WHEN** les filons du royaume de départ (fer, or, pierre) sont affichés
- **THEN** leurs couleurs sont distinctes les unes des autres

### Requirement: Qualité visible par des points
Chaque filon SHALL avoir une qualité parmi trois niveaux (pure, moyenne, impure). Elle SHALL être indiquée par des points sur ses tuiles, d'autant plus nombreux que la roche est impure ; une roche pure SHALL avoir le moins de points.

#### Scenario: Comparer deux filons
- **WHEN** deux filons de même nature et de qualités différentes sont affichés
- **THEN** le plus impur porte visiblement plus de points que le plus pur

### Requirement: Un filon ne s'épuise jamais
Un filon SHALL fournir du minerai sans limite : extraire du minerai ne le modifie ni sur la carte ni dans sa qualité.

#### Scenario: Extraction répétée
- **WHEN** des centaines d'unités de minerai sont extraites du même filon
- **THEN** le filon est identique à l'écran et reste exploitable
