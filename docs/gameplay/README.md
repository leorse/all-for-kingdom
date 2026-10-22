# Gameplay d'All For Kingdom

Cette documentation décrit **le jeu du point de vue de celui qui y joue** : les métiers, les ressources, les règles. Elle ne parle pas de code. Elle sert à discuter et à décider du gameplay avant de l'implémenter.

Chaque page distingue trois états :

- **En place** : ça existe dans le jeu aujourd'hui.
- **Décidé** : c'est arrêté, mais pas encore dans le jeu.
- **À définir** : une question ouverte, avec ce qu'on sait déjà.

Les idées en vrac, pas encore décidées, sont dans `IDEAS.md` à la racine.

## Le principe

On incarne tour à tour les habitants d'une cité médiévale. Ceux qu'on n'incarne pas sont joués par l'IA, qui leur fait mener leur journée : travailler le jour, dormir la nuit. Sans personne d'incarné, la cité tourne seule : c'est le mode AFK.

## Les pages

**Les métiers**

| Métier | Page | État |
|---|---|---|
| Paysan | [paysan.md](paysan.md) | en place |
| Forgeron | [forgeron.md](forgeron.md) | en place, à étoffer |
| Mineur | [mineur.md](mineur.md) | décidé, pas encore en jeu |
| Garde | [garde.md](garde.md) | en place (rondes seulement) |
| Roi | [roi.md](roi.md) | pas de travail propre encore |
| Chef de guerre | [chef-de-guerre.md](chef-de-guerre.md) | pas de travail propre encore |
| Tavernier | [tavernier.md](tavernier.md) | pas de travail propre encore |

**Les règles communes**

- [Temps, repos et fatigue](temps-et-repos.md) : le cycle jour/nuit, la forme, le sommeil, l'incarnation.
- [Ressources et stocks](ressources-et-stocks.md) : ce que le royaume produit et stocke, et où.
- [La maison et son entrepôt](habitat.md) : le logement de chaque personnage, et ce qu'il deviendra.
- [Le royaume](royaume.md) : la carte et la disposition des lieux.
