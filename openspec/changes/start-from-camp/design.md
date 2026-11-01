# Design

## Context

`Scenario::village()` pose tout : bâtiments, habitants, coffres. Le cœur sait déjà ce qu'est un habitat (classes fixes), un coffre, un transporteur. Rien ne sait construire. Les règles décidées sont dans `knowledge/regles/` (`depart-et-camp`, `construction`, `batiments`, `propriete-et-affectation`).

## Goals / Non-Goals

**Goals:** le vrai début de partie jouable, de bout en bout : camp, pécule, plan posé par le roi à la souris, fondations, livraison, construction, bâtiment fini relié par une route ; une interface lisible à thème médiéval ; tout le comportement testé dans `afk-core`, sans fenêtre.

**Non-Goals:** embauche, attribution des lits et des métiers, plans de production (bûcheron, mine, champ, forge), consommation des graines, ajout d'assets, agrandissement, annulation, IA du roi.

## Decisions

- **Un module `building`** : `PlanKind` (maison, entrepôt, taverne), `Cost`, `plan_cost` et `plan_build_secs` (surface × réglage par type + assets de base + lits en plus), `check_plan` (taille, lits, terrain, personnages dans l'emprise), `Site` (le plan posé et son état) et `Building` (entrepôt ou taverne finis). Les réglages sont dans `Config` (tableaux indexés par `PlanKind::index`), à calibrer avec le game designer.
- **Un chantier est un état, pas un lieu à part.** `SiteState { Waiting, Delivering, Building, Done }`. À la pose, la zone devient `Tile::Foundation` (solide : elle bloque le passage). Chaque pas, `update_sites` démarre la livraison d'un chantier en attente si le royaume peut payer (`available` : le stock des coffres moins ce que les chantiers en livraison attendent encore), dans l'ordre des placements, sans qu'un chantier trop cher bloque les suivants.
- **Le tas de matériaux est un contenant.** À la livraison, un `Container` de genre `Site(i)` apparaît devant le chantier (`Tile::Crate`) et ne reçoit que ce qu'il lui faut (`needs`). Le planificateur de transport existant le traite comme un consommateur (`wants`) : transporteurs et artisans libres l'alimentent sans code de livraison à part. Le chantier passe en construction quand le tas est complet. Les contenants ne disparaissent jamais (les indices sont stables) : un tas fini est `retire`, ou devient le coffre de l'entrepôt (`become_depot`).
- **Construire, c'est un travail partagé.** `WorkKind::Build` additionne les coups de tous les artisans du chantier, comme l'abattage des arbres : `Game::build_sites` somme les `build_delta` et finit le chantier à 100 %. Le joueur qui incarne un artisan construit en regardant les fondations (`Interact`).
- **L'artisan.** Sa routine regarde d'abord s'il y a un chantier en construction (priorité absolue) ; sinon il exécute la routine du transporteur, qui sait déjà trouver le meilleur voyage, puis traîne au camp.
- **La maison de taille libre.** `HabitatSpec` garde ses classes pour le village mais gagne `w`, `h` et `free` ; l'étagère devient facultative. `HabitatSpec::house(x, y, w, h, beds)` range les lits par rangées contre le mur nord (une rangée sur deux), la table et le bac d'eau de part et d'autre de la porte, au milieu du mur sud, et valide. La maison bâtie est donc un `HabitatSpec` ordinaire : places libres, embauche et dessin fonctionnent sans changement.
- **Entrepôt et taverne** : un `Building` (murs, porte, mobilier solide). L'entrepôt reçoit le coffre du chantier comme coffre de dépôt (`ContainerKind::Depot`, capacité 1000). Le coffre du camp est `ContainerKind::Camp` : même comportement de dépôt, capacité 100.
- **La route** : à la fin d'un chantier, `lay_road` trace depuis la case devant la porte jusqu'à la place du camp le trajet ordinaire (qui préfère les routes existantes) et change l'herbe en chemin.
- **Les vagabonds** : `Role::Vagabond`, sans lit, sans heure de dormir, créés par `Game` à intervalle régulier (`spawn_vagabonds`) sur un bord praticable, dans la limite de `vagabond_max`. Ils vont vers un point au hasard autour du camp, ou vers la taverne s'il y en a une ; leur temps écoulé (quatre fois plus lent avec une taverne), ils partent vers le bord et sont marqués `gone` (jamais retirés de la liste, pour que les `CharacterId` restent des indices).
- **Une commande, `PlacePlan`.** Le frontend ne décide d'aucune règle : il envoie le rectangle, le cœur valide (le joueur incarne un roi, `check_plan`). `Game` expose de quoi dessiner l'interface : `plan_cost`, `plan_secs`, `check_plan`, `available`, `can_afford`, `is_king`, `sites`, `buildings`.
- **L'interface** (`plan_ui`, `theme`, `construction_view`) : panneau de cartes à la souris ; mode « déplacer » (le rectangle suit le curseur), clic pour fixer le coin, mode « agrandir » (la souris étire), clic pour valider ; bleu, rouge ou gris ; barre d'information (coût, stock, durée, lits) ; Échap annule l'étape, puis le plan. Le temps ne s'arrête jamais. Un clic pris par l'interface n'incarne personne.

## Risks / Trade-offs

- [Le pécule dépasse vite : bois 90 pour un coffre de 100] → réglages dans `Config`, à calibrer ; aucun producteur n'est constructible dans cette étape, donc le pécule est la seule ressource.
- [Le tas d'un chantier peut tomber sur la place du camp, occupée] → le chantier attend que la case soit libre.
- [Les vagabonds ne servent à rien sans embauche] → ils animent le camp ; l'embauche et l'attribution viendront ensuite (`propriete-et-affectation.md`).
- [Le coût et les durées sont des ordres de grandeur] → testés en relatif (la surface et le type changent le coût et la durée), pas en absolu.

## Open Questions

- Les valeurs de coût, de durée et du pécule (game designer).
- Qui livre quand plusieurs chantiers se disputent le même coffre ; comment le roi exprimera ses priorités.
- L'agrandissement d'un bâtiment existant par la même interface.
