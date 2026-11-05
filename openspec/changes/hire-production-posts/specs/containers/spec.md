# containers Specification (delta)

## MODIFIED Requirements

### Requirement: Limite de 200 par ressource
Un coffre posé par le roi SHALL contenir au plus **200 unités de sa seule ressource** (réglage de la configuration). Un grand coffre d'entrepôt SHALL contenir au plus **200 unités au total, tous types confondus**. Le coffre du camp et les coffres du village de départ gardent leur règle actuelle. Aucun dépôt, de producteur ou de transporteur, ne SHALL dépasser ces limites.

#### Scenario: Contenant plein
- **WHEN** un coffre de bois contient déjà 200 unités de bois
- **THEN** plus aucune unité n'y est ajoutée

#### Scenario: Grand coffre
- **WHEN** un grand coffre contient déjà 200 unités de ressources mélangées
- **THEN** aucune unité de n'importe quel type n'y est ajoutée

### Requirement: Dépôt par le producteur
Au camp, tout producteur (bûcheron, mineur, paysan) SHALL **porter lui-même** sa production jusqu'au coffre le plus proche qui l'accepte (voir `chests`), y compris le mineur, qui ne dépose plus à distance. Au village de départ, le minerai extrait reste ajouté au contenant du filon sans trajet.

#### Scenario: Le mineur au camp
- **WHEN** un mineur du camp termine une unité de fer
- **THEN** il la garde en main et la porte au coffre de fer le plus proche

#### Scenario: Le mineur au village
- **WHEN** un mineur du village de départ termine une unité de fer
- **THEN** le contenant de sa mine contient une unité de fer de plus, sans trajet
