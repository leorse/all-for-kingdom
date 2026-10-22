# Ressources et stocks

## Aujourd'hui — en place

Le royaume a trois ressources, affichées en haut à droite de l'écran sous forme de **compteurs globaux** :

| Ressource | D'où elle vient | À quoi elle sert |
|---|---|---|
| Blé | récolté par le [paysan](paysan.md) | à rien pour l'instant |
| Fer | stock de départ (60) | consommé par le [forgeron](forgeron.md) |
| Outils | forgés par le forgeron | à rien pour l'instant |

Il n'y a pas de transport : une ressource apparaît dans le compteur dès que le travail est fini. Il n'y a pas de limite.

## Décidé : les ressources du mineur

Le [mineur](mineur.md) apporte trois minerais, un par nature de filon :

- **fer** et **or** : le forgeron en fait des **lingots** ;
- **cuivre** : usage à définir.

## Décidé : la chaîne de production

Le jeu s'organise en chaîne :

> **filon** → minerai → **lingot** → **objet** (outil, arme, pièce d'or…)

Le mineur et le forgeron en sont deux maillons. Le roi pilote le dernier, en disant au forgeron quoi fabriquer d'abord.

## Décidé : chaque métier a un stock limité

Il faudra **gérer les stocks**. Chaque métier a sa propre réserve, avec une capacité maximale :

- Le mineur stocke son minerai dans **sa mine**, qui est aussi sa maison.
- Le paysan et le forgeron auront le même principe.
- **Quand la réserve est pleine, le travailleur s'arrête.** Il ne produit plus tant que la place n'est pas libérée.

C'est un point essentiel : produire sans limite n'a plus de sens. Il faut équilibrer les métiers entre eux. Trop de mineurs pour un forgeron, et les mines se bloquent. Trop peu, et le forgeron manque de fer.

La capacité d'un stock peut grandir grâce aux améliorations de la maison (voir [habitat.md](habitat.md)).

## À définir

- La **capacité** de chaque stock.
- Les ressources sont-elles toujours en **compteur global**, ou chaque stock reste-t-il local à sa maison ? Qui transporte ?
- Les **pièces d'or** : leur circulation, les taxes (voir l'idée « Économie » dans `IDEAS.md`).
- Le **cuivre** : son usage.
- Si le stock de matières premières du forgeron est plein, que fait-il ? Il s'arrête, ou son propre stock d'objets est-il le seul limité ?
