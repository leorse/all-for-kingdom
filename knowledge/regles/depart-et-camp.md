---
type: Règle
title: Le départ de partie et le camp de base
description: La partie commence avec le roi et deux artisans sur une carte sans bâtiment, un petit pécule dans des coffres, sans entrepôt, autour d'un camp de base à améliorer. Décidé dans le principe, pas encore en jeu.
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
- La partie commence avec **le roi et deux artisans**.
- Le **roi place les premiers bâtiments** (voir [construction.md](/regles/construction.md)). Les artisans les construisent : c'est leur métier (voir [artisan.md](/metiers/artisan.md)).
- Un **petit pécule** de ressources de départ : du **bois**, de la **pierre**, des **graines**… Sa composition et ses quantités ne sont pas arrêtées. Le **nombre de ressources** du jeu non plus : il se déduira des besoins des bâtiments.
- **Pas d'entrepôt** au départ. Les ressources sont **dans des coffres, et c'est tout** (voir [ressources-et-stocks.md](/regles/ressources-et-stocks.md)).
- Des **vagabonds se promènent** sur la carte **sans qu'il y ait de taverne** (voir [tavernier.md](/metiers/tavernier.md)).

## Le camp de base — décidé dans le principe

Le **point de départ** est un simple **camp de base**, qui sera **amélioré par la suite**. Ce qu'il contient et ce qu'il devient ne sont pas encore fixés.

**Toutes les routes mènent au camp de base** (voir [construction.md](/regles/construction.md)).

## À définir

- **Ce qu'est le camp** : un bâtiment comme les autres, avec ses assets minimaux ? Piste, **non décidée** : un coffre, un feu et des couchages pour le roi et les deux artisans, amélioré plus tard en château.
- **Où dorment** le roi et les artisans la première nuit.
- Où est le **pécule** : dans le coffre du camp ?
- Les **forêts et les filons** existent-ils déjà sur la carte vide (la nature est là, pas les bâtiments), ou le roi doit-il aussi les placer ?
- **Qui livre** les matériaux avant que le royaume ait des transporteurs ?
- Le **château**, ou le camp amélioré : que devient-il, et le roi y dort-il ?
