# Design

## Context

Aujourd'hui `Character::home` est une `TilePos` obligatoire : tout le sommeil (`go_to_bed`, `lie_down`, `is_sleep_time`) et le compte des places (`free_places_in`) s'appuient dessus. Un personnage a un `Role` fixé à la création et le travaille toujours (`run_ai` choisit la routine du rôle). Un vagabond est un personnage de rôle `Vagabond`, sans lit, qui se promène puis repart (`vagabond_routine`, `gone`). Les maisons bâties (`HabitatSpec::house`) offrent des lits mais rien n'y envoie personne. Voir `knowledge/regles/propriete-et-affectation.md`, `construction.md`, `metiers/vagabond.md`.

## Goals / Non-Goals

**Goals:** le geste complet du roi (parler, affecter à un poste ou à un lit, remplacer, réaffecter), les règles du cœur testées sans fenêtre, une interface lisible (mini-carte, listes), le tout réglable.

**Non-Goals:** les plans de production et leurs postes, les compétences, les salaires, l'IA du roi, l'embauche à distance.

## Decisions

- **Trois notions séparées sur `Character`.** `role` (métier) ne change que par une affectation ; `post: Option<PostId>` (le poste tenu) ; `home: Option<TilePos>` (le lit). `home` devient facultatif : `go_to_bed` ne fait rien sans lit, `is_sleep_time` reste vrai mais personne ne s'endort, la fatigue monte jusqu'au plafond et l'efficacité (déjà 0,5 à 100) fait le reste. `free_places` compte les lits dont aucun personnage n'est `home`.
- **Les postes sont des données dérivées des bâtiments.** `Game::posts()` liste `Post { id, role, tile, building }` : l'entrepôt fournit `warehouse_posts` postes de transporteur (devant la porte, à l'intérieur), la taverne un poste de tavernier (derrière ses tables), **la forge un poste de forgeron par enclume (le point de forge, juste au nord de l'enclume)**, le camp un poste d'artisan par personne (illimité, un seul marqueur). Un poste est occupé si un personnage le tient (`post`). Aucune structure à synchroniser : la liste se recalcule depuis `buildings()`.
- **Sans poste, on ne travaille pas.** Dans le camp (`camp.is_some()`), une personne dont le rôle a des postes ne travaille que si elle en tient un ; sinon elle reste « sans poste » : `idle_routine` (près de son lit, ou du camp, sans rien faire). **Dans le village (banc d'essai), rien ne change** : personne ne dépend d'un poste.
- **Parler = une pause, pas un état de l'IA.** `Character::paused: bool` : `update` rend la main tout de suite (pas de `run_ai`, pas de progression de travail, la fatigue continue). Chemin, but, cargaison et travail en cours restent intacts : **reprendre est donc exact par construction**. `Game` garde `talking: Option<CharacterId>` par joueur. La personne se tourne vers le roi. **Une personne endormie peut être abordée** : `Talk` la réveille (`wake_up`) puis la met en pause ; si la conversation s'achève sans décision et que c'est encore son heure de dormir, la routine habituelle la renvoie se coucher.
- **Le ralentissement est une vitesse, pas une pause.** Chaque pas, `Env::king_near_vagabond(pos)` (le roi incarné à moins de `slow_range` tuiles) multiplie la vitesse d'un vagabond par `vagabond_slow_factor`. Aucun effet sur les autres rôles. Ne dépend pas du bouton.
- **Commandes.** `Talk { player }` (la personne la plus proche à moins de `talk_range`, hors endormis et hors roi), `EndTalk { player }`, `AssignPost { player, who, post }`, `AssignBed { player, who, bed }`. Le cœur valide (le joueur incarne un roi, `who` est en conversation) ; le frontend ne décide rien.
- **Remplacer.** `AssignPost` sur un poste occupé : le titulaire perd son poste, garde son rôle (il passe en `idle_routine`). Réaffecter quelqu'un qui tenait un autre poste libère celui-là. Le rôle de la personne devient celui du poste ; un vagabond affecté cesse d'être vagabond (il ne repart plus, il compte parmi les habitants).
- **Lit automatique.** Quand une personne tient un poste et n'a pas de lit, `Game::auto_bed` lui donne le lit libre le plus proche **du poste** (distance de Manhattan), et la réévalue à chaque pas tant qu'elle n'en a pas (une maison finie donne donc un lit au premier qui en manque). L'attribution manuelle (`AssignBed`) remplace : l'ancien occupant perd son lit (puis le reprend automatiquement s'il a un poste et qu'un lit est libre). Les lits du camp sont des lits comme les autres.
- **La mini-carte est une vue.** Le frontend dessine toute la carte au zoom qui la fait tenir à l'écran (pas de nouveau moteur de rendu), recouverte de marqueurs de postes ou de lits. À gauche, la liste des métiers avec « libres/total » ; les métiers qui ont des postes libres en premier. Elle ne met pas le jeu en pause. Échap ou clic droit la ferme sans rien changer (la personne reprend). La liste des employés est un panneau à part, toujours disponible au roi.
- **La forge est un type de plan comme les autres** (`PlanKind::Forge`, quatrième entrée des tableaux de `Config`). À la fin du chantier, `Building::build` pose l'enclume (`Tile::Anvil`) sur la rangée intérieure qui laisse libre la case au nord (`forge_spot`), un coffre `ContainerKind::Forge` à l'extérieur (comme celui du village), et le décor. La forge du village garde son comportement. L'enclume et les règles de forge (`at_forge_post`, `ANVIL_CONTACT`) sont réutilisées telles quelles ; seul change le fait que le forgeron ait besoin d'un poste.
- **Réglages** dans `Config` : `talk_range`, `slow_range`, `vagabond_slow_factor`, `warehouse_posts`.

## Risks / Trade-offs

- [`home` facultatif touche beaucoup de code et de tests] → fait en premier, test par test ; `home: TilePos` reste pour le village via un constructeur qui l'enveloppe dans `Some`.
- [Un poste déplacé ou détruit] → rien n'est démoli pour l'instant ; la liste des postes se recalcule des bâtiments existants.
- [Les artisans réaffectés ne construisent plus] → c'est voulu ; le premier chantier peut attendre. Un avertissement dans l'interface (« plus d'artisan ») est à envisager.
- [Une forge sans fer] → aucune mine n'est constructible et le camp n'a pas de fer au départ : la forge est bâtie mais inactive. Pistes : un peu de fer dans le pécule, ou un plan de mine (`production-plans`).
- [Sans les autres plans de production, trois métiers seulement sur la mini-carte] → `production-plans` est le compagnon recommandé ; la spec ne le suppose pas.
- [Un vagabond ralenti peut rester longtemps près du roi] → son temps de séjour continue de courir ; s'il atteint zéro il repart.

## Open Questions

- Touche pour parler (proposée : `E`), `talk_range` (proposée : 2 tuiles), `slow_range`, valeurs de ralentissement et du nombre de postes.
- Faut-il que le roi puisse **réaffecter à distance** depuis la liste des employés, sans s'approcher ?
- Un vagabond affecté reçoit-il un nom, un visage, un métier au hasard ? (compétences)
- Un coût ou un salaire d'embauche, avec l'économie ?
- Le fer de la forge : un peu dans le pécule, ou attendre la mine ? Une forge peut-elle avoir plusieurs enclumes à la pose ?
- Une **satisfaction** des habitants (lit, confort…) dont la chute pousserait à quitter le royaume : voir `knowledge/idees.md`, pas dans cette spec.
- Le lit d'un titulaire remplacé : le garde-t-il ? (proposé : oui, il vit toujours dans le royaume.)
