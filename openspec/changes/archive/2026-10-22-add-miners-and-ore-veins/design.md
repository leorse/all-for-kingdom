# Design

## Context

Le cœur (`afk-core`) ne connaît pas l'écran. Un travail (`Work`) vise une tuile ; `Game::step` laisse les personnages agir sur un monde en lecture seule (`Env`), puis applique les `WorkDone` (stock, parcelles). Les tuiles sont un `enum Tile` `Copy` sans donnée associée. Le forgeron consomme `Stock` (compteurs globaux) ; la réservation d'une cible passe par `claimed_tile()` (la tuile travaillée). Le scénario est de la donnée pure (`CharacterSpec` : lit, postes). Voir `proposal.md` pour la motivation et `docs/gameplay/mineur.md` pour les règles.

## Goals / Non-Goals

**Goals:**
- Réutiliser la mécanique de travail existante (barre de progression, vitesse joueur ×2, fatigue, annulation) sans cas particulier côté joueur.
- Garder les filons et mines dans le cœur et dans le scénario, sans notion de rendu.
- Ne rien changer au forgeron : il lit déjà le compteur de fer.

**Non-Goals:**
- Entrepôt local de la mine et capacité (le mineur ne se bloque pas), forgeron avec stock local, transporteurs, lingots, usages du cuivre et de l'or, amélioration des mines, mines posées par le roi.
- Épuisement ou variation des filons.

## Decisions

- **Tuile `Tile::Ore`, solide, sans donnée ; nature et qualité dans une liste de filons.** Alternative : `Tile::Ore(kind, quality)` — rejeté, `Tile` reste `Copy` et léger et `find_tiles` continue de marcher. Un `Vein { origin, kind, quality }` (nature : fer/cuivre/or/pierre ; qualité : 3 niveaux) vit dans le scénario puis dans `Game`, avec une recherche « quel filon contient cette tuile ». Le rendu lit la même liste pour la couleur et les points (1, 2, 3 points par tuile selon l'impureté).
- **Qualité par filon, pas par tuile.** Les points d'une tuile ne sont qu'une représentation de la qualité unique du bloc. Le nombre de niveaux (3) et les coups par niveau (p. ex. 3 / 5 / 8) sont des réglages de `Config`, ajustables par `balance`.
- **Durée d'un travail = coups × durée d'un coup.** `WorkKind::Mine` ; le travail mémorise sa durée, calculée au démarrage depuis la qualité du filon visé. Alternative : un `WorkKind` par qualité — rejeté. Une unité de minerai par travail terminé.
- **Le minerai va dans `Stock`.** À la fin d'un `Mine`, `Game` ajoute une unité de la ressource correspondant à la nature du filon (`Iron`, `Copper`, `Gold`, `Stone`). Pas de structure d'entrepôt, pas de capacité : c'est l'étape simplifiée voulue. Le forgeron n'est pas touché. Quand l'entrepôt local arrivera, le dépôt passera de `Stock` à la mine ; la liaison mineur → mine est donc déjà posée dès maintenant (voir plus bas).
- **Mine = bâtiment + lit + filon.** Le scénario décrit chaque mine (`MineSpec` : bâtiment, porte, filon accolé). Le mineur est rattaché à sa mine par son lit, qui est dans le bâtiment ; son filon est celui de sa mine (filon attitré). Un mineur ne travaille que ce filon.
- **Réservation par la case de poste.** Le claim ne peut pas être la tuile du filon (sinon un seul mineur par filon). Pour `WorkKind::Mine`, `claimed_tile()` renvoie la case où le mineur se tient. L'IA choisit la case voisine libre la plus proche du filon (modèle de `forge_routine`), parmi les 8 cases orthogonales.
- **Nouvelles ressources** `Copper`, `Gold`, `Stone` dans `Resource` et dans le HUD (libellés Cuivre, Or, Pierre). Le cuivre est posé nulle part au départ (aucun usage).
- **Scénario.** Trois mines/mineurs (fer, or, pierre) placées dans les zones libres de la carte 64×40, hors routes, champs et bâtiments existants ; un test d'accessibilité les couvre (lits, portes, cases voisines des filons). Le fer de départ (`starting_iron`) est conservé.
- **Rendu.** Filon : couleur de nature (fer gris, cuivre cuivré, or doré, pierre claire) avec points selon la qualité ; mine : bâtiment comme une maison ; mineur : tenue dédiée ; compteurs ajoutés au HUD.

## Risks / Trade-offs

- [Le travail à durée variable touche `Work`/`WorkKind`, partagés avec paysan et forgeron] → ajouter la durée au `Work` sans changer le comportement des autres, vérifié par les tests existants.
- [Sans capacité, le mineur produit sans fin : les compteurs montent sans limite] → accepté pour cette étape ; l'entrepôt limité reste décidé dans `docs/gameplay/mineur.md` et viendra ensuite.
- [Équilibrage : le fer arrive plus vite que le forgeron n'en consomme] → sans conséquence tant qu'il n'y a pas de plafond ; `balance` sert à régler les coups par qualité.
- [Filons solides : un mauvais placement peut couper un chemin] → test de connectivité du scénario étendu.

## Open Questions

- La pierre a-t-elle des qualités comme les autres ? Le design suppose que oui (même mécanique).
- Valeurs finales : coups par qualité et durée d'un coup, à fixer avec `balance` sans changer l'approche.
- Sort du fer de départ une fois les mines en place.
