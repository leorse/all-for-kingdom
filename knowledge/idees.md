---
type: Idées
title: Idées en vrac
description: Idées pas encore décidées (bandits, économie, mobilier, ennemis, arrivants, mine souterraine, mini-jeux). Rien ici n'est une règle.
status: draft
tags: [idees, a-decider]
---

# Idées

Vrac d'idées pour plus tard. Rien ici n'est décidé ni planifié : quand une idée mûrit, elle est décrite dans les [métiers](/metiers/paysan.md) et [règles](/regles/temps-et-repos.md) puis devient un changement OpenSpec.

## Bandits
- Ils circulent dans le royaume, peuvent voler et attaquer.
- Seuls les gardes peuvent se défendre.
- Un voleur qui s'enfuit prend le plus court chemin vers la sortie du château, sans suivre les routes ; les gardes le poursuivent de même (voir [garde](/metiers/garde.md)).
- À définir : ce qu'ils volent, d'où ils viennent, ce que ça coûte au royaume.

## Le roi trace les plans
- Le roi place lui-même les bâtiments : champs, maisons, etc. Un artisan vient les construire, avec du bois et de la pierre.
- Remplace le placement codé en dur dans `scenario.rs` (voir `kingdom-layout`, qui précise que le scénario est une donnée pure).
- Fait partie du gameplay du rôle de roi.
- À terme la carte est vide : le roi place des zones et les routes se tracent seules (inspiration *The Settlers*).
- Détails dans [artisan](/metiers/artisan.md), [roi](/metiers/roi.md) et [royaume](/regles/royaume.md).

## Bûcheron, transporteurs
- Le bûcheron coupe des arbres et produit du bois pour la construction.
- Les transporteurs portent les ressources d'un endroit à un autre : les stocks deviennent des lieux.
- Détails dans [bucheron](/metiers/bucheron.md) et [transporteur](/metiers/transporteur.md).

## Nouveaux arrivants
- De temps en temps, des vagabonds arrivent dans le royaume et sont accueillis à la taverne. Ils ne servent à rien tant qu'ils n'ont pas de rôle.
- Le roi les affecte à un rôle (paysan, forgeron, etc.). Taverne pleine : plus de vagabonds.
- Les arrivants ont des compétences : bons paysans, bons forgerons, chevaliers… Une mauvaise affectation est contre-productive.
- Améliorer la taverne donne plus de lits.
- Détails dans [tavernier](/metiers/tavernier.md) et [competences](/regles/competences.md).

## Mineurs
Décidé dans les grandes lignes : voir [mineur](/metiers/mineur.md). Reste à trancher le nombre de niveaux de qualité, l'usage du cuivre, la capacité des entrepôts.

## Mine souterraine (vue en coupe)
Remplacerait les filons à ciel ouvert actuels (voir [mineur](/metiers/mineur.md), qui décrit l'état en jeu).
- La mine devient l'**entrée d'une grotte** : le mineur finit son travail **sous terre**, en vue en coupe, façon *Minecraft* ou *Core Keeper*. Il creuse des blocs, avec ou sans minerai.
- Filons d'or, de cuivre, etc. cachés dans la roche.
- **Dangers** : creuser réveille des bestioles, le mineur doit se défendre. Sans le joueur, l'IA se défend aussi.
- **Profondeur quasi infinie** : creuser plus profond coûte de plus en plus cher. Cela renouvelle les ressources (la mine ne s'épuise pas, le jeu ne s'arrête pas) et donne des monstres plus forts et plus nombreux, plus de trésors.
- À trancher : un bloc par tuile ou non ; ce qui se passe quand un niveau est vidé ; comment l'IA mine et combat sous terre ; le lien avec le coffre au pied du filon et les transporteurs ; l'équilibre avec le forgeron.
- Dépend de l'[Économie](#économie--pièces-dor-et-taxes) (le coût de la profondeur) : l'économie et la gestion des ressources sont à traiter d'abord ou en même temps.

## Mini-jeux en mode possession
Chaque métier (ou presque) est un prétexte à un mini-jeu quand le joueur possède le personnage. L'IA fait le métier normalement : le mini-jeu apporte un bonus, surtout de qualité, et n'est jamais obligatoire. Idéalement 30 à 60 s.
- [Tavernier](/metiers/tavernier.md) : plus la taverne est entretenue, plus de vagabonds viennent, avec une renommée propre (en plus de celle du royaume) ; des vagabonds qualifiés ou des chevaliers arrivent. Mini-jeu à la *Overcooked* : prendre les ingrédients, cuisiner, répondre aux commandes des clients.
- [Forgeron](/metiers/forgeron.md) : choisir l'objet (si les matériaux sont là), chauffer le métal, frapper aux bons endroits sur la forme visible, tremper. QTE possible.
- [Paysan](/metiers/paysan.md) : choisir la graine, semer, arroser, désherber, traiter les nuisibles, récolter.
- Guerrier : s'entraîner.
- Bûcheron, mineur, pêcheur, meunier : à imaginer. Pistes : rythme de coups et direction de chute de l'arbre ; mine souterraine (ci-dessus).
- À définir : un cadre commun (début, durée, note de 0 à 1 qui module le résultat), la renommée de la taverne comme nouvelle ressource, le lien avec l'économie actuelle.

## Forgeron : liste d'objets et priorités du roi
- Le forgeron a une liste d'objets qu'il sait faire : lingots, outils, armes, pièces d'or, et d'autres plus tard.
- Le roi lui donne des priorités ; il fabrique selon ces priorités et selon son stock.
- Détails dans [forgeron](/metiers/forgeron.md).

## Gestion des stocks
- Chaque métier a un stock limité (le mineur dans sa mine, le paysan, le forgeron...). Stock plein : il arrête de produire.
- Implique d'équilibrer le nombre de travailleurs entre les métiers.
- Détails dans [ressources-et-stocks](/regles/ressources-et-stocks.md).

## Amélioration des maisons
- Une maison s'améliore : confort, sommeil plus rapide, travailleur plus rapide, entrepôt plus grand.
- Probablement lié à l'idée « Mobilier » ci-dessous.
- Détails dans [habitat](/regles/habitat.md).

## Région et royaume ennemi
- Le royaume est dans une région du monde, avec un ennemi qui a les mêmes besoins et les mêmes villageois.
- Il peut envoyer des vagues d'ennemis qui attaquent le royaume.
- Implique une carte plus grande que le royaume et une notion de région.

## Économie : pièces d'or et taxes
- Une monnaie (pièce d'or) et des taxes levées sur les habitants.
- Entretenir un guerrier ou un garde coûte de l'argent.
- À définir : qui perçoit (le roi ?), ce que ça finance d'autre, ce qui se passe quand les caisses sont vides.

## Mobilier et personnalisation de l'habitat
- Chaque personnage a une maison. En mode IA, il ne fait que son travail et dormir : il ne s'occupe pas de son logement.
- Le joueur ne peut changer le mobilier que lorsqu'il possède le personnage (mode possession).
- Le mobilier affecte les performances du personnage (par exemple le repos, donc la forme, et le rendement au travail).
- Chaque meuble coûte de l'argent (voir Économie) : il faut arbitrer entre confort et dépenses.
- À définir : d'où vient l'argent (le personnage gagne-t-il un salaire ?), la liste des meubles et leurs effets, et si on peut revendre un meuble.
