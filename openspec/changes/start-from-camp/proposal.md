# Proposal

## Why

Le jeu démarre aujourd'hui dans un village déjà bâti. Le gameplay décidé (`knowledge/regles/depart-et-camp.md`, `construction.md`, `batiments.md`) est tout autre : la carte est sans bâtiment, la partie commence au camp de base avec le roi et deux artisans, et c'est le roi qui pose les plans. Il faut le vrai début de partie, de la pose d'un plan à son bâtiment fini, avant d'y brancher le reste (embauche, métiers, économie).

## What Changes

- **Le départ de partie** : une carte sans bâtiment mais avec sa nature (lacs, forêts, filons, arbres). Au milieu, un **camp de base** : place de terre battue, feu, trois lits et un coffre qui contient tout (100 de chaque ressource au plus). Le roi et deux artisans y dorment. Un **pécule** (bois, pierre, graines) est dans le coffre. Pas d'entrepôt. Le village préconstruit reste le banc d'essai (`--village`).
- **Les plans du roi** : trois types (maison, entrepôt, taverne), chacun avec une taille minimale. Le roi place un rectangle de la taille qu'il veut au-dessus du minimum. Le coût en bois et en pierre et la durée de construction suivent la surface, le type et les assets.
- **Le chantier** : les fondations (solides) apparaissent à la pose. Le chantier n'avance que si **toutes** les ressources sont disponibles : elles sont alors portées en tas devant le chantier ; quand tout est là, les artisans construisent (à plusieurs, les travaux s'additionnent). Un artisan construit d'abord ; libre, il fait le travail d'un transporteur. Le bâtiment prend la place des fondations et une **route** le relie au camp.
- **Les maisons de taille libre** : au-dessus d'un minimum (6×5), avec le nombre de lits que la place permet ; lit, table et bac d'eau posés automatiquement.
- **L'entrepôt** (capacité 1000 par ressource) et **la taverne** (qui retient les vagabonds).
- **Les vagabonds** passent par le camp : ils arrivent par le bord de la carte, se promènent, puis repartent ; une taverne les retient quatre fois plus longtemps.
- **L'interface du roi**, à thème médiéval : liste de plans, rectangle bleu (payable), rouge (pas assez de ressources) ou gris (impossible), coût et durée en direct.
- Hors périmètre : embaucher un vagabond et lui attribuer un lit ou un métier, les plans de bûcheron, de mine, de champ et de forge, les graines consommées au semis, l'ajout d'assets après coup, l'agrandissement d'un bâtiment existant, l'annulation d'un chantier, l'IA du roi qui poserait des plans.

## Capabilities

### New Capabilities
- `game-start`: la carte de départ, le camp, le pécule, les personnages de départ, les vagabonds.
- `construction`: les plans, le coût, la durée, les fondations, la livraison, la construction, les routes, les bâtiments produits.
- `plan-interface`: le panneau des plans et la pose d'un plan par le roi.

### Modified Capabilities
- `housing`: une maison peut avoir une taille libre au-dessus d'un minimum.

## Impact

- `afk-core` : nouveau module `building` (plans, coûts, chantiers) ; `character` (rôles artisan et vagabond, travail de construction partagé) ; `container` (coffre du camp, tas de chantier) ; `habitat` (maison de taille libre, étagère facultative) ; `game` (`PlacePlan`, chantiers, vagabonds) ; `scenario` (`Scenario::camp`) ; `map` (feu de camp, fondations) ; `config`, `resources`, `transport`.
- `afk-game` : `plan_ui`, `theme`, `construction_view` ; `habitat_view` (entrepôt et taverne), `render`, `input`, `main`.
- `knowledge/` : `regles/*`, `metiers/*`, `moteur/*`, `graphisme/*`, `log.md` ; `README.md`.
