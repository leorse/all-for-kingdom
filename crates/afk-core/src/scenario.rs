//! Le contenu de départ d'une partie : carte et personnages.
//! C'est de la donnée pure : plus tard, elle pourra venir d'un fichier au lieu d'être écrite en dur.

use crate::character::Role;
use crate::map::{Tile, TileMap};
use crate::math::TilePos;

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
    /// Le petit village de départ.
    pub fn village() -> Scenario {
        let (w, h) = (32, 20);
        let mut map = TileMap::new(w, h, Tile::Grass);

        // bordure d'eau
        map.fill_rect(0, 0, w, 1, Tile::Water);
        map.fill_rect(0, h - 1, w, 1, Tile::Water);
        map.fill_rect(0, 0, 1, h, Tile::Water);
        map.fill_rect(w - 1, 0, 1, h, Tile::Water);

        // routes
        map.fill_rect(1, 10, w - 2, 1, Tile::Path);
        map.fill_rect(15, 6, 1, h - 7, Tile::Path);

        // champs
        map.fill_rect(2, 3, 8, 6, Tile::Field);

        // bâtiments publics : x, y, largeur, hauteur, porte
        map.add_building(12, 1, 7, 5, TilePos::new(15, 5)); // château
        map.add_building(20, 3, 6, 5, TilePos::new(22, 7)); // forge
        map.add_building(3, 12, 6, 5, TilePos::new(5, 12)); // caserne
        map.add_building(20, 12, 6, 5, TilePos::new(22, 12)); // taverne

        // l'enclume, au milieu de la forge
        map.set(TilePos::new(23, 5), Tile::Anvil);

        // maisons
        map.add_building(10, 12, 5, 4, TilePos::new(12, 12)); // maison du paysan
        map.add_building(26, 3, 5, 4, TilePos::new(28, 6)); // maison du forgeron
        map.add_building(27, 12, 4, 4, TilePos::new(28, 12)); // maison du tavernier

        // chemins entre les portes et la grande route
        map.fill_rect(22, 8, 1, 2, Tile::Path);
        map.set(TilePos::new(5, 11), Tile::Path);
        map.set(TilePos::new(22, 11), Tile::Path);
        map.set(TilePos::new(12, 11), Tile::Path);
        map.fill_rect(28, 7, 1, 3, Tile::Path);
        map.set(TilePos::new(28, 11), Tile::Path);

        // quelques arbres (seulement sur l'herbe)
        for &(x, y) in &[
            (11, 8), (12, 14), (13, 17), (18, 15), (28, 4), (29, 6), (28, 9),
            (29, 15), (27, 17), (2, 18), (10, 3), (1, 13), (30, 11),
        ] {
            let p = TilePos::new(x, y);
            if map.get(p) == Tile::Grass {
                map.set(p, Tile::Tree);
            }
        }

        let characters = vec![
            spec("Paysan", Role::Farmer, (11, 14), &[(3, 4), (5, 4), (7, 4), (3, 6), (5, 6), (7, 6)]),
            spec("Forgeron", Role::Blacksmith, (27, 4), &[(22, 5), (24, 5), (23, 6)]),
            spec("Roi", Role::King, (17, 2), &[(15, 3), (13, 3), (17, 3)]),
            spec("Chef de guerre", Role::Warlord, (4, 13), &[(5, 14), (7, 14), (6, 10), (10, 10)]),
            spec("Tavernier", Role::Innkeeper, (29, 14), &[(22, 14), (24, 14), (22, 13)]),
            // les gardes dorment à la caserne et font des rondes sur les routes du royaume
            spec(
                "Garde de jour",
                Role::Guard,
                (7, 13),
                &[(15, 9), (24, 10), (28, 10), (22, 11), (15, 12), (15, 17), (10, 10), (5, 11)],
            ),
            spec(
                "Garde de nuit",
                Role::Guard,
                (7, 15),
                &[(15, 7), (28, 10), (15, 17), (4, 10), (22, 10)],
            )
            .on_night_shift(),
        ];

        // les lits
        for c in &characters {
            map.set(c.home, Tile::Bed);
        }

        Scenario { map, characters }
    }

    /// Le village avec `farmers` paysans au total (au moins 1, au plus 6), pour tester la
    /// main-d'œuvre agricole. Les paysans supplémentaires dorment dans la maison du paysan.
    pub fn village_with_farmers(farmers: usize) -> Scenario {
        const EXTRA_BEDS: [(i32, i32); 5] = [(12, 14), (13, 14), (11, 13), (12, 13), (13, 13)];
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
                work_spots: vec![TilePos::new(2 + i as i32, 3)],
            });
        }
        scenario
    }
}
