# posts-and-roles Specification (delta)

## MODIFIED Requirements

### Requirement: Les bâtiments fournissent des postes
Un poste SHALL être une place de travail d'un métier, tenue par une seule personne à la fois. Un entrepôt fournit des postes de transporteur, une taverne un poste de tavernier, une forge un poste de forgeron par enclume. **Un coffre posé par le roi fournit un poste de bûcheron, de mineur ou de paysan, sans bâtiment.** Le camp fournit des postes d'artisan, en nombre illimité. Les postes se déduisent des bâtiments et des coffres existants.

#### Scenario: Un coffre de bûcheron
- **WHEN** le roi pose un coffre de bûcheron
- **THEN** un poste de bûcheron, libre, apparaît à la tuile du coffre

#### Scenario: Un coffre de paysan
- **WHEN** le roi pose un coffre de paysan
- **THEN** un poste de paysan, libre, apparaît à la tuile du coffre

### Requirement: Un métier à poste ne travaille qu'avec son poste
Au camp, une personne dont le métier a des postes (artisan, transporteur, tavernier, forgeron, **bûcheron, mineur, paysan**) SHALL ne travailler que si elle en tient un.

#### Scenario: Bûcheron sans poste
- **WHEN** un bûcheron remplacé n'a plus de poste
- **THEN** il ne coupe plus rien
