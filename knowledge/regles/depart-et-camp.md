---
type: Règle
title: Le départ de partie et le camp de base
description: La partie commence avec le roi et deux artisans dans un camp de base de trois lits, sur une carte sans bâtiment, avec un petit pécule dans des coffres et sans entrepôt. Décidé pour l'instant, pas encore en jeu.
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

## Le camp de base — décidé dans le principe

Le **point de départ** est un simple **camp de base avec trois lits** : ceux du roi et des deux artisans. Décidé **pour l'instant** : cela pourra changer plus tard. Il sera **amélioré par la suite** ; ce qu'il contient d'autre et ce qu'il devient ne sont pas encore fixés.

**Toutes les routes mènent au camp de base** (voir [construction.md](/regles/construction.md)).

## À définir

- **Ce qu'est le camp** en dehors de ses trois lits : un bâtiment comme les autres, avec ses assets minimaux ? Piste, **non décidée** : un coffre et un feu, amélioré plus tard en château.
- Où est le **pécule** : dans le coffre du camp ?
- Les **forêts et les filons** existent-ils déjà sur la carte vide (la nature est là, pas les bâtiments), ou le roi doit-il aussi les placer ?
- Le **château**, ou le camp amélioré : que devient-il, et le roi y dort-il ?
