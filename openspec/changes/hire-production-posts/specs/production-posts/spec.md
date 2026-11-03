# production-posts Specification (delta)

## ADDED Requirements

### Requirement: Un coffre suffit pour un poste de production
Un bûcheron, un mineur ou un paysan SHALL travailler à un **poste** qui est un **coffre** posé par le roi, sans bâtiment. Un coffre est un poste, tenu par une seule personne.

#### Scenario: Pas de bâtiment
- **WHEN** le roi pose un coffre de bûcheron au bord d'une forêt
- **THEN** un poste de bûcheron, libre, apparaît, sans qu'il y ait de bâtiment

### Requirement: Poser un coffre-poste
Le roi SHALL pouvoir poser un coffre de bûcheron, de mineur ou de paysan sur une tuile libre, à portée de la bonne matière : une tuile de forêt, une case d'accès d'un filon, un champ (ou la place d'un champ). La pose SHALL être immédiate et payée tout de suite en bois dans le coffre du camp. Elle SHALL être refusée si la tuile est occupée (fondation, tas de matériaux, personne dessus), hors de portée de la matière, ou si le bois manque.

#### Scenario: Près d'une forêt
- **WHEN** le roi pose un coffre de bûcheron à moins de la portée d'une tuile de forêt
- **THEN** le coffre est posé et le coût est retiré du coffre du camp

#### Scenario: Loin de la matière
- **WHEN** le roi pose un coffre de mineur sans aucun filon à portée
- **THEN** la pose est refusée et la carte dit pourquoi

#### Scenario: Sans bois
- **WHEN** le coffre du camp n'a pas assez de bois
- **THEN** la pose est refusée (carte rouge)

### Requirement: La zone de travail est celle du coffre
Un bûcheron SHALL couper les arbres de la forêt la plus proche de son coffre, à portée de travail ; un mineur SHALL frapper le filon le plus proche à portée de son coffre ; ils déposent dans **leur** coffre. Un coffre plein SHALL arrêter la production de son titulaire.

#### Scenario: Un bûcheron
- **WHEN** un vagabond est affecté au poste d'un coffre de bûcheron
- **THEN** il coupe les arbres autour de ce coffre et y dépose le bois

#### Scenario: Coffre plein
- **WHEN** le coffre du mineur est plein
- **THEN** il ne mine plus tant qu'un transporteur n'a pas vidé le coffre

### Requirement: Le coffre du mineur ne contient qu'un minerai
Le coffre d'un poste de mineur SHALL prendre la nature du filon visé à la pose (fer, or, cuivre, pierre) et ne recevoir que ce minerai.

#### Scenario: Un filon d'or
- **WHEN** le roi pose un coffre de mineur contre le filon d'or
- **THEN** ce coffre ne reçoit que de l'or

### Requirement: Le mineur n'est pas lié à la surface
Le poste d'un mineur SHALL désigner une **source de minerai** dont la surface n'est que le premier cas ; la mine souterraine, plus tard, en sera un autre. Tant qu'elle n'existe pas, aucun poste souterrain n'est proposé.

#### Scenario: Pas de souterrain
- **WHEN** le roi cherche un poste de mineur
- **THEN** seuls des coffres à côté d'un filon de surface sont possibles

### Requirement: Les postes de production sont réservés au camp
Au camp (partie normale), un bûcheron, un mineur ou un paysan sans poste SHALL ne rien faire. Dans le village de départ, ils gardent leurs forêts, filons et champs fixes.

#### Scenario: Remplacé
- **WHEN** un bûcheron perd son poste
- **THEN** il garde son métier et ne coupe plus rien

#### Scenario: Village
- **WHEN** le village de départ est lancé
- **THEN** ses bûcherons, mineurs et paysans travaillent comme avant
