//! Le contenu de départ d'une partie : carte et personnages.
//! C'est de la donnée pure : plus tard, elle pourra venir d'un fichier au lieu d'être écrite en dur.

use crate::character::Role;
use crate::container::{place_crates, ContainerKind, ContainerSpec};
use crate::forest::{Forest, LodgeSpec};
use crate::habitat::{HabitatClass, HabitatSpec};
use crate::map::{Tile, TileMap};
use crate::math::TilePos;
use crate::mine::{MineSpec, OreKind, Quality, Vein};

pub struct CharacterSpec {
    pub name: String,
    pub role: Role,
    /// Tuile du lit (doit se trouver dans un bâtiment).
    pub home: TilePos,
    /// Travaille la nuit et dort le jour (les gardes de nuit, par exemple).
    pub night_shift: bool,
    /// Postes de travail visités par l'IA (le premier est la position de départ).
    /// Pour un garde, c'est l'itinéraire de patrouille, parcouru dans l'ordre.
    pub work_spots: Vec<TilePos>,
}

impl CharacterSpec {
    /// Ce personnage travaille la nuit et dort le jour.
    fn on_night_shift(mut self) -> Self {
        self.night_shift = true;
        self
    }
}

pub struct Scenario {
    pub map: TileMap,
    pub veins: Vec<Vein>,
    /// Une mine par mineur : son lit et le filon qu'il exploite.
    pub mines: Vec<MineSpec>,
    pub forests: Vec<Forest>,
    /// Une cabane par bûcheron : son lit et la forêt qu'il exploite.
    pub lodges: Vec<LodgeSpec>,
    /// Les coffres, tous à l'extérieur des bâtiments.
    pub containers: Vec<ContainerSpec>,
    /// Les habitats : bâtiments où l'on dort, à taille fixe (le château n'en fait pas partie).
    pub habitats: Vec<HabitatSpec>,
    pub characters: Vec<CharacterSpec>,
}

fn spots(list: &[(i32, i32)]) -> Vec<TilePos> {
    list.iter().map(|&(x, y)| TilePos::new(x, y)).collect()
}

fn spec(name: &str, role: Role, home: (i32, i32), list: &[(i32, i32)]) -> CharacterSpec {
    CharacterSpec {
        name: name.to_string(),
        role,
        home: TilePos::new(home.0, home.1),
        night_shift: false,
        work_spots: spots(list),
    }
}

impl Scenario {
    /// Le village de départ : les bâtiments publics au centre, les maisons et les champs loin
    /// vers les bords, reliés par de longues routes (le royaume dépasse largement l'écran).
    pub fn village() -> Scenario {
        Scenario::build(1)
    }

    /// Le village avec `farmers` paysans au total (au moins 1, au plus 6), pour tester la
    /// main-d'œuvre agricole. La maison du paysan grandit avec leur nombre : petite pour un,
    /// moyenne jusqu'à trois, grande jusqu'à six ; chaque paysan a son lit.
    pub fn village_with_farmers(farmers: usize) -> Scenario {
        assert!(farmers <= HabitatClass::Large.max_places(), "pas assez de lits pour {farmers} paysans");

        let mut scenario = Scenario::build(farmers.max(1));
        let first_home = scenario.characters.iter().find(|c| c.role == Role::Farmer).expect("le paysan").home;
        let beds: Vec<TilePos> = scenario
            .habitats
            .iter()
            .find(|h| h.beds.contains(&first_home))
            .expect("la maison du paysan")
            .beds
            .clone();
        for i in 1..farmers.max(1) {
            scenario.characters.push(CharacterSpec {
                name: format!("Paysan {}", i + 1),
                role: Role::Farmer,
                home: beds[i],
                night_shift: false,
                work_spots: vec![TilePos::new(3 + i as i32, 4)],
            });
        }
        scenario
    }

    fn build(farmers: usize) -> Scenario {
        let (w, h) = (64, 40);
        let mut map = TileMap::new(w, h, Tile::Grass);

        // bordure d'eau
        map.fill_rect(0, 0, w, 1, Tile::Water);
        map.fill_rect(0, h - 1, w, 1, Tile::Water);
        map.fill_rect(0, 0, 1, h, Tile::Water);
        map.fill_rect(w - 1, 0, 1, h, Tile::Water);

        // routes : la grande route est-ouest et l'axe nord-sud du château
        map.fill_rect(1, 20, w - 2, 1, Tile::Path);
        map.fill_rect(31, 16, 1, h - 17, Tile::Path);

        // champs, à l'extrémité nord-ouest du royaume
        map.fill_rect(3, 4, 8, 6, Tile::Field);
        map.fill_rect(7, 10, 1, 10, Tile::Path); // route des champs

        // bâtiments publics : x, y, largeur, hauteur, porte (le château a son propre fonctionnement,
        // la forge et la taverne ne sont pas des habitats)
        map.add_building(28, 11, 7, 5, TilePos::new(31, 15)); // château
        map.add_building(36, 13, 6, 5, TilePos::new(38, 17)); // forge
        map.add_building(36, 22, 6, 5, TilePos::new(38, 22)); // taverne

        // l'enclume, au milieu de la forge
        map.set(TilePos::new(39, 15), Tile::Anvil);

        // chemins entre les portes et la grande route
        map.fill_rect(38, 18, 1, 2, Tile::Path);
        map.set(TilePos::new(21, 21), Tile::Path);
        map.set(TilePos::new(38, 21), Tile::Path);
        map.fill_rect(8, 21, 1, 2, Tile::Path); // la maison du paysan
        map.fill_rect(56, 9, 1, 11, Tile::Path);
        map.fill_rect(56, 21, 1, 7, Tile::Path);

        // mines : un bâtiment accolé à un filon de 2x2, relié à la route par un chemin
        let veins = vec![
            Vein::new(45, 3, OreKind::Iron, Quality::Medium),
            Vein::new(25, 34, OreKind::Gold, Quality::Impure),
            Vein::new(47, 32, OreKind::Stone, Quality::Pure),
        ];
        for vein in &veins {
            vein.place(&mut map);
        }
        map.fill_rect(46, 10, 1, 10, Tile::Path);
        map.set(TilePos::new(30, 31), Tile::Path);
        map.fill_rect(48, 21, 1, 6, Tile::Path);

        // forêts : une zone d'arbres, avec la cabane du bûcheron à côté et un chemin vers la route
        let forests = vec![Forest::new(13, 2, 12, 7), Forest::new(8, 32, 12, 6)];
        for forest in &forests {
            forest.plant(&mut map);
        }
        map.fill_rect(18, 21, 1, 6, Tile::Path);

        // l'entrepôt central, près du château : le chemin de sa porte vers la route
        map.fill_rect(28, 17, 3, 1, Tile::Path);

        // les habitats : une taille fixe, un lit (2×1) par place, une table (2×1), une étagère (2×1) et un bac d'eau
        // (voir `habitat.rs`)
        use HabitatClass::{Large, Medium, Small};
        let farmer_house = match farmers {
            0..=1 => HabitatSpec::new(4, 23, Small, (8, 23), &[(5, 24)], (8, 26), (5, 26), (9, 24)),
            2..=3 => HabitatSpec::new(4, 23, Medium, (8, 23), &[(5, 24), (10, 24), (5, 26)], (8, 26), (10, 26), (7, 24)),
            _ => HabitatSpec::new(
                4,
                23,
                Large,
                (8, 23),
                &[(5, 24), (10, 24), (5, 26), (10, 26), (5, 28), (10, 28)],
                (7, 26),
                (7, 28),
                (9, 26),
            ),
        };
        let habitats = vec![
            farmer_house, // maison du paysan (sud-ouest)
            HabitatSpec::new(54, 4, Small, (56, 8), &[(58, 5)], (55, 5), (58, 7), (55, 7)), // maison du forgeron (nord-est)
            HabitatSpec::new(54, 28, Small, (56, 28), &[(58, 29)], (55, 31), (58, 31), (55, 29)), // maison du tavernier (sud-est)
            HabitatSpec::new(19, 22, Medium, (21, 22), &[(22, 23), (24, 23), (22, 25)], (20, 25), (24, 25), (26, 25)), // caserne
            HabitatSpec::new(43, 5, Small, (46, 9), &[(44, 6)], (47, 6), (47, 8), (44, 8)), // mine de fer (nord)
            HabitatSpec::new(23, 29, Small, (29, 31), &[(24, 30)], (24, 32), (27, 32), (28, 30)), // mine d'or
            HabitatSpec::new(45, 27, Small, (48, 27), &[(46, 28)], (49, 28), (49, 30), (46, 30)), // mine de pierre
            HabitatSpec::new(8, 10, Small, (8, 12), &[(12, 11)], (9, 11), (9, 13), (13, 13)), // cabane nord
            HabitatSpec::new(16, 27, Small, (18, 27), &[(20, 28)], (17, 30), (20, 30), (17, 28)), // cabane sud
            HabitatSpec::new(19, 15, Medium, (27, 17), &[(20, 16), (22, 16), (24, 16)], (20, 18), (23, 18), (26, 16)), // entrepôt central
        ];
        for habitat in &habitats {
            habitat.build(&mut map);
        }

        // quelques arbres (seulement sur l'herbe)
        for &(x, y) in &[
            (14, 14), (24, 14), (26, 8), (45, 14), (48, 26), (26, 32), (36, 33), (44, 34),
            (10, 16), (16, 34), (3, 22), (2, 34), (50, 12), (58, 14), (60, 24), (52, 36),
            (18, 6), (44, 6), (22, 4), (8, 32), (60, 34), (4, 16), (28, 24), (46, 20),
        ] {
            let p = TilePos::new(x, y);
            // le décor ne bouche pas les couloirs des forêts
            if map.get(p) == Tile::Grass && !forests.iter().any(|f| f.contains(p)) {
                map.set(p, Tile::Tree);
            }
        }

        let characters = vec![
            spec("Paysan", Role::Farmer, (5, 24), &[(4, 5), (6, 5), (8, 5), (4, 7), (6, 7), (8, 7)]),
            spec("Forgeron", Role::Blacksmith, (58, 5), &[(38, 15), (40, 15), (39, 16)]),
            spec("Roi", Role::King, (33, 12), &[(31, 13), (29, 13), (33, 13)]),
            spec("Chef de guerre", Role::Warlord, (22, 23), &[(20, 24), (22, 24), (22, 20), (26, 20)]),
            spec("Tavernier", Role::Innkeeper, (58, 29), &[(38, 24), (40, 24), (38, 23)]),
            // les gardes dorment à la caserne et font des rondes sur les routes du royaume
            spec(
                "Garde de jour",
                Role::Guard,
                (24, 23),
                &[(31, 19), (44, 20), (56, 19), (38, 21), (31, 25), (31, 37), (21, 21), (12, 20), (7, 19)],
            ),
            spec(
                "Garde de nuit",
                Role::Guard,
                (22, 25),
                &[(31, 17), (56, 19), (31, 37), (7, 19), (38, 19)],
            )
            .on_night_shift(),
        ];

        let mine_beds = [(44, 6), (24, 30), (46, 28)];
        let mut characters = characters;
        characters.push(spec("Mineur de fer", Role::Miner, mine_beds[0], &[(44, 3), (47, 3)]));
        characters.push(spec("Mineur d'or", Role::Miner, mine_beds[1], &[(24, 34), (27, 34)]));
        characters.push(spec("Mineur de pierre", Role::Miner, mine_beds[2], &[(46, 32), (49, 32)]));
        let mines = mine_beds
            .iter()
            .enumerate()
            .map(|(vein, &(x, y))| MineSpec { vein, bed: TilePos::new(x, y) })
            .collect();

        let lodge_beds = [(12, 11), (20, 28)];
        characters.push(spec("Bucheron nord", Role::Lumberjack, lodge_beds[0], &[(15, 9), (20, 9)]));
        characters.push(spec("Bucheron sud", Role::Lumberjack, lodge_beds[1], &[(12, 31), (14, 31)]));
        let lodges = lodge_beds
            .iter()
            .enumerate()
            .map(|(forest, &(x, y))| LodgeSpec { forest, bed: TilePos::new(x, y) })
            .collect();

        // les trois transporteurs dorment à l'entrepôt central
        let depot_beds = [(20, 16), (22, 16), (24, 16)];
        for (i, bed) in depot_beds.iter().enumerate() {
            let name = format!("Transporteur {}", i + 1);
            characters.push(spec(&name, Role::Transporter, *bed, &[(21 + i as i32, 17)]));
        }

        // les coffres : toujours dehors, au niveau de l'extraction de la ressource
        let crate_at = |kind, x, y| ContainerSpec { kind, tile: TilePos::new(x, y) };
        let containers = vec![
            crate_at(ContainerKind::Farm, 11, 6),
            crate_at(ContainerKind::Mine(0), 48, 4),
            crate_at(ContainerKind::Mine(1), 28, 35),
            crate_at(ContainerKind::Mine(2), 50, 33),
            crate_at(ContainerKind::Lodge(0), 22, 9),
            crate_at(ContainerKind::Lodge(1), 15, 31),
            crate_at(ContainerKind::Forge, 42, 15),
            crate_at(ContainerKind::Depot, 28, 18),
        ];
        place_crates(&mut map, &containers);

        // le lit du roi, au château : le château n'est pas un habitat
        for c in &characters {
            if !habitats.iter().any(|h| h.beds.contains(&c.home)) {
                map.set(c.home, Tile::Bed);
            }
        }

        Scenario { map, veins, mines, forests, lodges, containers, habitats, characters }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_connected(scenario: &Scenario) {
        let map = &scenario.map;
        let hub = scenario.characters.iter().find(|c| c.role == Role::King).unwrap().work_spots[0];
        for c in &scenario.characters {
            let targets = c.work_spots.iter().copied().chain(std::iter::once(c.home));
            for t in targets {
                assert!(map.in_bounds(t), "{} : {:?} hors de la carte", c.name, t);
                assert!(t == c.home || !map.get(t).is_solid(), "{} : poste {:?} sur une tuile solide", c.name, t);
                assert!(
                    t == hub || !map.find_path(hub, t).is_empty(),
                    "{} : {:?} n'est pas joignable depuis le château",
                    c.name,
                    t
                );
            }
        }
    }

    #[test]
    fn village_is_bigger_than_a_default_window_and_fully_connected() {
        let village = Scenario::village();
        // fenêtre par défaut 1024x640 à 32 px par tuile = 32x20 tuiles
        assert!(village.map.width() > 32 && village.map.height() > 20);
        assert_connected(&village);
    }

    #[test]
    fn every_vein_post_is_reachable_and_each_mine_is_next_to_its_vein() {
        for scenario in [Scenario::village(), Scenario::village_with_farmers(6)] {
            let hub = scenario.characters.iter().find(|c| c.role == Role::King).unwrap().work_spots[0];
            assert_eq!(scenario.veins.len(), 3);
            for vein in &scenario.veins {
                assert!(vein.tiles().iter().all(|&t| scenario.map.get(t) == Tile::Ore));
                let posts = vein.access_tiles(&scenario.map);
                assert!(posts.len() >= 4, "{:?} : trop peu de cases d'accès", vein.kind);
                for p in posts {
                    assert!(!scenario.map.find_path(hub, p).is_empty(), "{p:?} inaccessible");
                }
            }
            for mine in &scenario.mines {
                let vein = &scenario.veins[mine.vein];
                let miner = scenario.characters.iter().find(|c| c.home == mine.bed).unwrap();
                assert_eq!(miner.role, Role::Miner);
                // le bâtiment de la mine touche son filon
                let touches = vein.tiles().iter().any(|&t| {
                    [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().any(|&(dx, dy)| {
                        scenario.map.get(TilePos::new(t.x + dx, t.y + dy)) == Tile::Wall
                    })
                });
                assert!(touches, "la mine de {:?} n'est pas accolée à son filon", vein.kind);
            }
        }
    }

    #[test]
    fn forests_are_reachable_and_cabins_are_next_to_them() {
        for scenario in [Scenario::village(), Scenario::village_with_farmers(6)] {
            let hub = scenario.characters.iter().find(|c| c.role == Role::King).unwrap().work_spots[0];
            assert_eq!(scenario.forests.len(), 2);
            for forest in &scenario.forests {
                let trees = forest.trees(&scenario.map);
                assert!(trees.len() > 20, "forêt trop clairsemée : {}", trees.len());
                // chaque arbre touche une case praticable, que l'on atteint à pied depuis le château
                for t in trees {
                    let post = scenario.map.walkable_neighbors(t).into_iter().next();
                    let post = post.unwrap_or_else(|| panic!("arbre {t:?} sans case voisine libre"));
                    assert!(!scenario.map.find_path(hub, post).is_empty(), "{post:?} inaccessible");
                }
            }
            for lodge in &scenario.lodges {
                let forest = &scenario.forests[lodge.forest];
                let lj = scenario.characters.iter().find(|c| c.home == lodge.bed).unwrap();
                assert_eq!(lj.role, Role::Lumberjack);
                // la cabane est à quelques tuiles de sa forêt
                let near = (forest.y - 4..forest.y + forest.h + 4).contains(&lodge.bed.y)
                    && (forest.x - 4..forest.x + forest.w + 4).contains(&lodge.bed.x);
                assert!(near, "la cabane de {} est loin de sa forêt", lj.name);
                for spot in &lj.work_spots {
                    assert!(!scenario.map.find_path(hub, *spot).is_empty());
                }
            }
        }
    }

    /// Distance de Manhattan entre un coffre et la tuile la plus proche d'une zone.
    fn distance_to(zone: impl IntoIterator<Item = TilePos>, chest: TilePos) -> i32 {
        zone.into_iter().map(|t| (t.x - chest.x).abs() + (t.y - chest.y).abs()).min().expect("zone vide")
    }

    #[test]
    fn production_chests_sit_where_the_resource_is_extracted() {
        use crate::container::ContainerKind;
        for scenario in [Scenario::village(), Scenario::village_with_farmers(6)] {
            let fields = scenario.map.find_tiles(Tile::Field);
            for spec in &scenario.containers {
                let near = match spec.kind {
                    ContainerKind::Farm => distance_to(fields.clone(), spec.tile),
                    ContainerKind::Mine(i) => distance_to(scenario.veins[i].tiles(), spec.tile),
                    ContainerKind::Lodge(i) => distance_to(scenario.forests[i].trees(&scenario.map), spec.tile),
                    ContainerKind::Forge | ContainerKind::Depot => continue,
                };
                assert!(near <= 2, "le coffre {:?} est à {near} cases de son lieu d'extraction", spec.kind);
                // il ne masque ni un poste de travail ni un poste de mineur
                for c in &scenario.characters {
                    assert!(!c.work_spots.contains(&spec.tile), "{} a un poste sur un coffre", c.name);
                }
                if let ContainerKind::Mine(i) = spec.kind {
                    assert!(!scenario.veins[i].access_tiles(&scenario.map).contains(&spec.tile));
                }
            }
        }
    }

    #[test]
    fn every_habitat_is_valid_furnished_and_intact_on_the_map() {
        for farmers in 1..=6 {
            let scenario = Scenario::village_with_farmers(farmers);
            for (i, h) in scenario.habitats.iter().enumerate() {
                assert_eq!(h.validate(), Ok(()), "habitat {i} en ({}, {})", h.x, h.y);
                // posé tel quel sur la carte : rien ne l'a écrasé après coup
                for &bed in &h.beds {
                    assert_eq!(scenario.map.get(bed), Tile::Bed, "tête du lit {bed:?}");
                    assert_eq!(scenario.map.get(TilePos::new(bed.x + 1, bed.y)), Tile::BedFoot, "pied du lit {bed:?}");
                }
                assert_eq!(scenario.map.get(TilePos::new(h.shelf.x, h.shelf.y)), Tile::Shelf);
                assert_eq!(scenario.map.get(TilePos::new(h.shelf.x + 1, h.shelf.y)), Tile::ShelfEnd);
                assert_eq!(scenario.map.get(TilePos::new(h.table.x + 1, h.table.y)), Tile::TableEnd);
                assert_eq!(scenario.map.get(h.table), Tile::Table);
                assert_eq!(scenario.map.get(h.trough), Tile::Trough);
                assert_eq!(scenario.map.get(h.door), Tile::Floor);
                assert_eq!(scenario.map.get(TilePos::new(h.x, h.y)), Tile::Wall);
                // pas deux habitats l'un sur l'autre
                for other in &scenario.habitats[i + 1..] {
                    let overlap = (h.x..h.x + h.width()).any(|x| {
                        (h.y..h.y + h.height()).any(|y| other.covers(TilePos::new(x, y)))
                    });
                    assert!(!overlap, "deux habitats se chevauchent en ({}, {}) et ({}, {})", h.x, h.y, other.x, other.y);
                }
            }
        }
    }

    #[test]
    fn everyone_but_the_king_sleeps_in_an_habitat_and_beds_are_not_shared() {
        for scenario in [Scenario::village(), Scenario::village_with_farmers(6)] {
            let mut seen = std::collections::HashSet::new();
            for c in &scenario.characters {
                assert!(seen.insert(c.home), "{} partage un lit", c.name);
                let habitat = scenario.habitats.iter().find(|h| h.beds.contains(&c.home));
                if c.role == Role::King {
                    assert!(habitat.is_none(), "le château n'est pas un habitat");
                } else {
                    assert!(habitat.is_some(), "{} ne dort pas dans un habitat", c.name);
                }
            }
            // au départ : autant de lits que de dormeurs, donc aucune place libre
            let beds: usize = scenario.habitats.iter().map(|h| h.beds.len()).sum();
            let sleepers = scenario.characters.iter().filter(|c| c.role != Role::King).count();
            assert_eq!(beds, sleepers);
        }
    }

    #[test]
    fn the_farmers_house_grows_with_the_number_of_farmers() {
        let class_of = |farmers| {
            let scenario = Scenario::village_with_farmers(farmers);
            let farmer = scenario.characters.iter().find(|c| c.role == Role::Farmer).unwrap();
            let house = scenario.habitats.iter().find(|h| h.beds.contains(&farmer.home)).unwrap();
            (house.class, house.beds.len())
        };
        assert_eq!(class_of(1), (HabitatClass::Small, 1));
        assert_eq!(class_of(2), (HabitatClass::Medium, 3));
        assert_eq!(class_of(3), (HabitatClass::Medium, 3));
        assert_eq!(class_of(4), (HabitatClass::Large, 6));
        assert_eq!(class_of(6), (HabitatClass::Large, 6));
    }

    #[test]
    fn habitats_have_the_expected_classes() {
        let scenario = Scenario::village();
        let class_at = |x, y| scenario.habitats.iter().find(|h| h.x == x && h.y == y).unwrap().class;
        assert_eq!(class_at(19, 22), HabitatClass::Medium, "caserne");
        assert_eq!(class_at(19, 15), HabitatClass::Medium, "entrepôt central");
        for (x, y) in [(4, 23), (54, 4), (54, 28), (43, 5), (23, 29), (45, 27), (8, 10), (16, 27)] {
            assert_eq!(class_at(x, y), HabitatClass::Small, "habitat en ({x}, {y})");
        }
        assert_eq!(scenario.habitats.len(), 10);
    }

    #[test]
    fn every_bed_in_every_habitat_is_reachable_from_the_castle() {
        for scenario in [Scenario::village(), Scenario::village_with_farmers(6)] {
            let hub = scenario.characters.iter().find(|c| c.role == Role::King).unwrap().work_spots[0];
            for h in &scenario.habitats {
                for &bed in &h.beds {
                    assert!(!scenario.map.find_path(hub, bed).is_empty(), "lit {bed:?} inaccessible depuis le château");
                }
            }
        }
    }

    #[test]
    fn every_chest_is_reachable_outdoors_and_the_depot_beds_are_too() {
        for scenario in [Scenario::village(), Scenario::village_with_farmers(6)] {
            let hub = scenario.characters.iter().find(|c| c.role == Role::King).unwrap().work_spots[0];
            assert_eq!(scenario.containers.len(), 8, "sept coffres de production et le dépôt central");
            for spec in &scenario.containers {
                assert_eq!(scenario.map.get(spec.tile), Tile::Crate, "{:?}", spec.kind);
                let posts = scenario.map.walkable_neighbors(spec.tile);
                assert!(!posts.is_empty(), "le coffre {:?} est inaccessible", spec.kind);
                assert!(
                    posts.iter().any(|&p| !scenario.map.find_path(hub, p).is_empty()),
                    "aucune case voisine du coffre {:?} n'est joignable",
                    spec.kind
                );
                // aucun coffre n'est dans un bâtiment : au moins une case voisine est de l'herbe ou un chemin
                assert!(
                    posts.iter().any(|&p| matches!(scenario.map.get(p), Tile::Grass | Tile::Path)),
                    "le coffre {:?} est dans un bâtiment",
                    spec.kind
                );
            }
            let transporters: Vec<_> = scenario.characters.iter().filter(|c| c.role == Role::Transporter).collect();
            assert_eq!(transporters.len(), 3);
            for t in transporters {
                assert!(!scenario.map.find_path(hub, t.home).is_empty(), "{} ne peut pas rejoindre son lit", t.name);
                assert!(t.work_spots.iter().all(|&s| !scenario.map.get(s).is_solid()));
            }
        }
    }

    #[test]
    fn village_with_all_farmers_is_fully_connected() {
        assert_connected(&Scenario::village_with_farmers(6));
    }

    #[test]
    fn houses_and_fields_are_far_from_the_center() {
        let village = Scenario::village();
        let center = TilePos::new(31, 20).center();
        for c in village.characters.iter().filter(|c| matches!(c.role, Role::Farmer | Role::Blacksmith | Role::Innkeeper)) {
            assert!(c.home.center().distance(center) > 16.0, "{} dort trop près du centre", c.name);
        }
        let farmer = village.characters.iter().find(|c| c.role == Role::Farmer).unwrap();
        assert!(farmer.work_spots[0].center().distance(center) > 16.0, "le champ est trop près du centre");
    }
}
