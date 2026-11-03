# crop-choice Specification (delta)

## ADDED Requirements

### Requirement: Seul le paysan choisit la culture
Quand le royaume possède plusieurs graines différentes, le choix de la culture à semer SHALL revenir au seul paysan. Le roi SHALL pouvoir définir une surface, jamais la culture.

#### Scenario: Le roi essaie
- **WHEN** le roi tente de choisir une culture
- **THEN** la demande est refusée

#### Scenario: Le paysan incarné choisit
- **WHEN** le joueur incarne un paysan et que deux graines sont disponibles
- **THEN** une liste lui propose la culture, et ses semis suivants la suivent

### Requirement: Le paysan de l'IA choisit seul
Un paysan que personne n'incarne SHALL semer la culture dont le royaume a le plus de graines, à défaut celle qu'il a cultivée en dernier.

#### Scenario: Plus de graines d'une espèce
- **WHEN** le royaume a davantage de graines d'une espèce que de l'autre
- **THEN** l'IA sème la plus fournie

### Requirement: Pas de choix sans alternative
Tant qu'une seule graine existe (le blé), aucun choix SHALL être proposé ni exigé.

#### Scenario: Le blé seul
- **WHEN** seule la graine de blé est disponible
- **THEN** le paysan sème du blé sans qu'on lui demande rien
