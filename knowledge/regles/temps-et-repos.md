---
type: Règle
title: Temps, repos et fatigue
description: Cycle jour/nuit, forme et fatigue, quart de nuit, incarnation, joueur 2x plus rapide que l'IA, mode AFK.
etat_en_jeu: en place
tags: [regle, temps, fatigue]
sources:
  - resource: crates/afk-core/src/clock.rs
    title: horloge
  - resource: crates/afk-core/src/config.rs
    title: réglages de fatigue et de cycle
  - resource: crates/afk-core/src/character.rs
    title: fatigue, efficiency()
---

# Temps, repos et fatigue

Ces règles valent pour tous les métiers.

## Le cycle jour/nuit — en place

- Une journée dure 10 minutes réelles (réglable).
- Le jour se lève à 6 h et tombe à 20 h.
- La nuit, l'IA rentre dormir dans son lit.

## Aller se coucher — en place

Pour rejoindre son lit, on emprunte les routes, comme pour tout trajet ordinaire.

## La forme — en place

- Chaque personnage a une jauge de **forme** : pleine = en forme.
- Elle baisse quand il marche ou travaille, plus lentement quand il reste à l'arrêt, et remonte en dormant.
- **Sous 50 de forme**, il va plus lentement et travaille moins vite.
- **À 15 de forme**, l'IA va se coucher même en plein jour.
- Le sommeil est la seule façon de récupérer. Dormir plus vite est une amélioration possible de la maison (voir [habitat.md](/regles/habitat.md)).

## Le quart de nuit — en place

Un personnage peut travailler la nuit et dormir le jour : c'est son heure de dormir qui est inversée. C'est le cas du garde de nuit.

## Incarner un personnage — en place

- On peut incarner n'importe quel habitant (touches `Tab`, `1` à `9`, ou clic). La caméra glisse jusqu'à lui.
- Le personnage incarné n'est **jamais couché de force** : on peut veiller, au prix de la forme. On se couche soi-même près de son lit.
- Un dormeur continue de dormir quand on incarne quelqu'un d'autre.
- Quand on lâche un personnage, l'IA reprend sa journée.

## Le joueur travaille plus vite que l'IA — en place

Quel que soit le métier, le joueur exécute ses travaux **2 fois plus vite** que l'IA. La fatigue ralentit tout le monde de la même façon.

C'est le principe du jeu : l'IA fait tourner la cité, mais incarner un personnage est plus efficace. C'est ce qui donne envie de passer d'un métier à l'autre.

## Le mode AFK — en place

Sans personne d'incarné, la cité tourne seule. La caméra reste sur le dernier personnage incarné.
