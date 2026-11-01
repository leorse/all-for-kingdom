---
type: Règle
title: Les bâtiments
description: Un bâtiment est un type, une taille minimale et des assets obligatoires ; la taille est libre au-dessus du minimum ; chaque asset améliore la vie des occupants ; un lit égale une personne. En place pour la maison, l'entrepôt et la taverne (assets de base seulement).
etat_en_jeu: en place (maison, entrepôt, taverne, assets de base) + décidé
tags: [regle, batiment, habitat, confort]
sources:
  - resource: crates/afk-core/src/building.rs
    title: PlanKind, tailles minimales, Building
  - resource: crates/afk-core/src/habitat.rs
    title: HabitatSpec::house (maison de taille libre), classes fixes du village
  - resource: crates/afk-core/src/game.rs
    title: free_places, can_hire
---

# Les bâtiments

Les maisons, entrepôts et tavernes **bâtis par le roi** suivent ce modèle (voir [construction.md](/regles/construction.md)). Les habitats du village de départ gardent trois classes de taille fixe (voir [habitat.md](/regles/habitat.md)).

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

## En place

- **Trois types** : maison (taille minimale 6×5, murs compris), entrepôt (8×6), taverne (9×7). Taille maximale 18×12 (un garde-fou).
- **La maison** a, au minimum, un lit (2×1), une table (2×1) et un bac d'eau (1×1), posés automatiquement (`HabitatSpec::house`) : les lits rangés au nord (une rangée sur deux), la table et le bac de part et d'autre de la porte, au milieu du mur sud. **Le nombre de lits** se règle à la pose, dans ce que la taille permet ; plus elle est grande, plus il y a de places. Pas d'étagère d'office.
- **L'entrepôt** : murs, porte, des tas de caisses contre le mur du fond ; son coffre est grand (`warehouse_capacity`).
- **La taverne** : murs, porte, cheminée, deux tables dans les coins du fond.
- Les assets obligatoires sont dans le prix (voir [construction.md](/regles/construction.md)). Ajouter des assets de confort après coup n'existe pas encore.

## Le village de départ, et ce qui change

- Le village garde trois classes de taille fixe (petite 7×5, moyenne 9×5, grande 9×7), un lit, une table, une étagère et un bac d'eau au minimum, des lits qui se partagent la pièce commune. Le mobilier a des tailles (lit, table, étagère en 2×1 ; bac en 1×1) et quatre rotations (`Rotation`, `Furniture`), ce qui sert de base aux assets.
- **Différence** : la règle décidée parle d'un minimum de **lit, bac d'eau, table** (sans étagère) ; les maisons bâties suivent cette règle (l'étagère est facultative), les habitats du village gardent leur étagère.
- Les classes fixes du village et la spec OpenSpec `housing` qui les décrit restent valables pour lui ; la spec `housing` du changement `start-from-camp` ajoute les maisons de taille libre.
- La vérification « l'espace autour est libre » (`can_expand`) reste utile pour l'agrandissement.

## À définir

- La **liste des assets** et l'effet de chacun sur la vie, le sommeil, le rendement.
- Le confort se compte-t-il **par personne** (assets par résident) ou **par bâtiment** ? Une table pour six lits, est-ce une maison à l'étroit ?
- Les **assets minimaux** des autres types (forge, taverne, mine, cabane…).
- Les **limites** d'un agrandissement (autre que l'espace libre).
- Comment on **ajoute un asset** après coup, et par qui (le roi, ou l'occupant quand on l'incarne : voir l'idée « Mobilier » dans [les idées](/idees.md)).
- Le coût d'un asset, et le coût de la surface par tuile.
