# Design

## Context

Mesures relevées sur `forgeron.png`, en pixels de dessin (1 case = 32 px), avec l'ancre de cellule à (32, 58) :

- **Enclume :** 35 px de haut et 41 px de large, corne comprise. Le socle fait 32 px de large et 17 px de haut (lignes 43 à 59). Il remplit donc presque exactement une demi-case.
- **Position actuelle de l'enclume :** elle est ancrée au centre de sa case, donc le bas du socle est 2 px sous ce centre.
- **Forgeron debout :** ses bottes s'arrêtent 4 px sous sa position (`pos`).
- **Images de forge :** elles s'arrêtent à la ligne 51 au lieu de 62. Le script de génération décale les jambes de 14 px vers le bas, puis de 14 px vers le haut. Le premier décalage les fait sortir de la cellule de 64 px, et ces pixels sont perdus. C'est la découpe que l'on voit à l'écran.
- **Collision :** le corps est un carré de `BODY_HALF = 0.28` case, centré sur `pos`, donc sur les pieds. L'enclume bloque à partir de `ANVIL_BACK = 0.22`.
- **Tri d'affichage :** la clé de l'enclume est `y + 0.99`.

## Goals / Non-Goals

**Goals :**

- Une seule géométrie, le socle, pour le dessin, la collision et le tri.
- Une pose de forge qui reste celle déjà validée, jambes en plus.

**Non-Goals :**

- Un système générique de hitbox par objet. Il sera conçu avec les coffres et les étagères, qui ajouteront la rotation et les dimensions.
- Changer la hitbox des personnages, ou réduire le socle latéralement : il fait déjà toute la largeur de la case.

## Decisions

### 1. Tout descend de 14 px, l'enclume comme le poste

Le point d'ancrage de l'enclume passe à `anvil.center() + 14 px` : le bas du socle tombe au bas de la case et le haut du socle à 15 px du haut de la case. Ce décalage se fait à l'affichage, car le dessin n'a pas la place de descendre de 14 px dans sa cellule.

Le poste passe de `anvil.y - 2/32` à `anvil.y + 12/32`, soit lui aussi 14 px plus bas. Le rapport entre le forgeron et l'enclume ne change donc pas : le marteau, la pince, le métal chaud et les étincelles (`c.pos + (2, -16)`) restent justes sans rien redessiner. Les bottes mordent de 1 px derrière le haut du socle, ce qui l'y pose bien.

*Alternative écartée :* poste à `11/32`, avec des bottes exactement tangentes. Tous les outils de forge seraient décalés de 1 px, à redessiner, sans gain visible.

### 2. La collision se déduit du poste, sans toucher à la hitbox des personnages

Le corps dépasse de `0.28` sous les pieds. Pour que le joueur qui pousse vers le sud s'arrête exactement au poste, l'enclume doit bloquer à partir de `12/32 + 0.28 = 0.655` de sa case. `ANVIL_BACK` devient cette valeur, calculée à partir des deux autres plutôt qu'écrite en dur. Vus depuis le nord, les pieds touchent bien le socle, ce que demande la spec, même si le rectangle bloquant est plus petit que le socle dessiné.

*Alternatives écartées :*
- Bloquer à exactement `0.5` : le joueur s'arrêterait 5 px avant le poste, et au poste son corps serait dans l'obstacle, donc incapable de repartir vers le nord sans correctif.
- Une hitbox limitée aux pieds pour tous : c'est plus propre à terme mais touche tous les personnages et tous les murs. L'utilisateur n'y tient pas pour l'instant, on la réexaminera avec les autres objets.

### 3. Ordre d'affichage : la ligne du haut du socle

La clé de tri de l'enclume devient `anvil.y + 0.5`, proche du haut du socle (`15/32`). Le forgeron au poste (`0.375`) est derrière, et quelqu'un dont les pieds sont plus bas que le haut du socle passe devant.

### 4. Un personnage peut se trouver sur la case de l'enclume

Le poste est maintenant dans la case de l'enclume, donc `tile()` y renvoie une case solide. On garde `tile()` tel quel, et l'on corrige là où cela compte :

- **Départ des trajets :** dans `find_path_cost`, un départ sur une case d'enclume part de sa case au nord. Le personnage remonte à pied depuis la moitié libre, sans jamais viser le sud à travers le socle. Tous les trajets, y compris ceux de l'IA, en profitent.
- **`drive` et `forge_routine` :** « je suis au poste ou juste à côté » accepte désormais `forge_spot(anvil)` et la case de l'enclume elle-même. Depuis ces deux cases, seul le dernier pas (`step_to_point`) est joué.
- **`Interact` :** même règle pour décider de lancer une approche à chemin vide.
- **`available_work` :** rien à changer. Il regarde déjà `[facing_tile, tile]`, donc il trouve l'enclume sous les pieds.

La trajectoire du dernier pas part du centre de la case nord ou de la moitié libre et reste entièrement dans la zone libre, au nord du rectangle bloquant.

*Alternative écartée :* modifier `tile()` pour renvoyer la case au nord, car 15 appelants s'en servent pour autre chose (réservations, tests, rendu).

### 5. Les images de forge : jambes entières et ombre

Dans le script de génération, les jambes de forge sont placées directement à leur hauteur finale, sans passer par le double décalage. Seules les jambes et les bottes réapparaissent dans le dessin. On régénère avec `sprites.bat`. `draw_character` affiche l'ombre aussi pendant le travail, pour rester cohérent avec le corps entier.

*Constaté en jeu :* au poste, les bottes tombent derrière le pied métallique de l'enclume (28 à 32 px de large, contre 16 px pour les jambes) et restent cachées. L'utilisateur a validé ce rendu, sans affiner l'enclume ni reculer le forgeron.

## Risks / Trade-offs

- **[Le forgeron paraît encore trop enfoncé derrière l'enclume]** → le rapport forgeron-enclume est conservé volontairement. Ce sont les jambes visibles qui doivent changer l'impression, à vérifier sur capture avant de toucher au dessin.
- **[Un personnage peut rester dans la moitié libre, sur une case solide]** → les tests couvrent le départ d'une IA depuis cette zone, notamment vers le sud, et la reprise par l'IA quand le joueur lâche le personnage.
- **[La collision n'est pas exactement le socle dessiné]** → assumé : c'est une conséquence d'un corps centré sur les pieds, comme pour tous les autres obstacles. On y reviendra si la hitbox des personnages change.
- **[Les tests actuels supposent un poste dans la case au nord]** → ils sont mis à jour, notamment `the_forge_post_is_centered_on_the_anvil_and_against_it`.
- **[`scenario` n'a qu'une enclume, au milieu de la forge]** → le passage par les côtés de la moitié libre reste possible, ce qui ne gêne pas.
