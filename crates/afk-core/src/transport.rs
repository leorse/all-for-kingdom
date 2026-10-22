//! Le transport : quels voyages un transporteur peut entreprendre d'un contenant à un autre.
//!
//! Le planificateur est pur : il lit les contenants et les réservations des autres transporteurs, et
//! renvoie le meilleur voyage. Personne ne dit aux transporteurs quoi porter (voir `docs/gameplay`).
//!
//! Règles :
//! - un contenant donne ce que son métier produit ; l'entrepôt central donne ce qu'un consommateur réclame ;
//! - une ressource va d'abord à un contenant qui la consomme (le fer à la forge), sinon à l'entrepôt central ;
//! - un voyage porte `load` unités, ou moins s'il remplit exactement la place qui reste à destination ;
//! - ce que les autres transporteurs vont prendre ou apporter est déjà compté (réservations).

use crate::container::Container;
use crate::game::CharacterId;
use crate::math::TilePos;
use crate::resources::Resource;

/// Un voyage : prendre `amount` de `resource` dans `from` et le porter à `to` (indices de contenants).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Job {
    pub from: usize,
    pub to: usize,
    pub resource: Resource,
    pub amount: u32,
}

/// Ce qu'un transporteur a promis de prendre (`incoming == false`) ou d'apporter (`incoming == true`).
#[derive(Clone, Copy, Debug)]
pub struct Reservation {
    pub owner: CharacterId,
    pub container: usize,
    pub resource: Resource,
    pub amount: u32,
    pub incoming: bool,
}

fn reserved(reservations: &[Reservation], me: CharacterId, container: usize, resource: Resource, incoming: bool) -> u32 {
    reservations
        .iter()
        .filter(|r| r.owner != me && r.container == container && r.resource == resource && r.incoming == incoming)
        .map(|r| r.amount)
        .sum()
}

fn distance(a: TilePos, b: TilePos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// Ce contenant peut-il donner cette ressource aux transporteurs ?
fn gives(containers: &[Container], source: &Container, resource: Resource) -> bool {
    if source.is_depot() {
        // l'entrepôt central redonne ce qu'un consommateur réclame, jamais le reste
        containers.iter().any(|c| !c.is_depot() && c.consumes() == Some(resource))
    } else {
        source.product() == Some(resource)
    }
}

/// Le meilleur voyage pour un transporteur situé en `from`, ou `None` s'il n'y en a aucun.
/// Priorité : destination consommatrice avant entrepôt central ; puis la plus grosse charge ; puis la
/// plus courte distance ; en dernier l'ordre des contenants (pour rester déterministe).
pub fn best_trip(
    containers: &[Container],
    reservations: &[Reservation],
    me: CharacterId,
    load: u32,
    from: TilePos,
) -> Option<Job> {
    let mut best: Option<((u8, std::cmp::Reverse<u32>, i32, usize, usize), Job)> = None;
    for (si, source) in containers.iter().enumerate() {
        for &resource in &Resource::ALL {
            if !gives(containers, source, resource) {
                continue;
            }
            let available = source.get(resource).saturating_sub(reserved(reservations, me, si, resource, false));
            if available == 0 {
                continue;
            }
            for (di, dest) in containers.iter().enumerate() {
                if di == si {
                    continue;
                }
                let priority = if dest.consumes() == Some(resource) {
                    0
                } else if dest.is_depot() && !source.is_depot() {
                    1
                } else {
                    continue;
                };
                let room = dest.room(resource).saturating_sub(reserved(reservations, me, di, resource, true));
                let amount = load.min(available).min(room);
                // une charge complète, ou celle qui remplit exactement la place restante
                if amount == 0 || !(amount == load || amount == room) {
                    continue;
                }
                let key = (
                    priority,
                    std::cmp::Reverse(amount),
                    distance(from, source.tile) + distance(source.tile, dest.tile),
                    si,
                    di,
                );
                if best.as_ref().map_or(true, |(k, _)| key < *k) {
                    best = Some((key, Job { from: si, to: di, resource, amount }));
                }
            }
        }
    }
    best.map(|(_, job)| job)
}

/// Où porter une cargaison déjà en main quand sa destination ne peut plus la recevoir : un contenant
/// qui la consomme, sinon l'entrepôt central, pourvu qu'il reste de la place. `None` : attendre.
pub fn best_destination(
    containers: &[Container],
    reservations: &[Reservation],
    me: CharacterId,
    resource: Resource,
    from: TilePos,
) -> Option<usize> {
    containers
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            let priority = if c.consumes() == Some(resource) {
                0
            } else if c.is_depot() {
                1
            } else {
                return None;
            };
            let room = c.room(resource).saturating_sub(reserved(reservations, me, i, resource, true));
            (room > 0).then(|| ((priority, distance(from, c.tile), i), i))
        })
        .min_by_key(|&(key, _)| key)
        .map(|(_, i)| i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::{ContainerKind, ContainerSpec};
    use crate::mine::{OreKind, Quality, Vein};

    const ME: CharacterId = CharacterId(0);
    const OTHER: CharacterId = CharacterId(1);

    /// 0 : mine de fer, 1 : mine d'or, 2 : forge, 3 : entrepôt central, 4 : cabane
    fn world() -> Vec<Container> {
        let veins = [Vein::new(0, 0, OreKind::Iron, Quality::Pure), Vein::new(9, 9, OreKind::Gold, Quality::Pure)];
        let at = |kind, x, y| Container::new(ContainerSpec { kind, tile: TilePos::new(x, y) }, 200, &veins);
        vec![
            at(ContainerKind::Mine(0), 0, 0),
            at(ContainerKind::Mine(1), 20, 0),
            at(ContainerKind::Forge, 10, 0),
            at(ContainerKind::Depot, 10, 10),
            at(ContainerKind::Lodge(0), 0, 10),
        ]
    }

    fn trip(world: &[Container], reservations: &[Reservation]) -> Option<Job> {
        best_trip(world, reservations, ME, 10, TilePos::new(10, 10))
    }

    #[test]
    fn iron_goes_to_the_forge_first() {
        let mut w = world();
        w[0].add(Resource::Iron, 25);
        let job = trip(&w, &[]).unwrap();
        assert_eq!((job.from, job.to, job.resource, job.amount), (0, 2, Resource::Iron, 10));
    }

    #[test]
    fn what_nobody_consumes_goes_to_the_depot() {
        let mut w = world();
        w[1].add(Resource::Gold, 10);
        let job = trip(&w, &[]).unwrap();
        assert_eq!((job.from, job.to, job.resource, job.amount), (1, 3, Resource::Gold, 10));
    }

    #[test]
    fn a_source_below_a_full_load_waits() {
        let mut w = world();
        w[0].add(Resource::Iron, 4);
        assert_eq!(trip(&w, &[]), None);
    }

    #[test]
    fn a_full_forge_sends_the_iron_to_the_depot() {
        let mut w = world();
        w[0].add(Resource::Iron, 30);
        w[2].add(Resource::Iron, 200);
        let job = trip(&w, &[]).unwrap();
        assert_eq!((job.from, job.to), (0, 3));
    }

    #[test]
    fn a_trip_may_just_top_up_the_remaining_room() {
        let mut w = world();
        w[0].add(Resource::Iron, 30);
        w[2].add(Resource::Iron, 196);
        let job = trip(&w, &[]).unwrap();
        // 4 de place à la forge : la charge qui la remplit exactement (la priorité l'emporte sur le dépôt)
        assert_eq!((job.from, job.to, job.amount), (0, 2, 4));
    }

    #[test]
    fn the_depot_gives_back_only_what_a_consumer_wants() {
        let mut w = world();
        w[3].add(Resource::Iron, 20);
        w[3].add(Resource::Wood, 50);
        let job = trip(&w, &[]).unwrap();
        assert_eq!((job.from, job.to, job.resource), (3, 2, Resource::Iron));
        // une fois la forge pleine, plus rien ne sort du dépôt (le bois ne retourne pas à la cabane)
        w[2].add(Resource::Iron, 200);
        assert_eq!(trip(&w, &[]), None);
    }

    #[test]
    fn the_depot_never_sends_to_itself_and_wood_stays_put_in_a_lodge() {
        let mut w = world();
        w[4].add(Resource::Wood, 30);
        let job = trip(&w, &[]).unwrap();
        assert_eq!((job.from, job.to, job.resource), (4, 3, Resource::Wood));
    }

    #[test]
    fn a_full_depot_receives_nothing() {
        let mut w = world();
        w[1].add(Resource::Gold, 50);
        w[3].add(Resource::Gold, 200);
        assert_eq!(trip(&w, &[]), None);
    }

    #[test]
    fn three_transporters_and_one_room_only_one_leaves() {
        let mut w = world();
        w[0].add(Resource::Iron, 100);
        w[2].add(Resource::Iron, 190); // place pour 10 à la forge
        let mut reservations = Vec::new();
        let mut leaving = 0;
        for who in [CharacterId(10), CharacterId(11), CharacterId(12)] {
            if let Some(job) = best_trip(&w, &reservations, who, 10, TilePos::new(10, 10)) {
                if job.to == 2 {
                    leaving += 1;
                }
                reservations.push(Reservation { owner: who, container: job.from, resource: job.resource, amount: job.amount, incoming: false });
                reservations.push(Reservation { owner: who, container: job.to, resource: job.resource, amount: job.amount, incoming: true });
            }
        }
        assert_eq!(leaving, 1, "la forge n'a de place que pour un voyage");
        let incoming_forge: u32 = reservations.iter().filter(|r| r.incoming && r.container == 2).map(|r| r.amount).sum();
        assert!(190 + incoming_forge <= 200);
    }

    #[test]
    fn my_own_reservations_do_not_block_me() {
        let mut w = world();
        w[0].add(Resource::Iron, 10);
        let mine = [Reservation { owner: ME, container: 0, resource: Resource::Iron, amount: 10, incoming: false }];
        assert!(trip(&w, &mine).is_some());
        let others = [Reservation { owner: OTHER, container: 0, resource: Resource::Iron, amount: 10, incoming: false }];
        assert_eq!(trip(&w, &others), None, "ces 10 fer sont déjà promis");
    }

    #[test]
    fn nearer_and_bigger_trips_come_first() {
        let mut w = world();
        w[0].add(Resource::Iron, 10);
        w[1].add(Resource::Gold, 30);
        // à charge égale (10), le plus proche gagne : on est en (10,10), la mine de fer est à 20 de là, celle d'or à 20 aussi ;
        // le fer va à la forge (priorité consommateur)
        let job = trip(&w, &[]).unwrap();
        assert_eq!(job.resource, Resource::Iron);
    }

    #[test]
    fn a_carried_cargo_is_rerouted_to_a_consumer_then_to_the_depot() {
        let mut w = world();
        assert_eq!(best_destination(&w, &[], ME, Resource::Iron, TilePos::new(0, 0)), Some(2));
        w[2].add(Resource::Iron, 200);
        assert_eq!(best_destination(&w, &[], ME, Resource::Iron, TilePos::new(0, 0)), Some(3));
        w[3].add(Resource::Iron, 200);
        assert_eq!(best_destination(&w, &[], ME, Resource::Iron, TilePos::new(0, 0)), None);
    }
}
