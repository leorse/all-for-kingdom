# Proposal

## Why

Le fer est aujourd'hui un stock de départ fini (60) : le forgeron l'épuise en moins d'une journée et le royaume s'arrête. Le mineur, sa mine et les filons sont décidés (`docs/gameplay/mineur.md`) mais absents du jeu. Les ajouter donne au royaume sa première vraie production continue et pose la base de la chaîne filon → minerai → forge.

## What Changes

- Des **filons** de 2×2 tuiles sont posés sur la carte, infinis, en quatre natures possibles (fer, cuivre, or, pierre) reconnaissables à la couleur. Leur **qualité** est lisible sur la carte par des points (les impuretés) et fixe le nombre de coups de pioche par unité de minerai.
- Un nouveau métier, le **mineur** : il donne des coups de pioche sur un filon depuis une case voisine, en le regardant. Le joueur l'incarne comme les autres (touche d'action, 2× plus vite que l'IA, fatigue) et l'IA le fait travailler le jour et dormir la nuit.
- La **mine** est un bâtiment accolé à son filon, maison du mineur (lit).
- **Étape simplifiée** : chaque unité extraite va **directement dans le compteur global** de sa nature. Pas encore d'entrepôt local ni de capacité, donc le mineur ne se bloque jamais. Le forgeron continue de consommer le compteur global de fer, qui est maintenant alimenté.
- Trois nouvelles ressources (cuivre, or, pierre) s'ajoutent au fer ; le HUD les affiche.
- Le scénario de départ gagne **trois filons (fer, or, pierre), trois mines et trois mineurs**, un par filon. Le cuivre existe comme nature mais n'est pas posé tant qu'il n'a pas d'usage.
- Hors périmètre, repoussé à plus tard : entrepôt local de la mine et sa capacité, forgeron qui prend son minerai dans un stock local, transporteur, lingots, usage du cuivre, pièces d'or, amélioration des maisons, placement de mines par le roi.

## Capabilities

### New Capabilities
- `ore-veins`: filons de 2×2 tuiles sur la carte, leurs natures, leur qualité visible et leur caractère infini.
- `mining`: le métier de mineur : coups de pioche, durée selon la qualité, joueur/IA, fatigue, postes autour du filon.
- `mine-house`: la mine, maison du mineur accolée au filon, et l'arrivée du minerai dans les compteurs globaux.

### Modified Capabilities
- `kingdom-layout`: le royaume de départ contient aussi les filons, mines et mineurs, tous accessibles à pied.

## Impact

- `crates/afk-core/src/map.rs` : nouvelle tuile solide de filon.
- `crates/afk-core/src/resources.rs`, `config.rs` : ressources Cuivre/Or/Pierre ; réglages (coups par qualité, durée d'un coup).
- Nouveau module `afk-core` pour les filons et mines ; `work.rs` (`WorkKind::Mine`), `character.rs` (`Role::Miner`, routine IA du mineur, réservation du poste), `game.rs` (travail disponible, ajout au compteur en fin de travail).
- `scenario.rs` : filons, mines, mineurs ; test de connectivité étendu.
- `crates/afk-game/src/render.rs` : dessin des filons (couleur + points), de la mine, du mineur, HUD des minerais.
- Tests de `game.rs` et outil `balance` : le fer n'est plus fini, à revoir.
- `docs/gameplay/*.md` (mineur, ressources, habitat, README) : état « en place » pour ce qui est fait, et ce qui reste décidé mais repoussé.
