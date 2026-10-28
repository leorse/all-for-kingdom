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

use crate::character::{forge_spot, Character, Env, Facing, Outcome, Role, BODY_HALF};
use crate::clock::Clock;
use crate::config::Config;
use crate::container::{container_at, place_crates, Container, ContainerKind};
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
}

struct PlayerSlot {
    possessed: Option<CharacterId>,
    last: CharacterId,
    move_dir: Vec2,
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
            .map(|&spec| Container::new(spec, config.container_capacity, &scenario.veins))
            .collect();
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

    /// Places libres d'un habitat : ses lits que personne n'occupe comme logement (aucun personnage n'y a son lit).
    pub fn free_places_in(&self, habitat: usize) -> u32 {
        let Some(h) = self.habitats.get(habitat) else { return 0 };
        h.bed_homes().iter().filter(|&&bed| !self.characters.iter().any(|c| c.home == bed)).count() as u32
    }

    /// Places libres dans tout le royaume.
    pub fn free_places(&self) -> u32 {
        (0..self.habitats.len()).map(|i| self.free_places_in(i)).sum()
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
                .any(|o| o.id != id && o.work.map_or(false, |w| !matches!(w.kind, WorkKind::Mine | WorkKind::Chop) && w.tile == tile))
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
            WorkKind::Till | WorkKind::Plant => true,
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
        for c in &self.characters {
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
                    if self.is_free_for(candidate, player) {
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
            Command::Interact { player } => {
                let Some(id) = self.possessed_by(player) else { return };
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
                    if kind == WorkKind::Forge && c.tile() != forge_spot(tile) {
                        // on ne forge que depuis le poste au nord de l'enclume : il y va seul
                        let path = self.map.find_path_mode(c.tile(), forge_spot(tile), PathMode::Roads(self.config.grass_cost));
                        if !path.is_empty() {
                            self.characters[id.0].start_approach(tile, path);
                        }
                    } else {
                        let secs = work_secs(kind, tile, &self.config, &self.veins);
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
        for i in 0..self.characters.len() {
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
            }
            WorkKind::Chop => {} // jamais émis : `Game` décide seul de la chute des arbres
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
        if self.characters[who].tile() != forge_spot(anvil) {
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
                        self.containers.iter().any(|k| !k.is_depot() && k.consumes() == Some(r))
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

    fn is_free_for(&self, character: CharacterId, player: PlayerId) -> bool {
        self.controller_of(character).map_or(true, |p| p == player)
    }

    fn possess(&mut self, player: PlayerId, character: CharacterId) {
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

    use crate::character::forge_spot;

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
    fn ai_blacksmith_forges_only_from_the_north_of_the_anvil_facing_down() {
        let mut game = new_game();
        ticks_until(&mut game, 10.0, |g| g.characters[SMITH.0].work.is_some());
        let smith = &game.characters[SMITH.0];
        assert_eq!(smith.tile(), forge_spot(ANVIL));
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
        assert_eq!(smith.tile(), forge_spot(ANVIL));
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
    fn at_the_forge_spot_one_press_forges_at_once() {
        let mut game = new_game();
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = forge_spot(ANVIL).center();
        game.characters[SMITH.0].facing = Facing::Down;
        game.apply(Command::Interact { player: P0 });
        assert_eq!(game.characters[SMITH.0].work.map(|w| w.kind), Some(WorkKind::Forge));
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
        ai.characters[SMITH.0].pos = forge_spot(ANVIL).center();
        let t_ai = ticks_until(&mut ai, 30.0, |g| g.stock().get(Resource::Tools) >= 1);

        // joueur : il se tourne vers l'enclume (en bas) puis appuie sur E
        let mut player = new_game();
        player.apply(Command::Possess { player: P0, character: SMITH });
        player.characters[SMITH.0].pos = forge_spot(ANVIL).center();
        player.apply(Command::Move { player: P0, dir: Vec2::new(0.0, 1.0) });
        run(&mut player, 0.1); // l'enclume bloque : il ne bouge pas mais la regarde
        player.apply(Command::Move { player: P0, dir: Vec2::ZERO });
        run(&mut player, 0.05);
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
