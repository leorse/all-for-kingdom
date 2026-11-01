//! Les plans du roi et les chantiers.
//!
//! Le roi choisit un plan dans une liste (`PlanKind`) et le place : un rectangle qu'il déplace et agrandit à
//! volonté au-dessus d'une taille minimale. Le coût en bois et en pierre, et la durée de la construction, se
//! calculent d'après la surface, le type de bâtiment et ses assets (voir `knowledge/regles/construction.md`).
//!
//! À la validation, les fondations (`Tile::Foundation`, solides) apparaissent. Le chantier attend d'avoir toutes
//! ses ressources, qui sont alors déposées dans un tas devant le chantier ; quand tout est là, les artisans
//! construisent. À la fin, le bâtiment prend la place des fondations et une route le relie au camp de base.

use crate::config::Config;
use crate::map::{Tile, TileMap};
use crate::math::TilePos;
use crate::resources::Resource;
use std::collections::BTreeMap;

/// Les types de bâtiments que le roi sait faire bâtir.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlanKind {
    /// Une maison : au moins un lit, une table et un bac d'eau ; chaque lit est une place (voir `HabitatSpec::house`).
    House,
    /// Un entrepôt : il contient beaucoup plus que le coffre du camp.
    Warehouse,
    /// Une taverne : elle retient les vagabonds qui passent par là.
    Tavern,
}

impl PlanKind {
    pub const ALL: [PlanKind; 3] = [PlanKind::House, PlanKind::Warehouse, PlanKind::Tavern];

    /// Position dans les tableaux de réglages de `Config`.
    pub fn index(self) -> usize {
        match self {
            PlanKind::House => 0,
            PlanKind::Warehouse => 1,
            PlanKind::Tavern => 2,
        }
    }

    /// Taille minimale, murs compris (largeur, hauteur).
    pub fn min_size(self) -> (i32, i32) {
        match self {
            PlanKind::House => crate::habitat::HabitatSpec::MIN_HOUSE,
            PlanKind::Warehouse => (8, 6),
            PlanKind::Tavern => (9, 7),
        }
    }

    /// Taille maximale d'un plan, murs compris : un garde-fou, pas une règle de jeu.
    pub fn max_size(self) -> (i32, i32) {
        (18, 12)
    }

    /// Les lits d'un plan de ce type : 1 au minimum pour une maison, aucun pour les autres.
    pub fn min_beds(self) -> u32 {
        u32::from(self == PlanKind::House)
    }
}

/// Un coût en matériaux de construction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cost {
    pub wood: u32,
    pub stone: u32,
}

impl Cost {
    pub fn get(&self, resource: Resource) -> u32 {
        match resource {
            Resource::Wood => self.wood,
            Resource::Stone => self.stone,
            _ => 0,
        }
    }

    /// Ce coût sous la forme dont un tas de chantier a besoin.
    pub fn as_needs(&self) -> BTreeMap<Resource, u32> {
        let mut needs = BTreeMap::new();
        if self.wood > 0 {
            needs.insert(Resource::Wood, self.wood);
        }
        if self.stone > 0 {
            needs.insert(Resource::Stone, self.stone);
        }
        needs
    }
}

/// Le coût d'un plan de cette forme : la surface bâtie au prix du type, plus ses assets de base (compris dans le
/// prix, posés automatiquement), plus chaque lit au-delà du premier pour une maison.
pub fn plan_cost(kind: PlanKind, w: i32, h: i32, beds: u32, cfg: &Config) -> Cost {
    let area = (w * h).max(0) as f32;
    let i = kind.index();
    let extra_beds = beds.saturating_sub(kind.min_beds());
    Cost {
        wood: (area * cfg.plan_wood_per_tile[i]).ceil() as u32 + cfg.plan_asset_wood[i] + extra_beds * cfg.extra_bed_wood,
        stone: (area * cfg.plan_stone_per_tile[i]).ceil() as u32 + cfg.plan_asset_stone[i],
    }
}

/// La durée de la construction pour un artisan en pleine forme, en secondes réelles : elle croît avec la
/// surface, et dépend du type de bâtiment et de ses assets.
pub fn plan_build_secs(kind: PlanKind, w: i32, h: i32, beds: u32, cfg: &Config) -> f32 {
    let i = kind.index();
    let extra_beds = beds.saturating_sub(kind.min_beds());
    (w * h).max(0) as f32 * cfg.plan_build_secs_per_tile[i] + cfg.plan_asset_secs[i] + extra_beds as f32 * cfg.extra_bed_secs
}

/// Pourquoi un plan ne peut pas être posé là.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanError {
    /// En dessous de la taille minimale de ce type.
    TooSmall,
    /// Au-dessus de la taille maximale.
    TooLarge,
    /// Plus de lits que la maison n'en peut contenir (ou aucun lit pour une maison).
    BadBeds,
    /// Du terrain qui ne se construit pas (eau, arbre, filon, coffre, champ, autre chantier...) ou hors de la carte.
    Blocked,
    /// Quelqu'un se tient sur l'emplacement.
    Occupied,
}

/// L'état d'un chantier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SiteState {
    /// Les fondations sont posées ; il manque des ressources, le chantier attend.
    Waiting,
    /// Toutes les ressources sont disponibles : on les apporte devant le chantier.
    Delivering,
    /// Tout est là : les artisans construisent.
    Building,
    /// Terminé.
    Done,
}

/// Un chantier : un plan posé sur la carte.
#[derive(Clone, Debug)]
pub struct Site {
    pub kind: PlanKind,
    /// Coin haut-gauche et taille, murs compris.
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub beds: u32,
    pub cost: Cost,
    pub state: SiteState,
    /// Avancement de la construction, de 0 à 1.
    pub progress: f32,
    /// Durée totale pour un artisan en pleine forme, en secondes réelles.
    pub total_secs: f32,
    /// Le tas de matériaux devant le chantier (indice dans la liste des contenants), dès que la livraison commence.
    pub pile: Option<usize>,
    /// Ce que la case du tas était avant d'y poser le coffre (herbe ou chemin) : on la rend telle quelle à la fin.
    pub pile_ground: Tile,
}

impl Site {
    pub fn contains(&self, t: TilePos) -> bool {
        (self.x..self.x + self.w).contains(&t.x) && (self.y..self.y + self.h).contains(&t.y)
    }

    /// La porte, au milieu du mur sud.
    pub fn door(&self) -> TilePos {
        TilePos::new(self.x + self.w / 2, self.y + self.h - 1)
    }

    /// La case devant la porte, à l'extérieur : c'est par là qu'on entre, et que la route démarre.
    pub fn front(&self) -> TilePos {
        TilePos::new(self.x + self.w / 2, self.y + self.h)
    }

    /// Où se pose le tas de matériaux : devant le chantier, dans le coin sud-ouest.
    pub fn pile_tile(&self) -> TilePos {
        TilePos::new(self.x + 1, self.y + self.h)
    }

    pub fn center(&self) -> TilePos {
        TilePos::new(self.x + self.w / 2, self.y + self.h / 2)
    }

    /// Les cases praticables qui touchent le chantier par un côté : d'où l'on construit.
    pub fn work_tiles(&self, map: &TileMap) -> Vec<TilePos> {
        let mut tiles = Vec::new();
        for x in self.x..self.x + self.w {
            tiles.push(TilePos::new(x, self.y - 1));
            tiles.push(TilePos::new(x, self.y + self.h));
        }
        for y in self.y..self.y + self.h {
            tiles.push(TilePos::new(self.x - 1, y));
            tiles.push(TilePos::new(self.x + self.w, y));
        }
        tiles.retain(|&t| map.in_bounds(t) && !map.get(t).is_solid());
        tiles
    }
}

/// Un bâtiment terminé qui n'est pas un habitat (les maisons sont des `HabitatSpec`) : un entrepôt ou une taverne.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Building {
    pub kind: PlanKind,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Building {
    pub fn covers(&self, t: TilePos) -> bool {
        (self.x..self.x + self.w).contains(&t.x) && (self.y..self.y + self.h).contains(&t.y)
    }

    pub fn door(&self) -> TilePos {
        TilePos::new(self.x + self.w / 2, self.y + self.h - 1)
    }

    pub fn front(&self) -> TilePos {
        TilePos::new(self.x + self.w / 2, self.y + self.h)
    }

    /// Les tuiles solides du mobilier posé dans l'intérieur : des tas de caisses contre le mur nord pour un entrepôt,
    /// deux tables dans les coins du fond pour une taverne. Le milieu reste libre, de la porte jusqu'au fond.
    pub fn decor_tiles(&self) -> Vec<TilePos> {
        let top = self.y + 1;
        match self.kind {
            PlanKind::Warehouse => (self.x + 1..self.x + self.w - 1).step_by(2).map(|x| TilePos::new(x, top)).collect(),
            PlanKind::Tavern => {
                let mut tiles = vec![TilePos::new(self.x + 1, top), TilePos::new(self.x + 2, top)];
                tiles.extend([TilePos::new(self.x + self.w - 3, top), TilePos::new(self.x + self.w - 2, top)]);
                tiles
            }
            PlanKind::House => Vec::new(),
        }
    }

    /// Pose le bâtiment sur la carte : murs, sol, porte, puis le mobilier solide.
    pub(crate) fn build(&self, map: &mut TileMap) {
        map.add_building(self.x, self.y, self.w, self.h, self.door());
        for t in self.decor_tiles() {
            map.set(t, Tile::Furniture);
        }
    }
}

/// Un plan de cette forme peut-il être posé ici ? (La carte, les personnages à l'intérieur et les chantiers déjà
/// posés, qui ont des fondations sur la carte, sont les seuls obstacles.)
pub fn check_plan(
    map: &TileMap,
    kind: PlanKind,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    beds: u32,
    bodies: &[TilePos],
) -> Result<(), PlanError> {
    let (min_w, min_h) = kind.min_size();
    let (max_w, max_h) = kind.max_size();
    if w < min_w || h < min_h {
        return Err(PlanError::TooSmall);
    }
    if w > max_w || h > max_h {
        return Err(PlanError::TooLarge);
    }
    let beds_ok = match kind {
        PlanKind::House => crate::habitat::HabitatSpec::max_house_beds(w, h).map_or(false, |max| (1..=max).contains(&beds)),
        _ => beds == 0,
    };
    if !beds_ok {
        return Err(PlanError::BadBeds);
    }
    let site = Site {
        kind,
        x,
        y,
        w,
        h,
        beds,
        cost: Cost::default(),
        state: SiteState::Waiting,
        progress: 0.0,
        total_secs: 0.0,
        pile: None,
        pile_ground: Tile::Grass,
    };
    let free = |t: TilePos| map.in_bounds(t) && matches!(map.get(t), Tile::Grass | Tile::Path);
    // l'emprise, la case devant la porte et le tas de matériaux : de l'herbe ou un chemin, loin du bord d'eau
    let in_land = x >= 1 && y >= 1 && x + w <= map.width() - 1 && y + h + 1 <= map.height() - 1;
    let ground_ok = (y..y + h).all(|yy| (x..x + w).all(|xx| free(TilePos::new(xx, yy))));
    if !in_land || !ground_ok || !free(site.front()) || !free(site.pile_tile()) {
        return Err(PlanError::Blocked);
    }
    if bodies.iter().any(|&b| site.contains(b)) {
        return Err(PlanError::Occupied);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> Config {
        Config::default()
    }

    fn grass() -> TileMap {
        TileMap::new(40, 30, Tile::Grass)
    }

    #[test]
    fn a_bigger_plan_costs_more_and_takes_longer() {
        let c = cfg();
        for kind in PlanKind::ALL {
            let (w, h) = kind.min_size();
            let beds = kind.min_beds();
            let small = (plan_cost(kind, w, h, beds, &c), plan_build_secs(kind, w, h, beds, &c));
            let big = (plan_cost(kind, w + 4, h + 2, beds, &c), plan_build_secs(kind, w + 4, h + 2, beds, &c));
            assert!(big.0.wood > small.0.wood && big.0.stone > small.0.stone, "{kind:?} : le coût suit la surface");
            assert!(big.1 > small.1, "{kind:?} : la durée suit la surface");
        }
    }

    #[test]
    fn the_kind_changes_the_price_and_the_time_for_the_same_surface() {
        let c = cfg();
        let house = (plan_cost(PlanKind::House, 9, 7, 1, &c), plan_build_secs(PlanKind::House, 9, 7, 1, &c));
        let tavern = (plan_cost(PlanKind::Tavern, 9, 7, 0, &c), plan_build_secs(PlanKind::Tavern, 9, 7, 0, &c));
        let warehouse = (plan_cost(PlanKind::Warehouse, 9, 7, 0, &c), plan_build_secs(PlanKind::Warehouse, 9, 7, 0, &c));
        assert_ne!(house, tavern);
        assert_ne!(house.0, warehouse.0);
        assert!(tavern.1 > warehouse.1, "la taverne demande plus de travail à surface égale");
    }

    #[test]
    fn an_extra_bed_adds_cost_and_time() {
        let c = cfg();
        let one = (plan_cost(PlanKind::House, 10, 6, 1, &c), plan_build_secs(PlanKind::House, 10, 6, 1, &c));
        let two = (plan_cost(PlanKind::House, 10, 6, 2, &c), plan_build_secs(PlanKind::House, 10, 6, 2, &c));
        assert!(two.0.wood > one.0.wood && two.1 > one.1);
        assert_eq!(two.0.stone, one.0.stone);
    }

    #[test]
    fn plans_below_the_minimum_or_above_the_maximum_are_refused() {
        let map = grass();
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 5, 5, 1, &[]), Err(PlanError::TooSmall));
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 4, 1, &[]), Err(PlanError::TooSmall));
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 5, 1, &[]), Ok(()));
        assert_eq!(check_plan(&map, PlanKind::Warehouse, 5, 5, 7, 6, 0, &[]), Err(PlanError::TooSmall));
        assert_eq!(check_plan(&map, PlanKind::Warehouse, 5, 5, 8, 6, 0, &[]), Ok(()));
        assert_eq!(check_plan(&map, PlanKind::Tavern, 5, 5, 30, 8, 0, &[]), Err(PlanError::TooLarge));
    }

    #[test]
    fn beds_must_fit_the_house() {
        let map = grass();
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 5, 0, &[]), Err(PlanError::BadBeds), "au moins un lit");
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 5, 3, &[]), Err(PlanError::BadBeds), "6x5 : deux lits au plus");
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 5, 2, &[]), Ok(()));
        assert_eq!(check_plan(&map, PlanKind::Warehouse, 5, 5, 8, 6, 1, &[]), Err(PlanError::BadBeds), "pas de lit à l'entrepôt");
    }

    #[test]
    fn water_trees_veins_crates_and_the_map_edge_block_a_plan() {
        for tile in [Tile::Water, Tile::Tree, Tile::Ore, Tile::Crate, Tile::Field, Tile::Foundation, Tile::Wall] {
            let mut map = grass();
            map.set(TilePos::new(8, 7), tile);
            assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 5, 1, &[]), Err(PlanError::Blocked), "{tile:?}");
        }
        let map = grass();
        assert_eq!(check_plan(&map, PlanKind::House, 0, 5, 6, 5, 1, &[]), Err(PlanError::Blocked), "au bord de l'eau");
        assert_eq!(check_plan(&map, PlanKind::House, 34, 5, 6, 5, 1, &[]), Err(PlanError::Blocked), "au bord est");
        assert_eq!(check_plan(&map, PlanKind::House, 5, 24, 6, 5, 1, &[]), Err(PlanError::Blocked), "la porte donne sur l'eau");
        // un chemin ne gêne pas
        let mut road = grass();
        road.set(TilePos::new(8, 7), Tile::Path);
        assert_eq!(check_plan(&road, PlanKind::House, 5, 5, 6, 5, 1, &[]), Ok(()));
    }

    #[test]
    fn the_door_front_and_the_material_pile_must_be_free_too() {
        let mut map = grass();
        let site = |m: &TileMap| check_plan(m, PlanKind::House, 5, 5, 6, 5, 1, &[]);
        map.set(TilePos::new(8, 10), Tile::Tree); // devant la porte (x + w/2 = 8)
        assert_eq!(site(&map), Err(PlanError::Blocked));
        let mut map = grass();
        map.set(TilePos::new(6, 10), Tile::Water); // là où serait le tas de matériaux
        assert_eq!(site(&map), Err(PlanError::Blocked));
    }

    #[test]
    fn a_body_on_the_spot_blocks_the_plan() {
        let map = grass();
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 5, 1, &[TilePos::new(7, 7)]), Err(PlanError::Occupied));
        assert_eq!(check_plan(&map, PlanKind::House, 5, 5, 6, 5, 1, &[TilePos::new(20, 20)]), Ok(()));
    }

    #[test]
    fn work_tiles_surround_the_site_and_skip_obstacles() {
        let mut map = grass();
        let site = Site {
            kind: PlanKind::House,
            x: 5,
            y: 5,
            w: 6,
            h: 5,
            beds: 1,
            cost: Cost::default(),
            state: SiteState::Building,
            progress: 0.0,
            total_secs: 10.0,
            pile: None,
            pile_ground: Tile::Grass,
        };
        let all = site.work_tiles(&map);
        assert_eq!(all.len(), 2 * 6 + 2 * 5);
        assert!(all.iter().all(|t| !site.contains(*t)));
        map.set(TilePos::new(4, 7), Tile::Tree);
        assert_eq!(site.work_tiles(&map).len(), all.len() - 1);
        assert_eq!(site.door(), TilePos::new(8, 9));
        assert_eq!(site.front(), TilePos::new(8, 10));
        assert_eq!(site.pile_tile(), TilePos::new(6, 10));
    }
}
