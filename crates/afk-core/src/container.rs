//! Les contenants : le coffre de chaque lieu de travail.
//!
//! Un contenant est une tuile `Tile::Crate` posée à l'extérieur d'un bâtiment. Il garde, pour chaque
//! ressource, une quantité plafonnée par la capacité (200). Il sait ce que son métier produit (que les
//! transporteurs peuvent emporter) et ce qu'il consomme (que les transporteurs doivent lui apporter).
//! Le dépôt central, lui, reçoit tout ce que personne ne consomme.

use crate::map::{Tile, TileMap};
use crate::math::TilePos;
use crate::mine::Vein;
use crate::resources::Resource;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerKind {
    /// Le coffre du champ du paysan.
    Farm,
    /// Le coffre de la mine d'un filon (indice dans la liste des filons).
    Mine(usize),
    /// Le coffre de la cabane d'une forêt (indice dans la liste des forêts).
    Lodge(usize),
    /// Le coffre de la forge.
    Forge,
    /// L'entrepôt central, qui reçoit ce que personne ne consomme.
    Depot,
}

/// Un contenant tel que le scénario le décrit : où il est et à qui il sert.
#[derive(Clone, Copy, Debug)]
pub struct ContainerSpec {
    pub kind: ContainerKind,
    pub tile: TilePos,
}

pub struct Container {
    pub kind: ContainerKind,
    /// La tuile du coffre.
    pub tile: TilePos,
    capacity: u32,
    stock: BTreeMap<Resource, u32>,
    /// Ce que son métier produit : les transporteurs peuvent l'emporter.
    product: Option<Resource>,
    /// Ce que son métier consomme : les transporteurs doivent le lui apporter.
    consumes: Option<Resource>,
}

impl Container {
    pub fn new(spec: ContainerSpec, capacity: u32, veins: &[Vein]) -> Container {
        let (product, consumes) = match spec.kind {
            ContainerKind::Farm => (Some(Resource::Wheat), None),
            ContainerKind::Mine(i) => (veins.get(i).map(|v| v.kind.resource()), None),
            ContainerKind::Lodge(_) => (Some(Resource::Wood), None),
            ContainerKind::Forge => (Some(Resource::Tools), Some(Resource::Iron)),
            ContainerKind::Depot => (None, None),
        };
        Container { kind: spec.kind, tile: spec.tile, capacity, stock: BTreeMap::new(), product, consumes }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn get(&self, resource: Resource) -> u32 {
        self.stock.get(&resource).copied().unwrap_or(0)
    }

    /// Place restante pour cette ressource.
    pub fn room(&self, resource: Resource) -> u32 {
        self.capacity.saturating_sub(self.get(resource))
    }

    /// Ajoute jusqu'à `amount` unités, dans la limite de la place. Renvoie ce qui est entré.
    pub(crate) fn add(&mut self, resource: Resource, amount: u32) -> u32 {
        let added = amount.min(self.room(resource));
        if added > 0 {
            *self.stock.entry(resource).or_insert(0) += added;
        }
        added
    }

    /// Retire jusqu'à `amount` unités. Renvoie ce qui est sorti.
    /// Vide le contenant : toutes les ressources disparaissent (la touche C de l'entrepôt central).
    pub(crate) fn clear(&mut self) {
        self.stock.clear();
    }

    pub(crate) fn take(&mut self, resource: Resource, amount: u32) -> u32 {
        let taken = amount.min(self.get(resource));
        if taken > 0 {
            *self.stock.entry(resource).or_insert(0) -= taken;
        }
        taken
    }

    pub fn product(&self) -> Option<Resource> {
        self.product
    }

    pub fn consumes(&self) -> Option<Resource> {
        self.consumes
    }

    pub fn is_depot(&self) -> bool {
        self.kind == ContainerKind::Depot
    }

    /// Un porteur (bûcheron, joueur) peut y déposer cette ressource : ce que le contenant consomme,
    /// sa propre production, ou n'importe quoi à l'entrepôt central.
    pub fn accepts(&self, resource: Resource) -> bool {
        self.is_depot() || self.consumes == Some(resource) || self.product == Some(resource)
    }

    /// Les ressources à afficher sur le coffre : celles qu'il produit ou consomme, même à zéro,
    /// et toutes celles qu'il contient.
    pub fn shown(&self) -> Vec<Resource> {
        let mut shown: Vec<Resource> = Vec::new();
        for r in [self.product, self.consumes].into_iter().flatten() {
            if !shown.contains(&r) {
                shown.push(r);
            }
        }
        for (&r, &n) in &self.stock {
            if n > 0 && !shown.contains(&r) {
                shown.push(r);
            }
        }
        shown.sort();
        shown
    }
}

/// Le contenant posé sur cette tuile de coffre, s'il y en a un.
pub fn container_at(containers: &[Container], tile: TilePos) -> Option<usize> {
    containers.iter().position(|c| c.tile == tile)
}

/// Pose les coffres sur la carte.
pub fn place_crates(map: &mut TileMap, specs: &[ContainerSpec]) {
    for spec in specs {
        map.set(spec.tile, Tile::Crate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mine::{OreKind, Quality};

    fn veins() -> Vec<Vein> {
        vec![Vein::new(1, 1, OreKind::Iron, Quality::Pure), Vein::new(5, 5, OreKind::Gold, Quality::Pure)]
    }

    fn container(kind: ContainerKind) -> Container {
        Container::new(ContainerSpec { kind, tile: TilePos::new(0, 0) }, 200, &veins())
    }

    #[test]
    fn a_container_never_holds_more_than_its_capacity() {
        let mut c = container(ContainerKind::Mine(0));
        assert_eq!(c.add(Resource::Iron, 190), 190);
        assert_eq!(c.add(Resource::Iron, 30), 10, "seule la place restante entre");
        assert_eq!(c.get(Resource::Iron), 200);
        assert_eq!(c.room(Resource::Iron), 0);
        assert_eq!(c.add(Resource::Iron, 1), 0);
    }

    #[test]
    fn a_full_resource_does_not_block_another() {
        let mut c = container(ContainerKind::Forge);
        c.add(Resource::Iron, 200);
        assert_eq!(c.add(Resource::Tools, 5), 5);
        assert_eq!(c.get(Resource::Tools), 5);
    }

    #[test]
    fn take_returns_only_what_is_there() {
        let mut c = container(ContainerKind::Depot);
        c.add(Resource::Wood, 4);
        assert_eq!(c.take(Resource::Wood, 10), 4);
        assert_eq!(c.get(Resource::Wood), 0);
        assert_eq!(c.take(Resource::Wood, 1), 0);
    }

    #[test]
    fn each_kind_produces_and_consumes_what_it_should() {
        assert_eq!(container(ContainerKind::Farm).product(), Some(Resource::Wheat));
        assert_eq!(container(ContainerKind::Mine(0)).product(), Some(Resource::Iron));
        assert_eq!(container(ContainerKind::Mine(1)).product(), Some(Resource::Gold));
        assert_eq!(container(ContainerKind::Lodge(0)).product(), Some(Resource::Wood));
        let forge = container(ContainerKind::Forge);
        assert_eq!((forge.product(), forge.consumes()), (Some(Resource::Tools), Some(Resource::Iron)));
        let depot = container(ContainerKind::Depot);
        assert_eq!((depot.product(), depot.consumes()), (None, None));
    }

    #[test]
    fn acceptance_follows_the_trade() {
        let forge = container(ContainerKind::Forge);
        assert!(forge.accepts(Resource::Iron) && forge.accepts(Resource::Tools));
        assert!(!forge.accepts(Resource::Gold));
        let mine = container(ContainerKind::Mine(1));
        assert!(mine.accepts(Resource::Gold) && !mine.accepts(Resource::Iron));
        let depot = container(ContainerKind::Depot);
        assert!(depot.accepts(Resource::Wheat) && depot.accepts(Resource::Stone));
    }

    #[test]
    fn the_shown_resources_include_products_even_at_zero() {
        let mut forge = container(ContainerKind::Forge);
        assert_eq!(forge.shown(), vec![Resource::Iron, Resource::Tools]);
        let mut depot = container(ContainerKind::Depot);
        assert!(depot.shown().is_empty());
        depot.add(Resource::Stone, 3);
        assert_eq!(depot.shown(), vec![Resource::Stone]);
        forge.add(Resource::Gold, 1);
        assert!(forge.shown().contains(&Resource::Gold));
    }
}
