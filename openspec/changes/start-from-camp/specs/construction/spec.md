# construction Specification

## Purpose
Définit les plans du roi, leur coût et leur durée, et la vie d'un chantier : fondations, livraison des matériaux, construction par les artisans, bâtiment fini et route.

## ADDED Requirements

### Requirement: Les types de plan et leur taille minimale
Le roi SHALL pouvoir choisir parmi une maison, un entrepôt et une taverne. Chaque type a une taille minimale (murs compris) ; la taille est libre au-dessus, jusqu'à un maximum. Une maison SHALL avoir au moins un lit et au plus ce que sa taille permet ; les autres types n'ont pas de lit.

#### Scenario: Taille minimale
- **WHEN** on pose un plan plus petit que le minimum de son type
- **THEN** il est refusé

#### Scenario: Lits
- **WHEN** on pose une maison sans lit, ou avec plus de lits qu'elle n'en peut loger
- **THEN** le plan est refusé

### Requirement: Le coût et la durée suivent la surface et le type
Le coût en bois et en pierre d'un plan, et la durée de sa construction, SHALL croître avec sa surface, dépendre de son type, et croître avec chaque lit au-delà du premier. Les assets de base sont compris dans le prix.

#### Scenario: Plus grand, plus cher et plus long
- **WHEN** on compare deux plans du même type dont l'un est plus grand
- **THEN** le plus grand coûte plus de bois et de pierre et prend plus de temps

#### Scenario: Le type compte
- **WHEN** on compare deux types sur la même surface
- **THEN** leurs coûts ou leurs durées diffèrent

### Requirement: Où poser un plan
Un plan SHALL être refusé si son emprise, la case devant sa porte ou la case de son tas de matériaux n'est pas de l'herbe ou un chemin (eau, arbre, filon, coffre, champ, mur, autre chantier), si elle touche le bord de la carte, ou si quelqu'un se tient sur l'emprise.

#### Scenario: Terrain bloqué
- **WHEN** l'emprise recouvre de l'eau, un arbre ou un filon
- **THEN** le plan est refusé

#### Scenario: Quelqu'un dans l'emprise
- **WHEN** un personnage se tient sur l'emplacement
- **THEN** le plan est refusé

### Requirement: Seul le roi pose un plan
Un plan SHALL être posé seulement par un joueur qui incarne le roi.

#### Scenario: Un artisan
- **WHEN** un joueur qui incarne un artisan demande à poser un plan
- **THEN** rien n'est posé

### Requirement: Les fondations bloquent le passage
À la pose d'un plan, des fondations SHALL apparaître sur toute son emprise et être infranchissables jusqu'à la fin de la construction.

#### Scenario: Fondations
- **WHEN** un plan est posé
- **THEN** son emprise est faite de fondations solides ; on ne les traverse pas

### Requirement: Un chantier ne commence que si tout est disponible
Un plan SHALL pouvoir être posé même sans les ressources nécessaires. Son chantier ne commence alors que lorsque le bois et la pierre dont il a besoin sont tous disponibles ; ce qui est promis à un chantier n'est plus disponible pour un autre. Les chantiers sont servis dans l'ordre des placements, sans qu'un chantier trop cher bloque les suivants.

#### Scenario: Sans ressources
- **WHEN** on pose un plan que le stock ne paie pas
- **THEN** le plan est accepté, le chantier attend et rien n'est pris dans les coffres

#### Scenario: Matériel promis
- **WHEN** deux plans dépassent ensemble le stock mais chacun y tient
- **THEN** un seul démarre, l'autre attend

### Requirement: Les matériaux sont portés devant le chantier
Quand un chantier peut démarrer, ses matériaux SHALL être portés en tas devant lui, par les transporteurs et par les artisans qui n'ont pas de chantier à construire. Le chantier passe en construction quand tout le matériel est là, et pas avant.

#### Scenario: Livraison puis construction
- **WHEN** un chantier démarre
- **THEN** un tas apparaît devant lui, se remplit, et le chantier ne passe en construction que lorsque le tas est complet

### Requirement: Les artisans construisent, d'abord
Un artisan SHALL construire dès qu'un chantier a tout son matériel, avant toute autre tâche. Les artisans qui construisent ensemble additionnent leur travail ; sans chantier à construire, un artisan fait le travail d'un transporteur. Le joueur qui incarne un artisan peut construire en regardant les fondations.

#### Scenario: Deux artisans
- **WHEN** deux artisans construisent le même chantier
- **THEN** il se termine deux fois plus vite qu'avec un seul

#### Scenario: Sans chantier
- **WHEN** un artisan n'a rien à construire et qu'un chantier attend ses matériaux
- **THEN** il porte des matériaux

### Requirement: Le bâtiment fini et sa route
À la fin d'un chantier, le bâtiment SHALL prendre la place des fondations, les matériaux livrés SHALL être consommés, et une route SHALL se tracer de la case devant sa porte jusqu'au camp de base. Aucune route n'apparaît avant la fin.

#### Scenario: Une maison
- **WHEN** le chantier d'une maison se termine
- **THEN** c'est un habitat avec une place libre par lit, ses murs et sa porte sont sur la carte, et le chemin jusqu'au camp est de la route

#### Scenario: Un entrepôt
- **WHEN** le chantier d'un entrepôt se termine
- **THEN** un coffre d'entrepôt vide, de beaucoup plus grande capacité que celui du camp, se trouve devant sa porte

#### Scenario: Une taverne
- **WHEN** le chantier d'une taverne se termine
- **THEN** c'est un bâtiment avec son mobilier, qui n'est pas un habitat
