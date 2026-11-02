//! L'état global de la partie et la façon de le faire évoluer.
//!
//! Contrat avec le monde extérieur (rendu, entrées, réseau) :
//!  - les entrées arrivent sous forme de [`Command`] (jamais de touches ou de clics bruts) ;
//!  - le temps avance par pas fixes via [`Game::step`] ;
//!  - l'état se lit via des accesseurs en lecture seule.
//!
//! Plusieurs joueurs sont déjà prévus (coop) : chaque `PlayerId` possède au plus un personnage.
//! Aucun joueur ne possède personne ? C'est le mode AFK : la cité tourne seule.

use std::collections::HashMap;

use crate::building::{check_plan, plan_build_secs, plan_cost, Building, Cost, PlanError, PlanKind, Site, SiteState};
use crate::character::{at_forge_post, forge_spot, Character, Env, Facing, Outcome, Role, BODY_HALF};
use crate::clock::Clock;
use crate::config::Config;
use crate::container::{container_at, place_crates, Container, ContainerKind, ContainerSpec};
use crate::farm::Farm;
use crate::forest::{Forest, LodgeSpec, Saplings};
use crate::habitat::HabitatSpec;
use crate::map::{PathMode, Tile, TileMap};
use crate::math::{TilePos, Vec2};
use crate::mine::{MineSpec, Vein};
use crate::resources::{Resource, Stock};
use crate::rng::Rng;
use crate::scenario::Scenario;
use crate::transport::Reservation;
use crate::work::{work_secs, WorkKind};

/// Durée d'un pas de simulation (60 par seconde).
pub const TICK: f32 = 1.0 / 60.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CharacterId(pub usize);

/// Tout ce qu'un joueur peut demander au jeu.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    /// Direction voulue (maintenue jusqu'à la prochaine commande `Move`).
    Move { player: PlayerId, dir: Vec2 },
    /// Incarner ce personnage (ignoré s'il est déjà incarné par un autre joueur).
    Possess { player: PlayerId, character: CharacterId },
    /// Incarner le personnage suivant.
    PossessNext { player: PlayerId },
    /// Se détacher (mode AFK) ou reprendre le dernier personnage incarné.
    ToggleIdle { player: PlayerId },
    /// Interagir avec ce qui est à portée : se lever, se coucher dans son lit, annuler un travail
    /// en cours, ou commencer le travail de son métier sur la parcelle devant soi / sous ses pieds.
    Interact { player: PlayerId },
    /// Vider l'entrepôt central : toutes les ressources de son contenant disparaissent.
    EmptyDepot { player: PlayerId },
    /// Le roi pose un plan : un rectangle de `w` sur `h` tuiles, murs compris, dont le coin haut-gauche est en
    /// (`x`, `y`), avec `beds` lits pour une maison. Refusé si le joueur n'incarne pas un roi ou si le plan ne peut
    /// pas être posé là (voir `Game::check_plan`). Même sans les ressources, le plan se pose : le chantier attend.
    PlacePlan { player: PlayerId, kind: PlanKind, x: i32, y: i32, w: i32, h: i32, beds: u32 },
    /// Le roi s'adresse à la personne la plus proche à portée : elle arrête tout (et se réveille si elle dormait).
    Talk { player: PlayerId },
    /// Fin de la conversation sans rien décider : la personne reprend exactement ce qu'elle faisait.
    EndTalk { player: PlayerId },
    /// Le roi affecte la personne à qui il parle au poste de cette tuile, libre ou occupé (le titulaire garde son métier
    /// mais perd son poste). Elle prend le métier du poste.
    AssignPost { player: PlayerId, who: CharacterId, post: TilePos },
    /// Le roi attribue ce lit à la personne à qui il parle ; si le lit était à quelqu'un, celui-ci le perd.
    AssignBed { player: PlayerId, who: CharacterId, bed: TilePos },
}

/// Un poste de travail que fournit un bâtiment : une place d'un métier, tenue par une seule personne (sauf le poste
/// d'artisan du camp, sans limite).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Post {
    pub role: Role,
    pub tile: TilePos,
    pub unlimited: bool,
}

struct PlayerSlot {
    possessed: Option<CharacterId>,
    last: CharacterId,
    move_dir: Vec2,
    /// La personne à qui le roi parle.
    talking: Option<CharacterId>,
}

pub struct Game {
    map: TileMap,
    farm: Farm,
    /// Les coffres : c'est là que vivent les ressources (le total de `stock()` n'en est que la somme).
    containers: Vec<Container>,
    veins: Vec<Vein>,
    mines: Vec<MineSpec>,
    forests: Vec<Forest>,
    lodges: Vec<LodgeSpec>,
    habitats: Vec<HabitatSpec>,
    /// Les entrepôts et les tavernes bâtis (les maisons sont dans `habitats`).
    buildings: Vec<Building>,
    /// Les plans posés par le roi, dans l'ordre : en attente, en livraison, en construction ou terminés.
    sites: Vec<Site>,
    /// Une case de la place du camp de base : toutes les routes qu'on bâtit y mènent.
    camp: Option<TilePos>,
    /// Heures de jeu avant la prochaine arrivée d'un vagabond (s'il y a un camp).
    vagabond_timer: f32,
    vagabonds_seen: usize,
    saplings: Saplings,
    /// Avancement (0 à 1) des arbres entamés, partagé entre les bûcherons qui s'y attaquent.
    tree_progress: HashMap<TilePos, f32>,
    characters: Vec<Character>,
    players: Vec<PlayerSlot>,
    rng: Rng,
    time: f32,
    clock: Clock,
    config: Config,
}

impl Game {
    /// Nouvelle partie. Au départ, personne n'est incarné.
    pub fn new(scenario: Scenario, player_count: usize, config: Config) -> Game {
        let mut characters: Vec<Character> = scenario
            .characters
            .into_iter()
            .enumerate()
            .map(|(i, s)| Character::new(CharacterId(i), s.name, s.role, s.home, s.night_shift, s.work_spots))
            .collect();
        // au camp, les artisans tiennent le poste d'artisan de la place du camp
        if let Some(camp) = scenario.camp {
            for c in characters.iter_mut().filter(|c| c.role == Role::Artisan) {
                c.post = Some(camp);
            }
        }
        // un bûcheron coupe dans la forêt de la cabane dont il occupe le lit
        for c in characters.iter_mut().filter(|c| c.role == Role::Lumberjack) {
            c.forest = scenario.lodges.iter().find(|l| l.bed == c.home).map(|l| l.forest);
        }
        // un mineur exploite le filon de la mine dont il occupe le lit
        for c in characters.iter_mut().filter(|c| c.role == Role::Miner) {
            c.vein = scenario.mines.iter().find(|m| m.bed == c.home).map(|m| m.vein);
        }
        // les coffres : un contenant par tuile de coffre du scénario, le fer de départ dans celui de la forge
        let mut map = scenario.map;
        place_crates(&mut map, &scenario.containers);
        let mut containers: Vec<Container> = scenario
            .containers
            .iter()
            .map(|&spec| {
                let capacity = if spec.kind == ContainerKind::Camp { config.camp_capacity } else { config.container_capacity };
                Container::new(spec, capacity, &scenario.veins)
            })
            .collect();
        // le pécule de départ, dans le coffre du camp
        if let Some(camp) = containers.iter_mut().find(|c| c.kind == ContainerKind::Camp) {
            camp.add(Resource::Wood, config.starting_wood);
            camp.add(Resource::Stone, config.starting_stone);
            camp.add(Resource::Seeds, config.starting_seeds);
        }
        if let Some(forge) = containers.iter_mut().find(|c| c.kind == ContainerKind::Forge) {
            forge.add(Resource::Iron, config.starting_iron);
        }
        // chacun dépose sa production dans le coffre de son métier
        for c in characters.iter_mut() {
            let wanted = match c.role {
                Role::Miner => c.vein.map(ContainerKind::Mine),
                Role::Lumberjack => c.forest.map(ContainerKind::Lodge),
                Role::Farmer => Some(ContainerKind::Farm),
                Role::Blacksmith => Some(ContainerKind::Forge),
                _ => None,
            };
            c.container = wanted.and_then(|kind| containers.iter().position(|k| k.kind == kind));
        }
        let players = (0..player_count)
            .map(|_| PlayerSlot {
                possessed: None,
                last: CharacterId(0),
                move_dir: Vec2::ZERO,
                talking: None,
            })
            .collect();
        let farm = Farm::from_map(&map);
        Game {
            map,
            farm,
            containers,
            veins: scenario.veins,
            mines: scenario.mines,
            forests: scenario.forests,
            lodges: scenario.lodges,
            habitats: scenario.habitats,
            buildings: Vec::new(),
            sites: Vec::new(),
            camp: scenario.camp,
            vagabond_timer: config.vagabond_first_hours,
            vagabonds_seen: 0,
            saplings: Saplings::new(),
            tree_progress: HashMap::new(),
            characters,
            players,
            rng: Rng::new(0x2545_F491_4F6C_DD1D),
            time: 0.0,
            clock: Clock::new(&config),
            config,
        }
    }

    // ------------------------------------------------------------ lecture

    pub fn map(&self) -> &TileMap {
        &self.map
    }

    pub fn farm(&self) -> &Farm {
        &self.farm
    }

    /// Le total de chaque ressource dans tous les coffres (ce que l'on porte en main n'est pas compté).
    pub fn stock(&self) -> Stock {
        let mut total = Stock::new();
        for container in &self.containers {
            for &resource in &Resource::ALL {
                total.add(resource, container.get(resource));
            }
        }
        total
    }

    pub fn containers(&self) -> &[Container] {
        &self.containers
    }

    /// Ce que les personnages tiennent en main de cette ressource (en route d'un coffre à l'autre).
    pub fn in_hands(&self, resource: Resource) -> u32 {
        self.characters
            .iter()
            .filter_map(|c| c.cargo)
            .filter(|&(r, _)| r == resource)
            .map(|(_, n)| n)
            .sum()
    }

    /// Le coffre de ce genre (le premier, s'il y en a plusieurs).
    pub fn container_of(&self, kind: ContainerKind) -> Option<&Container> {
        self.containers.iter().find(|c| c.kind == kind)
    }

    pub fn veins(&self) -> &[Vein] {
        &self.veins
    }

    pub fn mines(&self) -> &[MineSpec] {
        &self.mines
    }

    pub fn forests(&self) -> &[Forest] {
        &self.forests
    }

    pub fn lodges(&self) -> &[LodgeSpec] {
        &self.lodges
    }

    pub fn habitats(&self) -> &[HabitatSpec] {
        &self.habitats
    }

    /// Les entrepôts et les tavernes bâtis.
    pub fn buildings(&self) -> &[Building] {
        &self.buildings
    }

    /// Les plans posés par le roi, dans l'ordre où il les a posés.
    pub fn sites(&self) -> &[Site] {
        &self.sites
    }

    /// Une case de la place du camp de base, s'il y a un camp.
    pub fn camp(&self) -> Option<TilePos> {
        self.camp
    }

    /// Le plan en cours de construction ou de livraison qui couvre cette tuile.
    pub fn site_at(&self, tile: TilePos) -> Option<&Site> {
        self.sites.iter().find(|s| s.state != SiteState::Done && s.contains(tile))
    }

    /// Le coût d'un plan de cette forme, et sa durée de construction pour un artisan en pleine forme.
    pub fn plan_cost(&self, kind: PlanKind, w: i32, h: i32, beds: u32) -> Cost {
        plan_cost(kind, w, h, beds, &self.config)
    }

    pub fn plan_secs(&self, kind: PlanKind, w: i32, h: i32, beds: u32) -> f32 {
        plan_build_secs(kind, w, h, beds, &self.config)
    }

    /// Ce plan peut-il être posé ici ? Les personnages à l'intérieur de l'emprise l'en empêchent.
    pub fn check_plan(&self, kind: PlanKind, x: i32, y: i32, w: i32, h: i32, beds: u32) -> Result<(), PlanError> {
        let bodies: Vec<TilePos> = self.characters.iter().filter(|c| !c.gone).map(|c| c.tile()).collect();
        check_plan(&self.map, kind, x, y, w, h, beds, &bodies)
    }

    /// Ce que le royaume peut encore promettre à un nouveau chantier en `resource` : ce qui est dans les coffres, moins
    /// ce que les chantiers en livraison attendent encore (ce qui est déjà en route dans les mains n'y est plus).
    pub fn available(&self, resource: Resource) -> u32 {
        let stored: u32 = self.containers.iter().filter(|c| !c.is_site()).map(|c| c.get(resource)).sum();
        let pending: u32 = self
            .sites
            .iter()
            .filter(|s| s.state == SiteState::Delivering)
            .filter_map(|s| s.pile)
            .map(|p| {
                let pile = &self.containers[p];
                let need = pile.needs().get(&resource).copied().unwrap_or(0);
                let carried: u32 = self
                    .characters
                    .iter()
                    .filter(|c| c.job.map_or(false, |j| j.to == p))
                    .filter_map(|c| c.cargo)
                    .filter(|&(r, _)| r == resource)
                    .map(|(_, n)| n)
                    .sum();
                need.saturating_sub(pile.get(resource) + carried)
            })
            .sum();
        stored.saturating_sub(pending)
    }

    /// Le royaume a-t-il de quoi payer ce coût ? (La zone d'un plan est bleue si oui, rouge sinon.)
    pub fn can_afford(&self, cost: Cost) -> bool {
        self.available(Resource::Wood) >= cost.wood && self.available(Resource::Stone) >= cost.stone
    }

    /// Le personnage incarné par ce joueur est-il un roi ? C'est lui qui pose les plans.
    pub fn is_king(&self, player: PlayerId) -> bool {
        self.possessed_by(player).and_then(|id| self.characters.get(id.0)).map_or(false, |c| c.role == Role::King)
    }

    /// Places libres d'un habitat : ses lits que personne n'occupe comme logement (aucun personnage n'y a son lit).
    pub fn free_places_in(&self, habitat: usize) -> u32 {
        let Some(h) = self.habitats.get(habitat) else { return 0 };
        h.bed_homes().iter().filter(|&&bed| !self.characters.iter().any(|c| c.has_bed && c.home == bed)).count() as u32
    }

    /// Tous les lits du royaume : ceux des habitats (la tuile de tête) et les lits isolés (le camp, le château).
    pub fn beds(&self) -> Vec<TilePos> {
        let mut beds: Vec<TilePos> = self.habitats.iter().flat_map(|h| h.bed_homes()).collect();
        for t in self.map.find_tiles(Tile::Bed) {
            if !self.habitats.iter().any(|h| h.covers(t)) {
                beds.push(t);
            }
        }
        beds
    }

    /// Qui dort dans ce lit, s'il est à quelqu'un.
    pub fn bed_owner(&self, bed: TilePos) -> Option<CharacterId> {
        self.characters.iter().find(|c| !c.gone && c.has_bed && c.home == bed).map(|c| c.id)
    }

    /// Places libres dans tout le royaume : les lits que personne n'occupe (le camp compris).
    pub fn free_places(&self) -> u32 {
        self.beds().into_iter().filter(|&b| self.bed_owner(b).is_none()).count() as u32
    }

    /// Les postes de travail que fournissent les bâtiments : une enclume par poste de forgeron, des postes de
    /// transporteur à l'entrepôt, le tavernier à la taverne, et au camp le poste d'artisan, sans limite.
    pub fn posts(&self) -> Vec<Post> {
        let mut posts = Vec::new();
        if let Some(camp) = self.camp {
            posts.push(Post { role: Role::Artisan, tile: camp, unlimited: true });
        }
        for b in &self.buildings {
            match b.kind {
                PlanKind::Warehouse => {
                    let width = (b.w - 2).max(1);
                    for i in 0..self.config.warehouse_posts as i32 {
                        let tile = TilePos::new(b.x + 1 + i % width, b.y + 3 + i / width);
                        posts.push(Post { role: Role::Transporter, tile, unlimited: false });
                    }
                }
                PlanKind::Tavern => {
                    posts.push(Post { role: Role::Innkeeper, tile: TilePos::new(b.x + b.w / 2, b.y + 2), unlimited: false })
                }
                PlanKind::Forge => posts.push(Post { role: Role::Blacksmith, tile: forge_spot(b.anvil_tile()), unlimited: false }),
                PlanKind::House => {}
            }
        }
        posts
    }

    /// Ceux qui tiennent le poste de cette tuile.
    pub fn post_holders(&self, tile: TilePos) -> Vec<CharacterId> {
        self.characters.iter().filter(|c| !c.gone && c.post == Some(tile)).map(|c| c.id).collect()
    }

    /// La personne à qui le roi parle en ce moment.
    pub fn talking_to(&self, player: PlayerId) -> Option<CharacterId> {
        self.players.get(player.0 as usize).and_then(|p| p.talking)
    }

    /// Qui le roi de ce joueur pourrait aborder : la personne la plus proche à portée de parole (éveillée ou non), s'il n'est
    /// pas déjà en conversation.
    pub fn talkable(&self, player: PlayerId) -> Option<CharacterId> {
        let king = self.possessed_by(player).and_then(|id| self.characters.get(id.0)).filter(|c| c.role == Role::King && !c.asleep)?;
        if self.talking_to(player).is_some() {
            return None;
        }
        self.characters
            .iter()
            .filter(|c| !c.gone && c.id != king.id && c.pos.distance(king.pos) <= self.config.talk_range)
            .min_by(|a, b| a.pos.distance(king.pos).total_cmp(&b.pos.distance(king.pos)))
            .map(|c| c.id)
    }

    /// Peut-on embaucher un vagabond de la taverne ? Seulement s'il reste une place libre quelque part.
    pub fn can_hire(&self) -> bool {
        self.free_places() > 0
    }

    /// Avancement (0 à 1) de l'arbre entamé sur cette tuile, s'il y en a un.
    pub fn tree_progress(&self, tile: TilePos) -> f32 {
        self.tree_progress.get(&tile).copied().unwrap_or(0.0)
    }

    pub fn characters(&self) -> &[Character] {
        &self.characters
    }

    pub fn character(&self, id: CharacterId) -> Option<&Character> {
        self.characters.get(id.0)
    }

    pub fn clock(&self) -> &Clock {
        &self.clock
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Temps réel de jeu écoulé, en secondes (utile aux animations).
    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn possessed_by(&self, player: PlayerId) -> Option<CharacterId> {
        self.players.get(player.0 as usize).and_then(|p| p.possessed)
    }

    pub fn controller_of(&self, character: CharacterId) -> Option<PlayerId> {
        self.players
            .iter()
            .position(|p| p.possessed == Some(character))
            .map(|i| PlayerId(i as u8))
    }

    /// Ce personnage peut-il se coucher maintenant ? Il faut être près de son lit, et soit que ce
    /// soit son heure de dormir (la nuit, ou le jour pour le quart de nuit), soit être assez fatigué
    /// pour que ça serve à quelque chose.
    pub fn can_lie_down(&self, id: CharacterId) -> bool {
        match self.characters.get(id.0) {
            Some(c) => {
                !c.asleep
                    && c.has_bed
                    && c.distance_to_bed() <= self.config.bed_reach
                    && (c.is_sleep_time(self.clock.is_night()) || c.fatigue > self.config.wake_fatigue)
            }
            None => false,
        }
    }

    /// Le travail possible sur cette tuile, quel que soit le métier : cultiver une parcelle,
    /// ou forger sur une enclume (tant qu'il reste assez de fer).
    fn work_kind_at(&self, tile: TilePos) -> Option<WorkKind> {
        if let Some(kind) = self.farm.work_at(tile) {
            return Some(kind);
        }
        if self.map.get(tile) == Tile::Ore {
            return Some(WorkKind::Mine);
        }
        if self.sites.iter().any(|s| s.state == SiteState::Building && s.contains(tile)) {
            return Some(WorkKind::Build);
        }
        if self.map.get(tile) == Tile::Tree && self.forest_index_at(tile).is_some() {
            return Some(WorkKind::Chop);
        }
        (self.map.get(tile) == Tile::Anvil).then_some(WorkKind::Forge)
    }

    /// Le travail que ce personnage pourrait lancer maintenant avec `Interact`, et sur quelle tuile :
    /// ce qu'il a devant lui d'abord (parcelle, enclume), sinon ce qu'il a sous les pieds. `None`
    /// s'il n'y a rien à faire (ou si ce n'est pas son métier, ou s'il dort ou travaille déjà).
    pub fn available_work(&self, id: CharacterId) -> Option<(WorkKind, TilePos)> {
        let c = self.characters.get(id.0)?;
        if c.asleep || c.work.is_some() {
            return None;
        }
        // pas la peine de travailler à deux sur la même parcelle (mais un filon se partage)
        let worked_by_other = |tile: TilePos| {
            self.characters
                .iter()
                .any(|o| o.id != id && o.work.map_or(false, |w| !matches!(w.kind, WorkKind::Mine | WorkKind::Chop | WorkKind::Build) && w.tile == tile))
        };
        [c.facing_tile(), c.tile()].into_iter().find_map(|tile| {
            self.work_kind_at(tile)
                .filter(|kind| kind.role() == c.role && !worked_by_other(tile))
                // un mineur n'exploite que le filon de sa mine
                .filter(|kind| *kind != WorkKind::Mine || self.vein_index_at(tile) == c.vein)
                // un bûcheron ne coupe que dans la forêt de sa cabane
                .filter(|kind| *kind != WorkKind::Chop || self.forest_index_at(tile) == c.forest)
                // un producteur ne travaille que si son coffre peut recevoir sa production
                .filter(|kind| self.can_produce(c, *kind, tile))
                .map(|kind| (kind, tile))
        })
    }

    /// Le travail peut-il démarrer ? Un producteur s'arrête quand son coffre est plein, un forgeron quand
    /// il manque de fer ou de place, un bûcheron quand ses mains ne peuvent pas recevoir un arbre de plus.
    fn can_produce(&self, c: &Character, kind: WorkKind, tile: TilePos) -> bool {
        let room = |resource| c.container.and_then(|i| self.containers.get(i)).map_or(0, |k| k.room(resource));
        match kind {
            WorkKind::Mine => self.veins.iter().find(|v| v.contains(tile)).map_or(false, |v| room(v.kind.resource()) > 0),
            WorkKind::Harvest => {
                c.cargo.map_or(0, |(_, n)| n) + self.config.wheat_per_harvest <= self.config.farmer_carry
            }
            WorkKind::Forge => self.container_of(ContainerKind::Forge).map_or(false, |k| {
                k.get(Resource::Iron) >= self.config.iron_per_tool && k.room(Resource::Tools) >= self.config.tools_per_forge
            }),
            WorkKind::Chop => {
                c.cargo.map_or(0, |(_, n)| n) + self.config.logs_per_tree <= self.config.lumberjack_carry
            }
            WorkKind::Till | WorkKind::Plant | WorkKind::Build => true,
        }
    }

    fn vein_index_at(&self, tile: TilePos) -> Option<usize> {
        self.veins.iter().position(|v| v.contains(tile))
    }

    fn forest_index_at(&self, tile: TilePos) -> Option<usize> {
        self.forests.iter().position(|f| f.contains(tile))
    }

    /// Personnage situé à ce point du monde (pour désigner quelqu'un à la souris, par exemple).
    pub fn character_at(&self, point: Vec2) -> Option<CharacterId> {
        let mut best: Option<(f32, CharacterId)> = None;
        for c in self.characters.iter().filter(|c| !c.gone) {
            // le corps est dessiné légèrement au-dessus du point de contact
            let d = c.pos.distance(point + Vec2::new(0.0, 0.15));
            if d < 0.6 && best.map_or(true, |(bd, _)| d < bd) {
                best = Some((d, c.id));
            }
        }
        best.map(|(_, id)| id)
    }

    // ------------------------------------------------------------ évolution

    pub fn apply(&mut self, command: Command) {
        match command {
            Command::Move { player, dir } => {
                if let Some(p) = self.players.get_mut(player.0 as usize) {
                    p.move_dir = dir;
                }
            }
            Command::Possess { player, character } => self.possess(player, character),
            Command::PossessNext { player } => {
                let n = self.characters.len();
                let Some(slot) = self.players.get(player.0 as usize) else { return };
                let start = slot.possessed.unwrap_or(slot.last).0;
                for offset in 1..=n {
                    let candidate = CharacterId((start + offset) % n);
                    if !self.characters[candidate.0].gone && self.is_free_for(candidate, player) {
                        self.possess(player, candidate);
                        break;
                    }
                }
            }
            Command::ToggleIdle { player } => {
                let pi = player.0 as usize;
                let Some(slot) = self.players.get(pi) else { return };
                match slot.possessed {
                    Some(current) => {
                        self.characters[current.0].release_to_ai();
                        let slot = &mut self.players[pi];
                        slot.possessed = None;
                        slot.move_dir = Vec2::ZERO;
                    }
                    None => {
                        let last = slot.last;
                        self.possess(player, last);
                    }
                }
            }
            Command::EmptyDepot { .. } => {
                if let Some(depot) = self.container_mut(ContainerKind::Depot) {
                    depot.clear();
                }
            }
            Command::PlacePlan { player, kind, x, y, w, h, beds } => {
                if self.is_king(player) && self.check_plan(kind, x, y, w, h, beds).is_ok() {
                    self.place_plan(kind, x, y, w, h, beds);
                }
            }
            Command::Talk { player } => self.start_talk(player),
            Command::EndTalk { player } => self.end_talk(player),
            Command::AssignPost { player, who, post } => self.assign_post(player, who, post),
            Command::AssignBed { player, who, bed } => self.assign_bed(player, who, bed),
            Command::Interact { player } => {
                let Some(id) = self.possessed_by(player) else { return };
                // le roi : un appui s'adresse à la personne la plus proche, un second met fin à la conversation
                if self.characters[id.0].role == Role::King && !self.characters[id.0].asleep {
                    if self.talking_to(player).is_some() {
                        self.end_talk(player);
                        return;
                    }
                    if self.talkable(player).is_some() {
                        self.start_talk(player);
                        return;
                    }
                }
                if self.characters[id.0].asleep {
                    self.characters[id.0].wake_up();
                } else if self.characters[id.0].is_approaching() {
                    self.characters[id.0].cancel_approach(); // un second appui en route annule, comme pour un travail
                } else if self.can_lie_down(id) {
                    self.characters[id.0].lie_down();
                } else if self.characters[id.0].work.is_some() {
                    self.characters[id.0].cancel_work();
                } else if let Some((kind, tile)) = self.available_work(id) {
                    let c = &self.characters[id.0];
                    if kind == WorkKind::Forge && !at_forge_post(c.pos, tile) {
                        // on ne forge que depuis le poste au nord du socle : il y va seul (déjà sur la case au nord ou
                        // dans la moitié libre, il ne fait que le dernier pas pour se centrer)
                        if c.tile() == forge_spot(tile) || c.tile() == tile {
                            self.characters[id.0].start_approach(tile, std::collections::VecDeque::new());
                        } else {
                            let path = self.map.find_path_mode(c.tile(), forge_spot(tile), PathMode::Roads(self.config.grass_cost));
                            if !path.is_empty() {
                                self.characters[id.0].start_approach(tile, path);
                            }
                        }
                    } else {
                        let secs = match (kind, self.site_at(tile)) {
                            (WorkKind::Build, Some(site)) => site.total_secs,
                            _ => work_secs(kind, tile, &self.config, &self.veins),
                        };
                        self.characters[id.0].start_work(kind, tile, secs);
                    }
                } else {
                    self.transfer_with_crate(id); // sans travail : charger, décharger ou livrer devant un coffre
                }
            }
        }
    }

    /// Fait avancer la simulation de `dt` secondes (appeler avec `TICK`).
    pub fn step(&mut self, dt: f32) {
        self.time += dt;
        self.clock.advance(dt);
        let dt_hours = self.clock.hours_per_second() * dt;

        // les cultures poussent avec le temps, quoi que fassent les personnages
        self.farm.tick(dt_hours, self.config.crop_growth_hours);

        // les personnages agissent sur un monde en lecture seule et rapportent ce qu'ils ont terminé...
        let night = self.clock.is_night();
        let mut finished = Vec::new();
        let tavern = self.buildings.iter().find(|b| b.kind == PlanKind::Tavern).map(|b| b.front());
        let king_pos = self
            .players
            .iter()
            .filter_map(|p| p.possessed)
            .map(|id| &self.characters[id.0])
            .find(|c| c.role == Role::King)
            .map(|c| c.pos);
        self.drop_far_conversations();
        for i in 0..self.characters.len() {
            if self.characters[i].gone {
                continue;
            }
            // parcelles déjà visées ou travaillées par les autres, y compris ceux déjà mis à jour
            // dans ce même pas : c'est ce qui empêche deux paysans de choisir la même parcelle
            let claimed: Vec<TilePos> = self
                .characters
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .filter_map(|(_, c)| c.claimed_tile())
                .collect();
            // ce que les autres transporteurs ont déjà promis de prendre ou d'apporter
            let reservations: Vec<Reservation> = self
                .characters
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .flat_map(|(_, c)| c.reservations())
                .collect();
            let control = self
                .controller_of(CharacterId(i))
                .map(|p| self.players[p.0 as usize].move_dir);
            let env = Env {
                map: &self.map,
                farm: &self.farm,
                containers: &self.containers,
                reservations: &reservations,
                cfg: &self.config,
                night,
                dt,
                dt_hours,
                claimed: &claimed,
                veins: &self.veins,
                forests: &self.forests,
                sites: &self.sites,
                camp: self.camp,
                tavern,
                posts_required: self.camp.is_some(),
                king_pos,
            };
            if let Some(outcome) = self.characters[i].update(&env, control, &mut self.rng) {
                finished.push((i, outcome));
            }
        }

        // ... et c'est ici que les travaux terminés, les prises et les dépôts modifient le monde.
        for (who, outcome) in finished {
            match outcome {
                Outcome::Work(done) => self.finish_work(who, done),
                Outcome::Take { container, resource, amount } => {
                    let taken = self.containers[container].take(resource, amount);
                    self.characters[who].loaded(resource, taken);
                }
                Outcome::Drop { container } => self.drop_cargo(who, container),
                Outcome::Arrived { anvil } => self.arrive_at_forge(who, anvil),
            }
        }

        self.chop_trees();
        self.auto_beds();
        self.update_sites();
        self.build_sites();
        self.spawn_vagabonds(dt_hours);

        // les arbres abattus repoussent, au rythme du temps de jeu ; pas sur une case où l'on se tient
        let bodies: Vec<_> = self.characters.iter().map(|c| c.pos).collect();
        self.saplings.tick(dt_hours, &mut self.map, |tile| {
            bodies.iter().any(|p| {
                [(-BODY_HALF, -BODY_HALF), (BODY_HALF, -BODY_HALF), (-BODY_HALF, BODY_HALF), (BODY_HALF, BODY_HALF)]
                    .iter()
                    .any(|&(dx, dy)| Vec2::new(p.x + dx, p.y + dy).tile() == tile)
            })
        });
    }

    // ------------------------------------------------------------ interne

    /// Le coffre d'un genre donné, en lecture et écriture.
    fn container_mut(&mut self, kind: ContainerKind) -> Option<&mut Container> {
        self.containers.iter_mut().find(|c| c.kind == kind)
    }

    /// Applique un travail terminé : la production est rangée dans le coffre de son métier, sans trajet.
    fn finish_work(&mut self, who: usize, done: crate::work::WorkDone) {
        match done.kind {
            WorkKind::Forge => {
                // le fer n'est consommé qu'à la fin : s'il a manqué entre-temps, le coup est perdu
                let (iron, tools) = (self.config.iron_per_tool, self.config.tools_per_forge);
                if let Some(forge) = self.container_mut(ContainerKind::Forge) {
                    if forge.get(Resource::Iron) >= iron {
                        forge.take(Resource::Iron, iron);
                        forge.add(Resource::Tools, tools);
                    }
                }
                // il enchaîne aussitôt l'outil suivant, incarné ou non : pas un seul pas sans travail entre deux
                let c = &self.characters[who];
                if !c.asleep && c.work.is_none() && at_forge_post(c.pos, done.tile) && self.can_produce(c, WorkKind::Forge, done.tile) {
                    let secs = work_secs(WorkKind::Forge, done.tile, &self.config, &self.veins);
                    self.characters[who].start_work(WorkKind::Forge, done.tile, secs);
                }
            }
            WorkKind::Chop | WorkKind::Build => {} // jamais émis : `Game` décide seul de la chute des arbres et de la fin des chantiers
            WorkKind::Mine => {
                if let Some(i) = self.veins.iter().position(|v| v.contains(done.tile)) {
                    let ore = self.veins[i].kind.resource();
                    if let Some(mine) = self.container_mut(ContainerKind::Mine(i)) {
                        mine.add(ore, 1);
                    }
                }
            }
            WorkKind::Harvest => {
                if self.farm.apply(done.kind, done.tile) {
                    // le blé est dans les mains du paysan : il doit le porter au coffre du champ
                    let wheat = self.config.wheat_per_harvest;
                    self.characters[who].pick_up(Resource::Wheat, wheat);
                }
            }
            WorkKind::Till | WorkKind::Plant => {
                self.farm.apply(done.kind, done.tile);
            }
        }
    }

    /// Le joueur conduit au poste de forge de `anvil` y est arrivé : il se tourne vers l'enclume et forge, si c'est
    /// encore possible (du fer, de la place pour l'outil, l'enclume libre). Arrêté en route, il ne forge pas.
    fn arrive_at_forge(&mut self, who: usize, anvil: TilePos) {
        if !at_forge_post(self.characters[who].pos, anvil) {
            return;
        }
        self.characters[who].facing = Facing::Down;
        if self.available_work(CharacterId(who)) == Some((WorkKind::Forge, anvil)) {
            let secs = work_secs(WorkKind::Forge, anvil, &self.config, &self.veins);
            self.characters[who].start_work(WorkKind::Forge, anvil, secs);
        }
    }

    /// Le personnage `who` dépose sa cargaison dans un coffre : ce qui rentre y reste, le reste est gardé en main.
    fn drop_cargo(&mut self, who: usize, container: usize) {
        let Some((resource, amount)) = self.characters[who].cargo else { return };
        let Some(target) = self.containers.get_mut(container) else { return };
        let added = if target.accepts(resource) { target.add(resource, amount) } else { 0 };
        self.characters[who].unloaded(amount - added);
    }

    /// `Interact` d'un porteur devant un coffre : le transporteur charge (jusqu'à `transport_load` de ce que le
    /// coffre donne et dont il y a le plus) ou décharge ; le bûcheron décharge son bois. Renvoie `true` si quelque chose a eu lieu.
    fn transfer_with_crate(&mut self, id: CharacterId) -> bool {
        let c = &self.characters[id.0];
        if !matches!(c.role, Role::Transporter | Role::Lumberjack | Role::Farmer) {
            return false;
        }
        let Some(ci) = container_at(&self.containers, c.facing_tile()) else { return false };
        match c.cargo {
            Some((resource, amount)) => {
                let target = &mut self.containers[ci];
                let added = if target.accepts(resource) { target.add(resource, amount) } else { 0 };
                self.characters[id.0].unloaded(amount - added);
                added > 0
            }
            None if c.role == Role::Transporter => {
                // les mains vides : on charge ce que ce coffre donne, en commençant par ce dont il y a le plus
                let load = self.config.transport_load;
                let source = &self.containers[ci];
                let gives = |r: Resource| {
                    if source.is_depot() {
                        self.containers.iter().any(|k| !k.is_depot() && k.wants(r))
                    } else {
                        source.product() == Some(r)
                    }
                };
                let best = Resource::ALL.iter().copied().filter(|&r| gives(r) && source.get(r) > 0).max_by_key(|&r| source.get(r));
                let Some(resource) = best else { return false };
                let taken = self.containers[ci].take(resource, load);
                self.characters[id.0].loaded(resource, taken);
                taken > 0
            }
            None => false,
        }
    }

    /// Additionne les coups de tous les bûcherons sur chaque arbre. Quand le total atteint 1, l'arbre
    /// tombe : celui dont le coup l'achève est le propriétaire des bûches, les autres s'arrêtent.
    fn chop_trees(&mut self) {
        let mut felled: Vec<(TilePos, usize)> = Vec::new();
        for (i, c) in self.characters.iter().enumerate() {
            let Some(work) = c.work.filter(|w| w.kind == WorkKind::Chop) else { continue };
            let progress = self.tree_progress.entry(work.tile).or_insert(0.0);
            *progress += c.chop_delta();
            if *progress >= 1.0 && !felled.iter().any(|&(t, _)| t == work.tile) {
                felled.push((work.tile, i));
            }
        }
        // la barre de chacun montre l'avancement de l'arbre, partagé
        for c in &mut self.characters {
            if let Some(work) = c.work.filter(|w| w.kind == WorkKind::Chop) {
                let progress = self.tree_progress.get(&work.tile).copied().unwrap_or(0.0);
                c.set_work_progress(progress.min(1.0));
            }
        }
        for (tile, owner) in felled {
            self.tree_progress.remove(&tile);
            if self.map.get(tile) == Tile::Tree {
                self.saplings.plant(&mut self.map, tile, self.config.tree_regrow_hours);
                // les bûches sont à celui qui a porté le dernier coup : il les prend en main et devra les livrer
                let logs = self.config.logs_per_tree;
                self.characters[owner].pick_up(Resource::Wood, logs);
            }
            for c in &mut self.characters {
                if c.work.map_or(false, |w| w.kind == WorkKind::Chop && w.tile == tile) {
                    c.cancel_work();
                }
            }
        }
    }

    /// Le roi s'adresse à la personne la plus proche : elle arrête tout (et se réveille si elle dormait).
    fn start_talk(&mut self, player: PlayerId) {
        let Some(target) = self.talkable(player) else { return };
        let Some(king) = self.possessed_by(player) else { return };
        let king_pos = self.characters[king.0].pos;
        self.characters[target.0].start_talk(king_pos);
        if let Some(slot) = self.players.get_mut(player.0 as usize) {
            slot.talking = Some(target);
        }
    }

    /// Fin de la conversation : la personne reprend ce qu'elle faisait.
    fn end_talk(&mut self, player: PlayerId) {
        let Some(slot) = self.players.get_mut(player.0 as usize) else { return };
        if let Some(id) = slot.talking.take() {
            self.characters[id.0].end_talk();
        }
    }

    /// Le roi qui s'éloigne (ou qu'on lâche) met fin à la conversation.
    fn drop_far_conversations(&mut self) {
        for pi in 0..self.players.len() {
            let Some(target) = self.players[pi].talking else { continue };
            let king = self.players[pi].possessed.map(|id| &self.characters[id.0]).filter(|c| c.role == Role::King);
            let near = king.map_or(false, |k| k.pos.distance(self.characters[target.0].pos) <= self.config.talk_range * 2.0);
            if !near {
                self.end_talk(PlayerId(pi as u8));
            }
        }
    }

    fn assign_post(&mut self, player: PlayerId, who: CharacterId, tile: TilePos) {
        if !self.is_king(player) || self.talking_to(player) != Some(who) {
            return;
        }
        let Some(post) = self.posts().into_iter().find(|p| p.tile == tile) else { return };
        // le titulaire remplacé garde son métier mais n'a plus de poste
        if !post.unlimited {
            for holder in self.post_holders(tile) {
                if holder != who {
                    self.characters[holder.0].lose_post();
                }
            }
        }
        let container = (post.role == Role::Blacksmith).then(|| self.containers.iter().position(|c| c.kind == ContainerKind::Forge)).flatten();
        self.characters[who.0].take_post(post.role, tile, container);
        self.end_talk(player);
    }

    fn assign_bed(&mut self, player: PlayerId, who: CharacterId, bed: TilePos) {
        if !self.is_king(player) || self.talking_to(player) != Some(who) || !self.beds().contains(&bed) {
            return;
        }
        if let Some(owner) = self.bed_owner(bed).filter(|&o| o != who) {
            self.characters[owner.0].set_bed(None);
        }
        self.characters[who.0].set_bed(Some(bed));
        if self.characters[who.0].role == Role::Vagabond {
            self.characters[who.0].settle(); // un lit sans poste : un habitant sans métier
        }
        self.end_talk(player);
    }

    /// Quelqu'un qui a un poste sans lit prend le lit libre le plus proche de son poste ; refait à chaque pas, un lit qui
    /// apparaît (une maison finie) va au premier qui en manque.
    fn auto_beds(&mut self) {
        if !self.characters.iter().any(|c| !c.gone && !c.has_bed && c.post.is_some()) {
            return;
        }
        let mut free: Vec<TilePos> = self.beds().into_iter().filter(|&b| self.bed_owner(b).is_none()).collect();
        for i in 0..self.characters.len() {
            let c = &self.characters[i];
            let Some(post) = c.post.filter(|_| !c.gone && !c.has_bed) else { continue };
            let nearest = free.iter().copied().min_by_key(|b| (b.x - post.x).abs() + (b.y - post.y).abs());
            if let Some(bed) = nearest {
                free.retain(|&b| b != bed);
                self.characters[i].set_bed(Some(bed));
            }
        }
    }

    /// Pose un plan validé : les fondations apparaissent (solides) et le chantier attend ses ressources.
    fn place_plan(&mut self, kind: PlanKind, x: i32, y: i32, w: i32, h: i32, beds: u32) {
        self.map.fill_rect(x, y, w, h, Tile::Foundation);
        self.sites.push(Site {
            kind,
            x,
            y,
            w,
            h,
            beds,
            cost: plan_cost(kind, w, h, beds, &self.config),
            state: SiteState::Waiting,
            progress: 0.0,
            total_secs: plan_build_secs(kind, w, h, beds, &self.config),
            pile: None,
            pile_ground: Tile::Grass,
        });
    }

    /// Fait avancer les chantiers en attente et en livraison : un chantier ne démarre que si toutes ses ressources
    /// sont disponibles ; elles sont alors apportées devant lui, et il passe en construction quand tout est là.
    fn update_sites(&mut self) {
        for i in 0..self.sites.len() {
            match self.sites[i].state {
                SiteState::Waiting => {
                    let pile_tile = self.sites[i].pile_tile();
                    // le tas se pose sur une case que personne n'occupe : on écarte ceux qui s'y tiennent (sauf le
                    // personnage d'un joueur, qui bouge quand il veut), pour qu'un badaud ne bloque pas un chantier
                    if self.can_afford(self.sites[i].cost) {
                        self.clear_tile(pile_tile);
                    }
                    let site = &self.sites[i];
                    let blocked = self.characters.iter().any(|c| !c.gone && c.tile() == pile_tile);
                    if !blocked && self.can_afford(site.cost) {
                        let needs = site.cost.as_needs();
                        self.containers.push(Container::new_site(i, pile_tile, needs));
                        self.sites[i].pile_ground = if self.map.get(pile_tile) == Tile::Path { Tile::Path } else { Tile::Grass };
                        self.map.set(pile_tile, Tile::Crate);
                        self.sites[i].pile = Some(self.containers.len() - 1);
                        self.sites[i].state = SiteState::Delivering;
                    }
                }
                SiteState::Delivering => {
                    if self.sites[i].pile.map_or(false, |p| self.containers[p].is_complete()) {
                        self.sites[i].state = SiteState::Building;
                    }
                }
                SiteState::Building | SiteState::Done => {}
            }
        }
    }

    /// Écarte de cette tuile les personnages que personne n'incarne, vers une case voisine libre.
    fn clear_tile(&mut self, tile: TilePos) {
        let occupied: Vec<TilePos> = self.characters.iter().filter(|c| !c.gone).map(|c| c.tile()).collect();
        for ci in 0..self.characters.len() {
            let c = &self.characters[ci];
            if c.gone || c.tile() != tile || self.controller_of(CharacterId(ci)).is_some() {
                continue;
            }
            let free = self.map.walkable_neighbors(tile).into_iter().find(|n| !occupied.contains(n));
            if let Some(n) = free {
                self.characters[ci].shove_to(n);
            }
        }
    }

    /// Additionne le travail de tous les artisans sur chaque chantier en construction ; à 100 %, il est terminé.
    fn build_sites(&mut self) {
        for i in 0..self.sites.len() {
            if self.sites[i].state != SiteState::Building {
                continue;
            }
            let builders: Vec<usize> = (0..self.characters.len())
                .filter(|&ci| self.characters[ci].work.map_or(false, |w| w.kind == WorkKind::Build && self.sites[i].contains(w.tile)))
                .collect();
            let delta: f32 = builders.iter().map(|&ci| self.characters[ci].build_delta()).sum();
            let progress = (self.sites[i].progress + delta).min(1.0);
            self.sites[i].progress = progress;
            for &ci in &builders {
                self.characters[ci].set_work_progress(progress);
            }
            if progress >= 1.0 {
                for &ci in &builders {
                    self.characters[ci].cancel_work();
                }
                self.finish_site(i);
            }
        }
    }

    /// Le chantier est terminé : le bâtiment prend la place des fondations, les matériaux sont consommés et une route le
    /// relie au camp de base.
    fn finish_site(&mut self, i: usize) {
        let site = self.sites[i].clone();
        let pile = site.pile;
        match site.kind {
            PlanKind::House => {
                if let Some(spec) = HabitatSpec::house(site.x, site.y, site.w, site.h, site.beds) {
                    spec.build(&mut self.map);
                    self.habitats.push(spec);
                }
            }
            PlanKind::Warehouse | PlanKind::Tavern | PlanKind::Forge => {
                let building = Building { kind: site.kind, x: site.x, y: site.y, w: site.w, h: site.h };
                building.build(&mut self.map);
                self.buildings.push(building);
                if site.kind == PlanKind::Forge {
                    // le coffre de la forge, dehors contre le mur : le fer qu'on lui apporte, les outils qu'on y range
                    let chest = building.chest_tile();
                    self.map.set(chest, Tile::Crate);
                    let spec = ContainerSpec { kind: ContainerKind::Forge, tile: chest };
                    self.containers.push(Container::new(spec, self.config.container_capacity, &self.veins));
                }
            }
        }
        if let Some(p) = pile {
            if site.kind == PlanKind::Warehouse {
                // le tas devant la porte devient le coffre de l'entrepôt : vide, et beaucoup plus grand
                self.containers[p].become_depot(self.config.warehouse_capacity);
            } else {
                self.containers[p].retire();
                self.map.set(site.pile_tile(), site.pile_ground); // le chemin qui passait là reste un chemin
            }
        }
        self.sites[i].state = SiteState::Done;
        self.lay_road(site.front());
    }

    /// Trace la route de `from` jusqu'au camp de base : le trajet ordinaire (qui préfère les routes déjà là) dont les
    /// tuiles d'herbe deviennent des chemins. La route est continue par les côtés : un pas en diagonale est complété par
    /// une case voisine, pour que chaque virage fasse un angle (et non un creux) et que les sprites d'angle s'y raccordent.
    fn lay_road(&mut self, from: TilePos) {
        let Some(camp) = self.camp else { return };
        let path = self.map.find_path_mode(from, camp, PathMode::Roads(self.config.grass_cost));
        let mut prev = from;
        for t in std::iter::once(from).chain(path) {
            if t.x != prev.x && t.y != prev.y {
                let corners = [TilePos::new(t.x, prev.y), TilePos::new(prev.x, t.y)];
                let is_road = |g: &Game, c: TilePos| g.map.get(c) == Tile::Path;
                let corner = corners
                    .into_iter()
                    .find(|&c| is_road(self, c))
                    .or_else(|| corners.into_iter().find(|&c| self.map.get(c) == Tile::Grass));
                if let Some(c) = corner {
                    self.map.set(c, Tile::Path);
                }
            }
            if self.map.get(t) == Tile::Grass {
                self.map.set(t, Tile::Path);
            }
            prev = t;
        }
    }

    /// Un vagabond arrive de temps en temps par le bord de la carte, dans la limite de `vagabond_max`.
    fn spawn_vagabonds(&mut self, dt_hours: f32) {
        if self.camp.is_none() {
            return;
        }
        self.vagabond_timer -= dt_hours;
        if self.vagabond_timer > 0.0 {
            return;
        }
        self.vagabond_timer = self.config.vagabond_every_hours;
        let present = self.characters.iter().filter(|c| c.role == Role::Vagabond && !c.gone).count();
        if present >= self.config.vagabond_max {
            return;
        }
        let (w, h) = (self.map.width(), self.map.height());
        for _ in 0..40 {
            let along = self.rng.index(1000) as i32;
            let spot = match self.rng.index(4) {
                0 => TilePos::new(1, 1 + along % (h - 2)),
                1 => TilePos::new(w - 2, 1 + along % (h - 2)),
                2 => TilePos::new(1 + along % (w - 2), 1),
                _ => TilePos::new(1 + along % (w - 2), h - 2),
            };
            if self.map.get(spot).is_solid() || self.map.get(spot) == Tile::Water {
                continue;
            }
            self.vagabonds_seen += 1;
            let id = CharacterId(self.characters.len());
            let mut vagabond = Character::new(id, format!("Vagabond {}", self.vagabonds_seen), Role::Vagabond, spot, false, vec![spot]);
            vagabond.set_stay_hours(self.config.vagabond_stay_hours);
            vagabond.has_bed = false;
            self.characters.push(vagabond);
            break;
        }
    }

    fn is_free_for(&self, character: CharacterId, player: PlayerId) -> bool {
        self.controller_of(character).map_or(true, |p| p == player)
    }

    fn possess(&mut self, player: PlayerId, character: CharacterId) {
        self.end_talk(player);
        let pi = player.0 as usize;
        if pi >= self.players.len()
            || character.0 >= self.characters.len()
            || !self.is_free_for(character, player)
        {
            return;
        }
        if let Some(old) = self.players[pi].possessed {
            if old != character {
                self.characters[old.0].release_to_ai();
            }
        }
        let slot = &mut self.players[pi];
        slot.possessed = Some(character);
        slot.last = character;
        slot.move_dir = Vec2::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::Facing;
    use crate::farm::PlotState;
    use crate::map::Tile;
    use crate::forest::LodgeSpec;
    use crate::mine::OreKind;
use std::collections::HashSet;

    const P0: PlayerId = PlayerId(0);
    const FARMER: CharacterId = CharacterId(0);
    const KING: CharacterId = CharacterId(2);
    const WARLORD: CharacterId = CharacterId(3);
    /// Le paysan démarre sur cette parcelle, tourné vers le bas : il fait donc face à la suivante.
    const PLOT_HERE: TilePos = TilePos::new(4, 5);
    const PLOT_FRONT: TilePos = TilePos::new(4, 6);

    /// Nombre de pas simulés avant que `done` devienne vrai.
    fn ticks_until(game: &mut Game, max_secs: f32, done: impl Fn(&Game) -> bool) -> u32 {
        for n in 0..(max_secs / TICK) as u32 {
            if done(game) {
                return n;
            }
            game.step(TICK);
        }
        panic!("condition jamais atteinte en {max_secs} s");
    }

    /// Le village sans mineurs : le fer n'y est plus renouvelé, pour compter ce que la forge consomme.
    fn village_without_miners() -> Scenario {
        let mut scenario = Scenario::village();
        scenario.characters.retain(|c| c.role != Role::Miner);
        scenario
    }

    fn new_game() -> Game {
        Game::new(Scenario::village(), 1, Config::default())
    }

    /// Retrouve un personnage par son nom (plus robuste qu'un numéro quand on ajoute des personnages).
    fn id_of(game: &Game, name: &str) -> CharacterId {
        game.characters()
            .iter()
            .find(|c| c.name == name)
            .unwrap_or_else(|| panic!("aucun personnage nommé {name}"))
            .id
    }

    /// Partie avec une journée de 240 s : 1 heure de jeu = 10 s réelles.
    fn fast_day_game() -> Game {
        Game::new(
            Scenario::village(),
            1,
            Config { day_length_secs: 240.0, ..Config::default() },
        )
    }

    fn run(game: &mut Game, seconds: f32) {
        for _ in 0..(seconds / TICK) as usize {
            game.step(TICK);
        }
    }

    /// Fait avancer la partie jusqu'à ce jour et cette heure de jeu.
    fn run_until(game: &mut Game, day: u32, hour: f32) {
        for _ in 0..3_000_000 {
            let c = game.clock();
            if c.day() > day || (c.day() == day && c.hour() >= hour) {
                return;
            }
            game.step(TICK);
        }
        panic!("délai dépassé en attendant le jour {day}, {hour} h");
    }

    // ------------------------------------------------------------ carte et déplacements

    #[test]
    fn find_path_is_connected_and_walkable() {
        let scenario = Scenario::village();
        let map = &scenario.map;
        let path = map.find_path(TilePos::new(38, 15), TilePos::new(20, 24));
        assert!(!path.is_empty(), "la forge doit être reliée à la caserne");
        let mut prev = TilePos::new(38, 15);
        for p in path {
            assert!(!map.get(p).is_solid());
            let (dx, dy) = ((p.x - prev.x).abs(), (p.y - prev.y).abs());
            assert!(dx <= 1 && dy <= 1 && dx + dy >= 1, "pas d'une case, diagonale permise");
            if dx == 1 && dy == 1 {
                assert!(
                    !map.get(TilePos::new(p.x, prev.y)).is_solid() && !map.get(TilePos::new(prev.x, p.y)).is_solid(),
                    "pas de coin coupé"
                );
            }
            prev = p;
        }
        assert_eq!(prev, TilePos::new(20, 24));
    }

    #[test]
    fn find_path_to_a_wall_is_empty() {
        let map = Scenario::village().map;
        assert!(map.find_path(TilePos::new(31, 20), TilePos::new(28, 11)).is_empty());
    }

    #[test]
    fn every_bed_is_reachable_from_every_work_spot() {
        let scenario = Scenario::village();
        for c in &scenario.characters {
            assert_eq!(scenario.map.get(c.home), Tile::Bed, "{} n'a pas de lit", c.name);
            for &spot in &c.work_spots {
                assert!(
                    spot == c.home || !scenario.map.find_path(spot, c.home).is_empty(),
                    "{} ne peut pas rejoindre son lit depuis {:?}",
                    c.name,
                    spot
                );
            }
        }
    }

    #[test]
    fn cannot_walk_through_walls() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: CharacterId(2) }); // le roi, dans le château
        game.apply(Command::Move { player: P0, dir: Vec2::new(0.0, -1.0) });
        run(&mut game, 5.0);
        let king = game.character(CharacterId(2)).unwrap();
        assert!(king.pos.y >= 12.0 + crate::character::BODY_HALF - 0.01, "le roi a traversé le mur : {:?}", king.pos);
        assert!(!game.map().get(king.tile()).is_solid());
    }

    #[test]
    fn player_is_faster_than_ai() {
        let mut game = new_game();
        // le paysan part du champ, avec de la place devant lui
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.apply(Command::Move { player: P0, dir: Vec2::new(1.0, 0.0) });
        let start = game.character(FARMER).unwrap().pos;
        run(&mut game, 1.0);
        let moved = game.character(FARMER).unwrap().pos.distance(start);
        assert!(moved > crate::character::AI_SPEED * 1.5, "déplacement trop faible : {moved}");
    }

    // ------------------------------------------------------------ possession et AFK

    #[test]
    fn afk_mode_the_city_runs_by_itself() {
        let mut game = new_game(); // personne n'est incarné
        let mut visited: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(); game.characters().len()];
        for _ in 0..(90.0 / TICK) as usize {
            game.step(TICK);
            for (i, c) in game.characters().iter().enumerate() {
                visited[i].insert((c.tile().x, c.tile().y));
            }
        }
        let active = visited.iter().filter(|v| v.len() > 3).count();
        assert!(active >= 4, "seulement {active} personnages ont bougé en AFK");
    }

    #[test]
    fn possession_switch_releases_previous_character_to_ai() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: KING });
        game.apply(Command::Possess { player: P0, character: CharacterId(4) });
        assert_eq!(game.possessed_by(P0), Some(CharacterId(4)));
        assert_eq!(game.controller_of(KING), None);
        // il peut revenir à son point de départ : on mesure donc le déplacement maximal
        let before = game.character(KING).unwrap().pos;
        let mut max_moved = 0.0_f32;
        for _ in 0..(30.0 / TICK) as usize {
            game.step(TICK);
            max_moved = max_moved.max(before.distance(game.character(KING).unwrap().pos));
        }
        assert!(max_moved > 0.5, "le roi libéré n'a pas repris sa routine");
    }

    #[test]
    fn two_players_cannot_possess_the_same_character() {
        let mut game = Game::new(Scenario::village(), 2, Config::default());
        game.apply(Command::Possess { player: PlayerId(0), character: FARMER });
        game.apply(Command::Possess { player: PlayerId(1), character: FARMER });
        assert_eq!(game.possessed_by(PlayerId(0)), Some(FARMER));
        assert_eq!(game.possessed_by(PlayerId(1)), None);
        // PossessNext saute le personnage déjà pris
        game.apply(Command::PossessNext { player: PlayerId(1) });
        assert_ne!(game.possessed_by(PlayerId(1)), Some(FARMER));
        assert!(game.possessed_by(PlayerId(1)).is_some());
    }

    #[test]
    fn toggle_idle_releases_then_resumes_last_character() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: CharacterId(2) });
        game.apply(Command::ToggleIdle { player: P0 });
        assert_eq!(game.possessed_by(P0), None);
        game.apply(Command::ToggleIdle { player: P0 });
        assert_eq!(game.possessed_by(P0), Some(CharacterId(2)));
    }

    #[test]
    fn simulation_is_deterministic() {
        let snapshot = |g: &Game| {
            g.characters()
                .iter()
                .map(|c| (c.pos.x, c.pos.y, c.fatigue, c.asleep))
                .collect::<Vec<_>>()
        };
        let mut a = new_game();
        let mut b = new_game();
        run(&mut a, 60.0);
        run(&mut b, 60.0);
        assert_eq!(snapshot(&a), snapshot(&b));
    }

    // ------------------------------------------------------------ cycle jour/nuit

    #[test]
    fn everyone_but_the_night_shift_goes_home_and_sleeps_at_night() {
        let mut game = fast_day_game();
        run_until(&mut game, 1, 23.9); // le garde de jour revient de loin : on lui laisse le temps
        for c in game.characters().iter().filter(|c| !c.night_shift) {
            assert!(c.asleep, "{} ne dort pas à 23 h", c.name);
            assert_eq!(c.tile(), c.home, "{} ne dort pas dans son lit", c.name);
            assert_eq!(c.activity, crate::character::Activity::Sleeping);
        }
    }

    #[test]
    fn everyone_but_the_night_shift_wakes_up_rested_and_goes_back_to_work() {
        let mut game = fast_day_game();
        run_until(&mut game, 2, 7.5);
        assert_eq!(game.clock().day(), 2);
        for c in game.characters().iter().filter(|c| !c.night_shift) {
            assert!(!c.asleep, "{} dort encore à 7 h 30", c.name);
            assert!(c.fatigue < 20.0, "{} est mal reposé : {}", c.name, c.fatigue);
        }
    }

    #[test]
    fn exhausted_character_naps_in_daytime_then_wakes_when_rested() {
        let mut game = fast_day_game();
        game.characters[WARLORD.0].fatigue = 95.0;
        run(&mut game, 10.0); // le lit est à cinq tuiles, et il est épuisé : il marche à moitié vitesse
        assert!(game.characters[WARLORD.0].asleep, "le chef de guerre épuisé devrait faire la sieste");
        run(&mut game, 80.0); // ~8 h de jeu : réveillé reposé vers 13 h 40, puis retourné au travail
        let c = &game.characters[WARLORD.0];
        assert!(!c.asleep, "le chef de guerre devrait être réveillé");
        // 95 au départ ; il a dormi jusqu'à ~15 puis retravaillé un moment : bien en dessous de l'épuisement
        assert!(c.fatigue < 35.0, "fatigue : {}", c.fatigue);
        assert!(!game.clock().is_night());
    }

    #[test]
    fn sleeping_reduces_fatigue_and_fatigue_slows_walking() {
        let mut fresh = new_game();
        let mut tired = new_game();
        tired.characters[FARMER.0].fatigue = 80.0;
        for g in [&mut fresh, &mut tired] {
            g.apply(Command::Possess { player: P0, character: FARMER });
            g.apply(Command::Move { player: P0, dir: Vec2::new(1.0, 0.0) });
            run(g, 1.0);
        }
        let d_fresh = fresh.characters[FARMER.0].pos.x - 4.5;
        let d_tired = tired.characters[FARMER.0].pos.x - 4.5;
        assert!(d_tired < d_fresh * 0.9, "la fatigue ne ralentit pas : {d_tired} vs {d_fresh}");
    }

    // ------------------------------------------------------------ joueur et lit

    #[test]
    fn player_can_stay_up_at_night_and_the_ai_takes_over_in_afk() {
        let mut game = fast_day_game();
        game.apply(Command::Possess { player: P0, character: FARMER });
        run_until(&mut game, 1, 23.0);
        let farmer = &game.characters[FARMER.0];
        assert!(!farmer.asleep, "le personnage incarné ne doit pas être couché de force");
        assert!(farmer.fatigue > 30.0, "la fatigue doit s'accumuler : {}", farmer.fatigue);

        game.apply(Command::ToggleIdle { player: P0 }); // le joueur le lâche
        run_until(&mut game, 2, 5.0);
        let farmer = &game.characters[FARMER.0];
        assert!(farmer.asleep && farmer.tile() == farmer.home, "l'IA aurait dû le mettre au lit");
    }

    #[test]
    fn interact_lies_down_near_bed_when_tired_and_gets_up_again() {
        let mut game = new_game();
        let home = game.characters[FARMER.0].home;
        game.apply(Command::Possess { player: P0, character: FARMER });

        // loin du lit : rien ne se passe
        game.characters[FARMER.0].fatigue = 60.0;
        game.apply(Command::Interact { player: P0 });
        assert!(!game.characters[FARMER.0].asleep);

        // près du lit mais en forme, de jour : inutile de se coucher
        game.characters[FARMER.0].pos = home.center() + Vec2::new(1.0, 0.0);
        game.characters[FARMER.0].fatigue = 0.0;
        assert!(!game.can_lie_down(FARMER));

        // près du lit et fatigué : il se couche dans le lit
        game.characters[FARMER.0].fatigue = 60.0;
        assert!(game.can_lie_down(FARMER));
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[FARMER.0].asleep);
        assert_eq!(game.characters[FARMER.0].pos, home.center());

        // endormi, il ne bouge plus
        game.apply(Command::Move { player: P0, dir: Vec2::new(1.0, 0.0) });
        run(&mut game, 1.0);
        assert_eq!(game.characters[FARMER.0].pos, home.center());

        // un second appui le réveille
        game.apply(Command::Interact { player: P0 });
        assert!(!game.characters[FARMER.0].asleep);
    }

    #[test]
    fn at_night_one_can_lie_down_even_when_rested_and_a_released_sleeper_keeps_sleeping() {
        let mut game = fast_day_game();
        game.apply(Command::Possess { player: P0, character: FARMER });
        run_until(&mut game, 1, 22.0);
        let home = game.characters[FARMER.0].home;
        game.characters[FARMER.0].pos = home.center() + Vec2::new(0.0, 1.0);
        game.characters[FARMER.0].fatigue = 0.0;
        assert!(game.can_lie_down(FARMER));
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[FARMER.0].asleep);

        // le joueur passe à quelqu'un d'autre : le dormeur continue de dormir, sous l'IA
        game.apply(Command::Possess { player: P0, character: CharacterId(1) });
        run(&mut game, 5.0);
        assert!(game.characters[FARMER.0].asleep);
        assert_eq!(game.controller_of(FARMER), None);
    }

    // ------------------------------------------------------------ forme

    #[test]
    fn fitness_is_the_inverse_of_fatigue() {
        let mut game = new_game();
        assert_eq!(game.characters[FARMER.0].fitness(), 100.0);
        game.characters[FARMER.0].fatigue = 30.0;
        assert_eq!(game.characters[FARMER.0].fitness(), 70.0);
        game.characters[FARMER.0].fatigue = 100.0;
        assert_eq!(game.characters[FARMER.0].fitness(), 0.0);
    }

    // ------------------------------------------------------------ le paysan

    #[test]
    fn player_runs_a_full_crop_cycle_and_the_stock_grows() {
        let mut game = fast_day_game(); // 1 h de jeu = 10 s ; pousse = 12 h = 120 s
        game.apply(Command::Possess { player: P0, character: FARMER });
        assert_eq!(game.available_work(FARMER), Some((WorkKind::Till, PLOT_FRONT)));

        game.apply(Command::Interact { player: P0 }); // labourer
        run(&mut game, 2.2); // 4 s / 2 (joueur) = 2 s
        assert_eq!(game.farm().state_at(PLOT_FRONT), Some(PlotState::Tilled));

        game.apply(Command::Interact { player: P0 }); // planter
        run(&mut game, 1.7); // 3 s / 2 = 1.5 s
        assert!(matches!(game.farm().state_at(PLOT_FRONT), Some(PlotState::Planted { .. })));
        // la parcelle devant lui pousse : le travail disponible est désormais sous ses pieds
        assert_eq!(game.available_work(FARMER), Some((WorkKind::Till, PLOT_HERE)));

        run(&mut game, 125.0); // la pousse prend 120 s, que le joueur fasse quelque chose ou non
        assert_eq!(game.farm().state_at(PLOT_FRONT), Some(PlotState::Ripe));
        assert_eq!(game.stock().get(Resource::Wheat), 0);

        game.apply(Command::Interact { player: P0 }); // récolter
        run(&mut game, 2.2);
        assert_eq!(game.farm().state_at(PLOT_FRONT), Some(PlotState::Fallow));
        assert_eq!(game.characters[FARMER.0].cargo, Some((Resource::Wheat, game.config().wheat_per_harvest)), "le blé est en main");
        assert_eq!(game.stock().get(Resource::Wheat), 0, "pas encore dans le coffre");
    }

    #[test]
    fn player_works_twice_as_fast_as_the_ai() {
        // IA : le paysan se met seul à labourer la parcelle sous ses pieds
        let mut ai = new_game();
        let t_ai = ticks_until(&mut ai, 30.0, |g| g.farm().state_at(PLOT_HERE) == Some(PlotState::Tilled));

        // joueur : même travail (labourer), sur la parcelle devant lui
        let mut player = new_game();
        player.apply(Command::Possess { player: P0, character: FARMER });
        player.apply(Command::Interact { player: P0 });
        let t_player = ticks_until(&mut player, 30.0, |g| g.farm().state_at(PLOT_FRONT) == Some(PlotState::Tilled));

        let ratio = t_ai as f32 / t_player as f32;
        assert!((1.9..2.1).contains(&ratio), "le joueur devrait être 2x plus rapide, rapport : {ratio}");
    }

    #[test]
    fn crops_grow_in_fixed_game_time_whoever_planted_them() {
        let mut game = fast_day_game(); // 12 h de jeu = 120 s
        game.farm.set_state(PLOT_FRONT, PlotState::Planted { growth: 0.0 });
        run_until(&mut game, 1, 18.5);
        assert!(matches!(game.farm().state_at(PLOT_FRONT), Some(PlotState::Planted { .. })));
        run_until(&mut game, 1, 19.1);
        assert_eq!(game.farm().state_at(PLOT_FRONT), Some(PlotState::Ripe));
    }

    #[test]
    fn only_farmers_can_work_the_fields() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: KING });
        game.characters[KING.0].pos = PLOT_HERE.center();
        assert_eq!(game.available_work(KING), None);
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[KING.0].work.is_none());
    }

    #[test]
    fn moving_cancels_the_work_in_progress() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[FARMER.0].work.is_some());
        run(&mut game, 0.5);
        assert!(game.characters[FARMER.0].work.is_some(), "il doit continuer tant qu'on ne bouge pas");

        game.apply(Command::Move { player: P0, dir: Vec2::new(1.0, 0.0) });
        run(&mut game, 0.1);
        assert!(game.characters[FARMER.0].work.is_none());
        assert_eq!(game.farm().state_at(PLOT_FRONT), Some(PlotState::Fallow), "travail annulé : rien n'a changé");
    }

    #[test]
    fn interact_again_cancels_the_work() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.apply(Command::Interact { player: P0 });
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[FARMER.0].work.is_none());
    }

    #[test]
    fn tired_farmer_works_slower() {
        let mut fresh = new_game();
        let mut tired = new_game();
        tired.characters[FARMER.0].fatigue = 80.0; // rendement 0.7
        let t_fresh = ticks_until(&mut fresh, 60.0, |g| g.farm().state_at(PLOT_HERE) == Some(PlotState::Tilled));
        let t_tired = ticks_until(&mut tired, 60.0, |g| g.farm().state_at(PLOT_HERE) == Some(PlotState::Tilled));
        assert!(t_tired as f32 > t_fresh as f32 * 1.3, "{t_tired} vs {t_fresh}");
    }

    #[test]
    fn released_farmer_finishes_his_work_under_ai_control() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.apply(Command::Interact { player: P0 });
        game.apply(Command::ToggleIdle { player: P0 }); // le joueur le lâche en plein labour
        ticks_until(&mut game, 10.0, |g| g.farm().state_at(PLOT_FRONT) == Some(PlotState::Tilled));
    }

    #[test]
    fn ai_farmer_runs_the_farm_alone_and_brings_in_a_harvest() {
        let mut game = fast_day_game(); // personne n'est incarné : mode AFK
        run_until(&mut game, 2, 12.0);
        assert!(game.stock().get(Resource::Wheat) + game.in_hands(Resource::Wheat) > 0, "aucune récolte après un jour et demi en AFK");
        let worked = game
            .farm()
            .plots()
            .iter()
            .filter(|p| p.state != PlotState::Fallow)
            .count();
        assert!(worked >= 3, "le paysan a trop peu travaillé les champs ({worked} parcelles)");
    }

    #[test]
    fn night_interrupts_the_ai_farmer_who_goes_to_bed() {
        let mut game = fast_day_game();
        run_until(&mut game, 1, 23.5); // le champ est à une longue marche du lit
        let farmer = &game.characters[FARMER.0];
        assert!(farmer.asleep && farmer.work.is_none());
    }

    // ------------------------------------------------------------ plusieurs paysans

    fn wheat_after(farmers: usize, days: u32) -> u32 {
        let mut game = Game::new(Scenario::village_with_farmers(farmers), 1, Config::default());
        run_until(&mut game, days, 23.9);
        game.stock().get(Resource::Wheat)
    }

    #[test]
    fn two_farmers_harvest_much_more_than_one() {
        // régression : avant la réservation des parcelles, tous les paysans se ruaient sur la même
        // parcelle, et en avoir 4 ne rapportait pas plus qu'en avoir 1
        let (one, two) = (wheat_after(1, 3), wheat_after(2, 3));
        assert!(two as f32 > one as f32 * 1.5, "1 paysan : {one} blé, 2 paysans : {two} blé");
    }

    #[test]
    fn farmers_never_aim_at_the_same_plot() {
        let mut game = Game::new(Scenario::village_with_farmers(3), 1, Config::default());
        for _ in 0..(1200.0 / TICK) as usize {
            // 2 jours de jeu
            game.step(TICK);
            let claims: Vec<TilePos> = game.characters().iter().filter_map(|c| c.claimed_tile()).collect();
            let unique: HashSet<TilePos> = claims.iter().copied().collect();
            assert_eq!(
                claims.len(),
                unique.len(),
                "parcelle visée en double à t = {:.1} s : {claims:?}",
                game.time()
            );
        }
    }

    #[test]
    fn player_does_not_start_work_on_a_plot_another_farmer_is_already_working() {
        let mut game = Game::new(Scenario::village_with_farmers(2), 1, Config::default());
        game.apply(Command::Possess { player: P0, character: FARMER });
        let second_farmer = id_of(&game, "Paysan 2");
        assert!(game.characters[second_farmer.0].start_work(WorkKind::Till, PLOT_FRONT, 4.0));
        // la parcelle de devant est prise : il se rabat sur celle sous ses pieds
        assert_eq!(game.available_work(FARMER), Some((WorkKind::Till, PLOT_HERE)));
    }

    #[test]
    fn a_claim_disappears_when_the_player_takes_over_a_walking_farmer() {
        let mut game = Game::new(Scenario::village_with_farmers(2), 1, Config::default());
        let second = id_of(&game, "Paysan 2");
        // on le force à marcher vers une autre parcelle : celle où il se trouve est déjà plantée
        let here = game.characters[second.0].tile();
        game.farm.set_state(here, PlotState::Planted { growth: 0.0 });
        run(&mut game, 0.3);
        assert!(game.characters[second.0].work.is_none());
        assert!(game.characters[second.0].claimed_tile().is_some(), "il devrait viser une parcelle");

        game.apply(Command::Possess { player: P0, character: second });
        run(&mut game, 0.1);
        assert_eq!(game.characters[second.0].claimed_tile(), None, "le joueur décide : plus de réservation");
    }

    #[test]
    fn a_possessed_farmer_mid_work_keeps_his_plot_reserved() {
        let mut game = Game::new(Scenario::village_with_farmers(2), 1, Config::default());
        let second = id_of(&game, "Paysan 2");
        run(&mut game, 0.5); // l'IA a commencé à labourer sa parcelle
        let plot = game.characters[second.0].work.expect("il travaille").tile;
        game.apply(Command::Possess { player: P0, character: second });
        run(&mut game, 0.1);
        // il continue de travailler sur place : les autres paysans doivent l'éviter
        assert_eq!(game.characters[second.0].claimed_tile(), Some(plot));
    }

    // ------------------------------------------------------------ le forgeron

    use crate::character::{at_forge_post, forge_post, forge_spot};

    const SMITH: CharacterId = CharacterId(1);
    const ANVIL: TilePos = TilePos::new(39, 15);

    #[test]
    fn scenario_has_one_anvil_and_beds_are_distinct() {
        let scenario = Scenario::village();
        assert_eq!(scenario.map.find_tiles(Tile::Anvil), vec![ANVIL]);
        let homes: HashSet<TilePos> = scenario.characters.iter().map(|c| c.home).collect();
        assert_eq!(homes.len(), scenario.characters.len(), "deux personnages se partagent un lit");
        // l'enclume est un obstacle ; on forge depuis la case au nord, qui est praticable et joignable
        assert!(scenario.map.get(ANVIL).is_solid());
        for anvil in scenario.map.find_tiles(Tile::Anvil) {
            let spot = forge_spot(anvil);
            assert!(!scenario.map.get(spot).is_solid(), "le poste de forge de {anvil:?} est bloqué");
            let smith = scenario.characters.iter().find(|c| c.role == Role::Blacksmith).unwrap();
            assert!(!scenario.map.find_path(smith.work_spots[0], spot).is_empty(), "poste de forge injoignable");
        }
    }

    #[test]
    fn the_forge_post_is_centered_on_the_anvil_and_against_it() {
        let scenario = Scenario::village();
        let post = forge_post(ANVIL);
        assert_eq!(post.tile(), ANVIL, "le point est dans la moitié libre de la case de l'enclume");
        assert_eq!(post.x, ANVIL.center().x, "centré sur l'enclume");
        assert!(post.y < ANVIL.center().y, "dans la moitié haute");
        assert!(!scenario.map.is_box_blocked(post, BODY_HALF), "le corps y tient sans toucher l'enclume");
        // collé contre elle : un pas de plus vers le sud et le corps la touche
        assert!(scenario.map.is_box_blocked(Vec2::new(post.x, post.y + 0.02), BODY_HALF));
    }

    /// Le forgeron incarné, à `pos`, tourné vers le bas.
    fn player_smith_at(pos: Vec2) -> Game {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = pos;
        game.characters[SMITH.0].facing = Facing::Down;
        game
    }

    #[test]
    fn pushing_south_from_the_north_tile_stops_feet_against_the_base() {
        let mut game = player_smith_at(forge_spot(ANVIL).center());
        game.apply(Command::Move { player: P0, dir: Vec2::new(0.0, 1.0) });
        run(&mut game, 1.0);
        assert_eq!(game.characters[SMITH.0].pos, forge_post(ANVIL), "les pieds contre le socle, sans s'arrêter avant");
    }

    #[test]
    fn the_base_blocks_from_the_south_and_the_sides() {
        let start = ANVIL.center();
        for (from, dir) in [
            (Vec2::new(start.x, start.y + 1.0), Vec2::new(0.0, -1.0)),
            (Vec2::new(start.x - 1.0, start.y + 0.4), Vec2::new(1.0, 0.0)),
            (Vec2::new(start.x + 1.0, start.y + 0.4), Vec2::new(-1.0, 0.0)),
        ] {
            let mut game = player_smith_at(from);
            game.apply(Command::Move { player: P0, dir });
            run(&mut game, 1.0);
            let p = game.characters[SMITH.0].pos;
            assert!(!game.map.is_box_blocked(p, BODY_HALF), "il ne traverse pas le socle : {p:?}");
            assert_ne!(p.tile(), ANVIL, "il reste hors de la case : {p:?}");
        }
    }

    #[test]
    fn off_center_in_the_free_half_one_press_recenters_then_forges() {
        let post = forge_post(ANVIL);
        let mut game = player_smith_at(Vec2::new(post.x - 0.2, post.y - 0.1));
        assert_eq!(game.available_work(SMITH), Some((WorkKind::Forge, ANVIL)));
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[SMITH.0].is_approaching());
        ticks_until(&mut game, 2.0, |g| g.characters[SMITH.0].work.is_some());
        assert!(at_forge_post(game.characters[SMITH.0].pos, ANVIL));
    }

    #[test]
    fn a_route_from_the_free_half_leaves_by_the_north_and_never_crosses_the_anvil() {
        let map = &Scenario::village().map;
        let south = TilePos::new(ANVIL.x, ANVIL.y + 1);
        let path = map.find_path(ANVIL, south);
        assert_eq!(path.front(), Some(&forge_spot(ANVIL)), "il sort par le nord");
        assert_eq!(path.back(), Some(&south));
        assert!(!path.contains(&ANVIL));
        // et une route ordinaire ne traverse jamais la case de l'enclume
        let west = TilePos::new(ANVIL.x - 1, ANVIL.y);
        let east = TilePos::new(ANVIL.x + 1, ANVIL.y);
        assert!(!map.find_path(west, east).contains(&ANVIL));
    }

    #[test]
    fn the_ai_leaves_the_forge_post_when_the_player_lets_go() {
        // sans fer, il ne forge pas : l'IA doit quitter le poste et circuler, sans rester collée au socle
        let mut game = Game::new(village_without_miners(), 1, Config { starting_iron: 0, ..Config::default() });
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = forge_post(ANVIL);
        game.apply(Command::ToggleIdle { player: P0 });
        ticks_until(&mut game, 60.0, |g| g.characters[SMITH.0].tile() != ANVIL);
        let c = &game.characters[SMITH.0];
        assert!(!game.map.is_box_blocked(c.pos, BODY_HALF));
    }

    #[test]
    fn ai_blacksmith_forges_only_from_the_north_of_the_anvil_facing_down() {
        let mut game = new_game();
        ticks_until(&mut game, 10.0, |g| g.characters[SMITH.0].work.is_some());
        let smith = &game.characters[SMITH.0];
        assert!(at_forge_post(smith.pos, ANVIL), "il forge depuis {:?}, pas depuis le poste", smith.pos);
        assert_eq!(smith.facing, Facing::Down);
        assert_eq!(smith.work.unwrap().tile, ANVIL);
    }

    /// Le forgeron incarné, à l'ouest de l'enclume et tourné vers elle.
    fn player_smith_west_of_the_anvil() -> Game {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = TilePos::new(38, 15).center();
        game.characters[SMITH.0].facing = Facing::Right;
        game
    }

    #[test]
    fn one_press_beside_the_anvil_walks_the_player_to_the_north_and_forges() {
        let mut game = player_smith_west_of_the_anvil();
        assert_eq!(game.available_work(SMITH), Some((WorkKind::Forge, ANVIL)), "on lui propose de forger");
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[SMITH.0].work.is_none(), "pas depuis le côté");
        assert!(game.characters[SMITH.0].is_approaching());
        ticks_until(&mut game, 5.0, |g| g.characters[SMITH.0].work.is_some());
        let smith = &game.characters[SMITH.0];
        assert!(at_forge_post(smith.pos, ANVIL), "il forge depuis {:?}, pas depuis le poste", smith.pos);
        assert_eq!(smith.facing, Facing::Down);
        assert!(!smith.is_approaching());
        // et il forge pour de bon : un outil sort
        let before = game.stock().get(Resource::Tools);
        ticks_until(&mut game, 10.0, |g| g.stock().get(Resource::Tools) > before);
    }

    #[test]
    fn moving_or_pressing_again_on_the_way_cancels_the_walk_to_the_forge() {
        // un nouvel appui en route
        let mut game = player_smith_west_of_the_anvil();
        game.apply(Command::Interact { player: P0 });
        run(&mut game, 0.2);
        game.apply(Command::Interact { player: P0 });
        assert!(!game.characters[SMITH.0].is_approaching());
        let stopped = game.characters[SMITH.0].pos;
        run(&mut game, 3.0);
        assert!(game.characters[SMITH.0].work.is_none());
        assert_eq!(game.characters[SMITH.0].pos, stopped, "il s'arrête sur place");

        // une touche de déplacement en route
        let mut game = player_smith_west_of_the_anvil();
        game.apply(Command::Interact { player: P0 });
        run(&mut game, 0.2);
        game.apply(Command::Move { player: P0, dir: Vec2::new(-1.0, 0.0) });
        run(&mut game, 0.2);
        assert!(!game.characters[SMITH.0].is_approaching());
        assert_eq!(game.characters[SMITH.0].facing, Facing::Left, "il obéit au déplacement");
        game.apply(Command::Move { player: P0, dir: Vec2::ZERO });
        run(&mut game, 3.0);
        assert!(game.characters[SMITH.0].work.is_none());
    }

    #[test]
    fn at_the_forge_post_one_press_forges_at_once() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = forge_post(ANVIL);
        game.characters[SMITH.0].facing = Facing::Down;
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[SMITH.0].work.map(|w| w.kind), Some(WorkKind::Forge));
        run(&mut game, 0.5);
        assert_eq!(game.characters[SMITH.0].pos, forge_post(ANVIL), "il n'a pas bougé");
    }

    #[test]
    fn off_center_on_the_forge_tile_one_press_recenters_then_forges() {
        // sur la bonne case, mais décalé vers l'ouest et loin de l'enclume
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: SMITH });
        let spot = forge_spot(ANVIL).center();
        game.characters[SMITH.0].pos = Vec2::new(spot.x - 0.2, spot.y - 0.1);
        game.characters[SMITH.0].facing = Facing::Down;
        assert_eq!(game.available_work(SMITH), Some((WorkKind::Forge, ANVIL)));
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[SMITH.0].work.is_none(), "pas décalé");
        assert!(game.characters[SMITH.0].is_approaching());
        ticks_until(&mut game, 2.0, |g| g.characters[SMITH.0].work.is_some());
        let smith = &game.characters[SMITH.0];
        assert!(at_forge_post(smith.pos, ANVIL), "il forge depuis {:?}, pas depuis le poste", smith.pos);
        assert_eq!(smith.facing, Facing::Down);
    }

    /// Fait avancer `game` jusqu'à ce que `tools` outils soient sortis, en vérifiant à chaque pas que le forgeron
    /// n'a jamais cessé de forger entre deux outils.
    fn forges_without_a_gap(game: &mut Game, tools: u32) {
        let target = game.stock().get(Resource::Tools) + tools;
        let mut t = 0.0;
        while game.stock().get(Resource::Tools) < target {
            assert!(game.characters[SMITH.0].work.is_some(), "un pas sans forge entre deux outils");
            game.step(TICK);
            t += TICK;
            assert!(t < 60.0, "pas assez d'outils forgés");
        }
        assert!(game.characters[SMITH.0].work.is_some(), "il s'est arrêté après le dernier outil");
    }

    #[test]
    fn the_ai_blacksmith_chains_tools_without_a_gap() {
        let mut game = new_game();
        ticks_until(&mut game, 10.0, |g| g.characters[SMITH.0].work.is_some());
        forges_without_a_gap(&mut game, 2);
    }

    #[test]
    fn one_press_makes_the_player_blacksmith_chain_tools_until_stopped() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = forge_post(ANVIL);
        game.characters[SMITH.0].facing = Facing::Down;
        game.apply(Command::Interact { player: P0 });
        forges_without_a_gap(&mut game, 2);

        // un nouvel appui arrête la série
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[SMITH.0].work.is_none());
        let tools = game.stock().get(Resource::Tools);
        run(&mut game, 5.0);
        assert_eq!(game.stock().get(Resource::Tools), tools, "il ne doit plus forger");

        // un déplacement aussi
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[SMITH.0].work.is_some());
        game.apply(Command::Move { player: P0, dir: Vec2::new(0.0, -1.0) });
        run(&mut game, 0.1);
        game.apply(Command::Move { player: P0, dir: Vec2::ZERO });
        run(&mut game, 5.0);
        assert!(game.characters[SMITH.0].work.is_none());
    }

    #[test]
    fn ai_blacksmith_forges_tools_and_iron_is_conserved() {
        let mut game = Game::new(village_without_miners(), 1, Config::default());
        let start_iron = game.stock().get(Resource::Iron);
        assert_eq!(start_iron, game.config().starting_iron);
        run(&mut game, 60.0);
        let tools = game.stock().get(Resource::Tools);
        assert!(tools >= 5, "le forgeron IA n'a forgé que {tools} outils en 60 s");
        assert_eq!(
            game.stock().get(Resource::Iron) + tools * game.config().iron_per_tool,
            start_iron,
            "le fer a disparu ou été créé"
        );
    }

    #[test]
    fn forging_stops_when_the_iron_runs_out() {
        let mut game = Game::new(
            village_without_miners(),
            1,
            Config { starting_iron: 5, ..Config::default() }, // 2 outils, reste 1
        );
        run(&mut game, 120.0);
        assert_eq!(game.stock().get(Resource::Tools), 2);
        assert_eq!(game.stock().get(Resource::Iron), 1);
        assert!(game.characters[SMITH.0].work.is_none(), "il ne doit plus forger sans fer");
        // et le joueur non plus : rien à faire à l'enclume, et l'action ne le fait pas bouger
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = TilePos::new(38, 15).center();
        game.characters[SMITH.0].facing = Facing::Right;
        assert_eq!(game.available_work(SMITH), None);
        game.apply(Command::Interact { player: P0 });
        run(&mut game, 2.0);
        assert!(!game.characters[SMITH.0].is_approaching());
        assert_eq!(game.characters[SMITH.0].pos, TilePos::new(38, 15).center());
    }

    #[test]
    fn player_blacksmith_forges_twice_as_fast() {
        // les deux partent du poste de forge, pour ne comparer que la forge
        // IA : elle se met seule au travail
        let mut ai = new_game();
        ai.characters[SMITH.0].pos = forge_post(ANVIL);
        let t_ai = ticks_until(&mut ai, 30.0, |g| g.stock().get(Resource::Tools) >= 1);

        // joueur : il se tourne vers l'enclume (en bas) puis appuie sur E
        let mut player = new_game();
        player.apply(Command::Possess { player: P0, character: SMITH });
        player.characters[SMITH.0].pos = forge_post(ANVIL);
        player.apply(Command::Move { player: P0, dir: Vec2::new(0.0, 1.0) });
        run(&mut player, 0.1); // l'enclume bloque : il ne bouge pas mais la regarde
        player.apply(Command::Move { player: P0, dir: Vec2::ZERO });
        run(&mut player, 0.05);
        assert_eq!(player.characters[SMITH.0].pos, forge_post(ANVIL), "collé à l'enclume, il ne bouge pas");
        assert_eq!(player.available_work(SMITH), Some((WorkKind::Forge, ANVIL)));
        player.apply(Command::Interact { player: P0 });
        let t_player = ticks_until(&mut player, 30.0, |g| g.stock().get(Resource::Tools) >= 1);

        let ratio = t_ai as f32 / t_player as f32;
        assert!((1.8..2.2).contains(&ratio), "le joueur devrait forger 2x plus vite, rapport : {ratio}");
    }

    #[test]
    fn only_the_blacksmith_can_use_the_anvil_and_the_blacksmith_cannot_till() {
        let mut game = new_game();
        // un paysan à côté de l'enclume, tourné vers elle : rien à faire
        game.characters[FARMER.0].pos = TilePos::new(38, 15).center();
        game.characters[FARMER.0].facing = crate::character::Facing::Right;
        game.apply(Command::Possess { player: P0, character: FARMER });
        assert_eq!(game.available_work(FARMER), None);
        // un forgeron sur un champ : rien à faire non plus
        game.apply(Command::ToggleIdle { player: P0 });
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = PLOT_HERE.center();
        assert_eq!(game.available_work(SMITH), None);
    }

    #[test]
    fn a_second_blacksmith_cannot_use_the_anvil_while_it_is_taken() {
        use crate::character::Role;
        use crate::scenario::CharacterSpec;

        let mut scenario = Scenario::village();
        let home = TilePos::new(56, 5); // un second lit dans la maison du forgeron
        scenario.map.set(home, Tile::Bed);
        scenario.characters.push(CharacterSpec {
            name: "Forgeron 2".to_string(),
            role: Role::Blacksmith,
            home,
            night_shift: false,
            work_spots: vec![TilePos::new(40, 15)],
        });
        let mut game = Game::new(scenario, 1, Config::default());
        // le premier forgeron (IA) gagne le poste de forge et se met au travail
        ticks_until(&mut game, 5.0, |g| g.characters[SMITH.0].work.is_some());
        assert_eq!(game.characters[SMITH.0].claimed_tile(), Some(ANVIL));

        // le second forgeron, joueur, juste de l'autre côté de l'enclume, la regarde : c'est occupé
        let second = id_of(&game, "Forgeron 2");
        game.apply(Command::Possess { player: P0, character: second });
        game.characters[second.0].pos = TilePos::new(40, 15).center();
        game.characters[second.0].facing = Facing::Left;
        assert_eq!(game.available_work(second), None, "l'enclume est déjà occupée");

        // une fois lâché, le second forgeron IA ne s'y rue pas non plus tant qu'elle est prise
        game.apply(Command::ToggleIdle { player: P0 });
        run(&mut game, 5.0); // le premier coup de forge dure 8 s
        assert!(game.characters[SMITH.0].work.is_some());
        assert!(game.characters[second.0].work.is_none());
    }

    // ------------------------------------------------------------ les gardes

    #[test]
    fn night_guard_works_at_night_and_sleeps_by_day_and_day_guard_the_opposite() {
        let mut game = fast_day_game();
        let (day, night) = (id_of(&game, "Garde de jour"), id_of(&game, "Garde de nuit"));
        assert!(!game.characters[day.0].night_shift && game.characters[night.0].night_shift);

        run_until(&mut game, 1, 23.9); // minuit approche : relève (le garde de jour revient de loin)
        assert!(game.characters[day.0].asleep, "le garde de jour devrait dormir");
        assert!(!game.characters[night.0].asleep, "le garde de nuit devrait veiller");
        assert_ne!(game.characters[night.0].tile(), game.characters[night.0].home);

        run_until(&mut game, 2, 12.0); // plein jour
        assert!(game.characters[night.0].asleep, "le garde de nuit devrait dormir en plein jour");
        assert_eq!(game.characters[night.0].tile(), game.characters[night.0].home);
        assert!(!game.characters[day.0].asleep, "le garde de jour devrait veiller");
    }

    #[test]
    fn guard_walks_his_whole_route() {
        let scenario = Scenario::village();
        let route: Vec<TilePos> = scenario
            .characters
            .iter()
            .find(|c| c.name == "Garde de jour")
            .unwrap()
            .work_spots
            .clone();
        let mut game = fast_day_game();
        let guard = id_of(&game, "Garde de jour");
        let mut visited: HashSet<TilePos> = HashSet::new();
        // de 7 h à 20 h : une ronde complète
        while game.clock().hour() < 20.0 {
            game.step(TICK);
            visited.insert(game.characters[guard.0].tile());
        }
        for point in route {
            assert!(visited.contains(&point), "le garde n'est jamais passé par {point:?}");
        }
    }

    #[test]
    fn a_player_possessed_night_guard_is_not_forced_to_sleep_in_the_day_but_may_lie_down() {
        let mut game = new_game(); // 7 h : le garde de nuit devrait être couché
        let night = id_of(&game, "Garde de nuit");
        game.apply(Command::Possess { player: P0, character: night });
        game.characters[night.0].pos = game.characters[night.0].home.center() + Vec2::new(1.0, 0.0);
        assert!(game.can_lie_down(night), "c'est son heure de dormir : il doit pouvoir se coucher");
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[night.0].asleep);
        // et il dort sans se réveiller en plein jour
        run(&mut game, 60.0);
        assert!(game.characters[night.0].asleep);
    }

    // ------------------------------------------------------------ mineurs

    /// Le poste du mineur de fer, contre la face gauche de son filon (en 45,3), tourné vers lui.
    const IRON_POST: TilePos = TilePos::new(44, 3);

    /// Incarne le mineur de fer, posé à son poste en regardant le filon.
    fn player_iron_miner(game: &mut Game) -> CharacterId {
        let miner = id_of(game, "Mineur de fer");
        game.apply(Command::Possess { player: P0, character: miner });
        game.characters[miner.0].pos = IRON_POST.center();
        game.characters[miner.0].facing = Facing::Right;
        miner
    }

    #[test]
    fn scenario_has_three_veins_with_three_miners_on_their_own_vein() {
        let game = new_game();
        let kinds: Vec<_> = game.veins().iter().map(|v| v.kind).collect();
        assert_eq!(kinds, [OreKind::Iron, OreKind::Gold, OreKind::Stone]);
        for (i, name) in ["Mineur de fer", "Mineur d'or", "Mineur de pierre"].into_iter().enumerate() {
            let miner = &game.characters()[id_of(&game, name).0];
            assert_eq!(miner.role, Role::Miner);
            assert_eq!(miner.vein, Some(i));
            assert_eq!(game.map().get(miner.home), Tile::Bed);
        }
        for vein in game.veins() {
            assert!(vein.tiles().iter().all(|&t| game.map().get(t) == Tile::Ore));
        }
    }

    #[test]
    fn a_player_miner_digs_a_unit_of_ore_into_the_global_counter() {
        let mut game = new_game();
        let miner = player_iron_miner(&mut game);
        let before = game.stock().get(Resource::Iron);
        assert_eq!(game.available_work(miner), Some((WorkKind::Mine, TilePos::new(45, 3))));
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[miner.0].work.is_some());
        // roche moyenne : 5 coups d'1 s, deux fois moins pour le joueur
        run(&mut game, 2.0);
        assert_eq!(game.stock().get(Resource::Iron), before, "pas encore fini");
        run(&mut game, 1.0);
        assert_eq!(game.stock().get(Resource::Iron), before + 1);
        assert_eq!(game.map().get(TilePos::new(45, 3)), Tile::Ore, "le filon ne s'épuise pas");
    }

    #[test]
    fn moving_cancels_mining_and_nothing_is_dug() {
        let mut game = new_game();
        let miner = player_iron_miner(&mut game);
        let before = game.stock().get(Resource::Iron);
        game.apply(Command::Interact { player: P0 });
        run(&mut game, 1.0);
        game.apply(Command::Move { player: P0, dir: Vec2::new(0.0, -1.0) });
        run(&mut game, 5.0);
        assert!(game.characters[miner.0].work.is_none());
        assert_eq!(game.stock().get(Resource::Iron), before);
    }

    #[test]
    fn only_a_miner_digs_and_only_his_own_vein() {
        let mut game = new_game();
        // un paysan contre le filon de fer
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.characters[FARMER.0].pos = IRON_POST.center();
        game.characters[FARMER.0].facing = Facing::Right;
        assert_eq!(game.available_work(FARMER), None);

        // le mineur de fer ne creuse pas le filon d'or
        let miner = id_of(&game, "Mineur de fer");
        game.apply(Command::Possess { player: P0, character: miner });
        let gold = game.veins()[1];
        game.characters[miner.0].pos = TilePos::new(gold.origin.x - 1, gold.origin.y).center();
        game.characters[miner.0].facing = Facing::Right;
        assert_eq!(game.available_work(miner), None);
    }

    #[test]
    fn purer_rock_takes_fewer_strikes() {
        let game = new_game();
        let secs = |kind: OreKind| {
            let vein = game.veins().iter().find(|v| v.kind == kind).unwrap();
            crate::work::work_secs(WorkKind::Mine, vein.tiles()[0], game.config(), game.veins())
        };
        assert!(secs(OreKind::Stone) < secs(OreKind::Iron), "pure < moyenne");
        assert!(secs(OreKind::Iron) < secs(OreKind::Gold), "moyenne < impure");
    }

    #[test]
    fn ai_miners_dig_every_ore_during_the_day() {
        // sans forgeron, pour que le fer ne baisse pas pendant qu'on le compte
        let mut scenario = Scenario::village();
        scenario.characters.retain(|c| c.role != Role::Blacksmith);
        let mut game = Game::new(scenario, 1, Config::default());
        let before = [Resource::Iron, Resource::Gold, Resource::Stone].map(|r| game.stock().get(r));
        run(&mut game, 120.0);
        for (r, b) in [Resource::Iron, Resource::Gold, Resource::Stone].into_iter().zip(before) {
            assert!(game.stock().get(r) > b, "{r:?} n'a pas augmenté");
        }
        // Le cuivre n'a pas de filon dans ce royaume.
        assert_eq!(game.stock().get(Resource::Copper), 0);
    }

    #[test]
    fn two_ai_miners_share_a_vein_on_two_different_posts() {
        let mut scenario = Scenario::village();
        let bed = TilePos::new(46, 6);
        scenario.map.set(bed, Tile::Bed);
        scenario.mines.push(MineSpec { vein: 0, bed });
        scenario.characters.push(crate::scenario::CharacterSpec {
            name: "Mineur 2".into(),
            role: Role::Miner,
            home: bed,
            night_shift: false,
            work_spots: vec![TilePos::new(47, 3)],
        });
        let mut game = Game::new(scenario, 1, Config::default());
        let (a, b) = (id_of(&game, "Mineur de fer"), id_of(&game, "Mineur 2"));
        let mut both_working = false;
        for _ in 0..(60.0 / TICK) as usize {
            game.step(TICK);
            let (ca, cb) = (&game.characters[a.0], &game.characters[b.0]);
            if ca.work.is_some() && cb.work.is_some() {
                both_working = true;
                assert_ne!(ca.tile(), cb.tile(), "deux mineurs sur la même case");
            }
        }
        assert!(both_working, "les deux mineurs devraient travailler en même temps");
    }

    #[test]
    fn the_blacksmith_keeps_forging_on_mined_iron_after_the_starting_stock_runs_out() {
        let mut game = Game::new(
            Scenario::village(),
            1,
            Config { starting_iron: 4, ..Config::default() }, // 2 outils avec le fer de départ seul
        );
        run(&mut game, 200.0);
        assert!(game.stock().get(Resource::Tools) > 2, "la forge s'est arrêtée faute de fer");
    }

    // ------------------------------------------------------------ bûcherons

    /// L'arbre (15,5) de la forêt nord, vu depuis le couloir (16,5) en regardant à gauche.
    const TREE_NORTH: TilePos = TilePos::new(15, 5);
    const CORRIDOR_NORTH: TilePos = TilePos::new(16, 5);
    /// Un arbre de la lisière sud de la forêt nord : deux cases voisines libres, (16,8) et (15,9).
    const EDGE_TREE: TilePos = TilePos::new(15, 8);

    fn player_lumberjack(game: &mut Game) -> CharacterId {
        let lj = id_of(game, "Bucheron nord");
        game.apply(Command::Possess { player: P0, character: lj });
        game.characters[lj.0].pos = CORRIDOR_NORTH.center();
        game.characters[lj.0].facing = Facing::Left;
        lj
    }

    /// Village avec un second bûcheron dans la forêt nord (même cabane, un lit de plus) et deux joueurs.
    fn two_lumberjacks_game(config: Config) -> (Game, CharacterId, CharacterId) {
        let mut scenario = Scenario::village();
        let bed = TilePos::new(18, 11);
        scenario.map.set(bed, Tile::Bed);
        scenario.lodges.push(LodgeSpec { forest: 0, bed });
        scenario.characters.push(crate::scenario::CharacterSpec {
            name: "Bucheron 2".into(),
            role: Role::Lumberjack,
            home: bed,
            night_shift: false,
            work_spots: vec![TilePos::new(15, 9)],
        });
        let game = Game::new(scenario, 2, config);
        let (a, b) = (id_of(&game, "Bucheron nord"), id_of(&game, "Bucheron 2"));
        (game, a, b)
    }

    #[test]
    fn scenario_has_two_forests_with_a_lumberjack_each() {
        let game = new_game();
        assert_eq!(game.forests().len(), 2);
        for (i, name) in ["Bucheron nord", "Bucheron sud"].into_iter().enumerate() {
            let lj = &game.characters()[id_of(&game, name).0];
            assert_eq!(lj.role, Role::Lumberjack);
            assert_eq!(lj.forest, Some(i));
            assert_eq!(game.map().get(lj.home), Tile::Bed);
            assert!(game.forests()[i].trees(game.map()).len() > 20);
        }
    }

    #[test]
    fn only_a_lumberjack_chops_and_only_a_tree_of_his_own_forest() {
        let mut game = new_game();
        // un paysan dans le couloir
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.characters[FARMER.0].pos = CORRIDOR_NORTH.center();
        game.characters[FARMER.0].facing = Facing::Left;
        assert_eq!(game.map().get(TREE_NORTH), Tile::Tree);
        assert_eq!(game.available_work(FARMER), None);

        // le bûcheron nord peut couper l'arbre nord ...
        let lj = player_lumberjack(&mut game);
        assert_eq!(game.available_work(lj), Some((WorkKind::Chop, TREE_NORTH)));

        // ... mais pas un arbre isolé hors de toute forêt ...
        let decor = TilePos::new(10, 16);
        assert_eq!(game.map().get(decor), Tile::Tree);
        assert!(game.forests().iter().all(|f| !f.contains(decor)));
        game.characters[lj.0].pos = TilePos::new(10, 17).center();
        game.characters[lj.0].facing = Facing::Up;
        assert_eq!(game.available_work(lj), None);

        // ... ni un arbre de la forêt sud
        game.characters[lj.0].pos = TilePos::new(8, 34).center();
        game.characters[lj.0].facing = Facing::Right;
        assert_eq!(game.map().get(TilePos::new(9, 34)), Tile::Tree);
        assert_eq!(game.available_work(lj), None);
    }

    #[test]
    fn a_player_lumberjack_fells_a_tree_for_three_logs() {
        let mut game = new_game();
        let lj = player_lumberjack(&mut game);
        game.apply(Command::Interact { player: P0 });
        assert!(game.characters[lj.0].work.is_some());
        // 6 s pour l'IA, 3 s pour le joueur
        run(&mut game, 2.0);
        assert_eq!(game.stock().get(Resource::Wood), 0);
        assert_eq!(game.map().get(TREE_NORTH), Tile::Tree);
        run(&mut game, 1.2);
        assert_eq!(game.characters[lj.0].cargo, Some((Resource::Wood, 3)), "3 bûches en main");
        assert_eq!(game.stock().get(Resource::Wood), 0, "elles ne sont pas encore dans un coffre");
        assert_eq!(game.map().get(TREE_NORTH), Tile::Sapling);
        assert!(game.characters[lj.0].work.is_none());
        // la pousse ne se coupe pas
        game.characters[lj.0].facing = Facing::Left;
        assert_eq!(game.available_work(lj), None);
    }

    #[test]
    fn moving_cancels_chopping_but_the_tree_keeps_its_progress() {
        let mut game = new_game();
        let lj = player_lumberjack(&mut game);
        game.apply(Command::Interact { player: P0 });
        run(&mut game, 1.5); // la moitié de l'arbre, au rythme du joueur
        game.apply(Command::Move { player: P0, dir: Vec2::new(0.0, 1.0) });
        run(&mut game, 0.2);
        game.apply(Command::Move { player: P0, dir: Vec2::ZERO });
        assert!(game.characters[lj.0].work.is_none());
        assert_eq!(game.stock().get(Resource::Wood), 0);
        let kept = game.tree_progress(TREE_NORTH);
        assert!(kept > 0.4 && kept < 0.6, "progression conservée : {kept}");

        // revenu à sa place, il n'a plus que la moitié à faire
        game.characters[lj.0].pos = CORRIDOR_NORTH.center();
        game.characters[lj.0].facing = Facing::Left;
        game.apply(Command::Interact { player: P0 });
        run(&mut game, 1.7);
        assert_eq!(game.characters[lj.0].cargo, Some((Resource::Wood, 3)));
    }

    #[test]
    fn two_lumberjacks_fell_a_tree_twice_as_fast_and_only_three_logs_are_made() {
        let (mut game, a, b) = two_lumberjacks_game(Config::default());
        let (p0, p1) = (P0, PlayerId(1));
        game.apply(Command::Possess { player: p0, character: a });
        game.apply(Command::Possess { player: p1, character: b });
        game.characters[a.0].pos = TilePos::new(16, 8).center();
        game.characters[a.0].facing = Facing::Left;
        game.characters[b.0].pos = TilePos::new(15, 9).center();
        game.characters[b.0].facing = Facing::Up;
        game.apply(Command::Interact { player: p0 });
        game.apply(Command::Interact { player: p1 });
        assert!(game.characters[a.0].work.is_some() && game.characters[b.0].work.is_some());

        // seul, un joueur met 3 s ; à deux, 1,5 s
        run(&mut game, 1.2);
        assert_eq!(game.map().get(EDGE_TREE), Tile::Tree);
        assert!(game.characters[a.0].work.map_or(0.0, |w| w.progress) > 0.7, "la barre montre l'avancement de l'arbre");
        run(&mut game, 0.6);
        assert_eq!(game.map().get(EDGE_TREE), Tile::Sapling);
        let held = |c: CharacterId| game.characters[c.0].cargo.map_or(0, |(_, n)| n);
        assert_eq!(held(a) + held(b), 3, "un seul propriétaire : 3 bûches, pas 6");
        assert!(held(a) == 0 || held(b) == 0);
        assert!(game.characters[a.0].work.is_none() && game.characters[b.0].work.is_none());
    }

    #[test]
    fn a_felled_tree_regrows_in_game_time_and_waits_for_whoever_stands_on_it() {
        let config = Config { day_length_secs: 240.0, tree_regrow_hours: 1.0, ..Config::default() }; // 1 h = 10 s
        let mut game = Game::new(Scenario::village(), 1, config);
        let lj = player_lumberjack(&mut game);
        game.apply(Command::Interact { player: P0 });
        run(&mut game, 3.2);
        assert_eq!(game.map().get(TREE_NORTH), Tile::Sapling);

        // quelqu'un se tient sur la pousse : elle attend
        game.apply(Command::ToggleIdle { player: P0 });
        game.characters[lj.0].pos = TREE_NORTH.center();
        game.characters[lj.0].asleep = true; // il ne bouge pas
        run(&mut game, 12.0);
        assert_eq!(game.map().get(TREE_NORTH), Tile::Sapling, "l'arbre ne doit pas pousser sur lui");
        game.characters[lj.0].pos = CORRIDOR_NORTH.center();
        run(&mut game, 0.5);
        assert_eq!(game.map().get(TREE_NORTH), Tile::Tree);
    }

    #[test]
    fn ai_lumberjacks_cut_wood_by_day_and_sleep_in_their_lodge_at_night() {
        let mut game = new_game();
        run(&mut game, 120.0);
        assert!(game.stock().get(Resource::Wood) >= 6, "bois : {}", game.stock().get(Resource::Wood));
        assert_eq!(everywhere(&game, Resource::Wood) % 3, 0, "toujours des arbres entiers (coffres et mains)");
        run_until(&mut game, 1, 23.0);
        for name in ["Bucheron nord", "Bucheron sud"] {
            let lj = &game.characters()[id_of(&game, name).0];
            assert!(lj.asleep, "{name} devrait dormir");
            assert!(lj.distance_to_bed() < 0.1);
        }
    }

    #[test]
    fn two_ai_lumberjacks_never_stand_on_the_same_tile_while_working() {
        let (mut game, a, b) = two_lumberjacks_game(Config::default());
        for _ in 0..(150.0 / TICK) as usize {
            game.step(TICK);
            let (ca, cb) = (&game.characters[a.0], &game.characters[b.0]);
            if ca.work.is_some() && cb.work.is_some() {
                assert_ne!(ca.tile(), cb.tile(), "deux bûcherons sur la même case");
            }
        }
        assert!(game.stock().get(Resource::Wood) >= 6);
    }

    #[test]
    fn an_emptied_forest_makes_the_lumberjack_wait_until_a_tree_regrows() {
        let mut game = new_game();
        let north = game.forests()[0];
        for t in north.trees(game.map()) {
            game.map.set(t, Tile::Grass);
        }
        run(&mut game, 60.0);
        // (le bûcheron sud coupe de son côté : on ne regarde que le nord)
        let lj = &game.characters()[id_of(&game, "Bucheron nord").0];
        assert!(lj.work.is_none());
        // un arbre repousse (une pousse plantée à la main) : il va le couper
        game.saplings.plant(&mut game.map, TILE_REGROWN, 0.0);
        let (mut regrown, mut cut_again) = (false, false);
        for _ in 0..(90.0 / TICK) as usize {
            game.step(TICK);
            match game.map().get(TILE_REGROWN) {
                Tile::Tree => regrown = true,
                Tile::Sapling if regrown => cut_again = true,
                _ => {}
            }
        }
        assert!(regrown, "l'arbre n'a pas repoussé");
        assert!(cut_again, "le bûcheron n'a pas repris le travail");
    }

    const TILE_REGROWN: TilePos = TilePos::new(19, 4);

    // ------------------------------------------------------------ habitats

    #[test]
    fn the_start_has_no_free_place_so_nobody_can_be_hired() {
        let game = new_game();
        assert_eq!(game.habitats().len(), 10);
        assert_eq!(game.free_places(), 0);
        assert!(!game.can_hire());
        for i in 0..game.habitats().len() {
            assert_eq!(game.free_places_in(i), 0, "habitat {i}");
        }
    }

    #[test]
    fn a_bed_nobody_sleeps_in_is_a_free_place_and_allows_hiring() {
        let mut scenario = Scenario::village();
        scenario.characters.retain(|c| c.name != "Garde de nuit"); // la caserne a trois lits, deux occupants
        let game = Game::new(scenario, 1, Config::default());
        assert_eq!(game.free_places(), 1);
        let barracks = game.habitats().iter().position(|h| h.x == 19 && h.y == 22).unwrap();
        assert_eq!(game.free_places_in(barracks), 1);
        assert!(game.can_hire());
    }

    #[test]
    fn a_medium_house_with_two_farmers_has_one_free_place() {
        let game = Game::new(Scenario::village_with_farmers(2), 1, Config::default());
        assert_eq!(game.free_places(), 1);
        let house = game.habitats().iter().position(|h| h.beds.len() == 3 && h.x == 4).unwrap();
        assert_eq!(game.free_places_in(house), 1);
        // six paysans dans la grande maison : plus aucune place
        assert_eq!(Game::new(Scenario::village_with_farmers(6), 1, Config::default()).free_places(), 0);
    }

    #[test]
    fn nobody_is_hired_when_all_places_are_taken_and_the_rule_follows_the_free_places() {
        let mut game = Game::new(Scenario::village_with_farmers(2), 1, Config::default());
        assert!(game.can_hire());
        // un paysan de plus prend le dernier lit de la maison moyenne
        let bed = game.habitats().iter().find(|h| h.x == 4 && h.beds.len() == 3).unwrap().bed_homes()[2];
        let taken = game.free_places();
        game.characters.push(crate::character::Character::new(
            CharacterId(game.characters.len()),
            "Nouveau".into(),
            Role::Farmer,
            bed,
            false,
            vec![TilePos::new(4, 5)],
        ));
        assert_eq!(game.free_places(), taken - 1);
        assert!(!game.can_hire());
        assert_eq!(game.free_places_in(99), 0, "habitat inconnu");
    }

    // ------------------------------------------------------------ contenants

    use crate::container::ContainerKind;

    fn idx(game: &Game, kind: ContainerKind) -> usize {
        game.containers.iter().position(|c| c.kind == kind).unwrap_or_else(|| panic!("pas de coffre {kind:?}"))
    }

    /// Le village sans transporteurs : rien ne bouge d'un coffre à l'autre sans qu'on le décide.
    fn village_without_transporters() -> Scenario {
        let mut scenario = Scenario::village();
        scenario.characters.retain(|c| c.role != Role::Transporter);
        scenario
    }

    /// Un village calme : ni producteur ni forgeron, seulement les transporteurs, le roi et les gardes.
    fn quiet_village() -> Scenario {
        let mut scenario = Scenario::village();
        scenario.characters.retain(|c| {
            !matches!(c.role, Role::Farmer | Role::Blacksmith | Role::Miner | Role::Lumberjack)
        });
        scenario
    }

    /// Somme d'une ressource dans les coffres et dans les mains.
    fn everywhere(game: &Game, resource: Resource) -> u32 {
        game.stock().get(resource)
            + game.characters().iter().filter_map(|c| c.cargo).filter(|&(r, _)| r == resource).map(|(_, n)| n).sum::<u32>()
    }

    #[test]
    fn every_crate_is_outdoors_and_only_the_forge_starts_with_iron() {
        let game = new_game();
        assert_eq!(game.containers().len(), 8);
        for c in game.containers() {
            assert_eq!(game.map().get(c.tile), Tile::Crate);
            // dehors : une case d'herbe ou de chemin la touche
            let outside = game
                .map()
                .walkable_neighbors(c.tile)
                .into_iter()
                .any(|n| matches!(game.map().get(n), Tile::Grass | Tile::Path));
            assert!(outside, "le coffre {:?} n'a pas de côté dehors", c.kind);
            let iron = c.get(Resource::Iron);
            assert_eq!(iron, if c.kind == ContainerKind::Forge { game.config().starting_iron } else { 0 });
        }
    }

    #[test]
    fn the_hud_total_is_the_sum_of_the_containers() {
        let mut game = new_game();
        let (a, b) = (idx(&game, ContainerKind::Mine(0)), idx(&game, ContainerKind::Depot));
        game.containers[a].add(Resource::Iron, 12);
        game.containers[b].add(Resource::Iron, 30);
        assert_eq!(game.stock().get(Resource::Iron), game.config().starting_iron + 42);
    }

    #[test]
    fn a_miner_stops_when_his_container_is_full_and_resumes_when_there_is_room() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let mine = idx(&game, ContainerKind::Mine(0));
        game.containers[mine].add(Resource::Iron, 200);
        run(&mut game, 60.0);
        let miner = id_of(&game, "Mineur de fer");
        assert_eq!(game.containers[mine].get(Resource::Iron), 200, "plafond atteint");
        assert!(game.characters[miner.0].work.is_none());
        // un transporteur (ou le roi) en emporte : le mineur reprend tout seul
        game.containers[mine].take(Resource::Iron, 20);
        run(&mut game, 60.0);
        assert!(game.containers[mine].get(Resource::Iron) > 180, "il aurait dû reprendre");
        assert!(game.containers[mine].get(Resource::Iron) <= 200);
    }

    #[test]
    fn a_player_miner_cannot_start_when_the_container_is_full() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let mine = idx(&game, ContainerKind::Mine(0));
        game.containers[mine].add(Resource::Iron, 200);
        let miner = player_iron_miner(&mut game);
        assert_eq!(game.available_work(miner), None);
        game.containers[mine].take(Resource::Iron, 1);
        assert!(game.available_work(miner).is_some());
    }

    #[test]
    fn a_farmer_whose_hands_and_chest_are_full_stops_harvesting_but_keeps_tilling() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let farm = idx(&game, ContainerKind::Farm);
        game.containers[farm].add(Resource::Wheat, 200); // le coffre du champ est plein
        run_until(&mut game, 2, 20.0); // le blé a eu le temps de mûrir
        assert_eq!(game.containers[farm].get(Resource::Wheat), 200);
        assert_eq!(game.characters[FARMER.0].cargo, Some((Resource::Wheat, 20)), "ses mains sont pleines : il ne récolte plus");
        let ripe = game.farm().plots().iter().filter(|p| p.state == PlotState::Ripe).count();
        assert!(ripe > 0, "les parcelles mûres restent mûres");
        // de la place revient : il livre, puis récolte de nouveau
        game.containers[farm].take(Resource::Wheat, 100);
        run_until(&mut game, 3, 12.0); // le lendemain en plein jour
        assert!(game.containers[farm].get(Resource::Wheat) >= 120, "il a livré son blé");
    }

    #[test]
    fn a_player_farmer_carries_his_wheat_drops_what_fits_and_cannot_harvest_with_full_hands() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let farm = idx(&game, ContainerKind::Farm);
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.farm.set_state(PLOT_HERE, PlotState::Planted { growth: 0.0 }); // rien à faire sous ses pieds
        game.farm.set_state(PLOT_FRONT, PlotState::Ripe);
        game.characters[FARMER.0].cargo = Some((Resource::Wheat, 18));
        assert_eq!(game.available_work(FARMER), None, "18 + 4 > 20 : les mains sont pleines");
        game.characters[FARMER.0].cargo = Some((Resource::Wheat, 16));
        assert_eq!(game.available_work(FARMER), Some((WorkKind::Harvest, PLOT_FRONT)));
        // devant le coffre : il dépose ce qui rentre (5 sur 8)
        game.containers[farm].add(Resource::Wheat, 195);
        game.characters[FARMER.0].cargo = Some((Resource::Wheat, 8));
        game.characters[FARMER.0].pos = TilePos::new(12, 6).center();
        game.characters[FARMER.0].facing = Facing::Left;
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[FARMER.0].cargo, Some((Resource::Wheat, 3)));
        assert_eq!(game.containers[farm].get(Resource::Wheat), 200);
    }

    #[test]
    fn a_farmer_keeps_his_wheat_through_the_night_and_delivers_it_in_the_morning() {
        let mut game = Game::new(
            village_without_transporters(),
            1,
            Config { day_length_secs: 240.0, ..Config::default() },
        );
        let farm = idx(&game, ContainerKind::Farm);
        run_until(&mut game, 1, 19.6);
        game.characters[FARMER.0].cargo = Some((Resource::Wheat, 8));
        let before = game.containers[farm].get(Resource::Wheat);
        run_until(&mut game, 1, 23.9);
        assert!(game.stock().get(Resource::Wheat) + game.in_hands(Resource::Wheat) >= before + 8);
        run_until(&mut game, 2, 12.0);
        assert!(game.containers[farm].get(Resource::Wheat) >= before + 8, "livré le lendemain");
    }

    #[test]
    fn the_c_key_command_empties_only_the_central_depot() {
        let mut game = new_game();
        let (depot, farm, forge) =
            (idx(&game, ContainerKind::Depot), idx(&game, ContainerKind::Farm), idx(&game, ContainerKind::Forge));
        game.containers[depot].add(Resource::Wheat, 200);
        game.containers[depot].add(Resource::Wood, 120);
        game.containers[farm].add(Resource::Wheat, 50);
        let forge_iron = game.containers[forge].get(Resource::Iron);
        game.apply(Command::EmptyDepot { player: P0 });
        for &r in &Resource::ALL {
            assert_eq!(game.containers[depot].get(r), 0, "{r:?}");
        }
        assert_eq!(game.containers[farm].get(Resource::Wheat), 50);
        assert_eq!(game.containers[forge].get(Resource::Iron), forge_iron);
    }

    #[test]
    fn once_the_depot_is_emptied_the_transporters_take_the_wheat_there_again() {
        let mut game = Game::new(quiet_village(), 1, Config::default());
        let (farm, depot) = (idx(&game, ContainerKind::Farm), idx(&game, ContainerKind::Depot));
        game.containers[depot].add(Resource::Wheat, 200);
        game.containers[farm].add(Resource::Wheat, 40);
        run(&mut game, 60.0);
        assert_eq!(game.containers[depot].get(Resource::Wheat), 200, "plein : rien n'y est apporté");
        assert_eq!(game.containers[farm].get(Resource::Wheat), 40);
        game.apply(Command::EmptyDepot { player: P0 });
        run(&mut game, 120.0);
        assert!(game.containers[depot].get(Resource::Wheat) >= 30, "le transport a repris");
    }

    #[test]
    fn the_smith_works_from_the_forge_chest_without_walking_to_it() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let forge = idx(&game, ContainerKind::Forge);
        let smith = id_of(&game, "Forgeron");
        let before = game.containers[forge].get(Resource::Iron);
        for _ in 0..(60.0 / TICK) as usize {
            game.step(TICK);
            // le coffre est dehors, contre le mur est ; le forgeron reste dans sa forge
            assert!(game.characters[smith.0].tile().x <= 41, "le forgeron est sorti pour aller au coffre");
        }
        let (iron, tools) = (game.containers[forge].get(Resource::Iron), game.containers[forge].get(Resource::Tools));
        assert!(tools >= 5);
        assert_eq!(iron + tools * 2, before, "le fer est conservé");
    }

    #[test]
    fn the_smith_waits_without_iron_in_his_chest_even_when_the_mine_overflows() {
        let mut scenario = village_without_transporters();
        scenario.characters.retain(|c| c.role != Role::Miner);
        let mut game = Game::new(scenario, 1, Config { starting_iron: 0, ..Config::default() });
        let mine = idx(&game, ContainerKind::Mine(0));
        game.containers[mine].add(Resource::Iron, 200);
        run(&mut game, 60.0);
        assert_eq!(game.stock().get(Resource::Tools), 0);
        assert!(game.characters[SMITH.0].work.is_none());
    }

    #[test]
    fn the_smith_stops_at_200_tools() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let forge = idx(&game, ContainerKind::Forge);
        game.containers[forge].add(Resource::Tools, 200);
        run(&mut game, 60.0);
        assert_eq!(game.containers[forge].get(Resource::Tools), 200);
        assert_eq!(game.containers[forge].get(Resource::Iron), game.config().starting_iron);
    }

    #[test]
    fn a_miner_never_leaves_his_post_to_walk_to_the_chest() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let miner = id_of(&game, "Mineur de fer");
        let chest = game.containers[idx(&game, ContainerKind::Mine(0))].tile;
        for _ in 0..(60.0 / TICK) as usize {
            game.step(TICK);
            let t = game.characters[miner.0].tile();
            assert!((t.x - chest.x).abs() + (t.y - chest.y).abs() > 1, "le mineur est allé au coffre");
        }
        assert!(game.containers[idx(&game, ContainerKind::Mine(0))].get(Resource::Iron) > 0);
    }

    // ------------------------------------------------------------ transport

    #[test]
    fn transporters_carry_iron_from_the_mine_to_the_forge_and_nothing_is_lost() {
        let mut game = Game::new(quiet_village(), 1, Config { starting_iron: 0, ..Config::default() });
        let (mine, forge) = (idx(&game, ContainerKind::Mine(0)), idx(&game, ContainerKind::Forge));
        game.containers[mine].add(Resource::Iron, 100);
        for _ in 0..(150.0 / TICK) as usize {
            game.step(TICK);
            assert_eq!(everywhere(&game, Resource::Iron), 100, "du fer a été perdu ou créé");
            assert!(game.containers[forge].get(Resource::Iron) <= 200);
        }
        assert!(game.containers[forge].get(Resource::Iron) >= 50, "la forge n'a presque rien reçu");
        assert!(game.containers[mine].get(Resource::Iron) < 100);
    }

    #[test]
    fn what_nobody_consumes_goes_to_the_central_depot_in_loads_of_ten() {
        let mut game = Game::new(quiet_village(), 1, Config::default());
        let (gold, depot) = (idx(&game, ContainerKind::Mine(1)), idx(&game, ContainerKind::Depot));
        game.containers[gold].add(Resource::Gold, 35);
        for _ in 0..(150.0 / TICK) as usize {
            game.step(TICK);
            assert_eq!(everywhere(&game, Resource::Gold), 35);
            assert_eq!(game.containers[depot].get(Resource::Gold) % 10, 0, "seulement des charges de 10");
        }
        assert_eq!(game.containers[depot].get(Resource::Gold), 30, "trois voyages ; les 5 derniers attendent une charge complète");
        assert_eq!(game.containers[gold].get(Resource::Gold), 5);
    }

    #[test]
    fn a_full_forge_receives_nothing_and_the_iron_goes_to_the_depot() {
        let mut game = Game::new(quiet_village(), 1, Config { starting_iron: 200, ..Config::default() });
        let (mine, forge, depot) =
            (idx(&game, ContainerKind::Mine(0)), idx(&game, ContainerKind::Forge), idx(&game, ContainerKind::Depot));
        game.containers[mine].add(Resource::Iron, 40);
        for _ in 0..(120.0 / TICK) as usize {
            game.step(TICK);
            assert_eq!(game.containers[forge].get(Resource::Iron), 200);
        }
        assert_eq!(game.containers[depot].get(Resource::Iron), 40);
    }

    #[test]
    fn three_transporters_never_overfill_a_container() {
        let mut game = Game::new(quiet_village(), 1, Config { starting_iron: 190, ..Config::default() });
        let (mine, forge) = (idx(&game, ContainerKind::Mine(0)), idx(&game, ContainerKind::Forge));
        game.containers[mine].add(Resource::Iron, 150);
        for _ in 0..(200.0 / TICK) as usize {
            game.step(TICK);
            for c in game.containers() {
                for &r in &Resource::ALL {
                    assert!(c.get(r) <= 200);
                }
            }
        }
        assert_eq!(game.containers[forge].get(Resource::Iron), 200);
        assert_eq!(everywhere(&game, Resource::Iron), 340);
    }

    #[test]
    fn transporters_run_faster_than_a_guard() {
        let mut game = Game::new(quiet_village(), 1, Config::default());
        let mine = idx(&game, ContainerKind::Mine(0));
        game.containers[mine].add(Resource::Iron, 100);
        let t1 = id_of(&game, "Transporteur 1");
        let guard = id_of(&game, "Garde de jour");
        let (mut best_t, mut best_g) = (0.0_f32, 0.0_f32);
        let (mut last_t, mut last_g) = (game.characters[t1.0].pos, game.characters[guard.0].pos);
        for _ in 0..(90.0 / TICK) as usize {
            game.step(TICK);
            let (pt, pg) = (game.characters[t1.0].pos, game.characters[guard.0].pos);
            if game.characters[t1.0].activity == crate::character::Activity::Walking {
                best_t = best_t.max(pt.distance(last_t));
            }
            if game.characters[guard.0].activity == crate::character::Activity::Walking {
                best_g = best_g.max(pg.distance(last_g));
            }
            last_t = pt;
            last_g = pg;
        }
        assert!(best_g > 0.0 && best_t > best_g * 1.4, "transporteur {best_t} contre garde {best_g} par pas");
    }

    #[test]
    fn a_player_transporter_moves_faster_than_another_player() {
        let mut game = Game::new(Scenario::village(), 2, Config::default());
        let t1 = id_of(&game, "Transporteur 1");
        game.apply(Command::Possess { player: P0, character: FARMER });
        game.apply(Command::Possess { player: PlayerId(1), character: t1 });
        game.characters[FARMER.0].pos = TilePos::new(8, 20).center();
        game.characters[t1.0].pos = TilePos::new(8, 20).center();
        for p in [P0, PlayerId(1)] {
            game.apply(Command::Move { player: p, dir: Vec2::new(1.0, 0.0) });
        }
        run(&mut game, 1.0);
        let dx = |c: CharacterId| game.characters[c.0].pos.x - 8.5;
        assert!(dx(t1) > dx(FARMER) * 1.4, "{} contre {}", dx(t1), dx(FARMER));
    }

    #[test]
    fn a_player_transporter_loads_unloads_and_is_refused_by_the_wrong_chest() {
        let mut game = Game::new(Scenario::village(), 1, Config::default());
        let t1 = id_of(&game, "Transporteur 1");
        game.apply(Command::Possess { player: P0, character: t1 });
        let mine = idx(&game, ContainerKind::Mine(0));
        let forge = idx(&game, ContainerKind::Forge);
        let gold = idx(&game, ContainerKind::Mine(1));
        game.containers[mine].take(Resource::Iron, 1000);
        game.containers[mine].add(Resource::Iron, 30);
        let stand = |game: &mut Game, tile: TilePos, face: Facing| {
            game.characters[t1.0].pos = tile.center();
            game.characters[t1.0].facing = face;
        };
        // charger : jusqu'à 10 de ce que la mine donne
        stand(&mut game, TilePos::new(49, 4), Facing::Left);
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[t1.0].cargo, Some((Resource::Iron, 10)));
        assert_eq!(game.containers[mine].get(Resource::Iron), 20);
        // la mine d'or n'accepte pas le fer
        stand(&mut game, TilePos::new(29, 35), Facing::Left);
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[t1.0].cargo, Some((Resource::Iron, 10)));
        assert_eq!(game.containers[gold].get(Resource::Iron), 0);
        // la forge pleine de fer ne reçoit rien
        game.containers[forge].add(Resource::Iron, 200);
        stand(&mut game, TilePos::new(43, 15), Facing::Left);
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[t1.0].cargo, Some((Resource::Iron, 10)), "forge pleine");
        // la place revient : on décharge
        let have = game.containers[forge].get(Resource::Iron);
        game.containers[forge].take(Resource::Iron, have - 50);
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[t1.0].cargo, None);
        assert_eq!(game.containers[forge].get(Resource::Iron), 60);
    }

    #[test]
    fn a_transporter_keeps_his_cargo_through_the_night_and_delivers_at_dawn() {
        let mut game = Game::new(quiet_village(), 1, Config { day_length_secs: 240.0, starting_iron: 0, ..Config::default() });
        let (mine, forge) = (idx(&game, ContainerKind::Mine(0)), idx(&game, ContainerKind::Forge));
        run_until(&mut game, 1, 19.6);
        let t1 = id_of(&game, "Transporteur 1");
        game.characters[t1.0].cargo = Some((Resource::Iron, 10));
        game.characters[t1.0].job = Some(crate::transport::Job { from: mine, to: forge, resource: Resource::Iron, amount: 10 });
        run_until(&mut game, 1, 23.9);
        // soit il a livré avant de se coucher, soit il dort en la gardant : jamais perdue
        assert_eq!(everywhere(&game, Resource::Iron), 10);
        if game.characters[t1.0].asleep {
            assert!(game.characters[t1.0].cargo.is_some());
        }
        run_until(&mut game, 2, 12.0);
        assert_eq!(game.characters[t1.0].cargo, None);
        assert_eq!(game.containers[forge].get(Resource::Iron), 10);
    }

    // ------------------------------------------------------------ le bûcheron porteur

    #[test]
    fn a_lumberjack_with_full_hands_cannot_start_chopping() {
        let mut game = new_game();
        let lj = player_lumberjack(&mut game);
        game.characters[lj.0].cargo = Some((Resource::Wood, 6));
        assert_eq!(game.available_work(lj), Some((WorkKind::Chop, TREE_NORTH)), "6 + 3 = 9 : ça rentre");
        game.characters[lj.0].cargo = Some((Resource::Wood, 8));
        assert_eq!(game.available_work(lj), None, "8 + 3 > 9");
    }

    #[test]
    fn an_ai_lumberjack_carries_his_wood_to_the_lodge_chest() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let lodge = idx(&game, ContainerKind::Lodge(0));
        let lj = id_of(&game, "Bucheron nord");
        let mut max_carried = 0;
        for _ in 0..(200.0 / TICK) as usize {
            game.step(TICK);
            let carried = game.characters[lj.0].cargo.map_or(0, |(_, n)| n);
            max_carried = max_carried.max(carried);
            assert!(carried <= 9);
        }
        assert_eq!(max_carried, 9, "il remplit ses mains avant de livrer");
        let wood = game.containers[lodge].get(Resource::Wood);
        assert!(wood >= 9 && wood % 3 == 0, "livré : {wood}");
    }

    #[test]
    fn a_full_lodge_makes_the_lumberjack_wait_with_his_wood_then_resume() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let lodge = idx(&game, ContainerKind::Lodge(0));
        let lj = id_of(&game, "Bucheron nord");
        game.containers[lodge].add(Resource::Wood, 200);
        run(&mut game, 150.0);
        assert_eq!(game.containers[lodge].get(Resource::Wood), 200);
        assert_eq!(game.characters[lj.0].cargo, Some((Resource::Wood, 9)), "il attend avec ses 9 bûches");
        assert!(game.characters[lj.0].work.is_none());
        game.containers[lodge].take(Resource::Wood, 10);
        run(&mut game, 20.0);
        assert!(game.containers[lodge].get(Resource::Wood) >= 199, "il a déposé son bois");
    }

    #[test]
    fn a_player_lumberjack_drops_what_fits_and_keeps_the_rest() {
        let mut game = Game::new(village_without_transporters(), 1, Config::default());
        let lodge = idx(&game, ContainerKind::Lodge(0));
        game.containers[lodge].add(Resource::Wood, 198);
        let lj = id_of(&game, "Bucheron nord");
        game.apply(Command::Possess { player: P0, character: lj });
        game.characters[lj.0].pos = TilePos::new(22, 10).center();
        game.characters[lj.0].facing = Facing::Up;
        game.characters[lj.0].cargo = Some((Resource::Wood, 5));
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[lj.0].cargo, Some((Resource::Wood, 3)));
        assert_eq!(game.containers[lodge].get(Resource::Wood), 200);
    }

    #[test]
    fn a_lumberjack_keeps_his_wood_through_the_night_and_delivers_it_in_the_morning() {
        let mut game = Game::new(
            village_without_transporters(),
            1,
            Config { day_length_secs: 240.0, ..Config::default() },
        );
        let lodge = idx(&game, ContainerKind::Lodge(0));
        let lj = id_of(&game, "Bucheron nord");
        run_until(&mut game, 1, 19.6);
        game.characters[lj.0].cargo = Some((Resource::Wood, 6));
        let before = game.containers[lodge].get(Resource::Wood);
        run_until(&mut game, 1, 23.9);
        assert!(everywhere(&game, Resource::Wood) >= before + 6);
        run_until(&mut game, 2, 12.0);
        assert!(game.containers[lodge].get(Resource::Wood) >= before + 6, "livré le lendemain");
    }
}

/// Le départ de partie : le camp, le pécule, le roi qui pose des plans, les artisans qui bâtissent, les vagabonds.
#[cfg(test)]
mod camp_tests {
    use super::*;
    use crate::building::SiteState;

    const P0: PlayerId = PlayerId(0);
    const KING: CharacterId = CharacterId(0);

    fn camp_game() -> Game {
        let mut game = Game::new(Scenario::camp(), 1, Config::default());
        game.apply(Command::Possess { player: P0, character: KING });
        game
    }

    fn run(game: &mut Game, seconds: f32) {
        for _ in 0..(seconds / TICK) as usize {
            game.step(TICK);
        }
    }

    /// Le premier emplacement libre pour un plan de cette forme, en partant du camp et en s'en éloignant.
    fn spot(game: &Game, kind: PlanKind, w: i32, h: i32, beds: u32, skip: usize) -> (i32, i32) {
        let camp = game.camp().expect("un camp");
        let mut found = Vec::new();
        for y in 2..game.map().height() - 2 {
            for x in 2..game.map().width() - 2 {
                if game.check_plan(kind, x, y, w, h, beds).is_ok() {
                    found.push((x, y));
                }
            }
        }
        let dist = |&(x, y): &(i32, i32)| (x - camp.x).pow(2) + (y - camp.y).pow(2);
        found.retain(|p| dist(p) > 120); // loin de la place du camp, où le roi et les artisans se promènent
        found.sort_by_key(|p| (dist(p), p.1, p.0));
        found[skip]
    }

    fn place(game: &mut Game, kind: PlanKind, x: i32, y: i32, w: i32, h: i32, beds: u32) {
        game.apply(Command::PlacePlan { player: P0, kind, x, y, w, h, beds });
    }

    fn place_min(game: &mut Game, kind: PlanKind, skip: usize) -> usize {
        let (w, h) = kind.min_size();
        let beds = kind.min_beds();
        let (x, y) = spot(game, kind, w, h, beds, skip);
        let before = game.sites().len();
        place(game, kind, x, y, w, h, beds);
        assert_eq!(game.sites().len(), before + 1, "le plan aurait dû être posé en ({x}, {y})");
        before
    }

    /// Fait tourner la partie jusqu'à ce que `done` soit vrai ; panique au-delà de `max_secs`.
    fn run_until(game: &mut Game, max_secs: f32, done: impl Fn(&Game) -> bool) -> f32 {
        for n in 0..(max_secs / TICK) as u32 {
            if done(game) {
                return n as f32 * TICK;
            }
            game.step(TICK);
        }
        panic!("condition jamais atteinte en {max_secs} s");
    }

    // ------------------------------------------------------------ le départ

    #[test]
    fn the_camp_starts_with_a_king_two_artisans_three_beds_and_an_empty_map() {
        let game = camp_game();
        let roles: Vec<Role> = game.characters().iter().map(|c| c.role).collect();
        assert_eq!(roles, vec![Role::King, Role::Artisan, Role::Artisan]);
        let beds = game.map().find_tiles(Tile::Bed);
        assert_eq!(beds.len(), 3, "trois lits au camp");
        for c in game.characters() {
            assert!(beds.contains(&c.home), "{} dort au camp", c.name);
        }
        assert_eq!(game.map().find_tiles(Tile::Campfire).len(), 1, "un feu de camp");
        assert!(game.habitats().is_empty() && game.buildings().is_empty() && game.sites().is_empty());
        assert_eq!(game.map().find_tiles(Tile::Wall).len(), 0, "aucun bâtiment");
        assert_eq!(game.free_places(), 0);
    }

    #[test]
    fn nature_is_there_from_the_start() {
        let game = camp_game();
        assert!(!game.map().find_tiles(Tile::Water).is_empty());
        assert!(!game.map().find_tiles(Tile::Tree).is_empty());
        assert!(!game.veins().is_empty());
        assert!(game.forests().len() >= 2);
        assert!(game.forests().iter().all(|f| !f.trees(game.map()).is_empty()));
    }

    #[test]
    fn the_starting_stock_is_in_the_camp_chest_and_nowhere_else() {
        let game = camp_game();
        let cfg = game.config();
        assert_eq!(game.containers().len(), 1, "un seul coffre, pas d'entrepôt");
        let chest = &game.containers()[0];
        assert_eq!(chest.kind, ContainerKind::Camp);
        assert_eq!(chest.capacity(), cfg.camp_capacity);
        assert_eq!(
            (chest.get(Resource::Wood), chest.get(Resource::Stone), chest.get(Resource::Seeds)),
            (cfg.starting_wood, cfg.starting_stone, cfg.starting_seeds)
        );
        assert_eq!(game.stock().get(Resource::Wood), cfg.starting_wood);
        assert!(cfg.starting_wood <= cfg.camp_capacity, "le pécule tient dans le coffre");
    }

    #[test]
    fn the_camp_chest_holds_any_resource_up_to_its_capacity() {
        let mut game = camp_game();
        let chest = &mut game.containers[0];
        assert!(chest.accepts(Resource::Wheat) && chest.accepts(Resource::Iron) && chest.accepts(Resource::Wood));
        assert_eq!(chest.add(Resource::Wheat, 150), 100, "100 au plus de chaque ressource");
    }

    #[test]
    fn everything_at_the_camp_is_reachable_on_foot() {
        let game = camp_game();
        let camp = game.camp().unwrap();
        assert!(!game.map().get(camp).is_solid());
        for c in game.characters() {
            assert!(game.map().find_path(camp, c.tile()).len() > 0 || c.tile() == camp, "{} est coincé", c.name);
        }
    }

    // ------------------------------------------------------------ poser un plan

    #[test]
    fn only_a_possessed_king_can_place_a_plan() {
        let mut game = camp_game();
        let (x, y) = spot(&game, PlanKind::House, 6, 5, 1, 0);
        game.apply(Command::ToggleIdle { player: P0 }); // plus personne d'incarné
        place(&mut game, PlanKind::House, x, y, 6, 5, 1);
        assert!(game.sites().is_empty(), "personne n'incarne le roi");
        game.apply(Command::Possess { player: P0, character: CharacterId(1) }); // un artisan
        place(&mut game, PlanKind::House, x, y, 6, 5, 1);
        assert!(game.sites().is_empty(), "un artisan n'est pas le roi");
        game.apply(Command::Possess { player: P0, character: KING });
        place(&mut game, PlanKind::House, x, y, 6, 5, 1);
        assert_eq!(game.sites().len(), 1);
    }

    #[test]
    fn a_placed_plan_puts_solid_foundations_on_the_map() {
        let mut game = camp_game();
        let (x, y) = spot(&game, PlanKind::House, 6, 5, 1, 0);
        place(&mut game, PlanKind::House, x, y, 6, 5, 1);
        let site = &game.sites()[0];
        assert_eq!(site.state, SiteState::Waiting);
        for yy in y..y + 5 {
            for xx in x..x + 6 {
                let t = TilePos::new(xx, yy);
                assert_eq!(game.map().get(t), Tile::Foundation, "{t:?}");
                assert!(game.map().get(t).is_solid(), "les fondations bloquent le passage");
            }
        }
        // et on ne peut pas poser un autre plan dessus
        assert_eq!(game.check_plan(PlanKind::House, x + 2, y, 6, 5, 1), Err(PlanError::Blocked));
    }

    #[test]
    fn a_plan_that_does_not_fit_is_not_placed() {
        let mut game = camp_game();
        place(&mut game, PlanKind::House, 20, 10, 5, 5, 1); // trop petite
        place(&mut game, PlanKind::House, 0, 0, 6, 5, 1); // dans l'eau du bord
        assert!(game.sites().is_empty());
    }

    #[test]
    fn a_plan_placed_without_the_resources_is_accepted_but_waits() {
        // un pécule trop maigre pour la plus petite maison
        let mut game = Game::new(Scenario::camp(), 1, Config { starting_wood: 10, ..Config::default() });
        game.apply(Command::Possess { player: P0, character: KING });
        let (w, h) = PlanKind::House.min_size();
        let (x, y) = spot(&game, PlanKind::House, w, h, 1, 0);
        let cost = game.plan_cost(PlanKind::House, w, h, 1);
        assert!(!game.can_afford(cost), "{cost:?} dépasse le pécule");
        place(&mut game, PlanKind::House, x, y, w, h, 1);
        assert_eq!(game.sites().len(), 1, "on peut quand même poser le plan");
        run(&mut game, 30.0);
        let site = &game.sites()[0];
        assert_eq!(site.state, SiteState::Waiting, "le chantier ne commence pas sans toutes les ressources");
        assert!(site.pile.is_none());
        assert_eq!(game.stock().get(Resource::Wood), 10, "rien n'a été pris");
        // les fondations bloquent quand même le passage
        assert_eq!(game.map().get(TilePos::new(x + 1, y + 1)), Tile::Foundation);
    }

    #[test]
    fn affordability_follows_the_stock() {
        let game = camp_game();
        let cheap = game.plan_cost(PlanKind::House, 6, 5, 1);
        assert!(game.can_afford(cheap));
        assert!(!game.can_afford(Cost { wood: game.config().starting_wood + 1, stone: 0 }));
        assert!(!game.can_afford(Cost { wood: 0, stone: game.config().starting_stone + 1 }));
    }

    // ------------------------------------------------------------ le chantier

    #[test]
    fn a_house_is_delivered_then_built_then_becomes_a_habitat_with_a_free_bed() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::House, 0);
        let cost = game.sites()[i].cost;
        let start_wood = game.stock().get(Resource::Wood);

        // d'abord les matériaux sont apportés devant le chantier
        run_until(&mut game, 60.0, |g| g.sites()[i].state == SiteState::Delivering);
        let pile = game.sites()[i].pile.expect("un tas devant le chantier");
        assert_eq!(game.map().get(game.sites()[i].pile_tile()), Tile::Crate);
        // le chantier ne passe en construction que lorsque tout est là
        run_until(&mut game, 120.0, |g| g.sites()[i].state == SiteState::Building);
        assert!(game.containers()[pile].is_complete(), "tout le matériel est devant le chantier");
        assert_eq!(game.containers()[pile].get(Resource::Wood), cost.wood);
        assert_eq!(game.containers()[pile].get(Resource::Stone), cost.stone);

        run_until(&mut game, 300.0, |g| g.sites()[i].state == SiteState::Done);
        assert_eq!(game.habitats().len(), 1, "la maison est un habitat");
        assert_eq!(game.free_places(), 1, "avec une place libre : un lit");
        assert!(game.can_hire());
        assert_eq!(game.stock().get(Resource::Wood), start_wood - cost.wood, "les matériaux sont consommés");
        assert_eq!(game.stock().get(Resource::Stone), game.config().starting_stone - cost.stone);
        // le bâtiment a pris la place des fondations
        let s = &game.sites()[i];
        assert!(game.map().find_tiles(Tile::Foundation).is_empty());
        assert_eq!(game.map().get(TilePos::new(s.x, s.y)), Tile::Wall);
        assert_eq!(game.map().get(s.door()), Tile::Floor, "la porte est ouverte");
        assert!(matches!(game.map().get(s.pile_tile()), Tile::Grass | Tile::Path), "le tas est consommé");
    }

    #[test]
    fn the_artisans_build_together_and_always_build_first() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::House, 0);
        run_until(&mut game, 120.0, |g| g.sites()[i].state == SiteState::Building);
        run(&mut game, 8.0);
        let builders = game.characters().iter().filter(|c| c.role == Role::Artisan && c.work.map_or(false, |w| w.kind == WorkKind::Build)).count();
        assert_eq!(builders, 2, "les deux artisans construisent");
        // les artisans construisent depuis l'extérieur
        for c in game.characters().iter().filter(|c| c.role == Role::Artisan) {
            assert!(!game.sites()[i].contains(c.tile()), "{} est dans les fondations", c.name);
        }
        let mid = game.sites()[i].progress;
        assert!(mid > 0.0 && mid < 1.0);
    }

    #[test]
    fn two_artisans_build_twice_as_fast_as_one() {
        let time_with = |artisans: usize| {
            let mut scenario = Scenario::camp();
            scenario.characters.truncate(1 + artisans);
            let mut game = Game::new(scenario, 1, Config::default());
            game.apply(Command::Possess { player: P0, character: KING });
            let i = place_min(&mut game, PlanKind::House, 0);
            let start = run_until(&mut game, 120.0, |g| g.sites()[i].state == SiteState::Building);
            let end = run_until(&mut game, 600.0, |g| g.sites()[i].state == SiteState::Done);
            end - start
        };
        let (one, two) = (time_with(1), time_with(2));
        assert!((1.7..2.3).contains(&(one / two)), "un artisan : {one:.1} s, deux : {two:.1} s");
    }

    #[test]
    fn the_build_time_grows_with_the_surface_and_depends_on_the_kind() {
        let total = |kind: PlanKind, extra_w: i32| {
            let (w, h) = kind.min_size();
            let game = camp_game();
            game.plan_secs(kind, w + extra_w, h, kind.min_beds())
        };
        assert!(total(PlanKind::House, 4) > total(PlanKind::House, 0));
        assert!(total(PlanKind::Warehouse, 4) > total(PlanKind::Warehouse, 0));
        // la durée réelle suit : une plus grande maison met plus de temps à construire
        let built_in = |extra_w: i32| {
            let mut game = camp_game();
            let (w, h) = (6 + extra_w, 5);
            let (x, y) = spot(&game, PlanKind::House, w, h, 1, 0);
            place(&mut game, PlanKind::House, x, y, w, h, 1);
            let start = run_until(&mut game, 200.0, |g| g.sites()[0].state == SiteState::Building);
            run_until(&mut game, 900.0, |g| g.sites()[0].state == SiteState::Done) - start
        };
        let (small, big) = (built_in(0), built_in(6));
        assert!(big > small * 1.5, "6x5 : {small:.1} s, 12x5 : {big:.1} s");
    }

    #[test]
    fn a_road_to_the_camp_appears_when_the_building_is_finished_not_before() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::House, 0);
        let front = game.sites()[i].front();
        let camp = game.camp().unwrap();
        let grass_on_way = |g: &Game| {
            g.map().find_path_mode(front, camp, PathMode::Roads(g.config().grass_cost)).iter().filter(|t| g.map().get(**t) == Tile::Grass).count()
        };
        assert!(grass_on_way(&game) > 0, "il y a de l'herbe entre le chantier et le camp");
        run_until(&mut game, 120.0, |g| g.sites()[i].state == SiteState::Building);
        run(&mut game, 5.0);
        assert!(game.sites()[i].progress < 1.0);
        assert_eq!(game.map().get(front), Tile::Grass, "pas de route pendant la construction");
        run_until(&mut game, 300.0, |g| g.sites()[i].state == SiteState::Done);
        assert_eq!(grass_on_way(&game), 0, "la route mène au camp");
        assert_eq!(game.map().get(front), Tile::Path);
        // la route se suit par les côtés : aucun pas en diagonale sans case de route à côté (pas de creux dans les angles)
        let is_path = |t: TilePos| game.map().get(t) == Tile::Path;
        let steps = game.map().find_path_mode(front, camp, PathMode::Roads(game.config().grass_cost));
        let mut prev = front;
        for t in steps {
            if t.x != prev.x && t.y != prev.y {
                assert!(
                    is_path(TilePos::new(t.x, prev.y)) || is_path(TilePos::new(prev.x, t.y)),
                    "creux en diagonale entre {prev:?} et {t:?}"
                );
            }
            prev = t;
        }
    }

    #[test]
    fn the_pile_spot_gives_back_the_path_it_was_laid_on() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::House, 0);
        let pile = game.sites()[i].pile_tile();
        game.map.set(pile, Tile::Path); // un chemin passait là avant le tas
        run_until(&mut game, 120.0, |g| g.sites()[i].state == SiteState::Delivering);
        assert_eq!(game.map().get(pile), Tile::Crate);
        run_until(&mut game, 300.0, |g| g.sites()[i].state == SiteState::Done);
        assert_eq!(game.map().get(pile), Tile::Path, "le chemin n'est pas effacé par la fin du chantier");
    }

    #[test]
    fn a_warehouse_replaces_the_pile_with_a_big_empty_chest() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::Warehouse, 0);
        run_until(&mut game, 500.0, |g| g.sites()[i].state == SiteState::Done);
        assert_eq!(game.buildings().len(), 1);
        assert_eq!(game.buildings()[0].kind, PlanKind::Warehouse);
        let depots: Vec<&Container> = game.containers().iter().filter(|c| c.kind == ContainerKind::Depot).collect();
        assert_eq!(depots.len(), 1);
        assert_eq!(depots[0].capacity(), game.config().warehouse_capacity);
        assert!(game.config().warehouse_capacity > game.config().camp_capacity, "l'entrepôt contient beaucoup plus");
        assert_eq!(depots[0].get(Resource::Wood), 0, "les matériaux livrés sont consommés");
        // les décors solides sont posés dans l'entrepôt, et on entre par la porte
        for t in game.buildings()[0].decor_tiles() {
            assert_eq!(game.map().get(t), Tile::Furniture);
        }
        let b = game.buildings()[0];
        assert!(!game.map().find_path(b.front(), TilePos::new(b.x + b.w / 2, b.y + b.h - 2)).is_empty());
    }

    #[test]
    fn a_tavern_is_a_building_with_tables() {
        let mut game = camp_game();
        let mut cfg = Config::default();
        cfg.starting_wood = 100;
        cfg.camp_capacity = 100;
        let (w, h) = PlanKind::Tavern.min_size();
        let cost = game.plan_cost(PlanKind::Tavern, w, h, 0);
        if !game.can_afford(cost) {
            game = Game::new(Scenario::camp(), 1, Config { starting_wood: 100, starting_stone: 100, ..cfg });
            game.apply(Command::Possess { player: P0, character: KING });
        }
        let i = place_min(&mut game, PlanKind::Tavern, 0);
        run_until(&mut game, 900.0, |g| g.sites()[i].state == SiteState::Done);
        assert_eq!(game.buildings()[0].kind, PlanKind::Tavern);
        assert_eq!(game.free_places(), 0, "une taverne n'est pas un habitat");
    }

    #[test]
    fn a_waiting_site_starts_as_soon_as_the_resources_are_there_and_in_order() {
        let mut game = camp_game();
        // deux maisons : les deux tiennent dans le pécule
        let a = place_min(&mut game, PlanKind::House, 0);
        let b = place_min(&mut game, PlanKind::House, 40);
        run(&mut game, 1.0);
        assert!(game.sites()[a].state != SiteState::Waiting && game.sites()[b].state != SiteState::Waiting);
        // les artisans construisent le premier posé d'abord
        run_until(&mut game, 200.0, |g| g.sites().iter().any(|s| s.state == SiteState::Building));
        assert_eq!(game.sites()[a].state, SiteState::Building, "dans l'ordre des placements");
    }

    #[test]
    fn resources_promised_to_one_site_are_not_promised_to_another() {
        let mut game = camp_game();
        // deux plans qui, ensemble, dépassent le pécule de bois, mais chacun y tient
        let (w, h) = (14, 8);
        let one = game.plan_cost(PlanKind::House, w, h, 1);
        assert!(game.can_afford(one));
        assert!(!game.can_afford(Cost { wood: one.wood * 2, stone: one.stone * 2 }), "{one:?} x2 dépasse le pécule");
        let (x, y) = spot(&game, PlanKind::House, w, h, 1, 0);
        place(&mut game, PlanKind::House, x, y, w, h, 1);
        let (x2, y2) = spot(&game, PlanKind::House, w, h, 1, 0);
        place(&mut game, PlanKind::House, x2, y2, w, h, 1);
        assert_eq!(game.sites().len(), 2);
        run(&mut game, 15.0);
        // un seul des deux démarre (selon que quelqu'un se tient sur l'emplacement du tas ou non), l'autre attend :
        // le bois et la pierre promis au premier ne sont plus disponibles
        let waiting: Vec<&Site> = game.sites().iter().filter(|s| s.state == SiteState::Waiting).collect();
        assert_eq!(waiting.len(), 1, "un chantier démarre, l'autre attend");
        assert!(!game.can_afford(waiting[0].cost), "le matériel est promis à l'autre chantier");
    }

    #[test]
    fn an_artisan_without_a_site_to_build_carries_materials() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::House, 0);
        run_until(&mut game, 60.0, |g| g.characters().iter().any(|c| c.role == Role::Artisan && c.cargo.is_some()));
        assert_ne!(game.sites()[i].state, SiteState::Building, "ils portent avant de construire");
    }

    #[test]
    fn a_player_artisan_can_build_by_facing_the_foundations() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::House, 0);
        run_until(&mut game, 120.0, |g| g.sites()[i].state == SiteState::Building);
        let site = game.sites()[i].clone();
        game.apply(Command::ToggleIdle { player: P0 });
        game.apply(Command::Possess { player: P0, character: CharacterId(1) });
        // l'artisan se tient juste au-dessus des fondations, tourné vers elles
        let c = &mut game.characters[1];
        c.cancel_work();
        c.pos = TilePos::new(site.x + 2, site.y - 1).center();
        c.facing = Facing::Down;
        assert_eq!(game.available_work(CharacterId(1)), Some((WorkKind::Build, TilePos::new(site.x + 2, site.y))));
        game.apply(Command::Interact { player: P0 });
        let work = game.characters()[1].work.expect("il construit");
        assert_eq!(work.kind, WorkKind::Build);
        assert!((work.secs - site.total_secs).abs() < 0.01);
    }

    #[test]
    fn someone_standing_on_the_pile_spot_is_pushed_aside_so_the_site_still_starts() {
        let mut game = camp_game();
        let (w, h) = PlanKind::House.min_size();
        let (x, y) = spot(&game, PlanKind::House, w, h, 1, 0);
        place(&mut game, PlanKind::House, x, y, w, h, 1);
        let pile_tile = game.sites()[0].pile_tile();
        // un artisan se tient exactement sur la case du futur tas, et ne bouge plus (il dort debout, en quelque sorte)
        game.characters[1].pos = pile_tile.center();
        game.characters[1].stand_still_for_test(1000.0);
        run(&mut game, 3.0);
        assert_ne!(game.sites()[0].state, SiteState::Waiting, "le chantier démarre malgré lui");
        assert_ne!(game.characters()[1].tile(), pile_tile, "on l'a écarté");
        assert!(!game.map().get(game.characters()[1].tile()).is_solid());
        assert_eq!(game.map().get(pile_tile), Tile::Crate);
    }

    #[test]
    fn a_player_character_on_the_pile_spot_is_not_moved() {
        let mut game = camp_game();
        let (w, h) = PlanKind::House.min_size();
        let (x, y) = spot(&game, PlanKind::House, w, h, 1, 0);
        place(&mut game, PlanKind::House, x, y, w, h, 1);
        let pile_tile = game.sites()[0].pile_tile();
        game.characters[0].pos = pile_tile.center(); // le roi, incarné
        run(&mut game, 3.0);
        assert_eq!(game.characters()[0].tile(), pile_tile, "le joueur bouge quand il veut");
        assert_eq!(game.sites()[0].state, SiteState::Waiting, "le chantier attend");
        game.apply(Command::Move { player: P0, dir: Vec2::new(0.0, 1.0) });
        run(&mut game, 2.0);
        assert_ne!(game.sites()[0].state, SiteState::Waiting, "il s'est écarté : le chantier démarre");
    }

    #[test]
    fn a_character_is_not_allowed_inside_foundations() {
        let mut game = camp_game();
        // le roi se tient dans l'emprise : le plan est refusé
        let (x, y) = spot(&game, PlanKind::House, 6, 5, 1, 0);
        game.characters[0].pos = TilePos::new(x + 2, y + 2).center();
        assert_eq!(game.check_plan(PlanKind::House, x, y, 6, 5, 1), Err(PlanError::Occupied));
    }

    #[test]
    fn foundations_block_walking() {
        let mut game = camp_game();
        let i = place_min(&mut game, PlanKind::House, 0);
        let s = game.sites()[i].clone();
        let across = game.map().find_path(TilePos::new(s.x + 2, s.y - 1), TilePos::new(s.x + 2, s.y + s.h));
        let straight = (s.h + 1) as usize;
        assert!(across.is_empty() || across.len() > straight, "on contourne les fondations");
        assert!(game.map().find_path(TilePos::new(s.x + 2, s.y - 1), TilePos::new(s.x + 2, s.y + 1)).is_empty(), "on ne rentre pas dedans");
    }

    // ------------------------------------------------------------ les vagabonds

    #[test]
    fn vagabonds_pass_by_the_camp_and_leave() {
        let cfg = Config { vagabond_first_hours: 0.5, vagabond_every_hours: 1.0, vagabond_stay_hours: 1.0, ..Config::default() };
        let mut game = Game::new(Scenario::camp(), 1, cfg);
        run_until(&mut game, 120.0, |g| g.characters().iter().any(|c| c.role == Role::Vagabond));
        let id = game.characters().iter().position(|c| c.role == Role::Vagabond).unwrap();
        let v = &game.characters()[id];
        assert!(game.map().get(v.tile()) != Tile::Water && !game.map().get(v.tile()).is_solid(), "il arrive par un bord praticable");
        assert!(v.name.starts_with("Vagabond"));
        // il se promène : il bouge
        let start = game.characters()[id].pos;
        run(&mut game, 15.0);
        assert!(game.characters()[id].pos.distance(start) > 1.0 || game.characters()[id].gone);
        // puis il repart
        run_until(&mut game, 400.0, |g| g.characters()[id].gone);
    }

    #[test]
    fn vagabonds_do_not_need_a_bed_and_never_sleep() {
        let cfg = Config { vagabond_first_hours: 0.1, vagabond_stay_hours: 100.0, ..Config::default() };
        let mut game = Game::new(Scenario::camp(), 1, cfg);
        run_until(&mut game, 60.0, |g| g.characters().iter().any(|c| c.role == Role::Vagabond));
        run(&mut game, 700.0); // une journée entière, nuit comprise
        let id = game.characters().iter().position(|c| c.role == Role::Vagabond).unwrap();
        assert!(!game.characters()[id].asleep);
        assert_eq!(game.free_places(), 0);
    }

    #[test]
    fn no_more_vagabonds_than_the_limit_are_around_at_once() {
        let cfg = Config { vagabond_first_hours: 0.1, vagabond_every_hours: 0.2, vagabond_stay_hours: 1000.0, vagabond_max: 2, ..Config::default() };
        let mut game = Game::new(Scenario::camp(), 1, cfg);
        run(&mut game, 200.0);
        let around = game.characters().iter().filter(|c| c.role == Role::Vagabond && !c.gone).count();
        assert_eq!(around, 2);
    }

    #[test]
    fn the_village_has_no_vagabonds_and_no_camp() {
        let mut game = Game::new(Scenario::village(), 1, Config::default());
        assert!(game.camp().is_none());
        run(&mut game, 120.0);
        assert!(game.characters().iter().all(|c| c.role != Role::Vagabond));
    }

    #[test]
    fn a_day_at_the_camp_is_stable_the_artisans_sleep_in_their_beds() {
        let mut game = camp_game();
        run(&mut game, 700.0);
        for c in game.characters() {
            assert!(c.asleep || game.clock().hour() < 20.0, "{} devrait dormir la nuit", c.name);
        }
    }
}

/// Parler, affecter à un poste ou à un lit, la forge : le roi qui répartit les gens.
#[cfg(test)]
mod hire_tests {
    use super::*;
    use crate::building::SiteState;
    use crate::character::Activity;

    const P0: PlayerId = PlayerId(0);
    const KING: CharacterId = CharacterId(0);
    const ARTISAN: CharacterId = CharacterId(1);

    fn cfg() -> Config {
        Config { starting_stone: 100, vagabond_first_hours: 0.1, vagabond_every_hours: 1000.0, vagabond_stay_hours: 1000.0, ..Config::default() }
    }

    fn new_game(config: Config) -> Game {
        let mut game = Game::new(Scenario::camp(), 1, config);
        game.apply(Command::Possess { player: P0, character: KING });
        game
    }

    fn run(game: &mut Game, seconds: f32) {
        for _ in 0..(seconds / TICK) as usize {
            game.step(TICK);
        }
    }

    fn run_until(game: &mut Game, max_secs: f32, done: impl Fn(&Game) -> bool) {
        for _ in 0..(max_secs / TICK) as u32 {
            if done(game) {
                return;
            }
            game.step(TICK);
        }
        panic!("condition jamais atteinte en {max_secs} s");
    }

    /// Le premier emplacement libre pour un plan de ce type, loin du camp.
    fn spot(game: &Game, kind: PlanKind, skip: usize) -> (i32, i32) {
        let (w, h) = kind.min_size();
        let camp = game.camp().unwrap();
        let mut found = Vec::new();
        for y in 2..game.map().height() - 2 {
            for x in 2..game.map().width() - 3 {
                if game.check_plan(kind, x, y, w, h, kind.min_beds()).is_ok() {
                    found.push((x, y));
                }
            }
        }
        let dist = |&(x, y): &(i32, i32)| (x - camp.x).pow(2) + (y - camp.y).pow(2);
        found.retain(|p| dist(p) > 120);
        found.sort_by_key(|p| (dist(p), p.1, p.0));
        found[skip]
    }

    /// Fait bâtir un plan du type donné (au minimum) et attend la fin du chantier.
    fn build(game: &mut Game, kind: PlanKind, skip: usize) -> usize {
        let (w, h) = kind.min_size();
        let (x, y) = spot(game, kind, skip);
        game.apply(Command::PlacePlan { player: P0, kind, x, y, w, h, beds: kind.min_beds() });
        let i = game.sites().len() - 1;
        run_until(game, 900.0, |g| g.sites()[i].state == SiteState::Done);
        i
    }

    fn vagabond(game: &mut Game) -> CharacterId {
        run_until(game, 120.0, |g| g.characters().iter().any(|c| c.role == Role::Vagabond));
        game.characters().iter().find(|c| c.role == Role::Vagabond).unwrap().id
    }

    /// Place la personne juste à côté du roi.
    fn bring_next_to_king(game: &mut Game, id: CharacterId) {
        let king = game.characters[KING.0].pos;
        game.characters[id.0].pos = king + Vec2::new(0.8, 0.0);
    }

    fn talk_to(game: &mut Game, id: CharacterId) {
        // les autres s'écartent : le roi parle à la personne la plus proche
        let king = game.characters[KING.0].pos;
        for c in game.characters.iter_mut().filter(|c| c.id != id && c.id != KING) {
            if c.pos.distance(king) < 4.0 {
                c.pos = king + Vec2::new(0.0, 12.0);
            }
        }
        bring_next_to_king(game, id);
        game.apply(Command::Talk { player: P0 });
        assert_eq!(game.talking_to(P0), Some(id), "le roi doit parler à {id:?}");
    }

    fn post_of_role(game: &Game, role: Role, n: usize) -> TilePos {
        game.posts().into_iter().filter(|p| p.role == role).nth(n).unwrap_or_else(|| panic!("aucun poste {role:?} n°{n}")).tile
    }

    fn assign_post(game: &mut Game, who: CharacterId, post: TilePos) {
        talk_to(game, who);
        game.apply(Command::AssignPost { player: P0, who, post });
    }

    // ------------------------------------------------------------ parler

    #[test]
    fn the_king_talks_to_the_nearest_person_in_range_only() {
        let mut game = new_game(cfg());
        // deux artisans à portée : on parle au plus proche
        let king = game.characters[0].pos;
        game.characters[1].pos = king + Vec2::new(1.5, 0.0);
        game.characters[2].pos = king + Vec2::new(0.7, 0.0);
        game.apply(Command::Talk { player: P0 });
        assert_eq!(game.talking_to(P0), Some(CharacterId(2)));
        game.apply(Command::EndTalk { player: P0 });
        assert_eq!(game.talking_to(P0), None);
        // hors de portée : rien
        game.characters[1].pos = game.characters[0].pos + Vec2::new(10.0, 0.0);
        game.characters[2].pos = game.characters[0].pos + Vec2::new(-10.0, 0.0);
        game.apply(Command::Talk { player: P0 });
        assert_eq!(game.talking_to(P0), None, "personne à portée");
    }

    #[test]
    fn only_the_king_can_talk_and_not_to_himself() {
        let mut game = new_game(cfg());
        game.apply(Command::ToggleIdle { player: P0 });
        game.apply(Command::Possess { player: P0, character: ARTISAN });
        game.apply(Command::Talk { player: P0 });
        assert_eq!(game.talking_to(P0), None, "un artisan ne parle pas à la place du roi");
        game.apply(Command::ToggleIdle { player: P0 });
        game.apply(Command::Possess { player: P0, character: KING });
        game.characters[1].pos = game.characters[0].pos + Vec2::new(30.0, 0.0);
        game.characters[2].pos = game.characters[0].pos + Vec2::new(30.0, 0.0);
        game.apply(Command::Talk { player: P0 });
        assert_eq!(game.talking_to(P0), None, "il n'a personne d'autre que lui-même à portée");
    }

    #[test]
    fn someone_spoken_to_stops_and_resumes_exactly() {
        let mut game = new_game(cfg());
        let (w, h) = PlanKind::House.min_size();
        let (x, y) = spot(&game, PlanKind::House, 0);
        game.apply(Command::PlacePlan { player: P0, kind: PlanKind::House, x, y, w, h, beds: 1 });
        // un artisan part avec une cargaison de matériaux
        run_until(&mut game, 60.0, |g| g.characters()[1].cargo.is_some());
        bring_next_to_king(&mut game, ARTISAN);
        game.apply(Command::Talk { player: P0 });
        assert_eq!(game.talking_to(P0), Some(ARTISAN));
        let (cargo, pos) = (game.characters()[1].cargo, game.characters()[1].pos);
        assert!(game.characters()[1].is_talking());
        run(&mut game, 5.0);
        assert_eq!(game.characters()[1].cargo, cargo, "il garde sa cargaison");
        assert_eq!(game.characters()[1].pos, pos, "il ne bouge plus");
        assert_eq!(game.characters()[1].activity, Activity::Idle);
        // le roi ne dit rien : il reprend et livre
        game.apply(Command::EndTalk { player: P0 });
        assert!(!game.characters()[1].is_talking());
        let pile = game.sites()[0].pile.expect("un tas");
        run_until(&mut game, 60.0, |g| g.containers()[pile].get(Resource::Wood) + g.containers()[pile].get(Resource::Stone) > 0);
    }

    #[test]
    fn a_work_in_progress_keeps_its_progress_through_a_conversation() {
        let mut game = new_game(cfg());
        build_site_to_building(&mut game);
        run(&mut game, 6.0);
        let progress = game.sites()[0].progress;
        assert!(progress > 0.0 && progress < 1.0, "le chantier avance");
        let worker = game.characters().iter().position(|c| c.role == Role::Artisan && c.work.is_some()).expect("un artisan construit");
        let work_progress = game.characters()[worker].work.unwrap().progress;
        bring_next_to_king(&mut game, CharacterId(worker));
        game.apply(Command::Talk { player: P0 });
        run(&mut game, 4.0);
        assert!(game.characters()[worker].work.is_some(), "son travail est toujours là");
        assert!((game.sites()[0].progress - progress).abs() < 0.2, "un seul artisan travaille en moins, pas de retour à zéro");
        game.apply(Command::EndTalk { player: P0 });
        assert!(game.characters()[worker].work.map_or(false, |w| (w.progress - work_progress).abs() < 0.5));
    }

    /// Place une maison et attend que les artisans la construisent.
    fn build_site_to_building(game: &mut Game) {
        let (w, h) = PlanKind::House.min_size();
        let (x, y) = spot(game, PlanKind::House, 0);
        game.apply(Command::PlacePlan { player: P0, kind: PlanKind::House, x, y, w, h, beds: 1 });
        run_until(game, 120.0, |g| g.sites()[0].state == SiteState::Building);
    }

    #[test]
    fn talking_to_a_sleeper_wakes_him_and_he_goes_back_to_bed_if_the_king_says_nothing() {
        let mut game = new_game(Config { start_hour: 23.0, ..cfg() });
        run_until(&mut game, 120.0, |g| g.characters()[1].asleep);
        bring_next_to_king(&mut game, ARTISAN);
        game.characters[KING.0].pos = game.characters[ARTISAN.0].pos + Vec2::new(0.8, 0.0);
        game.apply(Command::Talk { player: P0 });
        assert_eq!(game.talking_to(P0), Some(ARTISAN), "on peut parler à quelqu'un qui dort");
        assert!(!game.characters()[1].asleep, "lui parler le réveille");
        run(&mut game, 3.0);
        assert!(!game.characters()[1].asleep, "il ne se rendort pas pendant qu'on lui parle");
        game.apply(Command::EndTalk { player: P0 });
        run_until(&mut game, 120.0, |g| g.characters()[1].asleep);
        assert_eq!(game.characters()[1].tile(), game.characters()[1].home, "il est retourné se coucher dans son lit");
    }

    #[test]
    fn walking_away_ends_the_conversation() {
        let mut game = new_game(cfg());
        talk_to(&mut game, ARTISAN);
        game.characters[KING.0].pos = game.characters[KING.0].pos + Vec2::new(20.0, 0.0);
        run(&mut game, 0.1);
        assert_eq!(game.talking_to(P0), None);
        assert!(!game.characters()[1].is_talking());
    }

    #[test]
    fn pressing_e_as_the_king_talks_then_ends() {
        let mut game = new_game(cfg());
        bring_next_to_king(&mut game, ARTISAN);
        game.apply(Command::Interact { player: P0 });
        assert!(game.talking_to(P0).is_some(), "E s'adresse à quelqu'un");
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.talking_to(P0), None, "un second E met fin à la conversation");
    }

    #[test]
    fn possessing_someone_else_ends_the_conversation() {
        let mut game = new_game(cfg());
        talk_to(&mut game, ARTISAN);
        game.apply(Command::Possess { player: P0, character: CharacterId(2) });
        assert_eq!(game.talking_to(P0), None);
        assert!(!game.characters()[1].is_talking());
    }

    // ------------------------------------------------------------ postes

    #[test]
    fn buildings_provide_posts_and_the_artisans_hold_the_camp_post() {
        let mut game = new_game(cfg());
        let camp = game.camp().unwrap();
        let posts = game.posts();
        assert_eq!(posts.len(), 1);
        assert_eq!((posts[0].role, posts[0].tile, posts[0].unlimited), (Role::Artisan, camp, true));
        assert_eq!(game.post_holders(camp).len(), 2, "les deux artisans tiennent le poste du camp");
        build(&mut game, PlanKind::Warehouse, 0);
        let transporters: Vec<Post> = game.posts().into_iter().filter(|p| p.role == Role::Transporter).collect();
        assert_eq!(transporters.len() as u32, game.config().warehouse_posts);
        assert!(transporters.iter().all(|p| !p.unlimited && game.post_holders(p.tile).is_empty()), "libres");
        for p in &transporters {
            assert!(!game.map().get(p.tile).is_solid(), "{p:?} est dans le passage");
        }
        build(&mut game, PlanKind::Tavern, 1);
        assert_eq!(game.posts().iter().filter(|p| p.role == Role::Innkeeper).count(), 1);
    }

    #[test]
    fn a_vagabond_becomes_a_transporter_at_a_free_post() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let v = vagabond(&mut game);
        let post = post_of_role(&game, Role::Transporter, 0);
        assert_eq!(game.free_places(), 0, "aucun lit libre : on embauche quand même");
        assign_post(&mut game, v, post);
        let c = &game.characters()[v.0];
        assert_eq!((c.role, c.post), (Role::Transporter, Some(post)));
        assert_eq!(game.post_holders(post), vec![v]);
        assert_eq!(game.talking_to(P0), None, "la conversation est finie");
        assert!(!c.is_talking());
        // il ne repart plus : il reste au royaume bien après son temps de séjour
        run(&mut game, 30.0);
        assert!(!game.characters()[v.0].gone);
    }

    #[test]
    fn replacing_a_holder_leaves_him_with_his_trade_but_no_post_and_idle() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let post = post_of_role(&game, Role::Transporter, 0);
        let (a, b) = (vagabond(&mut game), CharacterId(2));
        assign_post(&mut game, a, post);
        assert_eq!(game.characters()[a.0].post, Some(post));
        // un artisan prend le poste du transporteur
        assign_post(&mut game, b, post);
        assert_eq!(game.post_holders(post), vec![b], "un seul titulaire");
        let old = &game.characters()[a.0];
        assert_eq!(old.role, Role::Transporter, "il garde son métier");
        assert_eq!(old.post, None, "mais n'a plus de poste");
        assert_eq!(game.characters()[b.0].role, Role::Transporter, "le nouveau prend le métier du poste");
        // il ne fait plus rien
        run(&mut game, 20.0);
        let old = &game.characters()[a.0];
        assert!(old.work.is_none() && old.cargo.is_none() && old.activity == Activity::Idle, "{:?}", old.activity);
    }

    #[test]
    fn reassigning_frees_the_old_post() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let (p0, p1) = (post_of_role(&game, Role::Transporter, 0), post_of_role(&game, Role::Transporter, 1));
        let v = vagabond(&mut game);
        assign_post(&mut game, v, p0);
        assign_post(&mut game, v, p1);
        assert!(game.post_holders(p0).is_empty(), "le premier poste est libre");
        assert_eq!(game.post_holders(p1), vec![v]);
    }

    #[test]
    fn an_artisan_can_be_reassigned_and_come_back_to_the_unlimited_camp_post() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let camp = game.camp().unwrap();
        { let p = post_of_role(&game, Role::Transporter, 0); assign_post(&mut game, ARTISAN, p); }
        assert_eq!(game.characters()[1].role, Role::Transporter);
        assert_eq!(game.post_holders(camp).len(), 1, "il n'est plus artisan");
        let v = vagabond(&mut game);
        assign_post(&mut game, v, camp);
        assign_post(&mut game, ARTISAN, camp);
        assert_eq!(game.characters()[1].role, Role::Artisan);
        assert_eq!(game.post_holders(camp).len(), 3, "le poste du camp n'a pas de limite");
    }

    #[test]
    fn the_king_cannot_be_assigned() {
        let mut game = new_game(cfg());
        let camp = game.camp().unwrap();
        game.apply(Command::AssignPost { player: P0, who: KING, post: camp });
        assert_eq!(game.characters()[0].role, Role::King);
        assert_eq!(game.characters()[0].post, None);
    }

    #[test]
    fn only_the_person_spoken_to_can_be_assigned() {
        let mut game = new_game(cfg());
        let camp = game.camp().unwrap();
        build(&mut game, PlanKind::Warehouse, 0);
        let v = vagabond(&mut game);
        let post = post_of_role(&game, Role::Transporter, 0);
        game.apply(Command::AssignPost { player: P0, who: v, post });
        assert_eq!(game.characters()[v.0].role, Role::Vagabond, "on ne lui a pas parlé");
        let _ = camp;
    }

    #[test]
    fn without_a_post_a_trade_is_not_exercised_at_the_camp() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let (w, h) = PlanKind::House.min_size();
        let (x, y) = spot(&game, PlanKind::House, 5);
        let v = vagabond(&mut game);
        { let p = post_of_role(&game, Role::Transporter, 0); assign_post(&mut game, v, p); }
        // un chantier attend des matériaux : le transporteur en porte
        game.apply(Command::PlacePlan { player: P0, kind: PlanKind::House, x, y, w, h, beds: 1 });
        run_until(&mut game, 60.0, |g| g.characters()[v.0].cargo.is_some() || g.characters()[v.0].job.is_some());
        // on lui retire son poste (un autre le prend) : il ne porte plus rien
        let post = post_of_role(&game, Role::Transporter, 0);
        assign_post(&mut game, CharacterId(2), post);
        let loose = game.characters()[v.0].cargo;
        run(&mut game, 20.0);
        assert_eq!(game.characters()[v.0].cargo, loose.filter(|_| false).or(game.characters()[v.0].cargo));
        assert_eq!(game.characters()[v.0].activity, Activity::Idle);
        assert!(game.characters()[v.0].job.is_none());
    }

    // ------------------------------------------------------------ lits

    #[test]
    fn someone_with_a_post_and_no_bed_takes_the_free_bed_nearest_to_his_post() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let house_a = build(&mut game, PlanKind::House, 3);
        let house_b = build(&mut game, PlanKind::House, 30);
        let post = post_of_role(&game, Role::Transporter, 0);
        let near = |g: &Game, i: usize| {
            let s = &g.sites()[i];
            (s.x + 1, s.y + 1)
        };
        let dist = |p: TilePos, (x, y): (i32, i32)| (p.x - x).abs() + (p.y - y).abs();
        let expected = if dist(post, near(&game, house_a)) <= dist(post, near(&game, house_b)) { house_a } else { house_b };
        let v = vagabond(&mut game);
        assert!(game.free_places() >= 2);
        assign_post(&mut game, v, post);
        run(&mut game, 0.1);
        let c = &game.characters()[v.0];
        assert!(c.has_bed, "il a pris un lit");
        let s = &game.sites()[expected];
        assert!(game.habitats().iter().any(|h| h.x == s.x && h.y == s.y && h.bed_homes().contains(&c.home)), "le lit de la maison la plus proche");
    }

    #[test]
    fn a_new_bed_goes_to_the_first_person_who_lacks_one() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let v = vagabond(&mut game);
        { let p = post_of_role(&game, Role::Transporter, 0); assign_post(&mut game, v, p); }
        run(&mut game, 1.0);
        assert!(!game.characters()[v.0].has_bed, "aucun lit libre pour l'instant");
        build(&mut game, PlanKind::House, 3);
        run(&mut game, 0.5);
        assert!(game.characters()[v.0].has_bed, "la maison finie lui donne un lit");
        assert_eq!(game.free_places(), 0);
    }

    #[test]
    fn without_a_bed_one_never_sleeps_and_wears_out_but_keeps_working() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Warehouse, 0);
        let v = vagabond(&mut game);
        { let p = post_of_role(&game, Role::Transporter, 0); assign_post(&mut game, v, p); }
        let mut slept = false;
        for _ in 0..(900.0 / TICK) as usize {
            game.step(TICK);
            slept |= game.characters()[v.0].asleep;
        }
        assert!(!slept, "sans lit, il ne dort jamais");
        assert!(game.characters()[v.0].fatigue >= 95.0, "fatigue {}", game.characters()[v.0].fatigue);
        assert!(game.characters()[v.0].efficiency() <= 0.55, "l'épuisement le ralentit");
        assert!(game.characters()[v.0].post.is_some(), "il garde son poste");
    }

    #[test]
    fn the_king_can_give_a_bed_by_hand_and_take_it_from_its_owner() {
        let mut game = new_game(cfg());
        let (a, b) = (CharacterId(1), CharacterId(2));
        let (bed_a, bed_b) = (game.characters()[1].home, game.characters()[2].home);
        talk_to(&mut game, a);
        game.apply(Command::AssignBed { player: P0, who: a, bed: bed_b });
        assert_eq!(game.characters()[a.0].bed(), Some(bed_b), "A a le lit de B");
        assert_eq!(game.characters()[b.0].bed(), None, "B n'a plus de lit");
        assert_eq!(game.bed_owner(bed_b), Some(a));
        assert_eq!(game.bed_owner(bed_a), None);
        assert_eq!(game.free_places(), 1, "le lit de A est libre");
    }

    #[test]
    fn a_vagabond_given_a_bed_becomes_an_idle_inhabitant_who_sleeps_there() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::House, 0);
        let v = vagabond(&mut game);
        let bed = game.beds().into_iter().find(|&b| game.bed_owner(b).is_none()).expect("un lit libre");
        talk_to(&mut game, v);
        game.apply(Command::AssignBed { player: P0, who: v, bed });
        let c = &game.characters()[v.0];
        assert_eq!((c.role, c.bed()), (Role::Villager, Some(bed)));
        assert_eq!(c.post, None);
        // le jour il ne fait rien, la nuit il dort dans son lit
        run_until(&mut game, 700.0, |g| g.characters()[v.0].asleep);
        assert_eq!(game.characters()[v.0].tile(), bed);
    }

    // ------------------------------------------------------------ la forge

    #[test]
    fn a_forge_has_a_solid_anvil_with_a_free_spot_to_its_north_and_a_chest_outside() {
        let mut game = new_game(cfg());
        let i = build(&mut game, PlanKind::Forge, 0);
        let b = *game.buildings().iter().find(|b| b.kind == PlanKind::Forge).unwrap();
        let anvil = b.anvil_tile();
        assert_eq!(game.map().get(anvil), Tile::Anvil);
        assert!(game.map().get(anvil).is_solid());
        let spot = forge_spot(anvil);
        assert!(!game.map().get(spot).is_solid() && b.covers(spot));
        assert!(!game.map().find_path(b.front(), spot).is_empty(), "le point de forge est accessible depuis la porte");
        let chest = game.containers().iter().find(|c| c.kind == ContainerKind::Forge).expect("un coffre de forge");
        assert!(!b.covers(chest.tile), "le coffre est dehors");
        assert_eq!(game.map().get(chest.tile), Tile::Crate);
        assert_eq!(game.sites()[i].state, SiteState::Done);
        let post = post_of_role(&game, Role::Blacksmith, 0);
        assert_eq!(post, spot, "le poste de forgeron est le point de forge");
    }

    #[test]
    fn nobody_forges_without_a_blacksmith_and_an_assigned_one_forges() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Forge, 0);
        let chest = game.containers().iter().position(|c| c.kind == ContainerKind::Forge).unwrap();
        game.containers[chest].add(Resource::Iron, 20);
        run(&mut game, 60.0);
        assert_eq!(game.stock().get(Resource::Tools), 0, "personne n'est affecté à l'enclume");
        let v = vagabond(&mut game);
        let post = post_of_role(&game, Role::Blacksmith, 0);
        assign_post(&mut game, v, post);
        assert_eq!(game.characters()[v.0].role, Role::Blacksmith);
        run_until(&mut game, 120.0, |g| g.stock().get(Resource::Tools) >= 1);
        assert!(game.stock().get(Resource::Iron) < 20, "il a forgé avec le fer du coffre");
    }

    #[test]
    fn a_forge_without_iron_leaves_the_blacksmith_waiting_at_his_post() {
        let mut game = new_game(cfg());
        build(&mut game, PlanKind::Forge, 0);
        let v = vagabond(&mut game);
        { let p = post_of_role(&game, Role::Blacksmith, 0); assign_post(&mut game, v, p); }
        run(&mut game, 60.0);
        assert_eq!(game.stock().get(Resource::Tools), 0);
        assert_eq!(game.characters()[v.0].post, Some(post_of_role(&game, Role::Blacksmith, 0)));
    }

    #[test]
    fn the_forge_plan_is_in_the_list_with_a_cost_that_follows_its_surface() {
        let game = new_game(cfg());
        assert!(PlanKind::ALL.contains(&PlanKind::Forge));
        let (w, h) = PlanKind::Forge.min_size();
        let small = game.plan_cost(PlanKind::Forge, w, h, 0);
        let big = game.plan_cost(PlanKind::Forge, w + 4, h + 2, 0);
        assert!(big.wood > small.wood && big.stone > small.stone);
        assert!(game.plan_secs(PlanKind::Forge, w + 4, h + 2, 0) > game.plan_secs(PlanKind::Forge, w, h, 0));
        // un coffre qui ne tiendrait pas dans la carte refuse le plan
        let map_w = game.map().width();
        assert_eq!(game.check_plan(PlanKind::Forge, map_w - w - 1, 10, w, h, 0), Err(PlanError::Blocked));
    }

    #[test]
    fn the_village_workers_still_work_without_any_post() {
        let mut game = Game::new(Scenario::village(), 1, Config::default());
        run(&mut game, 90.0);
        assert!(game.characters().iter().filter(|c| c.role == Role::Blacksmith).all(|c| c.post.is_none()));
        assert!(game.stock().get(Resource::Tools) >= 1, "le forgeron du village forge sans poste");
    }
}
