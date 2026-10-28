//! Les personnages de la cité : besoins (fatigue, sommeil), travail et comportement (contrôle direct ou IA).
//! Un personnage expose son état (`role`, `facing`, `activity`, `fatigue`, `work`...) ; c'est au rendu
//! de décider quel sprite, quelle animation ou quelle jauge en tirer.
//!
//! Un personnage ne modifie jamais le monde directement : quand un travail se termine, il renvoie un
//! `WorkDone` et c'est `Game` qui en applique les effets (parcelle, stock...).

use crate::config::Config;
use crate::farm::Farm;
use crate::game::CharacterId;
use crate::map::{Tile, TileMap};
use crate::math::{TilePos, Vec2};
use crate::resources::{Resource, Stock};
use crate::rng::Rng;
use crate::work::{forge_station, Work, WorkDone, WorkKind};
use std::collections::VecDeque;

/// Vitesse d'un personnage dirigé par le joueur (tuiles par seconde).
pub const PLAYER_SPEED: f32 = 3.4;
/// Vitesse d'un personnage dirigé par l'IA : plus lent, c'est l'avantage d'incarner quelqu'un.
pub const AI_SPEED: f32 = 1.7;
/// Demi-côté de la boîte de collision (en tuiles).
pub const BODY_HALF: f32 = 0.28;

/// Contexte de simulation d'un pas, partagé par tous les personnages (en lecture seule).
pub(crate) struct Env<'a> {
    pub map: &'a TileMap,
    pub farm: &'a Farm,
    pub stock: &'a Stock,
    pub cfg: &'a Config,
    pub night: bool,
    /// Durée du pas en secondes réelles.
    pub dt: f32,
    /// Durée du pas en heures de jeu.
    pub dt_hours: f32,
    /// Tuiles visées ou travaillées par les AUTRES personnages en ce moment (voir `claimed_tile`).
    pub claimed: &'a [TilePos],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Farmer,
    Blacksmith,
    King,
    Warlord,
    Innkeeper,
    Guard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activity {
    Idle,
    Walking,
    Working,
    Sleeping,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facing {
    Down,
    Up,
    Left,
    Right,
}

impl Facing {
    fn from_dir(dir: Vec2) -> Facing {
        if dir.x.abs() > dir.y.abs() {
            if dir.x < 0.0 {
                Facing::Left
            } else {
                Facing::Right
            }
        } else if dir.y < 0.0 {
            Facing::Up
        } else {
            Facing::Down
        }
    }
}

pub struct Character {
    pub id: CharacterId,
    pub name: String,
    pub role: Role,
    /// Centre du personnage, en coordonnées monde (tuiles).
    pub pos: Vec2,
    pub facing: Facing,
    pub activity: Activity,
    /// Tuile de son lit.
    pub home: TilePos,
    /// Travaille la nuit et dort le jour.
    pub night_shift: bool,
    /// Fatigue de 0 (en forme) à 100 (épuisé).
    pub fatigue: f32,
    pub asleep: bool,
    /// Travail en cours, s'il y en a un.
    pub work: Option<Work>,

    // --- état interne de l'IA
    work_spots: Vec<TilePos>,
    path: VecDeque<TilePos>,
    goal: Option<TilePos>,
    /// Tuile de travail vers laquelle on se dirige (parcelle, enclume...) : elle est réservée
    /// pour que personne d'autre ne la vise en même temps.
    work_target: Option<TilePos>,
    /// Prochain point de l'itinéraire de patrouille (gardes).
    patrol_next: usize,
    wait: f32,
    at_work: bool,
    stuck: f32,
}

impl Character {
    pub(crate) fn new(
        id: CharacterId,
        name: String,
        role: Role,
        home: TilePos,
        night_shift: bool,
        work_spots: Vec<TilePos>,
    ) -> Self {
        assert!(!work_spots.is_empty(), "un personnage a besoin d'au moins un poste de travail");
        Character {
            id,
            name,
            role,
            pos: work_spots[0].center(),
            facing: Facing::Down,
            activity: Activity::Idle,
            home,
            night_shift,
            fatigue: 0.0,
            asleep: false,
            work: None,
            work_spots,
            path: VecDeque::new(),
            goal: None,
            work_target: None,
            patrol_next: 0,
            wait: 0.0,
            at_work: false,
            stuck: 0.0,
        }
    }

    pub fn tile(&self) -> TilePos {
        self.pos.tile()
    }

    /// La tuile de travail que ce personnage occupe ou vise en ce moment : celle où il travaille,
    /// sinon celle vers laquelle il marche. Les autres l'évitent. C'est une information dérivée de son
    /// état : elle disparaît toute seule dès qu'il change de plan (nuit, joueur qui le prend,
    /// travail terminé...).
    pub fn claimed_tile(&self) -> Option<TilePos> {
        self.work.map(|w| w.tile).or(self.work_target)
    }

    /// La tuile juste devant lui.
    pub fn facing_tile(&self) -> TilePos {
        let t = self.tile();
        match self.facing {
            Facing::Down => TilePos::new(t.x, t.y + 1),
            Facing::Up => TilePos::new(t.x, t.y - 1),
            Facing::Left => TilePos::new(t.x - 1, t.y),
            Facing::Right => TilePos::new(t.x + 1, t.y),
        }
    }

    pub fn distance_to_bed(&self) -> f32 {
        self.pos.distance(self.home.center())
    }

    /// Est-ce l'heure de dormir pour lui ? La nuit pour la plupart, le jour pour le quart de nuit.
    pub fn is_sleep_time(&self, night: bool) -> bool {
        night != self.night_shift
    }

    /// Forme de 0 (épuisé) à 100 (en pleine forme) : l'inverse de la fatigue.
    pub fn fitness(&self) -> f32 {
        (100.0 - self.fatigue).clamp(0.0, 100.0)
    }

    /// Rendement de 0.5 à 1.0 : plein jusqu'à 50 de fatigue, puis il baisse jusqu'à moitié à 100.
    /// S'applique aux déplacements et à la vitesse de travail.
    pub fn efficiency(&self) -> f32 {
        if self.fatigue <= 50.0 {
            1.0
        } else {
            1.0 - (self.fatigue.min(100.0) - 50.0) / 50.0 * 0.5
        }
    }

    // ------------------------------------------------------------ pas de simulation

    /// Un pas de simulation complet pour ce personnage.
    /// `control` : `Some(direction)` s'il est incarné par un joueur, `None` s'il est piloté par l'IA.
    pub(crate) fn update(&mut self, env: &Env, control: Option<Vec2>, rng: &mut Rng) -> Option<WorkDone> {
        if self.asleep {
            self.sleep_tick(env); // dort, qu'il soit incarné ou non
            return None;
        }
        match control {
            Some(dir) => self.drive(dir, env),
            None => self.run_ai(env, rng),
        }
        let done = self.advance_work(env, control.is_some());
        self.tick_fatigue(env);
        done
    }

    // ------------------------------------------------------------ sommeil

    pub(crate) fn lie_down(&mut self) {
        self.pos = self.home.center();
        self.asleep = true;
        self.activity = Activity::Sleeping;
        self.facing = Facing::Down;
        self.work = None;
        self.path.clear();
        self.goal = None;
        self.work_target = None;
        self.wait = 0.0;
        self.at_work = false;
        self.stuck = 0.0;
    }

    pub(crate) fn wake_up(&mut self) {
        self.asleep = false;
        self.activity = Activity::Idle;
        self.wait = 0.0;
    }

    /// Un pas de sommeil : la fatigue baisse ; en dehors de son heure de sommeil, on se réveille
    /// une fois reposé. Pendant son heure de sommeil, on dort jusqu'au bout même si on est reposé.
    fn sleep_tick(&mut self, env: &Env) {
        self.fatigue = (self.fatigue - env.cfg.sleep_recovery_per_hour * env.dt_hours).max(0.0);
        self.activity = Activity::Sleeping;
        if !self.is_sleep_time(env.night) && self.fatigue <= env.cfg.wake_fatigue {
            self.wake_up();
        }
    }

    /// Fatigue gagnée pendant un pas passé éveillé.
    fn tick_fatigue(&mut self, env: &Env) {
        let rate = match self.activity {
            Activity::Walking | Activity::Working => env.cfg.fatigue_active_per_hour,
            _ => env.cfg.fatigue_idle_per_hour,
        };
        self.fatigue = (self.fatigue + rate * env.dt_hours).min(100.0);
    }

    // ------------------------------------------------------------ travail

    /// Commence un travail (si c'est bien le métier de ce personnage). Renvoie `true` si démarré.
    pub(crate) fn start_work(&mut self, kind: WorkKind, tile: TilePos) -> bool {
        if kind.role() != self.role || self.asleep {
            return false;
        }
        self.work = Some(Work { kind, tile, progress: 0.0 });
        self.activity = Activity::Working;
        true
    }

    /// Place le personnage à son poste de forge, face à l'enclume (voir `forge_station`), avant de forger.
    pub(crate) fn take_forge_station(&mut self, anvil: TilePos) {
        self.pos = forge_station(anvil).center();
        self.facing = Facing::Down;
        self.path.clear();
        self.goal = None;
    }

    pub(crate) fn cancel_work(&mut self) {
        self.work = None;
    }

    /// Fait avancer le travail en cours. Le joueur travaille plus vite que l'IA ; la fatigue ralentit.
    fn advance_work(&mut self, env: &Env, by_player: bool) -> Option<WorkDone> {
        let efficiency = self.efficiency();
        let work = self.work.as_mut()?;
        let speed = if by_player { env.cfg.player_work_speed } else { 1.0 };
        work.progress += env.dt / work.kind.duration_secs(env.cfg) * speed * efficiency;
        let (kind, tile, finished) = (work.kind, work.tile, work.progress >= 1.0);

        if finished {
            self.work = None;
            self.activity = Activity::Idle;
            Some(WorkDone { kind, tile })
        } else {
            self.activity = Activity::Working;
            None
        }
    }

    // ------------------------------------------------------------ déplacement

    /// Déplacement avec collisions, un axe à la fois (on glisse le long des murs).
    fn try_move(&mut self, delta: Vec2, map: &TileMap) {
        if delta.x != 0.0 {
            let next = Vec2::new(self.pos.x + delta.x, self.pos.y);
            if !map.is_box_blocked(next, BODY_HALF) {
                self.pos = next;
            }
        }
        if delta.y != 0.0 {
            let next = Vec2::new(self.pos.x, self.pos.y + delta.y);
            if !map.is_box_blocked(next, BODY_HALF) {
                self.pos = next;
            }
        }
    }

    /// Contrôle direct par un joueur. Bouger interrompt le travail en cours.
    fn drive(&mut self, dir: Vec2, env: &Env) {
        let dir = dir.normalized_or_zero();
        self.work_target = None; // le joueur décide : plus de tuile réservée par l'IA
        if self.work.is_some() {
            if dir == Vec2::ZERO {
                return; // il continue de travailler sur place
            }
            self.work = None;
        }
        if dir == Vec2::ZERO {
            self.activity = Activity::Idle;
            return;
        }
        let speed = PLAYER_SPEED * self.efficiency();
        self.try_move(dir * (speed * env.dt), env.map);
        self.facing = Facing::from_dir(dir); // même contre un obstacle : on le regarde
        self.activity = Activity::Walking;
    }

    /// Le joueur quitte ce personnage : l'IA reprend depuis l'endroit où il se trouve.
    /// S'il dormait, il continue de dormir ; s'il travaillait, il termine son travail.
    pub(crate) fn release_to_ai(&mut self) {
        self.path.clear();
        self.goal = None;
        self.work_target = None;
        self.wait = 0.3;
        self.at_work = false;
        self.stuck = 0.0;
        if !self.asleep && self.work.is_none() {
            self.activity = Activity::Idle;
        }
    }

    /// Avance d'un pas le long du chemin en cours. Renvoie `true` quand le chemin vient de se terminer.
    fn follow_path(&mut self, env: &Env) -> bool {
        let Some(&next) = self.path.front() else {
            return false;
        };
        let goal = next.center();
        let to_goal = goal - self.pos;
        let dist = to_goal.length();
        let step = AI_SPEED * self.efficiency() * env.dt;
        self.activity = Activity::Walking;

        if dist <= step.max(0.03) {
            self.pos = goal;
            self.path.pop_front();
            self.stuck = 0.0;
            return self.path.is_empty();
        }

        let dir = to_goal * (1.0 / dist);
        self.facing = Facing::from_dir(dir);
        let before = self.pos;
        self.try_move(dir * step, env.map);

        // Anti-blocage : si on n'avance plus pendant 1 s, on abandonne ce trajet.
        if self.pos.distance(before) < step * 0.25 {
            self.stuck += env.dt;
            if self.stuck > 1.0 {
                self.path.clear();
                self.stuck = 0.0;
                self.wait = 0.2;
            }
        } else {
            self.stuck = 0.0;
        }
        false
    }

    // ------------------------------------------------------------ IA

    /// Routine de l'IA, par ordre de priorité : dormir (à son heure, ou par épuisement), finir son
    /// travail en cours, puis la routine propre à son métier.
    fn run_ai(&mut self, env: &Env, rng: &mut Rng) {
        let exhausted = self.fatigue >= env.cfg.exhausted_fatigue;
        if self.is_sleep_time(env.night) || exhausted {
            self.work = None;
            self.work_target = None;
            self.go_to_bed(env);
            return;
        }
        // trajet vers le lit abandonné : l'heure de dormir est passée
        if self.goal == Some(self.home) {
            self.path.clear();
            self.goal = None;
        }
        if self.work.is_some() {
            return; // il continue son travail
        }
        match self.role {
            Role::Farmer => self.farm_routine(env, rng),
            Role::Blacksmith => self.forge_routine(env, rng),
            Role::Guard => self.patrol_routine(env, rng),
            _ => self.wander_routine(env, rng),
        }
    }

    fn go_to_bed(&mut self, env: &Env) {
        self.wait = 0.0;
        self.at_work = false;

        if self.path.is_empty() && self.tile() == self.home {
            self.lie_down();
            return;
        }
        if self.goal != Some(self.home) || self.path.is_empty() {
            self.path = env.map.find_path(self.tile(), self.home);
            self.goal = Some(self.home);
            self.activity = Activity::Idle; // si le lit est inaccessible, on attend sur place
            return;
        }
        self.follow_path(env);
    }

    /// Le paysan : travaille la parcelle sur laquelle il se trouve, sinon va à la plus utile
    /// (récolte, puis semis, puis labour), sinon se promène dans ses champs en attendant que ça pousse.
    /// Il ne vise jamais une parcelle déjà visée ou travaillée par un autre (`env.claimed`) :
    /// sans ça, tous les paysans se rueraient sur la même parcelle et leur travail serait doublé pour rien.
    fn farm_routine(&mut self, env: &Env, rng: &mut Rng) {
        if self.wait > 0.0 {
            self.wait -= env.dt;
            self.activity = Activity::Idle;
            return;
        }
        if !self.path.is_empty() {
            // la parcelle visée reste réservée pendant toute la marche, et jusqu'au tour suivant l'arrivée
            if self.follow_path(env) {
                self.goal = None;
            }
            return;
        }

        // à partir d'ici, le plan précédent est terminé : on en choisit un nouveau
        self.work_target = None;
        let here = self.tile();
        if let Some(kind) = env.farm.work_at(here) {
            if !env.claimed.contains(&here) {
                self.start_work(kind, here); // enchaîne sur place : récolte, labour, semis sans marcher
                return;
            }
        }
        if let Some(target) = env.farm.pick_target(here, env.claimed) {
            self.path = env.map.find_path(here, target);
            if self.path.is_empty() {
                self.wait = 1.0; // parcelle inaccessible : on réessaiera
            } else {
                self.goal = Some(target);
                self.work_target = Some(target);
            }
            self.activity = Activity::Idle;
            return;
        }
        self.wander_routine(env, rng); // rien à faire : tout pousse
    }

    /// Le forgeron : tant qu'il y a du fer, va se placer à son poste, juste au-dessus d'une enclume libre
    /// (voir `forge_station`), face à elle, et forge. Sans fer, il traîne dans sa forge en attendant.
    fn forge_routine(&mut self, env: &Env, rng: &mut Rng) {
        if self.wait > 0.0 {
            self.wait -= env.dt;
            self.activity = Activity::Idle;
            return;
        }
        if !self.path.is_empty() {
            if self.follow_path(env) {
                self.goal = None;
            }
            return;
        }

        self.work_target = None;
        let here = self.tile();
        let enough_iron = env.stock.get(Resource::Iron) >= env.cfg.iron_per_tool;
        if enough_iron {
            let anvils: Vec<TilePos> = env
                .map
                .find_tiles(Tile::Anvil)
                .into_iter()
                .filter(|a| !env.claimed.contains(a))
                .filter(|&a| !env.map.get(forge_station(a)).is_solid())
                .collect();

            // déjà à son poste devant une enclume libre : il se tourne vers elle et forge
            if let Some(&anvil) = anvils.iter().find(|&&a| forge_station(a) == here) {
                self.facing = Facing::Down;
                self.start_work(WorkKind::Forge, anvil);
                return;
            }

            // sinon : il marche jusqu'au poste de l'enclume libre la plus proche
            let best = anvils
                .iter()
                .filter_map(|&a| {
                    let path = env.map.find_path(here, forge_station(a));
                    (!path.is_empty()).then(|| (path.len(), a, path))
                })
                .min_by_key(|(len, _, _)| *len);
            if let Some((_, anvil, path)) = best {
                self.goal = path.back().copied();
                self.path = path;
                self.work_target = Some(anvil);
                self.activity = Activity::Idle;
                return;
            }
        }
        self.wander_routine(env, rng); // pas de fer ou pas d'enclume libre
    }

    /// Le garde : fait sa ronde, point par point dans l'ordre, et s'arrête un instant à chacun.
    fn patrol_routine(&mut self, env: &Env, rng: &mut Rng) {
        if self.wait > 0.0 {
            self.wait -= env.dt;
            self.activity = if self.at_work { Activity::Working } else { Activity::Idle };
            return;
        }
        self.at_work = false;

        if self.path.is_empty() {
            let target = self.work_spots[self.patrol_next % self.work_spots.len()];
            if target == self.tile() {
                // déjà arrivé (point de départ, par exemple) : on surveille un instant puis on repart
                self.patrol_next += 1;
                self.wait = rng.range_f32(1.5, 3.5);
                self.at_work = true;
                return;
            }
            self.path = env.map.find_path(self.tile(), target);
            if self.path.is_empty() {
                self.patrol_next += 1; // point inaccessible : on passe au suivant
                self.wait = 1.0;
            } else {
                self.goal = Some(target);
            }
            self.activity = Activity::Idle;
            return;
        }

        if self.follow_path(env) {
            self.goal = None;
            self.patrol_next += 1;
            self.wait = rng.range_f32(1.5, 3.5); // arrivé : il surveille un instant
            self.at_work = true;
        }
    }

    /// Va d'un poste de travail à l'autre et y reste un moment.
    /// (Pour les métiers qui n'ont pas encore de vrai travail.)
    fn wander_routine(&mut self, env: &Env, rng: &mut Rng) {
        if self.wait > 0.0 {
            self.wait -= env.dt;
            self.activity = if self.at_work { Activity::Working } else { Activity::Idle };
            return;
        }
        self.at_work = false;

        if self.path.is_empty() {
            let target = self.work_spots[rng.index(self.work_spots.len())];
            if target == self.tile() {
                self.wait = rng.range_f32(1.0, 2.5);
                self.at_work = true;
                return;
            }
            self.path = env.map.find_path(self.tile(), target);
            self.goal = Some(target);
            if self.path.is_empty() {
                self.goal = None;
                self.wait = 1.0; // cible inaccessible : on réessaiera
            }
            self.activity = Activity::Idle;
            return;
        }

        if self.follow_path(env) {
            self.goal = None;
            self.wait = rng.range_f32(2.0, 5.0); // arrivé : il "travaille" sur place
            self.at_work = true;
        }
    }
}
