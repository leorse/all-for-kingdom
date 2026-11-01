---
type: Règle
title: Le royaume
description: Le royaume : carte 64x40, disposition, déplacements sur routes, carte vide et zones décidées.
etat_en_jeu: en place + décidé
tags: [regle, carte, deplacement]
sources:
  - resource: crates/afk-core/src/scenario.rs
    title: disposition de départ
  - resource: crates/afk-core/src/map.rs
    title: carte et chemins
---

# Le royaume

## La carte — en place

- Le royaume est **plus grand que l'écran** : 64×40 tuiles, entouré d'eau.
- Le zoom est fixe. La caméra suit le personnage incarné : elle glisse jusqu'à lui quand on change de personnage, puis ne défile que lorsqu'il s'approche du bord de l'écran.

## La disposition — en place

- **Au centre** : les bâtiments publics, le château, la forge, la caserne et la taverne, reliés par la grande route est-ouest et l'axe nord-sud.
- **Loin du centre** : les maisons et les champs. Le champ est à l'extrémité nord-ouest.
- Tout est **relié par des routes**. Chaque personnage doit pouvoir atteindre son lit et ses postes de travail.
- Il y a quelques arbres isolés, qui bloquent le passage, et deux forêts dont on peut couper les arbres.

## Les déplacements — en place

- Les habitants **suivent les routes** : à distance proche, ils préfèrent la route à l'herbe, et ne la quittent que pour atteindre un but qui n'y est pas (champ, coffre, filon, arbre). Le surcoût de l'herbe est un réglage (`grass_cost`).
- Ils marchent **en diagonale** (sans couper le coin d'un mur ou d'un coffre) et arrondissent leurs virages : le tracé est une courbe, pas un escalier.
- Une personne qui **va se coucher** emprunte les routes.
- Le **plus court chemin sans les routes** existe aussi, réservé à la fuite d'un voleur vers la sortie du château et à ses poursuivants (les gardes) : voir [garde.md](/metiers/garde.md). Les voleurs n'existent pas encore.

## Décidé : les filons

Des **filons** de 2×2 tuiles (fer, cuivre, or, pierre) seront répartis sur la carte, chacun avec une **mine** attenante qui sert de maison au mineur. Voir [mineur.md](/metiers/mineur.md).

Comme les maisons et les champs, ils seront **loin du centre**.

## Décidé : le roi bâtit — carte vide et zones

**À terme, la carte sera vide.** Le [roi](/metiers/roi.md) y place des **zones** (champs, habitat, mines, forêt exploitée…) et un [artisan](/metiers/artisan.md) construit. Le royaume grandit donc selon ses décisions, et non plus selon un scénario figé. L'inspiration est *The Settlers*. **La nature est déjà là au départ** : forêts, filons, eau, lacs, arbres ; seuls les bâtiments manquent (voir [depart-et-camp.md](/regles/depart-et-camp.md)).

Les **routes se tracent d'elles-mêmes** pour alléger le travail du joueur, dès que le bâtiment est terminé, et **toutes mènent au camp de base**. Elles servent aux déplacements et aux [transporteurs](/metiers/transporteur.md). Le roi place des plans rectangulaires ([construction.md](/regles/construction.md)) ; la partie commence avec le roi et deux artisans, un camp de base et un petit pécule ([depart-et-camp.md](/regles/depart-et-camp.md)). Les **fondations bloquent le passage**.

### Deux cartes de départ

- **Le camp** (`Scenario::camp`, par défaut) : une carte **sans bâtiment** mais avec sa nature (deux lacs, deux forêts, des filons, des arbres) et un camp au milieu. Le roi bâtit le reste (voir [depart-et-camp.md](/regles/depart-et-camp.md)). Les routes se tracent à la fin de chaque chantier, vers le camp.
- **Le village** (`Scenario::village`, avec `--village`) : tout est **posé à la main**. C'est volontaire : cela permet de **tout tester** sans avoir à construire le royaume.

## Décidé : les forêts sont limitées

Les arbres **poussent**, et il y a des **zones de forêt** en nombre limité (deux au départ). Voir [bucheron.md](/metiers/bucheron.md).

## À définir

- Où sont les premiers filons : au bord du royaume, regroupés dans une zone rocheuse ?
- La carte s'agrandit-elle, et y a-t-il une notion de région autour du royaume (voir l'idée « Région et royaume ennemi » dans [les idées](/idees.md)) ?
- La **taille des zones** : combien de tuiles pour un champ, une forêt, un quartier d'habitation ? Pas encore défini.
- Comment une construction avance : tout d'un coup, ou par étapes visibles selon l'avancement des artisans ? (Les fondations sont décidées : voir [construction.md](/regles/construction.md).)
- Comment la **route** se trace vers le camp de base : plus court chemin, ou autre règle ?
- Les **arbres** actuels deviennent la matière du [bûcheron](/metiers/bucheron.md) : certains resteront-ils décoratifs ?
