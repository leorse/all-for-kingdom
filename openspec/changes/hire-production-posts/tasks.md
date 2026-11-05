# Tasks

## 1. Coffres dans le cœur

- [ ] 1.1 `Config` : `chest_wood`, `chest_capacity`, `chest_reach`, `work_reach`, `field_reach`, `warehouse_chests`, `big_chest_capacity` ; `ContainerKind::Chest(i)` (une ressource, 200) ; tests (un coffre de bois refuse la pierre, plein à 200)
- [ ] 1.2 `Command::PlaceChest` : valider (roi incarné, tuile libre, bois au camp), retirer le coût, créer le coffre ; tests (refus occupé, refus sans bois, rien posé automatiquement avec un champ)
- [ ] 1.3 Entrepôt bâti : plus de coffre extérieur, `warehouse_chests` grands coffres intérieurs (200 mélangés, tous types) ; accès par la porte, sans entrer ; tests (somme 200 tous types, rien dehors, dépôt et prélèvement à la porte)
- [ ] 1.4 Dépôt au plus proche : le producteur sans joueur porte sa cargaison au contenant le plus proche qui l'accepte (coffre de sa ressource ou entrepôt) ; le mineur du camp porte aussi ; tests (plus proche, mauvais coffre ignoré, tout plein : il attend)
- [ ] 1.5 Le village de départ est inchangé ; test de non-régression (mêmes totaux sur une journée qu'avant)

## 2. Postes de production

- [ ] 2.1 `Game::posts()` inclut les coffres posés près de leur matière (bois-forêt, minerai-filon de même nature, blé-champ), `Role::has_posts` s'étend à bûcheron, mineur, paysan, `AssignPost` les accepte ; tests (poste libre à la pose, pas de poste loin de la matière ou au mauvais filon, vagabond devenu bûcheron, remplacement, remplacé sans poste ne fait rien)
- [ ] 2.2 Bûcheron au camp : coupe l'arbre le plus proche de son coffre, y rapporte son bois ; test (la forêt la plus proche, coffre plein = attente)
- [ ] 2.3 Mineur au camp : frappe le filon de la nature de son coffre, rapporte son minerai ; poste lié à une `MineSite` (surface seulement) ; tests

## 3. Le champ du paysan

- [ ] 3.1 `Command::PlaceField` (roi ou paysan incarné) : carré 3×3 d'herbe libre, gratuit, refusé sinon ; parcelles ajoutées en cours de partie à `Farm` ; tests
- [ ] 3.2 Rattachement d'un champ au coffre de blé le plus proche à portée, recalculé à chaque pose de coffre ; le paysan n'entretient que ses champs ; tests (champ sans coffre : attente, égalité : premier coffre, deux paysans ne se disputent pas)
- [ ] 3.3 `ChooseCrop` : accepté du seul paysan incarné quand plusieurs graines existent, refusé du roi ; l'IA sème la culture la plus fournie ; tests avec une graine factice

## 4. Interface

- [ ] 4.1 Cartes « Coffre » (choix de la ressource) et « Champ » dans `plan_ui.rs` (bleu/rouge/gris, motif du refus, poste annoncé) ; vérifier visuellement
- [ ] 4.2 Grands coffres dessinés dans l'entrepôt ; repères des coffres-poste sur la mini-carte et dans la liste des métiers ; choix de culture pour le paysan incarné ; vérifier visuellement
- [ ] 4.3 Aide (`H`) et README à jour

## 5. Documentation et vérification

- [ ] 5.1 `knowledge/` : paysan, bûcheron, mineur, transporteur, ressources et stocks, propriété et affectation, construire, départ, moteur (coffres et transport), log ; sources et `etat_en_jeu`
- [ ] 5.2 `cargo test` complet puis une partie depuis le camp : poser un coffre de bois près d'une forêt, embaucher un vagabond, vérifier que le bois monte ; idem mineur et paysan (champ 3×3) ; bâtir un entrepôt et vérifier que le transport passe par la porte
