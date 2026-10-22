//! Le contenu de départ d'une partie : carte et personnages.
//! C'est de la donnée pure : plus tard, elle pourra venir d'un fichier au lieu d'être écrite en dur.

use crate::character::Role;
use crate::container::{place_crates, ContainerKind, ContainerSpec};
use crate::forest::{Forest, LodgeSpec};
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

        // bâtiments publics, regroupés au centre : x, y, largeur, hauteur, porte
        map.add_building(28, 11, 7, 5, TilePos::new(31, 15)); // château
        map.add_building(36, 13, 6, 5, TilePos::new(38, 17)); // forge
        map.add_building(19, 22, 6, 5, TilePos::new(21, 22)); // caserne
        map.add_building(36, 22, 6, 5, TilePos::new(38, 22)); // taverne

        // l'enclume, au milieu de la forge
        map.set(TilePos::new(39, 15), Tile::Anvil);

        // maisons, loin du centre
        map.add_building(10, 26, 5, 4, TilePos::new(12, 26)); // maison du paysan (sud-ouest)
        map.add_building(54, 4, 5, 4, TilePos::new(56, 7)); // maison du forgeron (nord-est)
        map.add_building(54, 28, 4, 4, TilePos::new(56, 28)); // maison du tavernier (sud-est)

        // chemins entre les portes et la grande route
        map.fill_rect(38, 18, 1, 2, Tile::Path);
        map.set(TilePos::new(21, 21), Tile::Path);
        map.set(TilePos::new(38, 21), Tile::Path);
        map.fill_rect(12, 21, 1, 5, Tile::Path);
        map.fill_rect(56, 8, 1, 12, Tile::Path);
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
        map.add_building(44, 5, 5, 4, TilePos::new(46, 8)); // mine de fer (nord)
        map.fill_rect(46, 9, 1, 11, Tile::Path);
        map.add_building(24, 30, 5, 4, TilePos::new(28, 31)); // mine d'or (sud-ouest du château)
        map.fill_rect(29, 31, 2, 1, Tile::Path);
        map.add_building(46, 28, 5, 4, TilePos::new(48, 28)); // mine de pierre (sud-est)
        map.fill_rect(48, 21, 1, 7, Tile::Path);

        // forêts : une zone d'arbres, avec la cabane du bûcheron à côté et un chemin vers la route
        let forests = vec![Forest::new(13, 2, 12, 7), Forest::new(8, 32, 12, 6)];
        for forest in &forests {
            forest.plant(&mut map);
        }
        map.add_building(16, 10, 5, 4, TilePos::new(18, 13)); // cabane nord
        map.fill_rect(18, 14, 1, 6, Tile::Path);
        map.add_building(16, 27, 5, 4, TilePos::new(18, 27)); // cabane sud
        map.fill_rect(18, 21, 1, 6, Tile::Path);

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
            spec("Paysan", Role::Farmer, (11, 28), &[(4, 5), (6, 5), (8, 5), (4, 7), (6, 7), (8, 7)]),
            spec("Forgeron", Role::Blacksmith, (55, 5), &[(38, 15), (40, 15), (39, 16)]),
            spec("Roi", Role::King, (33, 12), &[(31, 13), (29, 13), (33, 13)]),
            spec("Chef de guerre", Role::Warlord, (20, 23), &[(21, 24), (23, 24), (22, 20), (26, 20)]),
            spec("Tavernier", Role::Innkeeper, (56, 30), &[(38, 24), (40, 24), (38, 23)]),
            // les gardes dorment à la caserne et font des rondes sur les routes du royaume
            spec(
                "Garde de jour",
                Role::Guard,
                (23, 23),
                &[(31, 19), (44, 20), (56, 19), (38, 21), (31, 25), (31, 37), (21, 21), (12, 20), (7, 19)],
            ),
            spec(
                "Garde de nuit",
                Role::Guard,
                (23, 25),
                &[(31, 17), (56, 19), (31, 37), (7, 19), (38, 19)],
            )
            .on_night_shift(),
        ];

        let mine_beds = [(47, 6), (25, 31), (47, 29)];
        let mut characters = characters;
        characters.push(spec("Mineur de fer", Role::Miner, mine_beds[0], &[(44, 3), (47, 3)]));
        characters.push(spec("Mineur d'or", Role::Miner, mine_beds[1], &[(24, 34), (27, 34)]));
        characters.push(spec("Mineur de pierre", Role::Miner, mine_beds[2], &[(46, 32), (49, 32)]));
        let mines = mine_beds
            .iter()
            .enumerate()
            .map(|(vein, &(x, y))| MineSpec { vein, bed: TilePos::new(x, y) })
            .collect();

        let lodge_beds = [(17, 11), (17, 28)];
        characters.push(spec("Bucheron nord", Role::Lumberjack, lodge_beds[0], &[(15, 9), (20, 9)]));
        characters.push(spec("Bucheron sud", Role::Lumberjack, lodge_beds[1], &[(9, 31), (14, 31)]));
        let lodges = lodge_beds
            .iter()
            .enumerate()
            .map(|(forest, &(x, y))| LodgeSpec { forest, bed: TilePos::new(x, y) })
            .collect();

        // l'entrepôt central, près du château : trois lits dedans, son coffre dehors à côté de la porte
        map.add_building(24, 16, 6, 4, TilePos::new(29, 18));
        map.set(TilePos::new(30, 18), Tile::Path);
        let depot_beds = [(25, 17), (26, 17), (27, 17)];
        for (i, bed) in depot_beds.iter().enumerate() {
            let name = format!("Transporteur {}", i + 1);
            characters.push(spec(&name, Role::Transporter, *bed, &[(26 + i as i32, 18)]));
        }

        // les coffres : toujours dehors, contre le bâtiment qu'ils servent
        let crate_at = |kind, x, y| ContainerSpec { kind, tile: TilePos::new(x, y) };
        let containers = vec![
            crate_at(ContainerKind::Farm, 13, 25),
            crate_at(ContainerKind::Mine(0), 49, 7),
            crate_at(ContainerKind::Mine(1), 30, 32),
            crate_at(ContainerKind::Mine(2), 51, 29),
            crate_at(ContainerKind::Lodge(0), 21, 11),
            crate_at(ContainerKind::Lodge(1), 18, 31),
            crate_at(ContainerKind::Forge, 42, 15),
            crate_at(ContainerKind::Depot, 30, 17),
        ];
        place_crates(&mut map, &containers);

        // les lits
        for c in &characters {
            map.set(c.home, Tile::Bed);
        }

        Scenario { map, veins, mines, forests, lodges, containers, characters }
    }

    /// Le village avec `farmers` paysans au total (au moins 1, au plus 6), pour tester la
    /// main-d'œuvre agricole. Les paysans supplémentaires dorment dans la maison du paysan.
    pub fn village_with_farmers(farmers: usize) -> Scenario {
        const EXTRA_BEDS: [(i32, i32); 5] = [(12, 28), (13, 28), (11, 27), (12, 27), (13, 27)];
        assert!(farmers <= 1 + EXTRA_BEDS.len(), "pas assez de lits pour {farmers} paysans");

        let mut scenario = Scenario::village();
        for i in 1..farmers.max(1) {
            let (bx, by) = EXTRA_BEDS[i - 1];
            let home = TilePos::new(bx, by);
            scenario.map.set(home, Tile::Bed);
            scenario.characters.push(CharacterSpec {
                name: format!("Paysan {}", i + 1),
                role: Role::Farmer,
                home,
                night_shift: false,
                work_spots: vec![TilePos::new(3 + i as i32, 4)],
            });
        }
        scenario
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
