//! Les ressources du royaume. Pour l'instant : de simples compteurs globaux, affichés à l'écran
//! (pas de transport ni de stockage physique).

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Resource {
    Wheat,
    Wood,
    Iron,
    Copper,
    Gold,
    Stone,
    Tools,
}

impl Resource {
    pub const ALL: [Resource; 7] = [
        Resource::Wheat,
        Resource::Wood,
        Resource::Iron,
        Resource::Copper,
        Resource::Gold,
        Resource::Stone,
        Resource::Tools,
    ];
}

#[derive(Default)]
pub struct Stock {
    items: BTreeMap<Resource, u32>,
}

impl Stock {
    pub fn new() -> Stock {
        Stock::default()
    }

    pub fn get(&self, resource: Resource) -> u32 {
        self.items.get(&resource).copied().unwrap_or(0)
    }

    pub(crate) fn add(&mut self, resource: Resource, amount: u32) {
        *self.items.entry(resource).or_insert(0) += amount;
    }

    /// Retire `amount` du stock. Renvoie `false` (et ne change rien) s'il n'y en a pas assez.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn take(&mut self, resource: Resource, amount: u32) -> bool {
        let have = self.get(resource);
        if have < amount {
            return false;
        }
        self.items.insert(resource, have - amount);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_starts_empty_and_accumulates() {
        let mut s = Stock::new();
        assert_eq!(s.get(Resource::Wheat), 0);
        s.add(Resource::Wheat, 4);
        s.add(Resource::Wheat, 3);
        assert_eq!(s.get(Resource::Wheat), 7);
    }

    #[test]
    fn take_removes_only_when_there_is_enough() {
        let mut s = Stock::new();
        s.add(Resource::Iron, 5);
        assert!(s.take(Resource::Iron, 3));
        assert_eq!(s.get(Resource::Iron), 2);
        assert!(!s.take(Resource::Iron, 3), "pas assez : refusé");
        assert_eq!(s.get(Resource::Iron), 2, "et rien n'a changé");
        assert!(s.take(Resource::Iron, 2));
        assert_eq!(s.get(Resource::Iron), 0);
        assert!(!s.take(Resource::Tools, 1));
    }

    #[test]
    fn wood_counts_logs_and_can_be_taken() {
        let mut s = Stock::new();
        s.add(Resource::Wood, 3);
        s.add(Resource::Wood, 3);
        assert_eq!(s.get(Resource::Wood), 6);
        assert!(s.take(Resource::Wood, 4));
        assert!(!s.take(Resource::Wood, 3));
        assert_eq!(s.get(Resource::Wood), 2);
    }

    #[test]
    fn every_ore_resource_accumulates_on_its_own() {
        let mut s = Stock::new();
        for (i, r) in [Resource::Copper, Resource::Gold, Resource::Stone].into_iter().enumerate() {
            s.add(r, i as u32 + 1);
        }
        assert_eq!((s.get(Resource::Copper), s.get(Resource::Gold), s.get(Resource::Stone)), (1, 2, 3));
        assert!(s.take(Resource::Gold, 2));
        assert_eq!(s.get(Resource::Gold), 0);
        assert_eq!(s.get(Resource::Stone), 3);
    }
}
