---
type: Règle
title: Le départ de partie et le camp de base
description: La partie commence avec le roi et deux artisans dans un camp de base temporaire (trois lits, un feu, un coffre de 100), sur une carte sans bâtiment mais avec sa nature, et un entrepôt à bâtir en priorité. Décidé pour l'instant, pas encore en jeu.
etat_en_jeu: décidé dans le principe
tags: [regle, depart, camp, ressources]
sources:
  - resource: crates/afk-core/src/scenario.rs
    title: le village de départ, préconstruit, qui sert de banc d'essai
---

# Le départ de partie et le camp de base

Règles **décidées dans leur principe, pas encore dans le jeu**. Le village préconstruit de `scenario.rs` reste le banc d'essai : il permet de **tout tester** sans avoir à construire le royaume.

## Au départ — décidé

- La **carte est vide** : aucun bâtiment.
- La partie commence avec **le roi et deux artisans**, qui dorment au **camp de base** (trois lits).
- Les artisans qui n'ont rien à construire font le **travail de transporteur** : ils livrent la matière aux chantiers (voir [artisan.md](/metiers/artisan.md)).
- Le **roi place les premiers bâtiments** (voir [construction.md](/regles/construction.md)). Les artisans les construisent : c'est leur métier (voir [artisan.md](/metiers/artisan.md)).
- Un **petit pécule** de ressources de départ : du **bois**, de la **pierre**, des **graines**… Sa composition et ses quantités ne sont pas arrêtées. Le **nombre de ressources** du jeu non plus : il se déduira des besoins des bâtiments.
- **Pas d'entrepôt** au départ. Les ressources sont **dans des coffres, et c'est tout** (voir [ressources-et-stocks.md](/regles/ressources-et-stocks.md)).
- Des **vagabonds se promènent** sur la carte **sans qu'il y ait de taverne** (voir [tavernier.md](/metiers/tavernier.md)).
- La **nature est déjà là** : forêts, filons, eau, lacs, arbres. Seuls les bâtiments manquent.

## Le camp de base — décidé pour l'instant

Le camp de base est **une zone temporaire** qui contient :

- **trois lits** : ceux du roi et des deux artisans ;
- **un feu de camp** ;
- **un coffre**, qui peut contenir **absolument tout** : du bois, du blé, de la pierre, sans être réservé à un matériau, avec des limites (voir plus bas).

Décidé **pour l'instant** : cela pourra changer plus tard.

**Toutes les routes mènent au camp de base** (voir [construction.md](/regles/construction.md)).

## L'entrepôt, première urgence — décidé

Le coffre du camp ne contient que **100 éléments**. Le joueur a **tout intérêt à construire un entrepôt dès les premières minutes** : il contient des quantités **beaucoup plus grandes**. L'entrepôt est le premier grand objectif de la partie (voir [ressources-et-stocks.md](/regles/ressources-et-stocks.md)).

## À définir

- La **limite de 100** du coffre du camp : par ressource, ou au total ? Les coffres actuels du jeu sont à **200 de chaque ressource** : le créateur dit « les mêmes limites que d'habitude » et aussi « 100 éléments chacun ». À trancher.
- Le **pécule** est-il dans le coffre du camp ? Sa composition et ses quantités ne sont pas arrêtées.
- Que devient la **zone temporaire** : améliorée, ou remplacée (un château, des maisons) ? Le roi dort-il toujours au camp ?
- La **capacité de l'entrepôt**, et ce qu'il coûte : assez bas pour être bâti dans les premières minutes.
