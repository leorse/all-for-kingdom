# Ressources et stocks

## Aujourd'hui — en place

Le royaume a sept ressources. Elles vivent dans des **coffres** (un par lieu de travail, plafonnés à 200 de chaque ressource) ; les compteurs affichés en haut à droite de l'écran en sont le **total**.

| Ressource | D'où elle vient | À quoi elle sert |
|---|---|---|
| Blé | récolté par le [paysan](paysan.md) | à rien pour l'instant |
| Fer | stock de départ (60), puis le [mineur](mineur.md) | consommé par le [forgeron](forgeron.md) |
| Outils | forgés par le forgeron | à rien pour l'instant |

Ce que le mineur et le forgeron produisent entre dans le coffre de leur lieu de travail dès que le travail est fini, sans trajet. Le paysan (son blé) et le bûcheron (son bois) le portent à la main jusqu'à leur coffre. Les [transporteurs](transporteur.md) font circuler le reste. Un coffre plein arrête son producteur.

## En place : le bois du bûcheron

Le [bûcheron](bucheron.md) coupe des arbres (3 bûches chacun), porte ses bûches à la main (9 au plus) et les dépose dans le coffre de sa forêt, en lisière.

## En place : le minerai des mineurs

Les mineurs déposent leur minerai (fer, or, pierre ; le cuivre n'a pas de filon) dans le coffre posé au pied de leur filon, sans marcher jusqu'à lui, tant que le coffre n'est pas plein (200).

## Décidé : les ressources du mineur

Le [mineur](mineur.md) apporte quatre minerais, un par nature de filon :

- **fer** et **or** : le forgeron en fait des **lingots** ;
- **pierre** : sert à construire (voir plus bas) ;
- **cuivre** : usage à définir.

## Décidé : la construction consomme du bois et de la pierre

Un nouveau besoin apparaît : les chantiers de l'[artisan](artisan.md).

- **Bois** : produit par le [bûcheron](bucheron.md).
- **Pierre** : un minerai de plus, extrait par le [mineur](mineur.md).

## Décidé : les transporteurs déplacent les ressources

Les ressources ne vont plus d'un compteur à l'autre sans effort : ce sont les [transporteurs](transporteur.md) qui les portent d'un endroit à un autre. Un stock devient donc un **lieu** (la mine, la forge, un chantier…).

## Décidé : la chaîne de production

Le jeu s'organise en chaîne :

> **filon** → minerai → *(transport)* → **lingot** → *(transport)* → **objet** (outil, arme, pièce d'or…)

Le mineur, le transporteur et le forgeron en sont les maillons. Pour la construction, la chaîne est parallèle : bûcheron et mineur de pierre → transporteur → artisan. Le roi pilote le dernier, en disant au forgeron quoi fabriquer d'abord.

## Décidé : chaque métier a un stock limité

Il faudra **gérer les stocks**. Chaque métier a sa propre réserve, avec une capacité maximale :

- Le mineur stocke son minerai dans **sa mine**, qui est aussi sa maison.
- Le paysan et le forgeron auront le même principe.
- **Quand la réserve est pleine, le travailleur s'arrête.** Il ne produit plus tant que la place n'est pas libérée.

C'est un point essentiel : produire sans limite n'a plus de sens. Il faut équilibrer les métiers entre eux. Trop de mineurs pour un forgeron, et les mines se bloquent. Trop peu, et le forgeron manque de fer.

La capacité d'un stock peut grandir grâce aux améliorations de la maison (voir [habitat.md](habitat.md)).

## À définir

- La **capacité** de chaque stock.
- Le **compteur global** affiché à l'écran disparaît-il, ou reste-t-il comme total des stocks du royaume ? (Le transport est confié aux transporteurs, voir [transporteur.md](transporteur.md).)
- Les **pièces d'or** : leur circulation, les taxes (voir l'idée « Économie » dans `IDEAS.md`).
- Le **cuivre** : son usage.
- Si le stock de matières premières du forgeron est plein, que fait-il ? Il s'arrête, ou son propre stock d'objets est-il le seul limité ?
