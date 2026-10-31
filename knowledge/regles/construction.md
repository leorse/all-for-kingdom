---
type: Règle
title: Construire : le plan, le chantier et l'interface du roi
description: Le roi choisit un plan dans une liste puis place un rectangle ; les fondations bloquent le passage ; les transporteurs livrent les matériaux, les artisans construisent ; les routes se tracent vers le camp de base. Décidé, pas encore en jeu.
etat_en_jeu: décidé
tags: [regle, construction, interface, roi]
sources:
  - resource: crates/afk-core/src/scenario.rs
    title: aujourd'hui, tout est préconstruit ici
  - resource: crates/afk-core/src/game.rs
    title: Command, la voie d'entrée des intentions du joueur
---

# Construire

Règles **décidées, pas encore dans le jeu**. Aujourd'hui tous les bâtiments sont préconstruits dans le scénario de départ. À terme la carte est vide au départ (voir [depart-et-camp.md](/regles/depart-et-camp.md)) et c'est le [roi](/metiers/roi.md) qui place les bâtiments, construits par les [artisans](/metiers/artisan.md).

## L'interface du roi — décidé

1. On **incarne le roi**.
2. On choisit un **plan** dans une **liste** (maison, forge, taverne…).
3. On **place ce plan** : une forme **rectangulaire ou carrée** en **surbrillance**, qu'on **déplace** et qu'on **agrandit** autant qu'on veut.
4. Pendant qu'on la change, on voit **le coût en pierre et en bois** se recalculer.
5. La zone est **bleue** quand le royaume a les ressources, **rouge** quand il ne les a pas.
6. On **valide**. Même en rouge, on peut poser la fondation : elle sera prise en compte, et les matériaux viendront quand il y en aura.

**Le temps ne change jamais** : le royaume continue de tourner à la même vitesse pendant qu'on place un plan (pas de pause, pas de ralenti).

Les assets de base du bâtiment sont **compris dans le prix et placés automatiquement** (voir [batiments.md](/regles/batiments.md)).

## Le chantier — décidé

1. À la validation, des **fondations** apparaissent à l'endroit choisi.
2. Les **transporteurs** posent les **matériaux requis devant les fondations**. Les **artisans** qui n'ont pas de chantier font aussi ce travail (voir [artisan.md](/metiers/artisan.md)) : c'est eux qui livrent au début de la partie.
3. Les **artisans** se mettent à construire.

Les **fondations bloquent le passage**.

## Les routes — décidé

**Dès que le bâtiment est terminé**, **une route se génère toute seule** (pas à la pose des fondations). **Toutes les routes mènent au camp de base** (voir [depart-et-camp.md](/regles/depart-et-camp.md)). Le joueur n'a pas à les tracer. Décidé pour l'instant : cela pourra changer plus tard.

## Les priorités — provisoire

Quand plusieurs chantiers attendent, les artisans les prennent **dans l'ordre des placements**. Ce n'est pas arrêté définitivement : le créateur ne sait pas encore s'il faudra une vraie priorité.

## À définir

- Peut-on **annuler** un chantier, et que devient la matière déjà livrée ?
- **Agrandir** un bâtiment existant : par la même interface (on étire un côté et on paie la différence) ?
- Le **coût** de chaque plan et de chaque asset, la **durée** d'un chantier, et le fait que les **deux artisans** bâtissent ensemble.
- Si la matière manque en cours de route : l'artisan **attend sur place**, ou va livrer lui-même (il fait aussi le transport quand il est libre) ?
- Le tracé d'une route **vers le camp** : plus court chemin, ou suit-elle les routes existantes ?
- Une construction **interdite** (arbre, eau, filon, autre bâtiment dans le rectangle) : la zone devient-elle d'une autre couleur ?
- Ce que fait **l'IA du roi** quand personne ne l'incarne : place-t-elle des plans ? (voir [roi.md](/metiers/roi.md))
