# assignment-interface Specification

## Purpose
Définit l'interface du roi pour affecter : l'invite, le menu, la mini-carte avec la liste des métiers, et le panneau des employés.

## ADDED Requirements

### Requirement: Une invite quand on peut parler
Quand le roi incarné est à portée de quelqu'un, l'écran SHALL l'indiquer avec la touche à presser. Quand le roi parle à quelqu'un, un menu SHALL proposer d'affecter à un poste, d'affecter à un lit, ou de ne rien faire.

#### Scenario: L'invite
- **WHEN** le roi est à portée de quelqu'un
- **THEN** un message indique qu'il peut lui parler

#### Scenario: Ne rien faire
- **WHEN** le roi choisit de ne rien faire
- **THEN** la conversation prend fin et la personne reprend ce qu'elle faisait

### Requirement: La mini-carte des postes
Choisir d'affecter à un poste SHALL ouvrir une vue de tout le royaume, très dézoomée. À gauche, une liste des métiers, ceux qui ont des postes libres en premier, chacun avec son nombre de postes libres sur le total. Le jeu continue de tourner derrière.

#### Scenario: L'ordre de la liste
- **WHEN** la mini-carte s'ouvre
- **THEN** les métiers qui ont au moins un poste libre sont avant ceux qui n'en ont pas

### Requirement: Vert pour libre, rouge pour occupé
Choisir un métier dans la liste SHALL mettre en surbrillance ses postes sur la carte : **vert** s'ils sont libres, **rouge** s'ils sont occupés. Un clic sur un poste vert SHALL affecter la personne ; un clic sur un poste rouge SHALL la mettre à sa place et laisser son titulaire sans poste.

#### Scenario: Libre
- **WHEN** le joueur clique un poste vert
- **THEN** la personne y est affectée et la mini-carte se ferme

#### Scenario: Occupé
- **WHEN** le joueur clique un poste rouge
- **THEN** la personne remplace le titulaire, qui n'a plus de poste

### Requirement: La mini-carte des lits
Choisir d'affecter à un lit SHALL ouvrir la même vue, avec les lits en vert quand ils sont libres et en rouge quand ils sont occupés ; un clic attribue le lit (en retirant l'ancien occupant du rouge).

#### Scenario: Lit vert
- **WHEN** le joueur clique un lit vert
- **THEN** la personne a ce lit

### Requirement: Fermer sans rien changer
Échap ou un clic droit SHALL fermer la mini-carte sans rien affecter ; la personne reprend alors ce qu'elle faisait.

#### Scenario: Échap
- **WHEN** le joueur ferme la mini-carte sans cliquer de poste
- **THEN** rien n'est affecté et la personne reprend

### Requirement: La liste des employés
Le roi SHALL disposer d'une liste de ses employés : pour chacun son nom, son métier, son poste ou son absence de poste, et un symbole quand il a un lit. Les vagabonds n'en font pas partie.

#### Scenario: La liste
- **WHEN** le roi ouvre la liste
- **THEN** chaque habitant y figure avec son métier et, s'il en a un, le symbole du lit

#### Scenario: Sans poste
- **WHEN** quelqu'un n'a plus de poste
- **THEN** la liste l'indique clairement
