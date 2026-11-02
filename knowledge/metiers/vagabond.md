---
type: Métier
title: Le vagabond
description: Le vagabond : une personne de passage, sans métier ni lit, qui se promène autour du camp puis repart ; une taverne le retient. En place ; l'embauche reste à faire.
etat_en_jeu: en place (il passe) ; embauche décidée, pas en jeu
tags: [metier, population, vagabond]
sources:
  - resource: crates/afk-core/src/character.rs
    title: vagabond_routine
  - resource: crates/afk-core/src/game.rs
    title: spawn_vagabonds
  - resource: crates/afk-core/src/config.rs
    title: vagabond_first_hours, vagabond_every_hours, vagabond_max, vagabond_stay_hours
---

# Le vagabond

Une personne qui **passe par là**, sans foyer et sans métier. Elle n'est pas un habitant : ce n'est pas encore quelqu'un qu'on peut embaucher.

## En place

- Il **arrive par le bord de la carte** (hors de l'eau), le premier au bout de quelques heures de jeu (`vagabond_first_hours`), puis un autre à intervalle régulier (`vagabond_every_hours`), **sans qu'il faille de taverne**. Il n'y en a jamais plus de `vagabond_max` à la fois.
- Il **se promène** autour du camp, s'arrête un instant, repart ailleurs. Il **n'a ni lit ni heure de sommeil** et ne travaille pas.
- Son temps de séjour écoulé (`vagabond_stay_hours`), il **repart** par le bord le plus proche et quitte le royaume.
- Une **taverne** bâtie le retient : il s'y rend une fois sur deux et son séjour dure quatre fois plus longtemps.
- Il n'est pas compté dans les habitants du HUD, mais on peut l'incarner avec Tab ou le clic.

## Décidé, pas en jeu

- Le roi **s'adresse** à lui (bouton, quand il est proche), puis l'**affecte à un poste ou à un lit** ; un lit n'est plus nécessaire. Près du roi, il **ralentit** toujours. Un vagabond n'a pas d'autre but que d'être embauché. Voir [propriete-et-affectation.md](/regles/propriete-et-affectation.md) (spec `hire-and-assign`).
- Les vagabonds auront des **compétences** (voir [competences.md](/regles/competences.md)) ; des personnes expérimentées ou des chevaliers pourront aussi passer.

## À définir

- Comment on embauche (clic sur le vagabond ? depuis la taverne ?) et ce qui se passe s'il n'y a pas de lit.
- Ce qui fait varier le **rythme d'arrivée** (prospérité, réputation, taverne).
- Un vagabond qu'on n'embauche pas dort-il quelque part, mange-t-il ?
