# plan-interface Specification

## Purpose
Définit l'interface avec laquelle le roi choisit et place ses plans.

## ADDED Requirements

### Requirement: Une liste de plans quand on incarne le roi
Quand le joueur incarne le roi, l'écran SHALL afficher une liste des plans, avec pour chacun son nom, sa taille minimale et son coût minimal. Quand il n'incarne pas le roi, un message lui dit de l'incarner.

#### Scenario: Le roi
- **WHEN** le joueur incarne le roi
- **THEN** la liste des plans est visible et un clic sur un plan le choisit

### Requirement: Placer un plan avec un rectangle
Une fois un plan choisi, un rectangle en surbrillance SHALL suivre la souris, à la taille minimale. Un clic SHALL fixer son coin haut-gauche ; la souris l'agrandit alors, jamais en dessous du minimum, et un second clic le valide. Le coût en bois et en pierre et la durée SHALL se recalculer à chaque changement de forme. Pour une maison, le nombre de lits se règle et reste dans ce que la taille permet.

#### Scenario: Agrandir
- **WHEN** le joueur étire le rectangle
- **THEN** le coût et la durée affichés augmentent

#### Scenario: Valider
- **WHEN** le joueur valide un plan posable
- **THEN** une commande de pose est envoyée au cœur et le joueur peut en poser un autre

### Requirement: Bleu, rouge ou gris
Le rectangle SHALL être bleu quand le royaume a les ressources, rouge quand il ne les a pas, gris quand le terrain ne permet pas de poser. Un plan rouge peut être validé ; un plan gris ne le peut pas.

#### Scenario: Pas assez de ressources
- **WHEN** le coût dépasse ce qui est disponible
- **THEN** le rectangle est rouge, et la validation reste possible

#### Scenario: Terrain bloqué
- **WHEN** l'emplacement est bloqué
- **THEN** le rectangle est gris et la validation est refusée avec un message

### Requirement: Annuler
Échap SHALL revenir de l'agrandissement au déplacement, puis du déplacement à la liste, sans quitter le jeu. Échap quitte le jeu seulement quand aucun plan n'est en cours.

#### Scenario: Échap pendant un plan
- **WHEN** un plan est en cours de placement et que le joueur appuie sur Échap
- **THEN** le jeu ne se ferme pas

### Requirement: Le temps ne s'arrête pas
Le placement d'un plan ne SHALL jamais mettre le jeu en pause ni changer sa vitesse.

#### Scenario: Pendant la pose
- **WHEN** le joueur place un plan
- **THEN** le royaume continue de tourner à la même vitesse

### Requirement: Un clic de l'interface n'incarne personne
Un clic pris par le panneau ou par un plan en cours de placement SHALL NOT incarner le personnage placé dessous.

#### Scenario: Clic sur le panneau
- **WHEN** le joueur clique sur le panneau des plans
- **THEN** aucun personnage n'est incarné
