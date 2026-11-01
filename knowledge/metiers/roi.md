---
type: Métier
title: Le roi
description: Le roi : répartit rôles, ressources et territoire. Le métier qui portera le plus de gameplay ; presque tout reste à faire.
etat_en_jeu: poser des plans en place ; le reste décidé
tags: [metier, gouvernance]
---

# Le roi

Il dirige le royaume. Son vrai rôle n'existe pas encore : c'est le métier qui va porter le plus de gameplay.

## Aujourd'hui — en place

Au camp de base, le roi se promène autour du feu et dort dans l'un des trois lits. **Incarné, il pose des plans** (et marche plus vite que les autres : `king_speed_factor`) : choisir dans la liste, placer un rectangle, valider (voir [construction.md](/regles/construction.md)). Dans le village (`--village`), il se promène dans le château et peut aussi poser des plans. Quand personne ne l'incarne, il ne pose rien.

## Le modèle : un roi qui répartit — décidé

Le roi ne fait pas le travail, il **répartit**. Comme dans *Kingdom Two Crowns* ou *The Settlers*, il **affecte les ressources et les rôles** et laisse les habitants exécuter. Ses leviers :

- **les rôles** : qui fait quoi (voir [competences.md](/regles/competences.md)) ;
- **les ressources** : les priorités quand plusieurs métiers veulent la même matière ;
- **le territoire** : où le royaume s'étend.

## Ce qu'on veut lui donner

Certaines parties sont décidées, d'autres encore des idées : chaque titre l'indique.

### Priorités du forgeron — décidé

Le roi **donne des priorités au forgeron** : quoi fabriquer d'abord (armes, outils, pièces d'or…). Le forgeron suit cet ordre selon ce qu'il a en stock. Voir [forgeron.md](/metiers/forgeron.md).

### Gérer les stocks — décidé dans le principe

Les stocks (minerai, blé, lingots…) devront être gérés. Le roi est le mieux placé pour cela. Voir [ressources-et-stocks.md](/regles/ressources-et-stocks.md).

### Placer des plans — en place

À terme, la **carte sera vide** et la partie commence avec **le roi et deux artisans** (voir [depart-et-camp.md](/regles/depart-et-camp.md)). **Incarner le roi** permet de choisir un **plan dans une liste**, puis de le **placer** : un rectangle en surbrillance qu'on déplace et qu'on agrandit, avec le coût en bois et en pierre qui se recalcule (bleu si le royaume a les ressources, rouge sinon). Une fois validé, des fondations apparaissent, les [transporteurs](/metiers/transporteur.md) livrent les matériaux et les [artisans](/metiers/artisan.md) construisent. Les routes se tracent d'elles-mêmes vers le camp de base. Le temps ne s'arrête jamais pendant qu'on place. Voir [construction.md](/regles/construction.md).

C'est le cœur du gameplay du roi : il décide où et quoi le royaume construit.

### Affecter les gens — décidé

Des vagabonds se promènent sur la carte, avec ou sans [taverne](/metiers/tavernier.md). **On n'embauche que s'il y a un lit.** Le roi **attribue ensuite le lit** à une personne embauchée qui n'a pas d'affectation, puis, **depuis un bâtiment de travail** comme une forge, il choisit **quelqu'un qui n'a pas de métier** et l'embauche comme forgeron (ou le métier du bâtiment). Voir [propriete-et-affectation.md](/regles/propriete-et-affectation.md). Il doit tenir compte des **compétences** : un bon forgeron aux champs est une erreur ([competences.md](/regles/competences.md)).

### Fixer les priorités des ressources — décidé dans le principe

Quand deux métiers ont besoin de la même matière, **le roi définit les priorités**. Les [transporteurs](/metiers/transporteur.md), qui sont autonomes, les appliquent. Comment le joueur les exprime n'est pas défini.

### Les finances — idée

Lever des **taxes** et entretenir gardes et guerriers. Voir l'idée « Économie » dans [les idées](/idees.md).

## À définir

- **Comment exprimer les priorités** de ressources : une liste ordonnée, des curseurs, des quantités à atteindre ?
- Au-delà des plans et des affectations, que fait encore le joueur qui incarne le roi (priorités, finances) ?
- Les décisions du roi s'appliquent-elles quand personne ne l'incarne (l'IA du roi) ?
