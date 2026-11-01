---
type: Règle
title: Propriété et affectation
description: Un bâtiment appartient à quelqu'un ; on n'embauche que s'il y a un lit ; le roi attribue les lits, puis le métier depuis le bâtiment de travail. Décidé, pas encore en jeu.
etat_en_jeu: décidé (seul le compte des places libres est en place)
tags: [regle, population, roi, affectation]
sources:
  - resource: crates/afk-core/src/game.rs
    title: free_places, can_hire (places libres et embauche)
---

# Propriété et affectation

Règles **décidées, pas encore dans le jeu**. En place aujourd'hui : le jeu compte les places libres (`free_places`) et sait si l'on peut embaucher (`can_hire`). Tout le reste est à faire.

## Principe — décidé

**Un bâtiment appartient à quelqu'un.** Le roi choisit le propriétaire dans une interface :

- dans une maison **à un lit** : le propriétaire ;
- dans une maison **à plusieurs lits** : le **locataire de chaque lit**.

## Au départ — décidé pour l'instant

Les **trois lits du camp de base** sont ceux du roi et des deux artisans (voir [depart-et-camp.md](/regles/depart-et-camp.md)).

## L'embauche — décidé

- Quelqu'un (un vagabond, par exemple) **n'est embauché que s'il y a un lit** pour lui : une **place libre** quelque part dans le royaume.
- Une personne **embauchée n'a pas de poste** : ni lit attribué, ni métier, tant que le roi ne lui en a pas donné.

## Les deux affectations — décidé

Il y a deux choses distinctes :

1. **Le lit.** Le roi choisit **à qui le lit appartient**, parmi les personnes embauchées **qui n'ont pas encore d'affectation**.
2. **Le métier.** Depuis un **bâtiment de travail** (une forge, par exemple), le roi choisit quelqu'un **parmi ceux qui n'ont pas de métier** et l'embauche **comme forgeron** (ou le métier de ce bâtiment).

Un forgeron a donc un lit dans une maison et la forge pour poste : deux désignations différentes.

## Ce que ça change

- Le roi n'a pas de menu de rôles à part : **l'affectation se fait depuis les bâtiments**. Un bâtiment de travail sans personne à son poste ne produit rien.
- Pour embaucher, il faut avoir construit un lit : les constructions et la population avancent ensemble (voir [construction.md](/regles/construction.md) et [batiments.md](/regles/batiments.md)).

## À définir

- L'attribution du lit se fait-elle **au moment de l'embauche**, ou plus tard, dans une étape séparée ?
- Que devient un lit dont l'occupant **s'en va** ou est réaffecté ? Peut-on **changer** un propriétaire ?
- Un bâtiment de travail avec **plusieurs postes** (plusieurs enclumes) : un propriétaire par poste ?
- Les **compétences** (voir [competences.md](/regles/competences.md)) sont-elles affichées dans la liste des candidats, pour éviter de mal placer un bon forgeron ?
- Les candidats sont-ils **seulement** les personnes embauchées, ou aussi les vagabonds de passage ?
- Un personnage incarné peut-il être **propriétaire** ? (Probablement non : seul le roi décide.)
