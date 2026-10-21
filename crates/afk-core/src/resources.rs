//! Les ressources du royaume. Pour l'instant : de simples compteurs globaux, affichés à l'écran
//! (pas de transport ni de stockage physique).

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Resource {
    Wheat,
    Iron,
    Tools,
}

impl Resource {
    pub const ALL: [Resource; 3] = [Resource::Wheat, Resource::Iron, Resource::Tools];
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
}
