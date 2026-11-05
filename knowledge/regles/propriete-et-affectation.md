---
type: Règle
title: Propriété et affectation
description: Le roi s'adresse à quelqu'un (E), l'affecte à un poste ou à un lit sur une mini-carte (vert libre, rouge occupé) ; lit automatique près du poste, sans lit on s'épuise ; liste des employés (L). En place ; l'équilibrage du départ viendra plus tard.
etat_en_jeu: en place (changement hire-and-assign) ; propriétaire de maison à plusieurs lits, compétences et satisfaction restent à définir
tags: [regle, population, roi, affectation]
sources:
  - resource: crates/afk-core/src/game.rs
    title: Command::{Talk, EndTalk, AssignPost, AssignBed}, posts, auto_beds, free_places
  - resource: crates/afk-core/src/character.rs
    title: paused, post, has_bed, take_post, lose_post, speed_factor (ralentissement du vagabond)
  - resource: crates/afk-game/src/assign_ui.rs
    title: invite « parler », menu, mini-carte, liste des employés
---

# Propriété et affectation

Règles **en place** (changement `hire-and-assign`), sauf mention contraire. Le **propriétaire** d'une maison à plusieurs lits et l'équilibrage du départ restent à faire.

## Principe — décidé

**Un bâtiment appartient à quelqu'un.** Le roi choisit le propriétaire dans une interface :

- dans une maison **à un lit** : le propriétaire ;
- dans une maison **à plusieurs lits** : le **locataire de chaque lit**.

## Au départ — décidé pour l'instant

Les **trois lits du camp de base** sont ceux du roi et des deux artisans (voir [depart-et-camp.md](/regles/depart-et-camp.md)).

## Parler, affecter — en place

Touche **`E`** près de quelqu'un (portée `talk_range`, 2 tuiles) : une invite « E parler » s'affiche au-dessus de la personne la plus proche. Une **fiche** s'ouvre, sans phrase : la photo et le prénom de la personne, puis trois icônes : « → outil » (affecter à un poste), « → lit » (donner un lit), une croix rouge (annuler ; `Echap` ou clic droit aussi). Aucun cercle n'entoure la personne. Sur la mini-carte, chaque personne est un point de la couleur de son métier. La conversation se termine aussi si le roi s'éloigne ou change de personnage. Pendant une conversation, le roi ne bouge pas, mais le temps continue.
Le roi peut aussi **se prendre un lit** : à portée d'un lit (et sans personne à aborder), une invite « B » + lit s'affiche ; `B` le lui attribue, même occupé (l'occupant le perd), et libère son ancien lit. Aucune conversation.

Le roi **s'adresse** à quelqu'un : proche de lui, il appuie sur un bouton. La personne **arrête tout et garde en mémoire ce qu'elle faisait** ; **lui parler alors qu'elle dort la réveille**, le roi n'attend pas ; si le roi ne dit rien, elle reprend exactement où elle en était. **Près d'un vagabond**, le roi le fait toujours **ralentir**, même sans lui parler (un vagabond n'a pas d'autre but que d'être embauché) ; près de n'importe qui d'autre il n'y a **aucun ralentissement** : la personne continue son travail, sauf si le roi lui parle.

Après avoir parlé, le roi peut **réaffecter** la personne (n'importe qui, artisans compris), l'affecter à un **poste** ou à un **lit**. Une **mini-carte** très dézoomée s'ouvre : à gauche la liste des métiers (ceux qui ont des postes libres d'abord) ; choisir un métier colore ses postes en **vert** (libre) ou **rouge** (occupé) ; un clic affecte, ou remplace. **Le temps ne s'arrête pas.**

## Métier, poste, lit — décidé

Trois choses distinctes. Le **métier** reste à la personne même sans poste. Un **poste** est une place de travail dans un bâtiment (une enclume pour un forgeron, un poste de transporteur à l'entrepôt, le tavernier à la taverne…), tenu par une seule personne. La **forge** est un nouveau plan du roi, avec son enclume : un poste de forgeron par enclume. Un **lit** est une place. Un titulaire **remplacé** garde son métier mais n'a plus de poste : **il ne fait plus rien**.

## Les postes de production — décidé (spec `hire-production-posts`)

Dès le départ, le roi peut faire un **bûcheron**, un **mineur** ou un **paysan** sans bâtiment : il pose un **coffre** (un objet à part, payé en bois, une ressource par coffre ; voir [coffres](/regles/coffres.md)). **Posé près de sa matière, le coffre est le poste** : bois près d'une forêt, minerai près d'un filon de même nature, blé près d'un champ (un carré de terre de 3×3, posé nu). Un coffre, un titulaire. Voir [bûcheron](/metiers/bucheron.md), [mineur](/metiers/mineur.md), [paysan](/metiers/paysan.md).

## Le lit — décidé

- Quelqu'un qui a un poste mais pas de lit prend **automatiquement le lit libre le plus proche de son poste**. Un lit qui apparaît (maison finie) va à celui qui en manque.
- **L'embauche n'exige plus de lit** (cette règle est abandonnée). Sans lit, la personne est embauchée mais **ne se repose jamais** : elle s'épuise et travaille moins vite.
- Le roi peut aussi **attribuer un lit à la main** ; l'occupant d'un lit repris le perd.

## La liste des employés — en place

Touche **`L`** : leur **nom**, leur **métier**, leur **poste** (ou « sans poste »), et un **lit** (pictogramme) ou une croix s'ils n'en ont pas. Le roi et les vagabonds n'y figurent pas. Un vagabond à qui on donne un lit sans poste devient un **habitant** (`Role::Villager`) : il dort et ne fait rien le jour.

## Au départ — décidé pour l'instant

Les **trois lits du camp de base** sont ceux du roi et des deux artisans (voir [depart-et-camp.md](/regles/depart-et-camp.md)).

## À définir

- Les **portées** exactes (`talk_range`, `slow_range`) à ajuster à l'usage.
- Réaffecter **à distance** depuis la liste des employés, sans s'approcher ?
- Les **compétences** (voir [competences.md](/regles/competences.md)) : tirées au hasard à l'arrivée, affichées dans la liste ?
- Un **coût** ou un **salaire** d'embauche, avec l'économie ?
- Le titulaire remplacé **garde-t-il son lit** (proposé : oui) ?
- Le **fer** de la forge : un peu dans le pécule, ou attendre un plan de mine ?
- Les postes des autres métiers de production (bûcheron, mine, champ) : changement compagnon `production-plans` (pas encore écrit).
- Une **satisfaction** des habitants (voir [les idées](/idees.md)), pas décidée.

La spec détaillée est le changement OpenSpec `hire-and-assign` (`openspec/changes/hire-and-assign/`), à réajuster à l'usage.
