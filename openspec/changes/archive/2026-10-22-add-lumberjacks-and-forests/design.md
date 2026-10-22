# Design

## Context

Le cœur (`afk-core`) ne connaît pas l'écran. `Tile::Tree` est une tuile solide qui sert de décor ; le scénario sème des arbres isolés sur l'herbe. Le mineur vient d'être ajouté sur le même modèle (voir `openspec/specs/mining`) : `WorkKind::Mine`, un travail dont la durée est portée par `Work`, un rattachement mineur → filon par le lit (`MineSpec`), une routine IA modelée sur la forge, et le produit ajouté directement à `Stock`. Le paysan et la ferme (`farm.rs`) gèrent déjà un minuteur en heures de jeu. Voir `proposal.md` pour la motivation et `docs/gameplay/bucheron.md`.

## Goals / Non-Goals

**Goals:**
- Réutiliser la mécanique de travail (barre de progression, ×2 joueur, fatigue, annulation) et le patron du mineur.
- Un arbre est un objet à usage unique mais qui se coupe à plusieurs : la progression est celle de l'arbre, partagée, et un seul bûcheron touche le bois.
- Une repousse en heures de jeu, indépendante de la vitesse, qui rend le bois réellement limité.

**Non-Goals:**
- Entrepôt local, capacité, transporteur, consommation du bois, outils, replantation à la main, forêts posées par le roi.

## Decisions

- **Forêt = zone rectangulaire du scénario ; arbres = `Tile::Tree` comme aujourd'hui.** Un arbre est abattable s'il est dans une forêt ; hors forêt c'est du décor. Alternative : une tuile `Forest` distincte — rejeté, ça double les tuiles et casse le décor existant. Le scénario remplit chaque forêt d'arbres en laissant un couloir libre toutes les trois colonnes, pour que chaque arbre touche un couloir et que les bûcherons puissent y circuler.
- **Pousse = `Tile::Sapling`, praticable, non coupable.** L'abattage remplace l'arbre par une pousse ; le jeu garde la liste des pousses avec leur heures restantes. Alternative : un minuteur sur la tuile d'arbre — rejeté, `Tile` reste `Copy` et sans donnée.
- **Repousse dans `Game::step`, en heures de jeu** (comme les cultures) : à chaque pas on décompte ; à zéro, la pousse redevient `Tile::Tree` sauf si un personnage se tient sur la case (alors elle attend, à zéro, et on réessaie au pas suivant). Réglage `tree_regrow_hours` (de l'ordre d'une journée de jeu), `Config`. Pas de repousse sur une case autre que celle de l'arbre coupé : la forêt ne s'étend jamais.
- **Réservation de la case de poste, comme le mineur.** `claimed_tile()` renvoie la case où le bûcheron se tient (travail `Chop` comme `Mine`) : deux bûcherons peuvent viser le même arbre mais pas la même case voisine (4 au plus).
- **Progression de l'arbre partagée, tenue par `Game`.** Un `Chop` ne se termine pas tout seul : à chaque pas, `Character` calcule sa contribution (durée du pas ÷ `chop_secs` × vitesse joueur × rendement, soit exactement la formule des autres travaux) et `Game` l'additionne à la progression de l'arbre (table tuile → fraction). La barre de chaque bûcheron affiche cette progression partagée. Quand le total atteint 1, l'arbre tombe ; le bûcheron dont la contribution franchit 1 (ordre des personnages en cas d'égalité sur un même pas) est le propriétaire du bois. Les autres bûcherons de cet arbre voient leur travail interrompu et choisissent un autre arbre. La progression d'un arbre entamé puis abandonné est conservée jusqu'à sa chute (aucun recul). Alternative : une barre individuelle par bûcheron avec un coup final partagé — rejeté, elle ne réduirait pas le nombre de coups.
- **Un travail `Chop`** : durée de référence fixe `chop_secs` (réglage). À la chute, `Game` vérifie que la tuile est encore un arbre de forêt, la passe en pousse et enregistre la pousse, puis ajoute `logs_per_tree` (3) bûches à `Stock` **au nom du propriétaire**. Sans entrepôt, la livraison est immédiate ; le propriétaire est connu (renvoyé par la chute) pour que le futur entrepôt n'ait qu'à lui faire porter les bûches à pied.
- **Rattachement bûcheron → forêt par le lit**, comme le mineur : `LodgeSpec { forest, bed }` dans le scénario, résolu dans `Game::new` en `Character.forest`. Un bûcheron n'abat que dans sa forêt (filtre dans `available_work`, comme le filon attitré).
- **Routine IA** (modèle `forge_routine`) : parmi les arbres abattables de sa forêt (réservés ou non) ayant une case voisine praticable, libre de toute réservation et joignable, choisir le plus proche par la longueur du chemin ; marcher jusqu'à cette case ; regarder l'arbre ; couper. Sans arbre disponible, `wander_routine` sur ses postes de travail (lisière de sa forêt).
- **Ressource `Wood`**, libellé Bois, dans `Resource::ALL` et le HUD. Le compteur compte des bûches.
- **Scénario.** Deux forêts (une au nord-ouest du château, une au sud-ouest) de 12 colonnes, remplies avant le semis des arbres isolés (qui ne se posent que sur l'herbe, donc ne les perturbent pas), deux cabanes avec leur chemin vers la route, deux bûcherons ajoutés à la fin de la liste des personnages pour ne pas décaler les identifiants existants. Un test de connectivité les couvre.
- **Rendu.** La pousse est une petite tige verte sur l'herbe ; le bûcheron a sa couleur ; le HUD ajoute Bois ; le message d'action « E : abattre ».

## Risks / Trade-offs

- [Une forêt trop petite ou une repousse trop lente épuise les arbres et les bûcherons attendent] → c'est le comportement voulu, mais l'équilibre initial doit tenir plusieurs jours : mesuré avec l'outil `balance` (arbres restants par forêt par jour) et réglé par `chop_secs` et `tree_regrow_hours`.
- [Pousse sur une case occupée] → l'arbre attend que la case soit libre ; un test couvre l'absence d'emprisonnement.
- [Arbres de forêt qu'aucun bûcheron n'atteint (entourés d'arbres)] → ils attendent que les voisins soient coupés ; le motif en colonnes de deux garantit au moins un côté libre au départ.
- [Les arbres isolés du décor ne sont pas coupables, ce qui peut surprendre à l'écran] → accepté ; la distinction se verra mieux quand le roi posera les forêts.

## Open Questions

- Valeurs finales : durée d'abattage, délai de repousse, taille des forêts, à fixer avec `balance` sans changer l'approche.
- Les arbres isolés du décor deviendront-ils coupables ? Hors périmètre ici.
