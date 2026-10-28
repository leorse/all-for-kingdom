# Design

## Context

La recherche de chemin actuelle (`TileMap::find_path`, `map.rs`) est un parcours en largeur en 4 directions : toutes les cases praticables coûtent pareil, la case `Tile::Path` n'est jamais distinguée de l'herbe. Les personnages suivent ensuite la liste de cases (`follow_path`, `character.rs`) en marchant vers le centre de chaque case, avec une collision par boîte (`is_box_blocked`). Une vingtaine d'appels à `find_path` choisissent chacun un but (poste de travail, lit, coffre, point de ronde). Les voleurs et leurs poursuivants n'existent pas encore dans le code ; voir `IDEAS.md`.

## Goals / Non-Goals

**Goals:**
- Un seul algorithme de chemin paramétré par un **mode**, utilisé partout.
- Diagonales sans coin coupé.
- Trajets déterministes (mêmes entrées, même chemin) pour garder les tests stables.

**Non-Goals:**
- Créer les voleurs, leur comportement de vol, la capture ou le combat : seul le mode de déplacement qu'ils utiliseront est fourni et testé.
- Routes dynamiques tracées par le roi.
- Évitement entre personnages.

## Decisions

**A\* à coûts, 8 directions, au lieu du BFS.** Coût d'un pas = longueur du pas (1 en ligne, √2 en diagonale) × coût de la case d'arrivée. Heuristique octile. Alternative : garder le BFS et lisser le résultat ; refusée car un BFS ne sait pas préférer la route.

**Deux modes : `Roads` et `Direct`.** En `Roads`, une route coûte 1 et l'herbe `grass_cost` (réglage de `config.rs`, valeur de départ 3). En `Direct`, toutes les cases praticables coûtent 1 : c'est le plus court chemin en distance. Le surcoût fini de l'herbe (plutôt qu'une interdiction) donne d'un coup les comportements voulus : suivre la route, la quitter pour un but qui n'y est pas (le transporteur n'a donc pas de cas particulier), et ne pas faire un détour disproportionné. Alternative : interdire l'herbe sauf aux extrémités ; refusée, fragile (un champ sans route ne serait plus atteignable) et inutilement dure.

**Choix du mode à l'appel.** `find_path(from, to, mode)`. Tous les trajets ordinaires, y compris `go_to_bed`, utilisent `Roads`. `Direct` est réservé à la fuite du voleur et à la poursuite des gardes ; le comportement de ces deux rôles s'écrira dans un changement ultérieur, qui n'aura qu'à passer ce mode. La « sortie du château » est un but fourni par le scénario.

**Pas de coin coupé.** Un pas diagonal n'est généré que si les deux cases orthogonales adjacentes sont praticables. `follow_path` garde sa collision par boîte : une diagonale entre deux cases libres est libre pour la boîte, donc l'anti-blocage existant suffit.

**Lissage à l'exécution, pas dans le chemin.** Le chemin reste une liste de cases (A\*), car les routines IA le comparent et le testent en cases. C'est `follow_path` qui lisse, de deux façons : (1) il coupe le coin d'un point de passage dès qu'il en est à moins de 0,7 tuile, si le segment vers le point suivant est libre pour le corps (`is_segment_clear`, échantillonné tous les 0,1) ; (2) le cap de marche tourne progressivement vers le but (rapport `TURN_RATE`) au lieu de pivoter d'un coup. Le dernier point n'est jamais coupé, donc l'arrivée est exacte. Plus simple et plus robuste qu'une subdivision de cap lissé, car la collision se vérifie là où on marche vraiment. Le dernier point est toujours le centre de la case but. Le regard (`Facing`) suit la tangente. Alternatives : spline de Catmull-Rom (passe par les cases, donc ne coupe pas les virages et reste anguleuse en terrain libre) ; lissage « string-pulling » par visibilité (rend les trajets droits, mais fait quitter les routes, contraire au but).

**Choix du but parmi plusieurs.** Les routines qui comparent des candidats (`path.len()` pour le poste le plus proche, enclume, coffre voisin) comparent désormais le **coût** du chemin, pas sa longueur en cases.

**Perfs.** Les routines IA appellent `find_path` à chaque décision ; un A\* à tas binaire reste du même ordre que le BFS actuel. Pas de cache à ce stade.

## Risks / Trade-offs

- [Les durées de trajet changent, des tests chiffrés échouent] → mettre à jour les assertions en même temps que `find_path`, vérifier que les journées complètes (IA, transporteurs) restent faisables.
- [Un surcoût trop bas ou trop haut rend les routes inutiles ou rigides] → réglage dans `config.rs`, valeur testée sur le scénario de départ.
- [Un virage arrondi frôle un coin et le personnage se bloque] → coin coupé seulement si le segment est libre, collisions du déplacement conservées, anti-blocage existant conservé.
- [Égalités de coût ⇒ chemins instables] → départage déterministe (ordre fixe des voisins, puis index).
- [Voleurs et poursuites non observables dans le jeu] → couverts par des tests unitaires du mode `Direct`.

## Open Questions

- Valeur finale de `grass_cost` (3 au départ), à ajuster en jouant.
