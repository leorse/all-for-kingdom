# Tasks

## 1. Le coffre-poste dans le cœur

- [ ] 1.1 `ChestKind {Lodge, Mine, Farm}`, `Config::{chest_wood, chest_reach, work_reach, field_reach}`, `Command::PlaceChest` : valider (roi incarné, tuile libre, matière à portée, bois au camp), retirer le coût, créer le contenant lié ; tests unitaires (refus loin de la matière, refus sans bois, coût retiré, un filon impose sa nature)
- [ ] 1.2 `Game::posts()` inclut les coffres-poste, `Role::has_posts` s'étend à bûcheron, mineur, paysan, `AssignPost` les accepte ; tests (poste libre à la pose, vagabond devenu bûcheron, remplacement, remplacé sans poste ne fait rien)
- [ ] 1.3 Bûcheron au camp : coupe l'arbre le plus proche de son coffre, livre à ce coffre ; test (la forêt la plus proche est choisie, un coffre plein arrête la coupe)
- [ ] 1.4 Mineur au camp : frappe le filon à portée de son coffre, dépose dans ce coffre ; test (la nature du coffre est celle du filon, plein = arrêt)
- [ ] 1.5 Le village de départ est inchangé ; test de non-régression (mêmes totaux sur une journée qu'avant)

## 2. Le champ du paysan

- [ ] 2.1 `Command::MarkField` (roi ou paysan incarné) : herbe libre en friche, gratuit, refusé sur le reste ; parcelles ajoutées en cours de partie à `Farm` ; tests
- [ ] 2.2 Rattachement d'une parcelle au coffre-poste de paysan le plus proche à portée, recalculé à la pose d'un coffre ; le paysan ne travaille que ses parcelles ; tests (parcelle sans coffre : attente, égalité : premier coffre, deux paysans ne se disputent pas)
- [ ] 2.3 `ChooseCrop` : accepté du seul paysan incarné quand plusieurs graines existent, refusé du roi ; l'IA sème la culture la plus fournie ; tests avec une graine factice

## 3. Interface

- [ ] 3.1 Cartes `Poste de bucheron / mineur / paysan` et carte `Champ` dans `plan_ui.rs` (tuile unique, bleu/rouge/gris, motif du refus) ; vérifier visuellement
- [ ] 3.2 Repères des coffres-poste sur la mini-carte et dans la liste des métiers ; choix de culture pour le paysan incarné ; vérifier visuellement
- [ ] 3.3 Aide (`H`) et README à jour

## 4. Documentation et vérification

- [ ] 4.1 `knowledge/` : paysan, bûcheron, mineur, propriété et affectation, construire, départ, log ; sources et `etat_en_jeu`
- [ ] 4.2 `cargo test` complet puis une partie depuis le camp : poser un coffre de bûcheron, embaucher un vagabond, vérifier que le bois monte ; idem mineur et paysan
