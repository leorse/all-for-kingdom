# Design

## Context

Aujourd'hui les bûcherons, mineurs et paysans n'existent que dans `Scenario::village()` : chacun reçoit sa forêt (`Character::forest`), son filon (`vein`) et un contenant (`ContainerKind::Lodge(i)`, `Mine(i)`, `Farm`) fixés au départ. Au camp, la carte a des forêts, des filons, aucun champ, et des vagabonds que `hire-and-assign` sait affecter à des postes de bâtiments. Il n'y a ni coffre posable, ni champ qu'on trace, ni choix de culture (`Resource::Seeds` est unique, le blé est la seule récolte).

## Goals / Non-Goals

**Goals:** pouvoir, **au camp, dès le début**, embaucher un bûcheron, un mineur ou un paysan avec un simple coffre ; le champ défini par le roi ou le paysan ; le choix de culture réservé au paysan ; tout testable sans fenêtre.

**Non-Goals:** mine souterraine, nouvelles cultures, déplacer ou détruire un coffre, compétences, salaires, IA du roi, équilibrage.

## Decisions

- **Un coffre-poste est un poste.** `Game::posts()` gagne les coffres posés par le roi : `Post { role, tile (la tuile du coffre), unlimited: false }`, un par coffre. Le rôle vient du type de coffre posé (bûcheron, mineur, paysan). Un seul titulaire ; remplacer, réaffecter, mini-carte : comme `hire-and-assign`, sans nouvelle règle. `Role::has_posts` s'étend aux trois métiers : au camp, sans poste ils ne travaillent pas (`idle_routine`) ; au village de départ (`posts_required` faux), rien ne change.
- **Poser le coffre.** `Command::PlaceChest { player, kind: ChestKind {Lodge, Mine, Farm}, tile }` : le joueur incarne le roi, la tuile est libre, et **à portée** de la bonne matière (`chest_reach`, en tuiles : une tuile de forêt pour le bûcheron, une case d'accès d'un filon pour le mineur, une parcelle du paysan **ou** une tuile libre assez près pour qu'on y trace un champ). Le coût (`chest_wood`) est retiré du coffre du camp tout de suite ; la pose est **immédiate** (pas de chantier, pas d'artisan : un coffre n'a pas de fondations). *Décision à confirmer avec le créateur* : un artisan pourrait le construire, pour rester cohérent avec « le roi place, l'artisan construit ».
- **La zone de travail, c'est le coffre et ce qu'il y a autour.** Un bûcheron coupe l'arbre le plus proche de **son** coffre dans la forêt qui le touche (`work_reach` en tuiles) ; un mineur frappe le filon le plus proche à portée de **son** coffre ; un paysan travaille les parcelles qui dépendent de **son** coffre. Les conteneurs existants (`Lodge(i)`, `Mine(i)`, `Farm`) sont réutilisés : un coffre posé crée un contenant du même genre, **lié** à la forêt, au filon ou au champ qu'il sert. Plusieurs coffres peuvent servir la même forêt ou le même filon (jusqu'à huit mineurs autour d'un filon, déjà vrai).
- **Le coffre du mineur ne reçoit qu'un minerai.** Il prend la nature du filon visé à la pose (fer, or, cuivre, pierre) ; un filon à portée de deux natures n'existe pas (un coffre est posé à côté d'un seul filon : le plus proche).
- **Le mineur n'est pas lié à la surface.** Le lien d'un poste de mineur est un `MineSite` qui vaut aujourd'hui `Surface(vein)` ; la mine souterraine ajoutera un autre cas sans toucher au poste. Aucun poste souterrain n'est proposé tant qu'elle n'existe pas.
- **La surface cultivable.** Un nouveau geste : `Command::MarkField { player, x, y, w, h }`, par le roi ou par un paysan incarné. Les tuiles d'herbe libres du rectangle deviennent des parcelles en friche (`Tile::Field`, comme aujourd'hui) ; la pose est gratuite et immédiate. Une parcelle dépend du coffre-poste de paysan **le plus proche à moins de `field_reach` tuiles** ; elle est recalculée quand un coffre est posé. Une parcelle sans coffre à portée **attend** : visible, mais personne ne la travaille. Le titulaire d'un coffre ne travaille que **ses** parcelles ; plusieurs paysans ne se piétinent donc pas, et la réservation actuelle des parcelles reste.
- **La culture.** Chaque parcelle porte une `Crop` au semis. Le **paysan seul** la choisit : `Command::ChooseCrop { player, crop }` n'est accepté que d'un paysan incarné (le roi est refusé). L'IA du paysan sème la culture dont le royaume a le plus de graines, à défaut la dernière cultivée. Tant qu'il n'existe qu'une graine, le choix n'est ni proposé ni nécessaire : le seul cas implémenté est le blé. Le stock de graines devient un compte **par culture** (`Seeds(Crop)`) quand la seconde arrivera ; d'ici là `Seeds` reste tel quel.
- **Interface.** Trois cartes dans la liste des plans (`Poste de bucheron`, `Poste de mineur`, `Poste de paysan`), tuile unique, même code couleur (bleu payable, rouge manque de bois, gris impossible, avec le motif : « loin de toute foret », « aucun filon a portee »…). Un geste distinct trace la surface cultivable (carte « Champ », gratuit, rectangle libre). Le choix de culture est une petite liste proposée au paysan incarné quand il y a plusieurs graines. La mini-carte montre les coffres-poste comme les autres postes.
- **Réglages** dans `Config` : `chest_wood`, `chest_reach`, `work_reach`, `field_reach`.

## Risks / Trade-offs

- [Un coffre posé sur un mauvais endroit est définitif : on ne peut ni le déplacer ni le détruire] → hors périmètre ; le coût du coffre est faible, le message de la carte explique le refus.
- [Plusieurs coffres de paysan proches se disputent les parcelles] → chaque parcelle va au plus proche ; égalité : le coffre posé le premier.
- [Les artisans réaffectés ne construisent plus] → inchangé depuis `hire-and-assign`.
- [Poser un coffre à la place d'un chantier] → un coffre se refuse là où il y a une fondation, un tas de matériaux ou quelqu'un.
