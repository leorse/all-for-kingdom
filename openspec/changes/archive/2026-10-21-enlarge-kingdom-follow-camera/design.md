# Design

## Context

`View::fit` (render.rs) choisit un `tile_px` pour que la carte entière tienne dans la fenêtre et centre le tout ; `origin` est donc fixe pour une taille de fenêtre donnée. `to_screen`/`to_world` sont les seuls points de conversion, utilisés par le rendu et par `input::poll` (clic). La vue est recalculée à chaque image dans `main.rs`. Le cœur (`afk-core`) ne connaît pas l'écran. Le HUD est dessiné en coordonnées écran. Le scénario `village()` code en dur une carte 32×20 et toutes les coordonnées.

## Goals / Non-Goals

**Goals:**
- Caméra purement côté `afk-game` ; aucune notion de caméra dans `afk-core`.
- Un seul endroit (`View`) pour la conversion monde↔écran, pour que clic et dessin restent cohérents.
- Dessin limité aux tuiles visibles.

**Non-Goals:**
- Zoom dynamique, déplacement libre de la caméra à la souris, minimap.
- Placement de bâtiments en jeu.
- Générateur de carte ; le scénario reste écrit à la main.

## Decisions

- **`View` devient persistante, avec un centre de caméra.** Elle est créée une fois (taille de tuile fixe, p. ex. 32 px) et mise à jour chaque image : `follow(cible, taille d'écran, taille de carte)`. Alternative : recréer une `View` par image comme aujourd'hui avec le centre en paramètre ; rejetée car le suivi à marge a besoin de l'état précédent.
- **Suivi à marge (dead zone) plutôt que centrage permanent.** La caméra ne bouge que si le personnage sort d'une zone centrale (marge d'environ 1/4 de l'écran de chaque côté) ; au changement de personnage, elle se recentre d'un coup. Évite le tremblement de la carte à chaque pas et correspond à « la carte se déplace s'il le faut ». Alternative : centrage permanent, plus simple mais carte constamment en mouvement.
- **Changement de personnage détecté par comparaison de l'identifiant incarné d'une image à l'autre**, dans `main.rs`, sans toucher au cœur : si l'identifiant change (ou au premier tour), recentrage. Couvre Tab, chiffres et clic sans les traiter séparément.
- **Recentrage animé, rapide** : le glissement dure environ 0,35 s (constante réglable dans `render.rs`) avec une courbe d'atténuation en fin de course. Il est mesuré en temps réel (`get_frame_time`), pas en temps de simulation, donc indépendant de la touche T. La caméra garde une position courante qui rejoint la position cible (personnage + bornage) ; la cible est recalculée à chaque image, ce qui gère un personnage qui bouge pendant l'animation. Un nouveau changement repart de la position courante. Pas d'animation à la première image. Alternative : amortissement exponentiel permanent (la caméra « rattrape » toujours sa cible) ; rejeté car la durée varie avec la distance et un trajet de l'autre bout de la carte serait trop lent ou trop brusque.
- **Pendant l'animation**, le suivi à marge est suspendu : seule la courbe pilote la caméra, puis le suivi reprend à l'arrivée.
- **Bornage** : le centre est contraint pour que la zone visible reste dans la carte ; si la carte est plus petite que l'écran sur un axe, elle est centrée sur cet axe (garde le comportement actuel pour les petits scénarios).
- **Rendu** : `draw_map` itère seulement sur la plage de tuiles visibles (calculée par la vue) ; les autres dessins (personnages, parcelles) ignorent ce qui est hors écran via un test de visibilité de la vue. Le HUD n'est pas affecté.
- **Zoom fixe** : constante dans `render.rs` (32 px, valeur actuelle en fenêtre 1024×640), pas d'option de ligne de commande pour l'instant.
- **Nouvelle carte** : 64×40 tuiles (le double dans chaque dimension, soit 2048×1280 px contre une fenêtre de 1024×640). Les bâtiments publics restent regroupés autour du centre (château, forge, taverne, caserne) ; les maisons et les champs sont repoussés vers les bords, reliés par de longues routes. Les valeurs exactes sont à fixer en implémentation, en suivant la spec `kingdom-layout`.

## Risks / Trade-offs

- [Trajets plus longs : fatigue et équilibrage changent] → relancer l'outil `balance` et les tests de `game.rs`, ajuster ceux qui supposent des durées de trajet. L'option `--day-seconds` n'est pas un cas d'usage visé ; on ne cherche pas à la garder cohérente avec la nouvelle taille.
- [Beaucoup de coordonnées codées en dur à retoucher (lits, rondes des gardes, `EXTRA_BEDS`)] → un test d'accessibilité de tous les lits et postes depuis les portes.
- [Pas de test de connectivité aujourd'hui] → en ajouter un dans `afk-core` qui vérifie via `find_path` que chaque `home` et `work_spot` est joignable.
- [Clic sur un personnage hors écran] → ignoré naturellement, la conversion écran→monde ne produit que des positions visibles.
