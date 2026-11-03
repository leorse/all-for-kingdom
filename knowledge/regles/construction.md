---
type: Règle
title: Construire : le plan, le chantier et l'interface du roi
description: Le roi choisit un plan dans une liste puis place un rectangle (bleu, rouge ou gris) ; les fondations bloquent le passage ; le chantier démarre quand tout est disponible, les matériaux sont portés devant, les artisans construisent ; une route mène au camp. En place pour la maison, l'entrepôt, la taverne et la forge.
etat_en_jeu: en place (quatre types de plan) + décidé
tags: [regle, construction, interface, roi]
sources:
  - resource: crates/afk-core/src/building.rs
    title: PlanKind, coûts, durées, check_plan, Site, Building
  - resource: crates/afk-core/src/game.rs
    title: PlacePlan, update_sites, build_sites, finish_site, lay_road
  - resource: crates/afk-core/src/character.rs
    title: artisan_routine
  - resource: crates/afk-game/src/plan_ui.rs
    title: l'interface du roi
  - resource: crates/afk-game/src/construction_view.rs
    title: fondations, planches dans la zone, enseigne
---

# Construire

Le [roi](/metiers/roi.md) place des plans ; les [artisans](/metiers/artisan.md) les construisent. En place pour quatre types : **maison**, **entrepôt**, **taverne**, **forge** (voir [batiments.md](/regles/batiments.md)). Le départ de partie est décrit dans [depart-et-camp.md](/regles/depart-et-camp.md).

## L'interface du roi — en place

Quand on incarne le roi (touche `1`), un **panneau de parchemin** à cadre de bois liste les plans : nom, taille minimale, coût minimal, pictogramme. Le panneau **se réduit** à son en-tête (bouton `-` / `+`) et **se déplace** (on tire l'en-tête), pour ne pas cacher l'endroit où l'on veut bâtir.

1. **Clic sur un plan** de la liste : il est choisi.
2. **Déplacer** : un rectangle en surbrillance suit la souris, à la taille minimale.
3. **Clic** : le coin haut-gauche est fixé. **Agrandir** : la souris étire le rectangle (jamais sous le minimum). Pour une maison, des boutons `-` et `+` règlent le **nombre de lits**, dans ce que la taille permet.
4. Une **barre d'information** montre le coût en bois et en pierre, le stock, la durée de construction et ce qui bloque ; elle se recalcule à chaque tuile.
5. **Clic** : on **valide**. On peut alors en poser un autre. **Bouton droit** (un clic bref ; en glissant il tire la carte) ou `Retour arrière` revient de l'agrandissement au déplacement, puis annule le plan ; `Échap` fait de même l'étape puis le plan (il ne quitte le jeu que s'aucun plan n'est en cours).

La zone est **bleue** si le royaume a les ressources, **rouge** s'il ne les a pas, **grise** si le terrain ne permet pas de poser (eau, arbre, filon, coffre, bord de carte, quelqu'un dessus). Un plan rouge peut être validé ; un plan gris non. Sur le rectangle : la porte (milieu du mur sud) et la case du tas de matériaux.

**Le temps ne change jamais** : pas de pause, pas de ralenti pendant qu'on place. Un clic pris par l'interface n'incarne personne.

Les assets de base sont **compris dans le prix et placés automatiquement**.

## Le coût et la durée — en place

Le coût en bois et en pierre est la **surface** (murs compris) au prix du type, plus les assets de base, plus chaque lit au-delà du premier (`plan_cost`). La durée suit la même logique (`plan_build_secs`) : par tuile, plus les assets, plus les lits. Ils croissent donc avec la surface et dépendent du type. Les valeurs (`plan_*` dans `Config`) sont des ordres de grandeur à calibrer.

## Le chantier — en place

1. À la validation, des **fondations** apparaissent sur toute l'emprise et **bloquent le passage**. Le chantier est « en attente ».
2. **Un chantier ne commence que si toutes les ressources sont disponibles** (dans les coffres, moins ce qui est promis aux autres chantiers). Tant qu'elles ne le sont pas, il attend, sans rien prendre. Les chantiers sont servis **dans l'ordre des placements**, un chantier trop cher ne bloquant pas les suivants.
3. Quand il peut démarrer, un **tas de matériaux** apparaît devant le chantier (coin sud-ouest) ; si quelqu'un se tient sur cette case, on l'écarte d'un pas (sauf le personnage d'un joueur, qui bouge quand il veut), et les **transporteurs** et les **artisans** sans chantier à construire y portent le bois et la pierre. Le tas affiche `Bois 12/26`.
4. **Dès que tout est là, le chantier commence** : les artisans construisent, **à plusieurs leur travail s'additionne** (deux artisans vont deux fois plus vite). Les **planches recouvrent la fondation** de la gauche vers la droite avec l'avancement (tout reste dans la zone du chantier), avec une barre et un pourcentage dans l'enseigne.
5. À la fin, **le bâtiment prend la place des fondations**, les matériaux sont consommés, et **une route se génère** de la porte jusqu'au camp de base (pas avant la fin). La route est **continue par les côtés** : un virage fait un angle, jamais un creux en diagonale (les futurs sprites d'angle s'y raccordent). Si le tas de matériaux était posé sur un chemin, **le chemin est rendu** quand le tas disparaît.

Un artisan a **toujours pour priorité de construire** ; il ne transporte que s'il n'a rien d'autre à faire. Le joueur qui incarne un artisan peut construire en regardant les fondations (`E`).

## Ce qui est produit

- **Maison** : un habitat de taille libre, meublé au minimum (un lit par place, une table, un bac d'eau) ; chaque lit est une **place libre** (voir [habitat.md](/regles/habitat.md)).
- **Entrepôt** : un bâtiment dont le tas devient le coffre de l'entrepôt, grand et vide.
- **Taverne** : un bâtiment avec cheminée et tables, qui retient les vagabonds (ce n'est pas un habitat).
- **Forge** : un bâtiment (8×6 au minimum) dont l'**enclume est posée automatiquement** au milieu ; le **coffre** de la forge est posé **à l'extérieur**, contre le mur est, et doit être libre. Elle offre **un poste de forgeron par enclume** (voir [forgeron](/metiers/forgeron.md)). Sans forgeron affecté et sans fer, elle ne produit rien.

## À définir

- Peut-on **annuler** un chantier, et que devient la matière déjà livrée ?
- **Agrandir** un bâtiment existant : par la même interface (on étire un côté et on paie la différence) ?
- Le **coût** et la **durée** exacts de chaque plan (game designer) ; ajouter des **assets** après coup.
- Quand les ressources deviennent disponibles pour un plan, elles sont **mises de côté** (en place : réservées par calcul, pas physiquement) ; comment le roi exprimera des **priorités** plus fines que l'ordre des placements.
- Le tracé d'une route **vers le camp** : en place, le trajet ordinaire qui préfère les routes déjà là.
- Ce que fait **l'IA du roi** quand personne ne l'incarne : elle ne pose aucun plan.
- Les **postes de production** (coffre de bûcheron, de mineur, de paysan, et le tracé d'un champ) sont décidés (spec `hire-production-posts`) : ils se posent sans chantier, ce ne sont pas des bâtiments.
