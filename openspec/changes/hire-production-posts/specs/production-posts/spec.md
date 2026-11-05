# production-posts Specification (delta)

## ADDED Requirements

### Requirement: Un coffre posé près de sa matière est un poste
Un bûcheron, un mineur ou un paysan SHALL travailler à un **poste** qui est un **coffre** posé par le roi, sans bâtiment. Le coffre est un objet à part (voir `chests`) qu'on pose volontairement ; **s'il est posé près de la matière que son contenu désigne, il fournit un poste** : un coffre de bois près d'une forêt, un poste de bûcheron ; un coffre d'un minerai (fer, or, cuivre, pierre) près d'un filon de cette nature, un poste de mineur ; un coffre de blé près d'un champ, un poste de paysan. Un coffre = un poste, tenu par une seule personne.

#### Scenario: Un coffre de bois près de la forêt
- **WHEN** le roi pose un coffre de bois à portée d'une tuile de forêt
- **THEN** un poste de bûcheron, libre, apparaît, sans bâtiment

#### Scenario: Un coffre de fer près d'un filon d'or
- **WHEN** le roi pose un coffre de fer à portée du seul filon d'or
- **THEN** aucun poste n'apparaît : le coffre est posé, mais il n'est pas à un filon de fer

#### Scenario: Un coffre de blé près d'un champ
- **WHEN** le roi pose un coffre de blé à portée d'un champ
- **THEN** un poste de paysan, libre, apparaît

### Requirement: La zone de travail est celle du coffre
Un bûcheron SHALL couper les arbres de la forêt la plus proche de son coffre, à portée de travail ; un mineur SHALL frapper le filon de la nature de son coffre le plus proche, à portée ; un paysan SHALL travailler les champs les plus proches de son coffre (voir `farm-surface`). Un coffre plein SHALL arrêter la production de son titulaire quand il ne peut plus rien déposer (voir `chests`).

#### Scenario: Un bûcheron
- **WHEN** un vagabond est affecté au poste d'un coffre de bois
- **THEN** il coupe les arbres autour de ce coffre et y rapporte son bois

### Requirement: Le mineur n'est pas lié à la surface
Le poste d'un mineur SHALL désigner une **source de minerai** dont le filon de surface n'est que le premier cas ; la mine souterraine, plus tard, en sera un autre. Tant qu'elle n'existe pas, aucun poste souterrain n'est proposé.

#### Scenario: Pas de souterrain
- **WHEN** le roi cherche un poste de mineur
- **THEN** seuls des coffres à côté d'un filon de surface sont possibles

### Requirement: Les postes de production sont réservés au camp
Au camp (partie normale), un bûcheron, un mineur ou un paysan sans poste SHALL ne rien faire. Dans le village de départ, ils gardent leurs forêts, filons, champs et coffres fixes.

#### Scenario: Remplacé
- **WHEN** un bûcheron perd son poste
- **THEN** il garde son métier et ne coupe plus rien

#### Scenario: Village
- **WHEN** le village de départ est lancé
- **THEN** ses bûcherons, mineurs et paysans travaillent comme avant
