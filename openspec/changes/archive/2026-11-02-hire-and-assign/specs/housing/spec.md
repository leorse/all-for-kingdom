# housing Specification (delta)

## MODIFIED Requirements

### Requirement: Embauche conditionnée
Embaucher quelqu'un SHALL ne plus dépendre des lits libres : on peut affecter un vagabond à un poste même s'il n'y a pas de place libre. Un habitant peut ne pas avoir de lit ; il ne se repose alors jamais (voir `bed-assignment`). Le jeu compte toujours les places libres.

#### Scenario: Sans place libre
- **WHEN** il n'y a aucune place libre
- **THEN** on peut quand même affecter un vagabond à un poste

#### Scenario: Compter les places
- **WHEN** quelqu'un perd son lit
- **THEN** le nombre de places libres augmente
