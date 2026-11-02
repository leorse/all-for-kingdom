# Tasks

## 1. Le modèle du cœur

- [ ] 1.1 `Character::home` facultatif ; sommeil, fatigue et `free_places` adaptés ; vérifier que tous les tests existants passent et qu'une personne sans lit ne dort jamais et s'épuise
- [ ] 1.2 `Post` et `Game::posts()` dérivés des bâtiments (entrepôt, taverne, camp) ; vérifier par des tests (libres, occupés, nombre par bâtiment)
- [ ] 1.3 Une personne sans poste ne travaille pas dans le camp (`idle_routine`) ; le village ne change pas ; vérifier par des tests

## 2. Parler

- [ ] 2.1 `Character::paused`, `Talk` et `EndTalk` ; vérifier que la personne arrête tout, regarde le roi et reprend exactement (même travail, même cargaison, même chemin)
- [ ] 2.2 Ralentissement des vagabonds près du roi, sans bouton ; vérifier qu'il ne touche aucun autre rôle

## 3. Affecter

- [ ] 3.1 `AssignPost` : poste libre, poste occupé (remplacement, le titulaire garde son métier), réaffectation (le poste quitté se libère), le vagabond devient habitant ; vérifier par des tests
- [ ] 3.2 Lit automatique (le plus proche du poste, réévalué quand un lit apparaît) et `AssignBed` (remplacement) ; vérifier par des tests

## 4. L'interface

- [ ] 4.1 Invite « parler » près de quelqu'un, menu (poste, lit, rien), reprise
- [ ] 4.2 Mini-carte : liste des métiers (libres d'abord), postes verts et rouges, clic pour affecter ou remplacer ; mode lits
- [ ] 4.3 Panneau des employés (nom, métier, poste, symbole de lit)

## 5. Documentation et vérification

- [ ] 5.1 `knowledge/` (règles, métiers, moteur, journal), `README.md`
- [ ] 5.2 `cargo test` complet ; une partie réelle : embaucher un vagabond transporteur depuis l'entrepôt et le voir livrer
