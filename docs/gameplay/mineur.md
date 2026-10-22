# Le mineur

Il extrait du **minerai** des filons du royaume. C'est lui qui alimente le [forgeron](forgeron.md). Ce métier est **en place** : filons, mineurs, mine-maison et coffre de la mine.

## En place aujourd'hui

- Trois filons sur la carte (fer, or, pierre), chacun avec sa mine accolée et son mineur. Le cuivre existe comme nature mais n'est pas posé.
- Qualité en **trois niveaux** (1, 2 ou 3 points par tuile) : **3, 5 ou 8 coups de pioche** d'une seconde par unité de minerai, pour la roche pure, moyenne ou impure. La pierre suit les mêmes règles.
- Un mineur a **un filon attitré** : celui de sa mine. Jusqu'à 8 mineurs peuvent se placer autour d'un même filon, un par case.
- Une unité extraite va **tout de suite dans le coffre de la mine**, posé **à côté du bâtiment** (le mineur n'a pas à y aller). Le coffre contient au plus **200** unités de son minerai : **une fois plein, le mineur ne mine plus** et attend qu'un [transporteur](transporteur.md) en emporte. Les transporteurs portent le fer à la forge, le reste à l'entrepôt central.
- La mine est pour l'instant seulement la **maison** du mineur (son lit).
- Le fer de départ (60) est conservé.

Ce qui suit en « décidé » et n'est **pas encore fait** : l'amélioration de la mine (confort, entrepôt plus grand). Le coffre est aujourd'hui **dehors**, et non dans la mine elle-même.

## Les filons — décidé

- Un filon est un **bloc de 2×2 tuiles** posé sur la carte.
- Il est **infini** : il ne s'épuise jamais.
- Il y en a de quatre natures, reconnaissables au premier coup d'œil :
  - **gris** : le fer ;
  - **cuivré** : le cuivre ;
  - **doré** : l'or ;
  - **pierre** : la pierre, utilisée pour construire (voir [artisan.md](artisan.md)). Sa couleur reste à définir.
- Sa **qualité** est indiquée par des **points** sur ses tuiles. Ces points sont les **impuretés** : plus il y en a, moins la roche est pure, et plus c'est long à extraire.

La nature et la qualité sont donc visibles sans avoir à miner. Choisir où installer un mineur, c'est choisir un filon avec ses yeux.

### Pourquoi 2×2

Un bloc de 2×2 a **8 cases tout autour**. Il peut donc être attaqué par **jusqu'à 8 mineurs** en même temps, un par case.

## Miner — décidé

- Le mineur donne des **coups de pioche** sur un filon, depuis une case voisine, en le regardant.
- Chaque **qualité** de roche correspond à un **nombre de coups de pioche** nécessaire pour extraire **une unité de minerai**. Une roche pure demande peu de coups, une roche pleine d'impuretés en demande beaucoup.
- Une unité de minerai extraite va dans le stock de sa mine.
- Le joueur mine 2 fois plus vite que l'IA, comme pour tous les métiers.
- La fatigue joue comme pour les autres : un mineur épuisé va plus lentement.

## La mine : sa maison et son entrepôt — décidé

La mine est **la maison du mineur**. Elle sert aussi d'**entrepôt de minerai**.

- Le mineur y dort et y revient chaque soir.
- Il y dépose ce qu'il extrait. Le minerai est **limité par la capacité de l'entrepôt**.
- **Une fois la limite atteinte, il ne mine plus.** Il attend qu'on vide l'entrepôt (le forgeron en consomme, ou le roi en emporte).
- Plus tard, la mine pourra être **améliorée** : plus de confort, sommeil plus rapide, mineur plus rapide, entrepôt plus grand. Voir [habitat.md](habitat.md).

La mine est attenante à son filon : on la construit contre lui.

## Ce que ça change dans le jeu

- Le fer cesse d'être un stock de départ fini : il devient une production continue.
- L'or arrive dans le royaume, ce qui prépare l'économie (pièces, taxes). Voir [ressources-et-stocks.md](ressources-et-stocks.md).
- Le royaume doit désormais équilibrer **le nombre de mineurs** avec **la capacité du forgeron** à transformer leur minerai.

## À définir

- Le **cuivre** : à quoi sert-il ? Pour le fer et l'or, on sait que le forgeron en fait des lingots.
- La **capacité de départ** d'un entrepôt de mine.
- Les **8 postes** autour d'un filon sont aujourd'hui tous utilisables : seront-ils limités par la taille de la mine ?
- Le **fer de départ** du royaume : on le garde, on le réduit, on le supprime ?
- Où placer les mines sur la carte ? D'abord dans le scénario fixé à l'avance, puis par le roi (voir [roi.md](roi.md)).
