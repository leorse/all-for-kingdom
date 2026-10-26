# Le paysan

Il cultive les champs et produit du **blé**.

## Les parcelles — en place

Chaque case de champ est une parcelle. Elle passe par ces étapes :

> en friche → **labourée** → **semée** → (pousse) → **mûre** → **récoltée** → en friche

- Labourer, semer et récolter sont des travaux qui prennent du temps (une barre de progression s'affiche).
- La **pousse** prend 12 heures de jeu. Elle ne dépend que du temps : personne ne peut l'accélérer, et elle continue la nuit.
- Chaque parcelle récoltée donne 4 blé.

## Le joueur — en place

- Il travaille la parcelle qu'il regarde, sinon celle sous ses pieds, avec la touche d'action. La parcelle visée est entourée en blanc.
- Bouger ou rappuyer sur la touche annule le travail en cours.
- Il travaille 2 fois plus vite que l'IA.

## L'IA — en place

Elle choisit toujours la parcelle la plus utile, dans cet ordre :

1. récolter ce qui est mûr ;
2. semer ce qui est labouré ;
3. labourer ce qui est en friche.

Quand tout pousse et qu'il n'y a rien à faire, elle se promène dans ses champs.

**Plusieurs paysans** se répartissent les parcelles : chacun réserve celle qu'il vise et les autres l'évitent. Sans cela, tous se rueraient sur la même.

## Équilibre actuel

Pour 48 parcelles :

- un paysan est saturé (13 h de travail par jour) et récolte environ 116 blé par jour ;
- **deux paysans** suffisent (environ 202 blé par jour) ;
- au-delà, c'est la terre qui plafonne (12 h de pousse), et les paysans en trop n'ont plus rien à faire. Cela servira quand il faudra répartir les gens entre les métiers.

## Où va le blé — en place, à faire évoluer

Le blé récolté est pris **en main** par le paysan (20 au plus, soit cinq récoltes) : il doit le **porter** jusqu'au **coffre du champ**, posé **à côté du champ**, jusqu'à 200. Mains pleines, il va livrer ; si le coffre est plein, il garde son blé et continue de labourer et de semer, mais ne récolte plus (les parcelles mûres attendent). Couché avec du blé, il le livre à son réveil. On voit un bloc jaune et la quantité sur lui. Les transporteurs emportent ensuite le blé à l'entrepôt central.

## Décidé

- Le paysan aura **une limite de stockage**, comme le mineur : une fois la limite atteinte, il arrête de récolter. Le principe est expliqué dans [ressources-et-stocks.md](ressources-et-stocks.md).
- Sa maison pourra être améliorée (voir [habitat.md](habitat.md)).

## À définir

- Où est stocké le blé : la maison du paysan, ou un grenier à part ?
- À quoi sert le blé (nourrir les habitants, être vendu ou taxé…) ?
