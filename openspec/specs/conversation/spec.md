# conversation Specification

## Purpose
Définit le roi qui s'adresse à quelqu'un : l'interruption de ce que la personne faisait, sa reprise exacte, et le ralentissement des vagabonds près du roi.

## Requirements

### Requirement: S'adresser à quelqu'un
Quand le joueur incarne le roi et qu'une personne (autre que le roi) se trouve à portée de parole, une commande SHALL lui permettre de s'adresser à elle : la personne la plus proche est choisie. **Une personne endormie peut être abordée : lui parler la réveille**, sans que le roi ait à attendre son réveil. Le jeu SHALL indiquer au joueur qu'il peut parler quand quelqu'un est à portée.

#### Scenario: À portée
- **WHEN** le roi incarné est à portée de quelqu'un d'éveillé et demande à lui parler
- **THEN** cette personne est en conversation avec le roi

#### Scenario: Trop loin
- **WHEN** personne n'est à portée
- **THEN** la demande n'a aucun effet

#### Scenario: Un dormeur
- **WHEN** le roi parle à quelqu'un qui dort
- **THEN** il se réveille aussitôt et la conversation commence

#### Scenario: Réveillé pour rien
- **WHEN** le roi réveille quelqu'un puis met fin à la conversation sans rien décider, et que c'est encore son heure de dormir
- **THEN** il retourne se coucher dans son lit, s'il en a un

### Requirement: Elle arrête tout et se tourne vers le roi
Une personne en conversation SHALL cesser de marcher, de travailler et de porter quoi que ce soit vers un but, et se tourner vers le roi. Sa fatigue continue de suivre le temps ; une personne réveillée ne se rendort pas pendant la conversation.

#### Scenario: Elle travaillait
- **WHEN** le roi parle à quelqu'un qui est en plein travail
- **THEN** le travail n'avance plus, la personne est immobile

### Requirement: Elle reprend là où elle en était
Si le roi met fin à la conversation sans rien décider, la personne SHALL reprendre exactement ce qu'elle faisait : même travail avec la même progression, même cargaison, même trajet.

#### Scenario: Reprise
- **WHEN** le roi parle à un porteur chargé, puis met fin à la conversation
- **THEN** le porteur a toujours sa cargaison et poursuit son voyage

#### Scenario: Le travail garde sa progression
- **WHEN** quelqu'un est à mi-travail au moment où on lui parle, puis qu'on le laisse
- **THEN** son travail reprend à mi-chemin et non à zéro

### Requirement: Un seul interlocuteur, et seulement le roi
Le roi SHALL parler à une seule personne à la fois. Seul un joueur qui incarne le roi peut parler, et le roi ne peut pas se parler à lui-même.

#### Scenario: Un artisan incarné
- **WHEN** le joueur incarne un artisan et demande à parler
- **THEN** rien ne se passe

### Requirement: S'éloigner met fin à la conversation
Si le roi s'éloigne au-delà de la portée de parole, la conversation SHALL prendre fin et la personne reprendre ce qu'elle faisait, sauf si le roi est en train d'affecter quelqu'un (menu ou mini-carte ouverts).

#### Scenario: Le roi s'éloigne
- **WHEN** le roi s'éloigne de la personne sans menu ouvert
- **THEN** elle reprend son activité

### Requirement: Un vagabond ralentit près du roi, quoi qu'il arrive
Un vagabond à moins d'une distance donnée du roi incarné SHALL se déplacer plus lentement, même si le roi ne lui parle pas. Les autres personnes SHALL ne subir aucun ralentissement ni interruption du seul fait que le roi est près d'elles.

#### Scenario: Le vagabond
- **WHEN** le roi incarné est près d'un vagabond sans lui parler
- **THEN** le vagabond avance plus lentement qu'à l'ordinaire

#### Scenario: Les autres
- **WHEN** le roi passe près d'un forgeron au travail sans lui parler
- **THEN** le forgeron continue son travail à son rythme

### Requirement: Le temps ne s'arrête pas
Parler et affecter SHALL ne jamais mettre le jeu en pause ni changer sa vitesse.

#### Scenario: Pendant la conversation
- **WHEN** le roi parle à quelqu'un
- **THEN** le royaume continue de tourner
