# Design

## Context

Aujourd'hui un « logement » n'est qu'un lit (`Tile::Bed`) posé par `Scenario::village()` dans un rectangle construit par `TileMap::add_building` (murs, sol, une porte). `CharacterSpec::home` désigne la tuile du lit ; les tailles des bâtiments ne suivent aucune règle. Le cœur ne sait pas ce qu'est un habitat, ni combien de places il offre. Voir `proposal.md` et `docs/gameplay/habitat.md`.

## Goals / Non-Goals

**Goals:**
- Un modèle de logement dans le cœur : classes de taille, mobilier obligatoire à taille connue (lit, table et étagère en 2×1, bac en 1×1), places, occupation.
- Remettre le scénario de départ aux normes, avec des tests qui garantissent la conformité de tout futur scénario.
- Poser les deux règles que le gameplay à venir utilisera (embauche, agrandissement) sous forme de fonctions pures testées.

**Non-Goals:**
- Poser un habitat en jeu (roi), faire arriver des vagabonds, agrandir réellement, donner un usage au mobilier, améliorer les maisons, chiffrer la construction, le château (il a son propre fonctionnement).

## Decisions

- **Trois classes dans un module `habitat.rs`.** `HabitatClass { Small, Medium, Large }` avec `size()` (7×5, 9×5, 9×7), `max_places()` (1, 3, 6) et `next()`. Les intérieurs (murs retirés) font 5×3, 7×3 et 7×5, soit 15, 21 et 35 tuiles. Mobilier au maximum de places : petite maison 2 (lit) + 2 (table) + 2 (étagère) + 1 (bac) = 7 tuiles, il en reste 8 ; moyenne 6 + 5 = 11, il en reste 10 ; grande 12 + 5 = 17, il en reste 18. Alternative : des tailles libres validées par une surface minimale — rejeté, l'agrandissement par classes demande des empreintes connues d'avance. L'agrandissement ajoute deux colonnes (petite → moyenne) puis deux rangées (moyenne → grande).
- **Mobilier de tailles fixes.** Un lit, une table et une étagère occupent chacun **2×1** tuiles (largeur 2, hauteur 1), le bac **1×1**. Un lit est repéré par sa **tuile de tête, à gauche** : c'est la tuile `home` du personnage. Tables et étagères sont repérées par leur tuile de gauche.
- **Tuiles par moitié.** Pour que carte, collisions et dessin restent simples (une tuile = un type), chaque meuble de 2×1 est posé en deux tuiles : `Bed` (tête) + `BedFoot` (pied), `Table` + `TableEnd`, `Shelf` + `ShelfEnd`, et `Trough` pour le bac. `Bed` et `BedFoot` sont praticables ; les autres sont solides (`is_solid`). Le rendu choisit le sprite par tuile ; un `Bed` sans `BedFoot` à sa droite (le lit du château) reste dessiné comme aujourd'hui.
- **`HabitatSpec { x, y, class, door, beds, table, shelf, trough }` dans le scénario**, comme les filons et les coffres. `beds`, `table`, `shelf` sont les tuiles de gauche ; `trough` une tuile. `build(map)` pose murs, sol, porte et meubles après validation.
- **Validation (`validate`)** : porte sur un mur non d'angle ; 1 à `max_places` lits ; chaque tuile de chaque meuble dans l'intérieur ; aucune tuile partagée entre deux meubles ; au moins 3 cases libres de tout meuble ; toutes les tuiles de lit joignables depuis la case intérieure derrière la porte en ne traversant que des cases praticables (donc pas la table, l'étagère ni le bac). Elle renvoie des erreurs lisibles, utilisées par les tests de scénario.
- **Places et occupation.** `Game` garde `habitats: Vec<HabitatSpec>` (lecture seule). Une place est un lit ; elle est occupée si un personnage a `home` égal à sa tuile de tête. `free_places_in(habitat)`, `free_places()` et `can_hire()` (`free_places() > 0`) sont calculés à la demande. Au départ, autant de lits que de personnages hébergés : aucune place libre. Le lit du roi (château) est hors habitats.
- **Agrandissement.** `can_expand(map)` calcule l'emprise de la classe suivante ancrée au même coin haut-gauche et exige que chaque case nouvelle soit de l'herbe ou un chemin dans la carte. Pure ; l'action viendra avec le roi.
- **Scénario remis aux normes.** Les positions changent (les pièces sont plus grandes). Coin haut-gauche, classe, porte :
  - caserne : moyenne (19,22), porte (21,22) ; entrepôt central : moyenne (19,15), porte (27,17) reliée à la route par (28..30,17), coffre déplacé en (28,18) ;
  - maison du paysan : (4,23), porte (8,23) reliée à la route par (8,21..22) ; petite, moyenne ou grande selon le nombre de paysans de `village_with_farmers` ;
  - maison du forgeron : petite (54,4), porte (56,8) ; maison du tavernier : petite (54,28), porte (56,28) ;
  - mines : fer (43,5) porte (46,9) ; or (23,29) porte (29,31) ; pierre (45,27) porte (48,27) ;
  - cabanes : nord (8,10), porte à gauche (8,12) sur le chemin des champs ; sud (16,27), porte (18,27) ;
  - le château est inchangé et hors habitats.
  Les chemins, le coffre de la cabane sud (17,32, dans un couloir de la forêt), les postes du bûcheron sud et ceux du chef de guerre et des transporteurs sont ajustés. L'organisation intérieure (lits, table, étagère, bac) de chaque habitat est dans `scenario.rs`, validée par les tests.
- **Rendu.** Lit à deux tuiles (tête avec oreiller, pied avec couverture), table à deux tuiles, étagère à deux tuiles, bac d'eau d'une tuile.

## Risks / Trade-offs

- [Les habitats plus grands rapprochent ou chevauchent des éléments voisins] → test de non-chevauchement entre habitats et avec les éléments du scénario (aucune tuile existante n'est écrasée) ; les positions ci-dessus sont celles qui passent ce test.
- [Le château, exempté, reste hors du modèle] → son lit est explicitement écarté de la vérification et des places libres ; son bâtiment n'est pas modifié.
- [Beaucoup de tests de `game.rs` citent des positions de lits ou de postes] → mises à jour ciblées ; la plupart passent par `id_of` et des constantes du scénario.
- [Un coffre en couloir de forêt (cabane sud)] → la proximité à la forêt reste vérifiée (au plus deux cases d'un arbre).

## Open Questions

- Le fonctionnement propre du château (palais, lits de la cour, cuisine…) est à définir plus tard ; en attendant il est hors du modèle d'habitat.
- Un usage des meubles (repas à la table, soif au bac, rangement à l'étagère) arrivera avec le confort des maisons.
