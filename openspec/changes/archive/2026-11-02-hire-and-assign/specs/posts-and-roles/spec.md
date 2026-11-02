# posts-and-roles Specification

## Purpose
Définit les métiers, les postes de travail que fournissent les bâtiments, et l'affectation, le remplacement et la réaffectation des personnes.

## ADDED Requirements

### Requirement: Métier, poste et lit sont trois choses distinctes
Chaque personne SHALL avoir un métier (rôle), éventuellement un poste de travail, éventuellement un lit. Le métier reste à la personne même quand elle n'a plus de poste.

#### Scenario: Remplacé
- **WHEN** quelqu'un perd son poste
- **THEN** il garde son métier, sans poste

### Requirement: Les bâtiments fournissent des postes
Un poste SHALL être une place de travail d'un métier dans un bâtiment, tenue par une seule personne à la fois. Un entrepôt fournit des postes de transporteur, une taverne un poste de tavernier, **une forge un poste de forgeron par enclume**. Le camp fournit des postes d'artisan, en nombre illimité. Les postes se déduisent des bâtiments existants.

#### Scenario: Un entrepôt
- **WHEN** un entrepôt est construit
- **THEN** de nouveaux postes de transporteur, libres, apparaissent

#### Scenario: Une forge
- **WHEN** une forge est construite
- **THEN** un poste de forgeron, libre, apparaît à chacune de ses enclumes

#### Scenario: Une taverne
- **WHEN** une taverne est construite
- **THEN** un poste de tavernier, libre, apparaît

### Requirement: Affecter à un poste libre
Le roi SHALL pouvoir affecter la personne à qui il parle à un poste libre. Elle prend le métier du poste, le tient, et va y travailler. Un vagabond ainsi affecté cesse d'être vagabond : il ne repart plus et compte parmi les habitants.

#### Scenario: Un vagabond transporteur
- **WHEN** le roi affecte un vagabond à un poste de transporteur libre
- **THEN** il devient transporteur, tient ce poste, et se met à transporter

### Requirement: Remplacer un titulaire
Le roi SHALL pouvoir affecter quelqu'un à un poste **occupé**. Le titulaire SHALL perdre son poste, garder son métier et ne plus rien faire ; le nouveau le tient.

#### Scenario: Remplacement
- **WHEN** le roi affecte A à un poste tenu par B
- **THEN** A tient le poste, B garde son métier mais n'a plus de poste et ne travaille plus

### Requirement: Réaffecter
Le roi SHALL pouvoir réaffecter n'importe quel habitant, y compris un artisan, sauf lui-même. Le poste que la personne quitte devient libre.

#### Scenario: Changer de poste
- **WHEN** le roi affecte quelqu'un qui tient un poste à un autre poste libre
- **THEN** le premier poste est libre et le second tenu

#### Scenario: Le roi
- **WHEN** le roi cherche à s'affecter lui-même
- **THEN** rien ne se passe

### Requirement: Sans poste, on ne travaille pas
Au camp, une personne dont le métier a des postes SHALL ne travailler que si elle en tient un ; sinon elle ne fait rien (elle reste près de son lit, ou du camp). Dans le village de départ, le comportement d'avant SHALL rester inchangé.

#### Scenario: Sans poste
- **WHEN** une personne n'a pas de poste au camp
- **THEN** elle n'exerce pas son métier

#### Scenario: Le village
- **WHEN** on joue dans le village de départ
- **THEN** chaque métier travaille comme avant, sans dépendre d'un poste

### Requirement: Embaucher n'exige pas de lit
Affecter quelqu'un à un poste SHALL être possible même s'il n'y a aucun lit libre.

#### Scenario: Aucun lit
- **WHEN** il n'y a pas de lit libre et que le roi affecte un vagabond à un poste
- **THEN** l'affectation a lieu
