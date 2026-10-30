# All For Kingdom : consignes pour Claude

## La base de connaissances `knowledge/` est la source de vérité

`knowledge/` est un bundle [OKF](https://github.com/GoogleCloudPlatform/open-knowledge-format) v0.2 : markdown avec en-tête YAML, un `type` obligatoire, des `sources` qui pointent vers le code. Elle contient les métiers, les règles de jeu, le moteur, le graphisme et les idées. **Commencer par `knowledge/index.md`.** Elle remplace l'ancien `docs/gameplay/`, `docs/art-direction.md` et `IDEAS.md`.

Hiérarchie en cas de désaccord : **code > `knowledge/` > specs OpenSpec (`openspec/specs/`)**. `knowledge/idees.md` n'est jamais une règle.

### À chaque changement du jeu

Dans **le même changement** que le code ou la décision :

1. mettre à jour la page concernée de `knowledge/` (règle, valeur de référence, apparence, structure du code) ;
2. vérifier ses `sources` (les fichiers et symboles existent toujours) et son `etat_en_jeu` (en place, décidé, à définir) ;
3. ajouter une ligne datée à `knowledge/log.md` ;
4. créer une page pour tout nouveau concept (métier, règle, module) et la lister dans `knowledge/index.md`.

Format d'une page : `type` obligatoire ; `title`, `description`, `tags`, `sources` recommandés ; liens absolus depuis la racine du bundle (`[texte](/metiers/paysan.md)`). Les valeurs d'équilibrage vivent dans `crates/afk-core/src/config.rs` et ne se recopient dans les pages que si c'est indispensable. Écrire en français, comme le reste du dépôt.

## Agents et skills

- `afk-game-designer` (skill `afk-regles-du-jeu`) : règles, équilibrage, calculs de durées.
- `afk-graphiste` (skill `afk-graphisme`) : dessin, sprites, animations, thème, tailles.

Les skills ne dupliquent pas la base : ils donnent la méthode et renvoient à `knowledge/`.

## Le projet

Deux crates : `afk-core` (règles, sans graphisme) et `afk-game` (macroquad, aucune règle). Voir `knowledge/moteur/architecture.md`. Les changements de comportement durable passent par OpenSpec (`openspec/`).
