---
type: Règle
title: Propriété et affectation
description: Le roi s'adresse à quelqu'un, l'affecte à un poste ou à un lit sur une mini-carte (vert libre, rouge occupé) ; lit automatique près du poste, sans lit on s'épuise ; liste des employés. Décidé et spécifié, pas encore en jeu.
etat_en_jeu: décidé (spec hire-and-assign) ; seul le compte des places libres est en place
tags: [regle, population, roi, affectation]
sources:
  - resource: crates/afk-core/src/game.rs
    title: free_places, can_hire (places libres et embauche)
---

# Propriété et affectation

Règles **décidées et spécifiées, pas encore dans le jeu**. En place aujourd'hui : le jeu compte les places libres (`free_places`). Tout le reste est à faire.

## Principe — décidé

**Un bâtiment appartient à quelqu'un.** Le roi choisit le propriétaire dans une interface :

- dans une maison **à un lit** : le propriétaire ;
- dans une maison **à plusieurs lits** : le **locataire de chaque lit**.

## Au départ — décidé pour l'instant

Les **trois lits du camp de base** sont ceux du roi et des deux artisans (voir [depart-et-camp.md](/regles/depart-et-camp.md)).

## Parler, affecter — décidé (spec `hire-and-assign`)

Le roi **s'adresse** à quelqu'un : proche de lui, il appuie sur un bouton. La personne **arrête tout et garde en mémoire ce qu'elle faisait** ; si le roi ne dit rien, elle reprend exactement où elle en était. **Près d'un vagabond**, le roi le fait toujours **ralentir**, même sans lui parler (un vagabond n'a pas d'autre but que d'être embauché) ; près de n'importe qui d'autre il n'y a **aucun ralentissement** : la personne continue son travail, sauf si le roi lui parle.

Après avoir parlé, le roi peut **réaffecter** la personne (n'importe qui, artisans compris), l'affecter à un **poste** ou à un **lit**. Une **mini-carte** très dézoomée s'ouvre : à gauche la liste des métiers (ceux qui ont des postes libres d'abord) ; choisir un métier colore ses postes en **vert** (libre) ou **rouge** (occupé) ; un clic affecte, ou remplace. **Le temps ne s'arrête pas.**

## Métier, poste, lit — décidé

Trois choses distinctes. Le **métier** reste à la personne même sans poste. Un **poste** est une place de travail dans un bâtiment (une enclume, un poste de transporteur…), tenu par une seule personne. Un **lit** est une place. Un titulaire **remplacé** garde son métier mais n'a plus de poste : **il ne fait plus rien**.

## Le lit — décidé

- Quelqu'un qui a un poste mais pas de lit prend **automatiquement le lit libre le plus proche de son poste**. Un lit qui apparaît (maison finie) va à celui qui en manque.
- **L'embauche n'exige plus de lit** (cette règle est abandonnée). Sans lit, la personne est embauchée mais **ne se repose jamais** : elle s'épuise et travaille moins vite.
- Le roi peut aussi **attribuer un lit à la main** ; l'occupant d'un lit repris le perd.

## La liste des employés — décidé

Le roi a une liste de ses employés : leur **nom**, leur **métier**, leur **poste** (ou l'absence de poste), et un **symbole** s'ils ont un lit.

## Au départ — décidé pour l'instant

Les **trois lits du camp de base** sont ceux du roi et des deux artisans (voir [depart-et-camp.md](/regles/depart-et-camp.md)).

## À définir

- La **touche** pour parler (proposée : `E`) et les **portées** (parler, ralentissement).
- Réaffecter **à distance** depuis la liste des employés, sans s'approcher ?
- Les **compétences** (voir [competences.md](/regles/competences.md)) : tirées au hasard à l'arrivée, affichées dans la liste ?
- Un **coût** ou un **salaire** d'embauche, avec l'économie ?
- Le titulaire remplacé **garde-t-il son lit** (proposé : oui) ?
- Les postes des métiers de production : voir le changement compagnon `production-plans` (pas encore écrit).

La spec détaillée est le changement OpenSpec `hire-and-assign` (`openspec/changes/hire-and-assign/`), à réajuster à l'usage.
