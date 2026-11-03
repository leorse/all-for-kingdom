# Proposal

## Why

Les vagabonds passent par le camp sans but, et les maisons bâties n'ont aucun usage : on ne peut ni embaucher, ni donner un métier, ni attribuer un lit. Le gameplay du roi (« il répartit ») n'a pas encore son geste central. Le créateur a décrit le parcours voulu (`knowledge/regles/propriete-et-affectation.md`, à jour avec ce changement) : le roi **s'adresse** à quelqu'un, qui interrompt ce qu'il faisait, puis l'affecte à un **poste** ou à un **lit** sur une **mini-carte**. Cette spec fixe ce comportement avant de l'implémenter ; elle est amenée à être **réajustée** à l'usage.

## What Changes

- **Parler.** Un bouton, quand le roi est proche de quelqu'un : la personne **arrête tout et garde en mémoire ce qu'elle faisait**. Si le roi ne dit rien, elle reprend exactement où elle en était.
- **La proximité du roi.** Près d'un **vagabond**, celui-ci **ralentit** toujours, sans qu'on lui parle ; près de n'importe qui d'autre, rien ne change et la personne continue son travail, sauf si le roi lui parle.
- **Métier, poste, lit.** Trois notions distinctes. Un **poste** est une place de travail dans un bâtiment (un seul titulaire à la fois). Un **métier** (rôle) reste à la personne même sans poste. Un **lit** est une place (voir `housing`).
- **Affecter, remplacer, réaffecter.** À un poste libre (vert) ou occupé (rouge). Le titulaire remplacé **garde son métier mais n'a plus de poste** : il ne fait plus rien. Tout le monde peut être réaffecté (artisans compris, le roi excepté).
- **Le lit automatique.** Quelqu'un qui a un poste sans lit prend le **lit libre le plus proche de son poste**. Sans lit disponible, il travaille quand même mais **ne se repose jamais** et s'épuise. Le roi peut aussi attribuer un lit à la main.
- **L'embauche n'exige plus de lit** (la règle « on n'embauche que s'il y a une place libre » est **abandonnée**) : c'est la fatigue qui sanctionne le manque de lits.
- **La mini-carte** : une vue très dézoomée, à gauche la liste des métiers (ceux qui ont des postes libres d'abord) ; choisir un métier colore ses postes en vert (libre) ou rouge (occupé) ; un clic affecte ou remplace. Le temps ne s'arrête pas.
- **La liste des employés** du roi : nom, métier, poste, symbole de lit.
- Postes fournis par les bâtiments : **entrepôt** (transporteurs), **taverne** (tavernier) et **forge** (un forgeron par enclume). Les artisans n'ont pas de bâtiment : leur poste est le camp, en nombre illimité.
- **Le plan de forge** : un nouveau type de plan, avec son enclume posée automatiquement, son coffre et son poste de forgeron.
- **Parler à un dormeur le réveille** : le roi n'attend pas qu'il se réveille.
- Hors périmètre : les autres **plans de production** (cabane de bûcheron, mine, champ) et leurs postes, l'**approvisionnement en fer** de la forge (aucune mine n'est constructible), les compétences, un salaire ou un coût d'embauche, l'IA du roi qui embaucherait seule, l'annulation d'une conversation par l'éloignement du roi pendant un menu.

## Capabilities

### New Capabilities
- `conversation`: le roi qui s'adresse à quelqu'un (même endormi), l'interruption et la reprise, le ralentissement des vagabonds.
- `forge-plan`: la forge dans la liste des plans, son enclume, son coffre, son poste de forgeron.
- `posts-and-roles`: métiers, postes, titulaires, affectation, remplacement, réaffectation, personne sans poste.
- `bed-assignment`: attribution automatique et manuelle des lits, fatigue sans lit.
- `assignment-interface`: la mini-carte, la liste des métiers, le vert et le rouge, la liste des employés.

### Modified Capabilities
- `housing`: un lit n'est plus une condition d'embauche ; un habitant peut ne pas avoir de lit.

## Impact

- `afk-core` : `PlanKind::Forge`, enclume et coffre de forge posés à la construction ; `Character` (lit facultatif, poste, métier conservé, interruption), `game` (commandes `Talk`, `EndTalk`, `AssignPost`, `AssignBed` ; postes ; lit automatique), `building` (postes de l'entrepôt et de la taverne), `character` (routines qui attendent un poste, ralentissement), `config`.
- `afk-game` : l'invite « parler », le menu, la mini-carte, la liste des métiers, le panneau des employés (`hud`, `plan_ui` voisin).
- `knowledge/` : `regles/propriete-et-affectation.md`, `metiers/vagabond.md`, `metiers/roi.md`, `regles/habitat.md`, `moteur/*`, `log.md`.
- Un compagnon recommandé : le changement `production-plans` (cabane de bûcheron, mine, champ et leurs postes). Avec la forge, la mini-carte offre déjà trois métiers ; sans mine, la forge manque de fer (voir les questions ouvertes).
