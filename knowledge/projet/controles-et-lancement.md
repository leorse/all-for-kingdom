---
type: Guide
title: Contrôles et lancement
description: Comment lancer le jeu, ses options, les touches, la caméra et le zoom.
tags: [projet, controles]
sources:
  - resource: crates/afk-game/src/input.rs
    title: touches
  - resource: crates/afk-game/src/main.rs
    title: options de lancement
---

# Contrôles et lancement

```
cargo run                          # jeu au camp de base, journée de 10 minutes
cargo run -- --village             # le village déjà bâti (banc d'essai)
cargo run -- --day-seconds 10      # journée de 10 secondes
cargo run -- --speed 20            # journée normale, vue en accéléré
cargo test                         # tests de la logique
```

| Touche | Action |
|---|---|
| ZQSD / WASD / flèches | bouger |
| Tab, 1 à 9, clic | incarner un personnage |
| E | action du métier (travailler, forger, miner, abattre, charger/décharger, se coucher, se lever, annuler) |
| C | vider l'entrepôt central |
| H, F1 | ouvrir ou fermer le panneau des commandes (le HUD ne montre plus de légende ; Échap le referme d'abord) |
| Espace | mode AFK on/off |
| Bouton droit maintenu + souris | déplacer la vue |
| Molette | zoom (4 paliers entiers) autour du curseur |
| T | vitesse de simulation (x1, x10, x60) |
| Clic dans le panneau des plans (le roi) | choisir un plan |
| Clic sur la carte (plan choisi) | fixer le coin, puis valider le rectangle |
| Retour arrière | du dimensionnement au déplacement du plan |
| Échap | fermer l'aide, sinon annuler l'étape d'un plan ; sans rien d'ouvert, quitter |

La caméra suit le personnage incarné ; déplacer la vue à la souris la rend libre jusqu'à ce qu'on bouge au clavier, qu'on change de personnage ou qu'on reprenne la main avec Espace. Voir [le rendu et la caméra](/moteur/rendu-et-camera.md) et [le temps et le repos](/regles/temps-et-repos.md).

## Version web — essai

`web/build.sh` construit le jeu pour le navigateur (`wasm32-unknown-unknown`, macroquad) dans `web/dist` ; `web/build.sh serve` le sert en local. Clavier et souris seulement : le **tactile** demanderait une couche d'entrées à part (joystick, boutons, gestes) et une interface du roi revue pour le doigt. Pas de sauvegarde. Essai, pas une cible de livraison.
