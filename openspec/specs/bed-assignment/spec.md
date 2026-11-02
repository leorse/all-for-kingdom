# bed-assignment Specification

## Purpose
Définit l'attribution des lits (automatique et manuelle) et ce qui arrive à quelqu'un qui n'en a pas.

## Requirements

### Requirement: Le lit le plus proche du poste
Une personne qui tient un poste et n'a pas de lit SHALL prendre automatiquement le lit libre le plus proche de son poste. Cette attribution SHALL se refaire tant qu'elle n'a pas de lit : un lit qui apparaît (maison finie) va au premier qui en manque.

#### Scenario: Plusieurs lits libres
- **WHEN** une personne reçoit un poste et que plusieurs lits sont libres
- **THEN** elle prend celui qui est le plus près de ce poste

#### Scenario: Un lit apparaît plus tard
- **WHEN** quelqu'un avec un poste n'a pas de lit, puis une maison est construite
- **THEN** il prend un lit de cette maison

### Requirement: Attribuer un lit à la main
Le roi SHALL pouvoir attribuer un lit à la personne à qui il parle, libre ou occupé. Si le lit était occupé, son occupant le perd (et en reprend un automatiquement s'il a un poste et qu'un lit est libre). Une personne peut ne pas avoir de poste mais avoir un lit.

#### Scenario: Lit occupé
- **WHEN** le roi attribue à A un lit qui était à B
- **THEN** A a ce lit et B n'en a plus

#### Scenario: Un lit sans poste
- **WHEN** le roi attribue un lit à quelqu'un qui n'a pas de poste
- **THEN** il dort dans ce lit et ne fait rien le jour

### Requirement: Le roi s'attribue n'importe quel lit
Le roi SHALL pouvoir s'attribuer lui-même n'importe quel lit, libre ou occupé, dès qu'il s'en approche (à portée de `talk_range`), sans avoir à parler à personne. Si le lit était occupé, son occupant le perd (et en reprend un automatiquement s'il a un poste et qu'un lit est libre). Le lit que le roi quitte devient libre. Le cœur valide : le joueur incarne un roi et le lit est à portée.

#### Scenario: Lit libre
- **WHEN** le roi est près d'un lit libre et se l'attribue
- **THEN** il dort désormais dans ce lit et son ancien lit est libre

#### Scenario: Lit occupé
- **WHEN** le roi se l'attribue un lit qui était à B
- **THEN** le roi a ce lit et B n'en a plus

#### Scenario: Trop loin
- **WHEN** le roi est hors de portée du lit
- **THEN** rien ne change

### Requirement: Sans lit, on ne se repose jamais
Une personne sans lit SHALL ne jamais dormir : sa fatigue monte jusqu'au maximum, et son efficacité baisse comme pour n'importe qui de fatigué. Elle continue de travailler si elle a un poste. L'absence de lit n'empêche pas d'embaucher.

#### Scenario: Pas de lit
- **WHEN** quelqu'un avec un poste n'a aucun lit pendant plusieurs jours
- **THEN** il ne dort pas, sa fatigue atteint le maximum et il travaille moins vite

### Requirement: Les lits libres se comptent toujours
Le nombre de lits libres SHALL rester la différence entre les lits et les personnes qui en ont un ; les lits du camp en font partie.

#### Scenario: Lits libres
- **WHEN** quelqu'un perd son lit
- **THEN** le nombre de lits libres augmente de un
