---
type: Règle
title: Les bâtiments
description: Un bâtiment est un type, une taille minimale et des assets obligatoires ; la taille est libre au-dessus du minimum ; chaque asset améliore la vie des occupants ; un lit égale une personne. Décidé, pas encore en jeu.
etat_en_jeu: décidé (remplace les trois classes de taille fixe en place)
tags: [regle, batiment, habitat, confort]
sources:
  - resource: crates/afk-core/src/habitat.rs
    title: habitats actuels (classes fixes, mobilier, rotations) : le point de départ du modèle
  - resource: crates/afk-core/src/game.rs
    title: free_places, can_hire
---

# Les bâtiments

Page de **règles décidées dans leur principe, pas encore dans le jeu**. Aujourd'hui les habitats ont trois classes de taille fixe (voir [habitat.md](/regles/habitat.md)) ; le modèle ci-dessous les remplace.

## Le principe — décidé

Un bâtiment est défini par :

- un **type** : maison, forge, taverne, mine, cabane… ;
- une **taille minimale** : pas de taille fixe, seulement un plancher ;
- des **assets obligatoires** : le minimum qu'il faut à ce type pour fonctionner.

**La maison**, par exemple, doit avoir au minimum **un lit, un bac d'eau et une table**.

Au-dessus du minimum, **la taille est libre** : on peut agrandir un bâtiment **autant qu'on veut** pour y ajouter d'autres assets.

## Les assets — décidé

- Un **asset** est un élément posé dans le bâtiment : lit, table, bac d'eau, cheminée, et d'autres encore à imaginer.
- **Chaque asset ajouté améliore la qualité de vie** de ceux qui y vivent.
- **Un lit égale une personne.** Si on manque de place, on construit une autre maison, ou on agrandit une maison et on y ajoute des lits.
- Les assets de base sont **compris dans le prix du bâtiment** et **placés automatiquement** quand le bâtiment est construit (voir [construction.md](/regles/construction.md)).

## La forme — décidé

Un bâtiment se place comme un **rectangle ou un carré** qu'on déplace et qu'on redimensionne (voir [construction.md](/regles/construction.md)).

## Le coût — décidé dans le principe

Le coût en **bois et en pierre** dépend de la forme choisie : il se recalcule à mesure qu'on change la taille. Les valeurs ne sont pas arrêtées ; le coût est la somme de la surface bâtie et des assets compris. Voir [ressources-et-stocks.md](/regles/ressources-et-stocks.md).

## Qui y vit — décidé

Un bâtiment **appartient à quelqu'un**, et chaque lit a son occupant. Voir [propriete-et-affectation.md](/regles/propriete-et-affectation.md).

## Aujourd'hui, en place, et ce qui change

- En place : trois classes de taille fixe (petite 7×5, moyenne 9×5, grande 9×7), un lit, une table, une étagère et un bac d'eau au minimum, des lits qui se partagent la pièce commune. Le mobilier a déjà des tailles (lit, table, étagère en 2×1 ; bac en 1×1) et quatre rotations (`Rotation`, `Furniture`), ce qui sert de base aux assets.
- **Différence à trancher** : la règle décidée parle d'un minimum de **lit, bac d'eau, table** (sans étagère). La règle en place exige aussi une étagère. L'étagère devient-elle un asset facultatif ?
- Les classes fixes, et la spec OpenSpec `housing` qui les décrit, disparaîtront avec l'implémentation de ce modèle. Le code prime tant qu'il n'est pas changé.
- La vérification « l'espace autour est libre » (`can_expand`) reste utile pour l'agrandissement.

## À définir

- La **liste des assets** et l'effet de chacun sur la vie, le sommeil, le rendement.
- Le confort se compte-t-il **par personne** (assets par résident) ou **par bâtiment** ? Une table pour six lits, est-ce une maison à l'étroit ?
- Les **assets minimaux** des autres types (forge, taverne, mine, cabane…).
- Les **limites** d'un agrandissement (autre que l'espace libre).
- Comment on **ajoute un asset** après coup, et par qui (le roi, ou l'occupant quand on l'incarne : voir l'idée « Mobilier » dans [les idées](/idees.md)).
- Le coût d'un asset, et le coût de la surface par tuile.
