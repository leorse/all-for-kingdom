# Design

## Context

Aujourd'hui les bûcherons, mineurs et paysans n'existent que dans `Scenario::village()` : chacun reçoit sa forêt, son filon, son champ et un contenant fixés au départ (`ContainerKind::Lodge(i)`, `Mine(i)`, `Farm`). Les contenants acceptent 200 de **chaque** ressource ; l'entrepôt a un grand contenant à l'extérieur (`warehouse_capacity`). Au camp, la carte a des forêts, des filons, aucun champ, et des vagabonds que `hire-and-assign` sait affecter à des postes de bâtiments. Il n'y a ni coffre posable, ni champ qu'on pose, ni choix de culture.

## Goals / Non-Goals

**Goals:** au camp, dès le début, embaucher un bûcheron, un mineur ou un paysan avec un simple coffre ; un modèle de coffre cohérent (une ressource par coffre, grands coffres de l'entrepôt, dépôt au plus proche) ; le champ de 3×3 posé nu ; le choix de culture réservé au paysan ; tout testable sans fenêtre.

**Non-Goals:** mine souterraine, nouvelles cultures, déplacer ou détruire un coffre, compétences, salaires, IA du roi, équilibrage.

## Decisions

- **Le coffre est un objet.** `Command::PlaceChest { player, resource, tile }` : le joueur incarne le roi, la tuile est libre (herbe ou chemin, ni fondation, ni tas, ni personne), le bois (`chest_wood`) est retiré du coffre du camp, la pose est **immédiate** (aucun chantier, aucun artisan). Un coffre = un `Container` de nouveau genre `Chest(i)` avec `resource: Resource` et capacité 200 de cette ressource. *À confirmer* : un artisan pourrait le construire, pour rester cohérent avec « le roi place, l'artisan construit ».
- **Un coffre, une ressource.** Un contenant `Chest` n'accepte que sa ressource. Sont des exceptions : le **coffre du camp** (tout, 100 de chacune), les contenants du **village de départ** (inchangés), et l'**entrepôt**.
- **L'entrepôt.** Un entrepôt bâti ne pose plus de coffre extérieur : `finish_site` crée `warehouse_chests` **grands coffres** (réglage) à l'intérieur, chacun un `Container` de genre `BigChest { warehouse, i }` qui accepte **tout type**, avec une capacité de **200 au total** (somme de tous les types). La forme actuelle (`Depot`, `warehouse_capacity` 1000 par ressource) est remplacée au camp ; le village garde son dépôt. Les grands coffres se dessinent à l'intérieur du bâtiment ; on n'y entre pas.
- **La porte suffit.** Déposer ou prendre dans un entrepôt se fait dès que le personnage **touche la tuile de la porte** (même logique que l'accès à un coffre depuis une case voisine : la porte est la case d'accès de tous ses grands coffres). Aucun chemin n'est calculé jusqu'à l'intérieur. Le transport actuel (`Job`, réservations) cible la porte ; le choix du grand coffre se fait à l'arrivée (celui qui a de la place, ou qui contient la ressource demandée).
- **Dépôt au plus proche.** Un producteur sans joueur porte sa production ; quand il doit déposer, il choisit le **contenant le plus proche (distance de chemin) qui accepte la ressource et n'est pas plein** : un `Chest` de cette ressource, ou l'entrepôt par sa porte. Il garde sa cargaison s'il n'y en a pas. Le mineur au camp porte lui aussi (fini le dépôt à distance) ; au village, rien ne change.
- **Un coffre près de sa matière est un poste.** À la pose, le jeu cherche la matière à portée (`chest_reach`) : une tuile de forêt pour un coffre de bois, une case d'accès d'un filon **de la nature du coffre** pour un coffre de minerai, une parcelle d'un champ pour un coffre de blé. S'il la trouve, `Game::posts()` inclut `Post { role, tile (la tuile du coffre), unlimited: false }` ; sinon le coffre est un simple stockage. Un seul titulaire ; remplacer, réaffecter, mini-carte : comme `hire-and-assign`. `Role::has_posts` s'étend aux trois métiers : au camp, sans poste ils ne travaillent pas (`idle_routine`) ; au village (`posts_required` faux), rien ne change.
- **La zone de travail, c'est le coffre.** Un bûcheron coupe l'arbre le plus proche de **son** coffre dans la forêt qui le touche (`work_reach`) ; un mineur frappe le filon de la nature de son coffre le plus proche ; un paysan entretient les champs qui dépendent de **son** coffre. Plusieurs coffres peuvent servir la même forêt ou le même filon.
- **Le mineur n'est pas lié à la surface.** Le lien d'un poste de mineur est un `MineSite`, qui vaut aujourd'hui `Surface(vein)` ; la mine souterraine ajoutera un autre cas sans toucher au poste.
- **Le champ.** `Command::PlaceField { player, x, y }` (roi ou paysan incarné) : gratuit, immédiat, 3×3 d'herbe libre, sinon refusé. Neuf `Tile::Field` en friche ; `Farm` accepte des parcelles en cours de partie. Un champ dépend du coffre de blé le plus proche à moins de `field_reach` (recalculé à chaque pose de coffre ; égalité : le premier posé) ; sans coffre à portée, il attend. Le paysan ne travaille que **ses** champs ; la réservation actuelle des parcelles reste.
- **La culture.** Chaque parcelle porte une `Crop` au semis. Le **paysan seul** la choisit : `Command::ChooseCrop { player, crop }` n'est accepté que d'un paysan incarné. L'IA sème la culture dont le royaume a le plus de graines, à défaut la dernière cultivée. Tant qu'il n'existe qu'une graine, rien n'est proposé : seul le blé est implémenté ; `Seeds` devient un compte par culture à l'arrivée d'une seconde.
- **Interface.** Carte « Coffre » (tuile unique ; menu de la ressource) et carte « Champ » (3×3, gratuit) dans la liste des plans ; couleurs bleu/rouge/gris et motif du refus ; la carte d'un coffre annonce le poste qu'il fournit. Grands coffres dessinés dans l'entrepôt. Choix de culture proposé au paysan incarné quand il y a plusieurs graines.
- **Réglages** dans `Config` : `chest_wood`, `chest_capacity` (200), `chest_reach`, `work_reach`, `field_reach`, `warehouse_chests`, `big_chest_capacity` (200).

## Risks / Trade-offs

- [Un coffre posé au mauvais endroit est définitif] → hors périmètre ; le coût du coffre est faible, la carte explique ce qu'il fournit.
- [Plusieurs coffres de blé proches se disputent un champ] → le plus proche l'emporte ; égalité : le premier posé.
- [Le nombre de grands coffres d'un entrepôt] → réglage fixe pour l'instant ; il pourrait dépendre de la taille du bâtiment.
- [Le transport ciblait un grand contenant extérieur] → la cible devient la porte ; les réservations de transporteurs doivent tenir compte de la somme 200 d'un grand coffre.
- [Le mineur du camp porte son minerai] → ses rendements baissent (trajets) ; à équilibrer avec le coffre au pied du filon.
- [Les artisans réaffectés ne construisent plus] → inchangé depuis `hire-and-assign`.
