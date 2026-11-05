---
type: Règle
title: Les coffres
description: Un coffre est un objet posé volontairement, pour une seule ressource (200 au plus) ; chacun dépose dans le coffre le plus proche qui l'accepte ; l'entrepôt a de grands coffres à l'intérieur (200 mélangés), accessibles par la porte. Décidé, pas encore en jeu.
etat_en_jeu: décidé (spec hire-production-posts) ; en place : un contenant par lieu de travail, grand contenant d'entrepôt à l'extérieur
tags: [regle, coffres, stocks, transport]
sources:
  - resource: crates/afk-core/src/container.rs
    title: Container, ContainerKind (à étendre : Chest, BigChest)
  - resource: crates/afk-core/src/game.rs
    title: finish_site (entrepôt), transport
---

# Les coffres

## Le principe — décidé

- Un **coffre** est un objet à part, que le [roi](/metiers/roi.md) **pose volontairement** : jamais posé tout seul avec un champ, une forêt ou un filon. Payé en bois au camp, posé tout de suite, sans chantier.
- **Un coffre, une seule ressource**, choisie à la pose, **200 au plus**.
- **Posé près de sa matière, un coffre est un poste** : bois près d'une forêt (bûcheron), minerai près d'un filon de même nature (mineur), blé près d'un champ (paysan). Voir [propriété et affectation](/regles/propriete-et-affectation.md).
- **Quand personne n'est incarné**, chacun **porte lui-même** sa production et la dépose dans le **coffre le plus proche qui l'accepte** ; sans coffre qui l'accepte, il garde sa cargaison et attend. Les [transporteurs](/metiers/transporteur.md) déplacent ensuite les ressources d'un coffre à l'autre.

## Exceptions — décidé

- **Le coffre du camp** contient tout (100 de chaque) : voir [le départ](/regles/depart-et-camp.md).
- **L'entrepôt** n'a **aucun coffre à l'extérieur** : il a de **grands coffres à l'intérieur**, chacun de **200 ressources de n'importe quel type, mélangés** (seule la somme compte). **On n'entre pas** : toucher la **porte** suffit pour déposer ou prendre (simplification du gameplay). Le nombre de grands coffres d'un entrepôt est un réglage, à calibrer.

## À définir

- Le **nombre** de grands coffres (fixe, ou selon la taille du bâtiment) et le **coût** d'un coffre.
- **Déplacer** ou détruire un coffre posé au mauvais endroit.
- Un artisan construit-il le coffre, au lieu de le poser instantanément ?
- Le **mineur** qui porte son minerai : son rendement baisse ; à équilibrer.
