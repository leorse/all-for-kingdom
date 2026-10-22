# mine-house Specification

## Purpose
Définit la mine : le bâtiment accolé à un filon, maison du mineur, et la façon dont le minerai extrait rejoint les ressources du royaume.

## Requirements

### Requirement: La mine est la maison du mineur
Une mine SHALL être un bâtiment contenant un lit, accolé à son filon et accessible par une porte. Le mineur y dort et y rentre chaque soir.

#### Scenario: Nuit du mineur
- **WHEN** la nuit tombe
- **THEN** le mineur IA rejoint le lit de sa mine et s'y couche

### Requirement: Le minerai rejoint le compteur global
Chaque unité de minerai extraite SHALL être ajoutée au compteur global de sa nature (fer, cuivre, or, pierre) à la fin du travail, sans stock local ni limite de capacité. Le HUD SHALL afficher ces quatre compteurs.

#### Scenario: Dépôt
- **WHEN** un mineur termine une unité sur le filon d'or
- **THEN** le compteur d'or augmente de un

#### Scenario: Pas de blocage
- **WHEN** un mineur a déjà extrait des dizaines d'unités
- **THEN** il continue de miner, aucun plafond ne l'arrête

### Requirement: Le forgeron consomme le compteur de fer
Le forgeron SHALL consommer son fer dans le compteur global de fer, qui reçoit à la fois le fer de départ et le fer extrait par les mineurs. Son comportement est par ailleurs inchangé.

#### Scenario: Le fer ne s'épuise plus
- **WHEN** le fer de départ est épuisé et qu'un mineur de fer a extrait du minerai
- **THEN** le forgeron continue de forger et le compteur de fer diminue
