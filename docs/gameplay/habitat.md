# La maison et son entrepôt

Chaque personnage a une **maison** avec un lit. C'est là qu'il dort, et qu'il revient chaque soir.

## Aujourd'hui — en place

- Les maisons sont **loin du centre**, vers les bords du royaume : maison du paysan au sud-ouest, du forgeron au nord-est, du tavernier au sud-est. Les gardes et le chef de guerre dorment à la caserne, le roi au château.
- Se coucher dans son lit fait récupérer la forme. Voir [temps-et-repos.md](temps-et-repos.md).
- Le joueur incarné peut se coucher près de son lit.
- Le **château** a son propre fonctionnement : il n'entre pas dans ce qui suit.

## En place : des habitats à taille fixe

Tout bâtiment où l'on dort (sauf le château) est un **habitat** de l'une de ces trois tailles, murs compris :

| Classe | Taille | Places (lits) |
|---|---|---|
| Petite maison | 7×5 (intérieur 5×3) | 1 |
| Maison moyenne | 9×5 (intérieur 7×3) | jusqu'à 3 |
| Grande maison | 9×7 (intérieur 7×5) | jusqu'à 6 |

- Chaque habitat contient **au moins un lit, une table, une étagère et un bac d'eau**, dans son intérieur. **Le lit, la table et l'étagère font 2 tuiles (2×1, ou 1×2 une fois tournés), le bac 1.** Tout meuble se pose dans quatre rotations : un lit peut être vertical, tête en haut, comme sur l'image de référence ([art-direction.md](../art-direction.md)). La table, l'étagère et le bac sont infranchissables ; les lits se traversent. Tous les lits restent accessibles depuis la porte.
- Une **place**, c'est un lit (sa tête, à gauche, désigne la place). Dans un habitat à plusieurs places, les lits se partagent **la même table, la même étagère et le même bac** (pièce commune).
- **Chaque personne a son lit**, jamais deux dans le même. Les mines, les cabanes et les maisons du paysan, du forgeron et du tavernier sont des petites maisons ; la caserne (chef de guerre et gardes) et l'entrepôt central (transporteurs) sont des maisons moyennes. La maison du paysan grandit avec leur nombre : petite pour un, moyenne jusqu'à trois, grande jusqu'à six.
- La table, l'étagère et le bac ne servent à rien d'autre pour l'instant (repas, soif, rangement et confort viendront plus tard) : ils comptent déjà dans la place qu'il faut prévoir.

### Places libres, embauche, agrandissement

- Le jeu sait compter les **places libres** : les lits que personne n'occupe. Au départ il n'y en a aucune.
- **On ne peut embaucher un vagabond de la taverne que s'il reste une place libre** quelque part dans le royaume.
- Un habitat pourra être **agrandi** (petite → moyenne → grande), mais seulement si l'espace autour est libre : l'emprise de la classe suivante, depuis le même coin haut-gauche, ne doit couvrir que de l'herbe ou des chemins (un arbre, de l'eau, un mur, un coffre, un champ ou un filon l'interdit). Le jeu sait vérifier cette place ; **l'action d'agrandir n'existe pas encore**.
- Quand le **roi** posera un nouvel habitat, ce sera le logement d'un nouvel arrivant (pas encore dans le jeu, comme l'arrivée des vagabonds à la taverne).

## En place : des coffres au lieu d'extraction

Chaque lieu de travail a un **coffre** posé **dehors**, au niveau de l'extraction de la ressource : au pied de chaque filon, en lisière de chaque forêt, à côté du champ du paysan, contre la forge, et à côté de la porte de l'entrepôt central des [transporteurs](transporteur.md). La maison reste le lieu où l'on dort ; le stock est dans le coffre. Voir [ressources-et-stocks.md](ressources-et-stocks.md).

## Décidé : la maison sert d'entrepôt

Pour certains métiers, la maison devait être aussi l'**entrepôt** du travailleur. C'est le cas du [mineur](mineur.md), dont la mine est à la fois sa maison et son stock de minerai. Aujourd'hui le stock est dans un coffre à côté ; le rapprocher de la maison reste possible.

## Décidé : les maisons s'améliorent

Plus tard, on pourra améliorer une maison. Les améliorations prévues :

- **plus de confort** ;
- **un sommeil plus rapide** : la forme remonte plus vite pendant la nuit ;
- **un travailleur plus rapide** ;
- **un entrepôt plus grand**.

## Idées liées

L'idée « Mobilier et personnalisation de l'habitat » dans `IDEAS.md` va dans le même sens : le joueur ne peut aménager la maison que lorsqu'il possède le personnage, chaque meuble coûte de l'argent, et le mobilier influe sur le repos et le rendement. Les améliorations ci-dessus et le mobilier sont probablement deux faces du même système : à rapprocher.

## Où habitent les nouveaux venus ?

Les vagabonds arrivent à la [taverne](tavernier.md). On ne peut en embaucher un que s'il reste une **place libre** quelque part (voir plus haut). Quand le roi posera un nouvel habitat, ce sera le logement d'un nouvel arrivant ; une maison à construire par l'[artisan](artisan.md) ou un agrandissement. Tout cela reste à brancher.

## À définir

- Le **coût** des améliorations, et qui paie (le personnage, le roi).
- Est-ce que **toutes les maisons** s'améliorent, ou seulement celles des métiers de production ?
- Comment se combinent le confort, les meubles et les améliorations ?

## En place : l'aspect des maisons

Les habitats sont dessinés **en coupe**, comme sur l'image de référence (voir [la direction artistique](../art-direction.md)) : une bande de toit de chaume et un mur de rondins au nord, avec des **fenêtres** (une, deux ou trois selon la taille) qui s'éclairent la nuit et une **cheminée de pierre** au feu animé ; des murs de bois sur les côtés ; un mur sud bas avec la porte ; un sol en **planches** ; un **paillasson** derrière la porte. Les meubles sont reconnaissables : lit à oreiller blanc et couverture rouge, table avec assiette et tasse, étagère à livres et flacons, baquet d'eau cerclé avec sa louche. Le dormeur est couché dans le sens de son lit. La cheminée et les fenêtres sont des décors du mur : ils ne prennent aucune place au sol. Le château garde son ancien aspect.
