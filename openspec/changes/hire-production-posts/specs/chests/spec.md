# chests Specification (delta)

## ADDED Requirements

### Requirement: Un coffre est un objet qu'on pose volontairement
Le roi SHALL pouvoir poser un coffre sur une tuile libre : un objet à part, jamais posé automatiquement avec un champ, une forêt ou un filon. La pose est immédiate et payée en bois dans le coffre du camp ; elle est refusée sur une tuile occupée (fondation, tas de matériaux, personne dessus, terrain solide) ou si le bois manque. Un coffre est un obstacle, auquel on accède depuis une case voisine.

#### Scenario: Pose
- **WHEN** le roi pose un coffre sur de l'herbe libre avec assez de bois au camp
- **THEN** le coffre apparaît et le coût est retiré du coffre du camp

#### Scenario: Rien n'est posé tout seul
- **WHEN** le roi trace un champ
- **THEN** aucun coffre n'apparaît

### Requirement: Un coffre, une seule ressource
Chaque coffre SHALL être consacré à **une seule ressource**, choisie à la pose (bois, pierre, fer, or, cuivre, blé…), et ne contenir que celle-là, jusqu'à **200 unités**. L'**entrepôt** et le **coffre du camp** font exception (voir plus bas).

#### Scenario: Un coffre de bois
- **WHEN** un coffre est consacré au bois
- **THEN** il ne reçoit ni pierre ni blé

#### Scenario: Plein
- **WHEN** un coffre contient 200 unités de sa ressource
- **THEN** plus aucune unité n'y est ajoutée

### Requirement: Chacun apporte sa ressource au coffre le plus proche
Quand personne n'est incarné, tout personnage qui produit une ressource (bûcheron, mineur, paysan…) SHALL la **porter lui-même** et la déposer dans le **coffre le plus proche qui l'accepte** : un coffre de cette ressource qui n'est pas plein, ou l'entrepôt (voir plus bas). Il ne dépose jamais dans un coffre d'une autre ressource. Sans coffre qui l'accepte, il garde sa cargaison et attend.

#### Scenario: Le coffre le plus proche
- **WHEN** un bûcheron porte du bois et que deux coffres de bois sont à portée de marche
- **THEN** il dépose dans le plus proche

#### Scenario: Le mauvais coffre
- **WHEN** le coffre le plus proche est un coffre de pierre
- **THEN** il l'ignore et va au coffre de bois suivant

#### Scenario: Tout est plein
- **WHEN** aucun coffre ne peut recevoir sa ressource
- **THEN** il attend avec sa cargaison

### Requirement: L'entrepôt a de grands coffres à l'intérieur
Un entrepôt SHALL n'avoir **aucun coffre à l'extérieur** : il contient de **grands coffres à l'intérieur**. Chaque grand coffre SHALL accueillir **200 ressources, de n'importe quel type, mélangés**, sans limite par type : seule la somme compte. Le nombre de grands coffres d'un entrepôt est un réglage (à calibrer).

#### Scenario: Un mélange
- **WHEN** un grand coffre contient 120 bois et 80 pierre
- **THEN** il est plein : il ne reçoit plus rien

#### Scenario: Aucun coffre dehors
- **WHEN** on regarde l'extérieur d'un entrepôt bâti
- **THEN** aucun coffre n'y est posé

### Requirement: Toucher la porte de l'entrepôt suffit
Un personnage (transporteur, producteur) qui dépose ou prend dans un entrepôt SHALL pouvoir le faire dès qu'il **touche la porte**, sans entrer. Le dépôt et le prélèvement se font alors dans les grands coffres, sans trajet à l'intérieur.

#### Scenario: Un transporteur à la porte
- **WHEN** un transporteur arrive à la porte d'un entrepôt avec du bois
- **THEN** le bois est déposé dans un grand coffre qui a de la place, et il repart sans être entré

#### Scenario: Prélèvement
- **WHEN** un transporteur arrive à la porte pour prendre de la pierre que l'entrepôt contient
- **THEN** il en prend et repart

### Requirement: Le coffre du camp ne change pas
Le coffre du camp SHALL continuer à contenir toutes les ressources (100 de chacune au plus) tant que le camp existe.

#### Scenario: Le pécule
- **WHEN** la partie démarre
- **THEN** le coffre du camp contient bois, pierre et graines
