//! L'état global de la partie et la façon de le faire évoluer.
//!
//! Contrat avec le monde extérieur (rendu, entrées, réseau) :
//!  - les entrées arrivent sous forme de [`Command`] (jamais de touches ou de clics bruts) ;
//!  - le temps avance par pas fixes via [`Game::step`] ;
//!  - l'état se lit via des accesseurs en lecture seule.
//!
//! Plusieurs joueurs sont déjà prévus (coop) : chaque `PlayerId` possède au plus un personnage.
//! Aucun joueur ne possède personne ? C'est le mode AFK : la cité tourne seule.

use crate::character::{Character, Env};
use crate::clock::Clock;
use crate::config::Config;
use crate::farm::Farm;
use crate::map::{Tile, TileMap};
use crate::math::{TilePos, Vec2};
use crate::resources::{Resource, Stock};
use crate::rng::Rng;
use crate::scenario::Scenario;
use crate::work::WorkKind;

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
}

struct PlayerSlot {
    possessed: Option<CharacterId>,
    last: CharacterId,
    move_dir: Vec2,
}

pub struct Game {
    map: TileMap,
    farm: Farm,
    stock: Stock,
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
        let characters = scenario
            .characters
            .into_iter()
            .enumerate()
            .map(|(i, s)| Character::new(CharacterId(i), s.name, s.role, s.home, s.night_shift, s.work_spots))
            .collect();
        let players = (0..player_count)
            .map(|_| PlayerSlot {
                possessed: None,
                last: CharacterId(0),
                move_dir: Vec2::ZERO,
            })
            .collect();
        let farm = Farm::from_map(&scenario.map);
        let mut stock = Stock::new();
        stock.add(Resource::Iron, config.starting_iron);
        Game {
            map: scenario.map,
            farm,
            stock,
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

    pub fn stock(&self) -> &Stock {
        &self.stock
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
        let has_iron = self.stock.get(Resource::Iron) >= self.config.iron_per_tool;
        (self.map.get(tile) == Tile::Anvil && has_iron).then_some(WorkKind::Forge)
    }

    /// Le travail que ce personnage pourrait lancer maintenant avec `Interact`, et sur quelle tuile :
    /// ce qu'il a devant lui d'abord (parcelle, enclume), sinon ce qu'il a sous les pieds. `None`
    /// s'il n'y a rien à faire (ou si ce n'est pas son métier, ou s'il dort ou travaille déjà).
    pub fn available_work(&self, id: CharacterId) -> Option<(WorkKind, TilePos)> {
        let c = self.characters.get(id.0)?;
        if c.asleep || c.work.is_some() {
            return None;
        }
        // pas la peine de travailler à deux sur la même parcelle
        let worked_by_other = |tile: TilePos| {
            self.characters
                .iter()
                .any(|o| o.id != id && o.work.map_or(false, |w| w.tile == tile))
        };
        [c.facing_tile(), c.tile()].into_iter().find_map(|tile| {
            self.work_kind_at(tile)
                .filter(|kind| kind.role() == c.role && !worked_by_other(tile))
                .map(|kind| (kind, tile))
        })
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
            Command::Interact { player } => {
                let Some(id) = self.possessed_by(player) else { return };
                if self.characters[id.0].asleep {
                    self.characters[id.0].wake_up();
                } else if self.can_lie_down(id) {
                    self.characters[id.0].lie_down();
                } else if self.characters[id.0].work.is_some() {
                    self.characters[id.0].cancel_work();
                } else if let Some((kind, tile)) = self.available_work(id) {
                    self.characters[id.0].start_work(kind, tile);
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
            let control = self
                .controller_of(CharacterId(i))
                .map(|p| self.players[p.0 as usize].move_dir);
            let env = Env {
                map: &self.map,
                farm: &self.farm,
                stock: &self.stock,
                cfg: &self.config,
                night,
                dt,
                dt_hours,
                claimed: &claimed,
            };
            if let Some(done) = self.characters[i].update(&env, control, &mut self.rng) {
                finished.push(done);
            }
        }

        // ... et c'est ici que les travaux terminés modifient le monde.
        for done in finished {
            match done.kind {
                WorkKind::Forge => {
                    // le fer n'est consommé qu'à la fin : s'il a manqué entre-temps, le coup est perdu
                    if self.stock.take(Resource::Iron, self.config.iron_per_tool) {
                        self.stock.add(Resource::Tools, self.config.tools_per_forge);
                    }
                }
                WorkKind::Harvest => {
                    if self.farm.apply(done.kind, done.tile) {
                        self.stock.add(Resource::Wheat, self.config.wheat_per_harvest);
                    }
                }
                WorkKind::Till | WorkKind::Plant => {
                    self.farm.apply(done.kind, done.tile);
                }
            }
        }
    }

    // ------------------------------------------------------------ interne

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
    use crate::farm::PlotState;
    use crate::map::Tile;
    use std::collections::HashSet;

    const P0: PlayerId = PlayerId(0);
    const FARMER: CharacterId = CharacterId(0);
    const KING: CharacterId = CharacterId(2);
    const WARLORD: CharacterId = CharacterId(3);
    /// Le paysan démarre sur cette parcelle, tourné vers le bas : il fait donc face à la suivante.
    const PLOT_HERE: TilePos = TilePos::new(3, 4);
    const PLOT_FRONT: TilePos = TilePos::new(3, 5);

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
        let path = map.find_path(TilePos::new(22, 5), TilePos::new(5, 14));
        assert!(!path.is_empty(), "la forge doit être reliée à la caserne");
        let mut prev = TilePos::new(22, 5);
        for p in path {
            assert!(!map.get(p).is_solid());
            assert_eq!((p.x - prev.x).abs() + (p.y - prev.y).abs(), 1);
            prev = p;
        }
        assert_eq!(prev, TilePos::new(5, 14));
    }

    #[test]
    fn find_path_to_a_wall_is_empty() {
        let map = Scenario::village().map;
        assert!(map.find_path(TilePos::new(15, 10), TilePos::new(12, 1)).is_empty());
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
        assert!(king.pos.y >= 2.0 + crate::character::BODY_HALF - 0.01, "le roi a traversé le mur : {:?}", king.pos);
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
        run_until(&mut game, 1, 23.0);
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
        run(&mut game, 6.0); // le lit est à 2 tuiles
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
        let d_fresh = fresh.characters[FARMER.0].pos.x - 3.5;
        let d_tired = tired.characters[FARMER.0].pos.x - 3.5;
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
        assert_eq!(game.stock().get(Resource::Wheat), game.config().wheat_per_harvest);
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
        assert!(game.stock().get(Resource::Wheat) > 0, "aucune récolte après un jour et demi en AFK");
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
        run_until(&mut game, 1, 22.0);
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
        assert!(game.characters[second_farmer.0].start_work(WorkKind::Till, PLOT_FRONT));
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

    const SMITH: CharacterId = CharacterId(1);
    const ANVIL: TilePos = TilePos::new(23, 5);

    #[test]
    fn scenario_has_one_anvil_and_beds_are_distinct() {
        let scenario = Scenario::village();
        assert_eq!(scenario.map.find_tiles(Tile::Anvil), vec![ANVIL]);
        let homes: HashSet<TilePos> = scenario.characters.iter().map(|c| c.home).collect();
        assert_eq!(homes.len(), scenario.characters.len(), "deux personnages se partagent un lit");
        // l'enclume est un obstacle, mais on peut l'atteindre depuis une case voisine
        assert!(scenario.map.get(ANVIL).is_solid());
        assert!(!scenario.map.walkable_neighbors(ANVIL).is_empty());
    }

    #[test]
    fn ai_blacksmith_forges_tools_and_iron_is_conserved() {
        let mut game = new_game();
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
            Scenario::village(),
            1,
            Config { starting_iron: 5, ..Config::default() }, // 2 outils, reste 1
        );
        run(&mut game, 120.0);
        assert_eq!(game.stock().get(Resource::Tools), 2);
        assert_eq!(game.stock().get(Resource::Iron), 1);
        assert!(game.characters[SMITH.0].work.is_none(), "il ne doit plus forger sans fer");
        // et le joueur non plus : rien à faire à l'enclume
        game.apply(Command::Possess { player: P0, character: SMITH });
        game.characters[SMITH.0].pos = TilePos::new(22, 5).center();
        game.characters[SMITH.0].facing = crate::character::Facing::Right;
        assert_eq!(game.available_work(SMITH), None);
    }

    #[test]
    fn player_blacksmith_forges_twice_as_fast() {
        // IA : il se met seul au travail, à côté de l'enclume
        let mut ai = new_game();
        let t_ai = ticks_until(&mut ai, 30.0, |g| g.stock().get(Resource::Tools) >= 1);

        // joueur : il se tourne vers l'enclume (à sa droite) puis appuie sur E
        let mut player = new_game();
        player.apply(Command::Possess { player: P0, character: SMITH });
        player.apply(Command::Move { player: P0, dir: Vec2::new(1.0, 0.0) });
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
        game.characters[FARMER.0].pos = TilePos::new(22, 5).center();
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
        use crate::character::{Facing, Role};
        use crate::scenario::CharacterSpec;

        let mut scenario = Scenario::village();
        let home = TilePos::new(28, 4); // un second lit dans la maison du forgeron
        scenario.map.set(home, Tile::Bed);
        scenario.characters.push(CharacterSpec {
            name: "Forgeron 2".to_string(),
            role: Role::Blacksmith,
            home,
            night_shift: false,
            work_spots: vec![TilePos::new(24, 5)],
        });
        let mut game = Game::new(scenario, 1, Config::default());
        run(&mut game, 1.0); // le premier forgeron (IA) est déjà au travail sur l'enclume
        assert_eq!(game.characters[SMITH.0].claimed_tile(), Some(ANVIL));

        // le second forgeron, joueur, juste de l'autre côté de l'enclume, la regarde : c'est occupé
        let second = id_of(&game, "Forgeron 2");
        game.apply(Command::Possess { player: P0, character: second });
        game.characters[second.0].pos = TilePos::new(24, 5).center();
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

        run_until(&mut game, 1, 23.0); // minuit approche : relève
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
}
