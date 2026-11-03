---
type: Règle
title: Le départ de partie et le camp de base
description: La partie commence avec le roi et deux artisans dans un camp de base temporaire (trois lits, un feu, un coffre de 100), sur une carte sans bâtiment mais avec sa nature, avec un pécule et des vagabonds qui passent. En place ; un entrepôt à bâtir en priorité.
etat_en_jeu: en place (banc d'essai : le village bâti, `--village`)
tags: [regle, depart, camp, ressources, vagabonds]
sources:
  - resource: crates/afk-core/src/scenario.rs
    title: Scenario::camp (le départ) et Scenario::village (le banc d'essai)
  - resource: crates/afk-core/src/game.rs
    title: pécule dans le coffre du camp, apparition des vagabonds
  - resource: crates/afk-core/src/config.rs
    title: camp_capacity, starting_wood, starting_stone, starting_seeds, vagabond_*
---

# Le départ de partie et le camp de base

`cargo run` démarre une partie au camp de base. `cargo run -- --village` garde le village préconstruit de `Scenario::village()`, qui reste le banc d'essai : il permet de **tout tester** sans avoir à construire le royaume (sans camp, sans vagabonds).

## Au départ — en place

- La **carte est sans bâtiment** (64×40) mais **la nature est déjà là** : deux lacs, deux forêts, des filons (pierre, fer, or ; pas de cuivre posé), des arbres isolés.
- La partie commence avec **le roi et deux artisans**, qui dorment chacun dans l'un des **trois lits** du camp.
- Le **roi place les premiers bâtiments** avec l'interface des plans, et les artisans les construisent (voir [construction.md](/regles/construction.md)).
- Un **petit pécule** dans le coffre du camp : du **bois**, de la **pierre** et des **graines** (`starting_wood`, `starting_stone`, `starting_seeds`). Les graines n'ont pas encore de consommateur. Le **nombre de ressources** du jeu se déduira des besoins des bâtiments ; il n'est pas arrêté.
- **Pas d'entrepôt** au départ : les ressources sont **dans le coffre du camp, et c'est tout** (voir [ressources-et-stocks.md](/regles/ressources-et-stocks.md)).
- Des **vagabonds se promènent** sans qu'il y ait de taverne (voir [vagabond.md](/metiers/vagabond.md)).

## Le camp de base — en place, temporaire

Le camp est **une place de terre battue** (9×5) qui contient :

- **trois lits** (des couchages à même le sol) : ceux du roi et des deux artisans ;
- **un feu de camp**, animé, qui éclaire le sol la nuit ;
- **un coffre**, qui contient **absolument tout** (bois, blé, pierre…) sans être réservé à un matériau, avec une limite de **100 de chaque ressource** (`camp_capacity`).

**Toutes les routes qu'on bâtit mènent au camp de base** : à la fin de chaque chantier, une route se trace de la porte du bâtiment jusqu'à la place du camp (voir [construction.md](/regles/construction.md)).

## L'entrepôt, première urgence — décidé

Le coffre du camp ne contient que 100 de chaque ressource. Le joueur a **tout intérêt à construire un entrepôt dès les premières minutes** : il contient des quantités beaucoup plus grandes (`warehouse_capacity`). C'est le premier grand objectif de la partie.

## À définir

- La **limite de 100** du coffre du camp : le créateur parle de « 100 éléments chacun » et de « mêmes limites que d'habitude » (200 par ressource ailleurs). Elle est **par ressource** dans le jeu ; à confirmer.
- Que devient la **zone temporaire** : améliorée, ou remplacée (un château, des maisons) ? Le roi dort-il toujours au camp ?
- La **composition et les quantités du pécule**, et la **capacité de l'entrepôt**, à calibrer avec le game designer.
- Aucun métier de production ne peut encore être bâti : bois et pierre ne se renouvellent pas tant qu'on ne peut pas poser de cabane de bûcheron ni de mine.

## Embaucher — en place

Les vagabonds qui passent peuvent être embauchés par le roi (`E` pour parler, poste ou lit sur la mini-carte) : voir [propriété et affectation](/regles/propriete-et-affectation.md). Le fer pour la forge n'est pas dans le pécule : l'équilibrage du départ viendra plus tard.

## Les premiers métiers de production — décidé

Dès le début le roi peut faire un bûcheron ou un mineur (un coffre suffit) et un paysan (un coffre et une surface cultivable) : voir [propriété et affectation](/regles/propriete-et-affectation.md). Spec `hire-production-posts`, pas encore en jeu.
