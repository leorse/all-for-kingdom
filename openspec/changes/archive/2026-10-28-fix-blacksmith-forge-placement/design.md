# Design

## Context

Ce qui existe aujourd'hui :

- `forge_spot(anvil)` renvoie la **case** au nord de l'enclume.
- Les contrôles « est-il au poste ? » comparent des cases : `c.tile() != forge_spot(tile)` dans `Interact`, la même comparaison dans `arrive_at_forge`, et `here == forge_spot(a)` dans `forge_routine`. Un joueur sur la bonne case mais décalé forge donc sur place.
- Le trajet d'approche (`follow_path`) finit exactement au **centre** de la case.
- Le corps fait `BODY_HALF = 0.28` tuile. Contre l'enclume, la position la plus basse possible est `anvil.y - 0.28`, soit 0,22 tuile (7 px de dessin) sous le centre de la case.
- Dans `forgeron.svg`, les 6 images `travail` ont le corps décalé de `DOWN = 32` unités (16 px) vers le bas, pour que l'enclume cache les jambes depuis le centre de la case. Ce décalage est visible quand on passe de debout à forger.
- Incarné, le forgeron ne relance rien seul : `drive` ne démarre aucun travail, seul `Interact` le fait. Il s'arrête donc après chaque outil.
- Quand l'IA finit un outil, `advance_work` met `work = None`. `Game::finish_work` range l'outil, et `forge_routine` ne relance la forge qu'au pas suivant. Pendant ce pas, `frame_for` dessine l'image debout.

## Goals / Non-Goals

**Goals :**

- Un point de poste unique, partagé par le jeu, l'IA et le rendu.
- Aucun décalage vertical entre l'image debout et les images de forge.
- Aucun pas sans travail entre deux outils de l'IA.

**Non-Goals :**

- Enchaîner automatiquement les autres travaux du joueur (miner, récolter...). Seule la forge change ici.
- La barre de travail qui passe sur l'enclume.
- Le visage de profil.

## Decisions

### 1. Le poste est un point : `forge_post(anvil) -> Vec2`

On le place à `(anvil.x + 0.5, anvil.y - 2/32)` : ses pieds sont à 2 pixels de dessin au-dessus de la case de l'enclume, soit 14 px sous le centre de la case nord.

Pour que le corps puisse s'y tenir, l'enclume ne bloque plus que le bas de sa case. Le haut de la case, `ANVIL_BACK = 0.22` tuile (dans `map.rs`), est « derrière » l'enclume vue de trois-quarts, et `is_box_blocked` l'ignore. Collé contre l'enclume, le joueur qui la pousse s'arrête donc à ce point. La recherche de chemin, qui travaille par cases, ne change pas : la case de l'enclume reste un obstacle.

*Révisé pendant l'implémentation :* la première version plaçait le point à `anvil.y - BODY_HALF - 0.01`, c'est-à-dire à 7 px seulement sous le centre de la case. Mesuré sur le dessin, le dessus de l'enclume n'aurait alors été qu'à environ 7 px au-dessus de ses pieds, et il aurait frappé à hauteur de ses chevilles. Avec le point plus bas, la pose de forge actuelle, déjà validée à l'œil, reste à l'identique à l'écran.

On garde `forge_spot` (la case), car `available_work`, les réservations d'enclume et la praticabilité de la case nord s'en servent.

Pour décider si le forgeron est au poste, on utilise `at_forge_post(pos, anvil)`, qui vérifie que la distance au point est inférieure à 0,02 tuile.

*Alternative écartée :* déplacer le poste au centre de la case et seulement redessiner. Collé contre l'enclume, le joueur serait de toute façon plus bas que le centre (plan 3 des captures), et il y aurait encore un sursaut. L'utilisateur demande explicitement qu'il « s'avance un peu plus ».

### 2. Le dernier pas jusqu'au point

Le trajet de cases (`find_path`) mène toujours au centre de la case nord. On ajoute ensuite un **dernier segment en ligne droite** jusqu'au point de poste. Ce segment reste à l'intérieur de la case nord : il n'y a pas d'obstacle, puisque l'enclume est au sud, au-delà du point.

On le met en œuvre avec `step_to_point` sur `Character`. Le point se déduit de l'enclume visée, donc aucun champ supplémentaire n'est nécessaire. Il est utilisé après la fin de `path` :

- par la conduite du joueur (`approach`) ;
- par `forge_routine`.

Si le chemin a été abandonné en route (bloqué) hors de la case nord, l'approche s'arrête là.

Le personnage avance vers ce point à sa vitesse de marche (images de marche vers le bas), puis l'`Outcome::Arrived` part comme aujourd'hui. Si le joueur est déjà sur la case nord, le chemin est vide et seul ce segment est joué : c'est le « recentrage ».

*Alternative écartée :* téléporter le personnage sur le point au départ de la forge. C'est ce que montre le défaut 1 : un saut visible.

### 3. Les images de forge sans décalage vertical

Le poste est 14 px plus bas que le centre de la case. Dans `forgeron.svg`, on remonte donc chaque image `travail` de 14 px (`POST`), et l'on trace son corps 14 px plus bas au lieu de 16 (`DOWN = 2 * POST` unités). Résultat :
- le corps est exactement à la hauteur de `debout_bas` ;
- le marteau, la pince et le métal chaud, eux, ne bougent pas à l'écran par rapport à l'enclume. Il n'y a rien à redessiner ;
- seules les épaules remontent de 2 px.

Ensuite :

- on régénère avec `sprites.bat` ;
- le test de concordance vérifie le PNG ;
- on contrôle à l'œil l'aperçu agrandi et une capture en jeu (debout au poste, puis en train de forger : même hauteur).

Les étincelles (`draw_forge_sparks`) partaient de `c.pos + (2, -2)` px. Les pieds étant désormais 14 px plus bas, elles partent de `c.pos + (2, -16)` px : c'est le même métal à l'écran.

### 4. Le forgeron enchaîne dans le même pas, IA ou joueur

Dans `Game::finish_work`, pour `WorkKind::Forge`, on relance aussitôt la forge du même personnage, **incarné ou non**, quand toutes ces conditions sont réunies :

- il est toujours `at_forge_post` ;
- `available_work`, ou la vérification du fer et de la place, permet encore de forger à cette enclume.

Ensuite, `work` n'est plus jamais `None` entre deux outils. Pour le joueur, la série s'arrête par les voies habituelles d'annulation d'un travail : un déplacement (`drive` met `work = None`) ou un nouvel appui sur E (`cancel_work`). Elle s'arrête aussi quand il n'y a plus de fer ou de place. Rien de nouveau n'est nécessaire côté entrée.

*Alternative écartée :* un « mode forge » côté rendu qui garderait l'image de forge pendant un pas. On masquerait l'état sans le corriger, et le rendu devrait deviner l'intention de l'IA.

## Risks / Trade-offs

- **Un personnage qui longe l'enclume par le nord peut maintenant mordre sur le haut de sa case.** Vu de trois-quarts, il passe derrière elle, ce qui est cohérent. L'enclume est dessinée devant lui.
- **Le point de poste doit rester libre, et un pas de plus vers le sud doit toucher l'enclume.** Un test le vérifie. Le dernier pas place `pos` directement sur le point, sans passer par `try_move`.
- **Un autre personnage se tient sur le point.** Le comportement reste celui d'aujourd'hui sur la case, puisque les collisions entre personnages n'existent pas.
- **Les tests qui posaient `pos = forge_spot(ANVIL).center()`** pour dire « au poste » ne sont plus au poste. Il faut les mettre à jour vers `forge_post`.
