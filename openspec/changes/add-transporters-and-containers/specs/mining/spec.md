## MODIFIED Requirements

### Requirement: Miner depuis une case voisine
Un mineur SHALL donner ses coups de pioche sur le filon de sa mine depuis une case praticable voisine, en le regardant, tant que le contenant de sa mine peut recevoir une unité de plus. Le joueur SHALL lancer le travail avec la touche d'action ; un mouvement ou une seconde pression l'annule, comme pour les autres métiers.

#### Scenario: Le joueur mine
- **WHEN** le mineur incarné se place contre son filon en le regardant et appuie sur la touche d'action
- **THEN** une barre de progression s'affiche et, à la fin, une unité de minerai de la nature du filon est ajoutée au contenant de sa mine

#### Scenario: Annuler
- **WHEN** le joueur bouge pendant qu'il mine
- **THEN** le travail est annulé et rien n'est déposé

#### Scenario: Contenant plein
- **WHEN** le joueur appuie sur la touche d'action devant son filon alors que le contenant de sa mine est plein de cette ressource
- **THEN** aucun travail ne démarre

### Requirement: Le mineur IA travaille le jour
Un mineur IA SHALL, dans son horaire de travail, se placer contre son filon, le regarder et miner tant que le contenant de sa mine peut recevoir du minerai ; quand il est plein il SHALL attendre, et reprendre dès qu'une place se libère. La nuit il SHALL rentrer dormir dans sa mine.

#### Scenario: Journée sans joueur
- **WHEN** la simulation tourne un jour entier sans personnage incarné
- **THEN** chaque mineur produit du minerai de jour tant que son contenant n'est pas plein, et dort dans sa mine la nuit

#### Scenario: Mine pleine
- **WHEN** le contenant d'une mine atteint 200 unités
- **THEN** le mineur IA cesse de miner et attend
