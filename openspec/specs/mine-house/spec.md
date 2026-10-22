# mine-house Specification

## Purpose
Définit la mine : le bâtiment accolé à un filon, maison du mineur, et la façon dont le minerai extrait rejoint les ressources du royaume.

## Requirements

### Requirement: La mine est la maison du mineur
Une mine SHALL être un bâtiment contenant un lit, accolé à son filon et accessible par une porte. Le mineur y dort et y rentre chaque soir.

#### Scenario: Nuit du mineur
- **WHEN** la nuit tombe
- **THEN** le mineur IA rejoint le lit de sa mine et s'y couche

### Requirement: Le minerai va dans le contenant de la mine
Chaque unité de minerai extraite SHALL être ajoutée au contenant de la mine, dans la limite de sa capacité (voir la capacité `containers`), à la fin du travail. Le HUD SHALL afficher les totaux des quatre minerais dans tous les contenants.

#### Scenario: Dépôt
- **WHEN** un mineur termine une unité sur le filon d'or
- **THEN** le contenant de la mine d'or contient une unité d'or de plus

#### Scenario: Pas de blocage
- **WHEN** un mineur a déjà extrait des dizaines d'unités et que le contenant de sa mine n'est pas plein
- **THEN** il continue de miner

#### Scenario: Plafond
- **WHEN** le contenant d'une mine contient 200 unités de son minerai
- **THEN** le mineur ne produit plus tant que du minerai n'est pas emporté

### Requirement: Le forgeron consomme le fer de son contenant
Le forgeron SHALL consommer son fer dans le contenant de la forge, qui reçoit le fer de départ puis celui que les transporteurs y apportent depuis les mines. Il manque de fer si ce contenant en manque, même si les mines en contiennent. Son comportement est par ailleurs inchangé.

#### Scenario: Le fer ne s'épuise plus
- **WHEN** le fer de départ est épuisé et qu'un transporteur a apporté du fer d'une mine à la forge
- **THEN** le forgeron continue de forger et le fer du contenant de la forge diminue

#### Scenario: Sans transporteur
- **WHEN** le fer de départ est épuisé, qu'aucun fer n'est apporté et que la mine de fer déborde
- **THEN** le forgeron attend
