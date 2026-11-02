# Tasks

## 1. Le modèle du cœur

- [x] 1.1 `Character::home` facultatif ; sommeil, fatigue et `free_places` adaptés ; vérifier que tous les tests existants passent et qu'une personne sans lit ne dort jamais et s'épuise
- [x] 1.2 `Post` et `Game::posts()` dérivés des bâtiments (entrepôt, taverne, camp) ; vérifier par des tests (libres, occupés, nombre par bâtiment)
- [x] 1.3 Une personne sans poste ne travaille pas dans le camp (`idle_routine`) ; le village ne change pas ; vérifier par des tests

- [x] 1.4 `PlanKind::Forge` : taille minimale, coût, durée, enclume et coffre posés à la construction, poste de forgeron ; vérifier par des tests (enclume solide avec la case nord libre et accessible, coffre à l'extérieur, personne ne forge sans forgeron affecté, un forgeron affecté forge)

## 2. Parler

- [x] 2.1 `Character::paused`, `Talk` (qui réveille un dormeur, lequel retourne se coucher si on ne décide rien) et `EndTalk` ; vérifier que la personne arrête tout, regarde le roi et reprend exactement (même travail, même cargaison, même chemin)
- [x] 2.2 Ralentissement des vagabonds près du roi, sans bouton ; vérifier qu'il ne touche aucun autre rôle

## 3. Affecter

- [x] 3.1 `AssignPost` : poste libre, poste occupé (remplacement, le titulaire garde son métier), réaffectation (le poste quitté se libère), le vagabond devient habitant ; vérifier par des tests
- [x] 3.2 Lit automatique (le plus proche du poste, réévalué quand un lit apparaît) et `AssignBed` (remplacement) ; vérifier par des tests

## 4. L'interface

- [x] 4.1 Invite « parler » près de quelqu'un, menu (poste, lit, rien), reprise
- [x] 4.2 Mini-carte : liste des métiers (libres d'abord), postes verts et rouges, clic pour affecter ou remplacer ; mode lits
- [x] 4.3 Panneau des employés (nom, métier, poste, symbole de lit)

## 5. Documentation et vérification

- [x] 5.1 `knowledge/` (règles, métiers, moteur, journal), `README.md`
- [x] 5.2 `cargo test` complet ; une partie réelle : embaucher un vagabond transporteur depuis l'entrepôt et le voir livrer, bâtir une forge et y affecter un forgeron
