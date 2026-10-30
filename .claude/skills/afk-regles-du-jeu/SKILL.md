---
name: afk-regles-du-jeu
description: Méthode d'équilibrage et de calcul pour All For Kingdom (AFK) : temps de trajet, durées, rendements, goulots entre métiers. Pointe vers la base OKF knowledge/ (métiers, règles, moteur) qui est la source de vérité des règles. À charger pour toute question de gameplay, d'équilibrage, de durée ou de rythme.
---

# AFK - Règles du jeu et équilibrage

**Les règles sont dans la base OKF `knowledge/`**, pas ici : ce skill donne la méthode de calcul.

## Lire d'abord

1. `knowledge/index.md` : la carte de la base (métiers, règles, moteur, graphisme, idées).
2. Selon la question : `knowledge/metiers/<métier>.md`, `knowledge/regles/*.md` (temps et repos, ressources et stocks, habitat, royaume), `knowledge/moteur/*.md` (configuration et unités, personnages et déplacement, carte et chemins, coffres et transport).
3. **Les valeurs chiffrées vivent dans le code** : `crates/afk-core/src/config.rs` (`Config::default`), `character.rs` (`PLAYER_SPEED`, `AI_SPEED`, `efficiency()`), `map.rs` (`DEFAULT_GRASS_COST`). Les relire avant de citer un nombre.

Hiérarchie en cas de désaccord : **code > base `knowledge/` > specs OpenSpec (`openspec/specs/`)**. Une page `knowledge/idees.md` n'est jamais une règle. Un désaccord entre sources est à remonter à l'utilisateur.

## Le piège des unités

Heures de jeu (fatigue, sommeil, pousse, repousse) contre secondes réelles (déplacements, durées de travail). Avec 600 s par jour, **1 h de jeu = 25 s réelles**. Détail : `knowledge/moteur/configuration.md`.

## Calculer un temps de trajet

`temps (s) = distance en tuiles / (vitesse x facteur x efficacité)`, puis divisé par 25 pour des heures de jeu. La vitesse ne dépend pas du terrain ; seul le **chemin** choisi change (routes préférées, donc détours). Mesurer la distance sur le chemin réel (`find_path_cost`, voir `knowledge/moteur/carte-et-chemins.md`), pas en ligne droite. Exemple pour 50 tuiles sans détour, avec les vitesses du moment (joueur 3,4 ; IA 1,7 ; transporteur IA 1,7 x 1,5) : joueur 14,7 s, IA 29,4 s, transporteur IA 19,6 s, IA épuisée (efficacité 0,5) 58,8 s. À recalculer avec les valeurs du code.

## Raisonner un équilibrage

1. Poser le **goulot** : quel métier ou quel stock limite la chaîne (blé ; minerai, forgeron, outils ; bois, construction) ?
2. Calculer à la main : temps de cycle = trajets aller-retour + durée de travail, à la vitesse IA ; rendement par heure de jeu = cycles par heure x gain.
3. Si le calcul ne suffit pas (fatigue, partage de parcelles, interactions), **simuler** : `cargo run --release -p afk-core --example balance -- <jours> <s_par_jour> <paysans>`, ou écrire un exemple voisin dans `crates/afk-core/examples/`.
4. Proposer **un seul** levier à changer (dans `config.rs`), avec l'effet chiffré et ce qu'il dégrade ailleurs.
5. Un réglage doit tenir pour l'IA (mode AFK, deux fois plus lente) **et** pour le joueur incarné (deux fois plus rapide au travail) : ne pas casser cet écart sans le dire.

## Maintenance (obligatoire)

Tout changement de règle ou de valeur de référence met à jour la page concernée de `knowledge/` **dans le même changement**, ajoute une ligne à `knowledge/log.md` et vérifie les `sources`. Un changement de règle durable concerne aussi les specs OpenSpec : le signaler.
