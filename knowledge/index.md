---
okf_version: "0.2"
---

# All For Kingdom : base de connaissances

Bundle [OKF](https://github.com/GoogleCloudPlatform/open-knowledge-format) v0.2 : **source de vérité** du jeu pour les agents, les skills et les humains. Chaque page a un `type`, une `description`, des `sources` (le code qui la justifie) et, quand c'est utile, un `etat_en_jeu` (en place, décidé, à définir). **En cas de désaccord, le code prime**, puis cette base, puis les specs OpenSpec (`openspec/specs/`, exigences testables).

Le principe du jeu : on incarne tour à tour les habitants d'une cité médiévale pendant que l'IA fait tourner les autres. Sans personne d'incarné, la cité tourne seule : c'est le mode AFK.

## Les métiers

* [Paysan](/metiers/paysan.md) - parcelles, labour, semis, récolte, blé
* [Forgeron](/metiers/forgeron.md) - forge à l'enclume, fer, outils, liste d'objets décidée
* [Mineur](/metiers/mineur.md) - filons 2x2, qualité, coups de pioche, mine-maison
* [Bûcheron](/metiers/bucheron.md) - forêts, abattage à plusieurs, repousse
* [Transporteur](/metiers/transporteur.md) - porte les ressources entre coffres
* [Artisan](/metiers/artisan.md) - construit les plans du roi (décidé, pas en jeu)
* [Garde](/metiers/garde.md) - rondes de jour et de nuit
* [Roi](/metiers/roi.md) - répartit rôles, ressources et territoire
* [Chef de guerre](/metiers/chef-de-guerre.md) - pas encore de travail propre
* [Tavernier](/metiers/tavernier.md) - la taverne, porte d'entrée de la population

## Les règles communes

* [Temps, repos et fatigue](/regles/temps-et-repos.md) - jour/nuit, forme, incarnation, joueur 2x plus rapide que l'IA
* [Compétences](/regles/competences.md) - principe décidé, pas en jeu
* [Ressources et stocks](/regles/ressources-et-stocks.md) - sept ressources, coffres de 200, chaîne de production
* [La maison et son entrepôt](/regles/habitat.md) - trois classes d'habitat, lits, améliorations
* [Le royaume](/regles/royaume.md) - la carte, la disposition, les déplacements

## Le moteur

* [Architecture](/moteur/architecture.md) - deux crates, Command, pas fixe, déterminisme
* [Configuration et unités](/moteur/configuration.md) - Config, heures de jeu contre secondes réelles, simulation d'équilibrage
* [Personnages et déplacement](/moteur/personnages-et-deplacement.md) - rôles, vitesses, efficacité
* [Carte et chemins](/moteur/carte-et-chemins.md) - tuiles, A*, coût de l'herbe
* [Ressources, coffres et transport](/moteur/coffres-et-transport.md)
* [Rendu, caméra et entrées](/moteur/rendu-et-camera.md) - le crate d'affichage

## Le graphisme

* [Direction artistique](/graphisme/direction-artistique.md) - pixel art, tuile de 32 px, palette, personnages, zoom, lumière
* [Inventaire graphique](/graphisme/inventaire.md) - ce qui est redessiné, ce qui ne l'est pas encore

## Le projet

* [Contrôles et lancement](/projet/controles-et-lancement.md)
* [Idées en vrac](/idees.md) - pas décidé, ne jamais traiter comme une règle

## Maintenance

Toute modification du jeu (règle, valeur d'équilibrage, apparence, structure du code) met à jour la page concernée **dans le même changement**, ajoute une ligne à [journal](/log.md) et vérifie les `sources`. Les valeurs chiffrées vivent dans le code (`config.rs`) ; les pages les expliquent et ne les recopient que lorsque c'est nécessaire.
