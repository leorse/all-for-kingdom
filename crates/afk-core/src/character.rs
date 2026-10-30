//! Les personnages de la cité : besoins (fatigue, sommeil), travail et comportement (contrôle direct ou IA).
//! Un personnage expose son état (`role`, `facing`, `activity`, `fatigue`, `work`...) ; c'est au rendu
//! de décider quel sprite, quelle animation ou quelle jauge en tirer.
//!
//! Un personnage ne modifie jamais le monde directement : quand un travail se termine, il renvoie un
//! `WorkDone` et c'est `Game` qui en applique les effets (parcelle, stock...).

use crate::config::Config;
use crate::container::Container;
use crate::farm::Farm;
use crate::game::CharacterId;
use crate::map::{PathMode, Tile, TileMap, ANVIL_CONTACT};
use crate::forest::Forest;
use crate::mine::Vein;
use crate::math::{TilePos, Vec2};
use crate::resources::Resource;
use crate::rng::Rng;
use crate::transport::{best_destination, best_trip, Job, Reservation};
use crate::work::{work_secs, Work, WorkDone, WorkKind};
use std::collections::VecDeque;

/// Vitesse d'un personnage dirigé par le joueur (tuiles par seconde).
pub const PLAYER_SPEED: f32 = 3.4;
/// Vitesse d'un personnage dirigé par l'IA : plus lent, c'est l'avantage d'incarner quelqu'un.
pub const AI_SPEED: f32 = 1.7;
/// Demi-côté de la boîte de collision (en tuiles).
pub const BODY_HALF: f32 = 0.28;
/// Distance (en tuiles) à laquelle on coupe le coin d'un trajet pour arrondir le virage.
const CORNER_RADIUS: f32 = 0.7;
/// Vitesse à laquelle le cap tourne vers le but (par seconde) : plus c'est bas, plus les virages sont larges.
const TURN_RATE: f32 = 7.0;

/// Contexte de simulation d'un pas, partagé par tous les personnages (en lecture seule).
pub(crate) struct Env<'a> {
    pub map: &'a TileMap,
    pub farm: &'a Farm,
    pub containers: &'a [Container],
    /// Ce que les AUTRES transporteurs ont promis de prendre ou d'apporter.
    pub reservations: &'a [Reservation],
    pub cfg: &'a Config,
    pub night: bool,
    /// Durée du pas en secondes réelles.
    pub dt: f32,
    /// Durée du pas en heures de jeu.
    pub dt_hours: f32,
    /// Tuiles visées ou travaillées par les AUTRES personnages en ce moment (voir `claimed_tile`).
    pub claimed: &'a [TilePos],
    pub veins: &'a [Vein],
    pub forests: &'a [Forest],
}

impl Env<'_> {
    /// Trajet ordinaire de `from` à `to` : il suit les routes (voir `PathMode::Roads`).
    pub fn path(&self, from: TilePos, to: TilePos) -> VecDeque<TilePos> {
        self.map.find_path_mode(from, to, self.roads())
    }

    /// Trajet ordinaire et son coût, pour comparer plusieurs buts.
    pub fn path_cost(&self, from: TilePos, to: TilePos) -> Option<(VecDeque<TilePos>, f32)> {
        self.map.find_path_cost(from, to, self.roads())
    }

    /// Trajet le plus court en distance, sans suivre les routes : fuite d'un voleur vers la sortie du château,
    /// poursuite d'un voleur par les gardes.
    #[allow(dead_code)] // en attente des voleurs (voir IDEAS.md, « Bandits »)
    pub fn direct_path(&self, from: TilePos, to: TilePos) -> VecDeque<TilePos> {
        self.map.find_path_mode(from, to, PathMode::Direct)
    }

    fn roads(&self) -> PathMode {
        PathMode::Roads(self.cfg.grass_cost)
    }

    /// Place restante dans ce contenant pour cette ressource (0 s'il n'y a pas de contenant).
    pub fn room(&self, container: Option<usize>, resource: Resource) -> u32 {
        container.and_then(|i| self.containers.get(i)).map_or(0, |c| c.room(resource))
    }

    pub fn stored(&self, container: Option<usize>, resource: Resource) -> u32 {
        container.and_then(|i| self.containers.get(i)).map_or(0, |c| c.get(resource))
    }
}

/// Ce qu'un pas de personnage demande à `Game` d'appliquer au monde.
pub(crate) enum Outcome {
    /// Un travail vient de se terminer.
    Work(WorkDone),
    /// Prendre jusqu'à `amount` de `resource` dans un contenant, en main.
    Take { container: usize, resource: Resource, amount: u32 },
    /// Déposer sa cargaison dans un contenant.
    Drop { container: usize },
    /// Le joueur, conduit jusqu'au poste de forge de cette enclume, vient d'y arriver (ou s'est arrêté en route).
    Arrived { anvil: TilePos },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Farmer,
    Blacksmith,
    King,
    Warlord,
    Innkeeper,
    Guard,
    Miner,
    Lumberjack,
    Transporter,
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

fn is_adjacent(a: TilePos, b: TilePos) -> bool {
    (a.x - b.x).abs() + (a.y - b.y).abs() == 1
}

/// La case juste au nord d'une enclume, par laquelle on rejoint son poste de forge.
pub fn forge_spot(anvil: TilePos) -> TilePos {
    TilePos::new(anvil.x, anvil.y - 1)
}

/// Le personnage est-il à un dernier pas du poste de forge de `anvil` : sur la case au nord, ou dans la moitié libre
/// de la case de l'enclume ?
fn beside_forge_post(tile: TilePos, anvil: TilePos) -> bool {
    tile == forge_spot(anvil) || tile == anvil
}

/// Le point exact où le forgeron se tient pour forger, dans la moitié libre de la case de l'enclume : centré sur elle,
/// les pieds contre le socle (voir `ANVIL_CONTACT`). Debout ou au travail, il y est dessiné à la même hauteur.
pub fn forge_post(anvil: TilePos) -> Vec2 {
    Vec2::new(anvil.x as f32 + 0.5, anvil.y as f32 + ANVIL_CONTACT)
}

/// Le personnage est-il au poste de forge de `anvil` (au point près, pas seulement sur la bonne case) ?
pub fn at_forge_post(pos: Vec2, anvil: TilePos) -> bool {
    pos.distance(forge_post(anvil)) < 0.02
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
    /// Pour un mineur : indice du filon de sa mine, le seul qu'il exploite.
    pub vein: Option<usize>,
    /// Pour un bûcheron : indice de la forêt de sa cabane, la seule où il coupe.
    pub forest: Option<usize>,
    /// Ce que le personnage porte en main : bois d'un bûcheron, charge d'un transporteur.
    pub cargo: Option<(Resource, u32)>,
    /// Son contenant : celui où il dépose sa production ou livre son bois.
    pub container: Option<usize>,
    /// Le voyage d'un transporteur en cours.
    pub(crate) job: Option<Job>,

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
    /// Direction de marche lissée (voir `follow_path`).
    heading: Vec2,
    /// Part d'arbre abattue pendant ce pas par ce bûcheron (voir `Game::step`, qui additionne les coups).
    chop_delta: f32,
    /// Un porteur qui s'est couché avec du bois en main le livre à son réveil avant de reprendre le travail.
    deliver_first: bool,
    /// Chargement ou déchargement demandé pendant ce pas : c'est `Game` qui l'applique.
    outcome: Option<Outcome>,
    /// Le joueur a demandé de forger depuis le côté de cette enclume : le personnage marche seul jusqu'à son poste.
    approach: Option<TilePos>,
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
            vein: None,
            forest: None,
            cargo: None,
            container: None,
            job: None,
            deliver_first: false,
            outcome: None,
            approach: None,
            chop_delta: 0.0,
            work_spots,
            path: VecDeque::new(),
            goal: None,
            work_target: None,
            patrol_next: 0,
            wait: 0.0,
            at_work: false,
            stuck: 0.0,
            heading: Vec2::ZERO,
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
        // qui mine ou abat occupe une case voisine de sa cible : c'est elle qui est réservée, pas la cible
        // (un filon est infini, un arbre se coupe à plusieurs)
        let worked = self.work.map(|w| if matches!(w.kind, WorkKind::Mine | WorkKind::Chop) { self.tile() } else { w.tile });
        worked.or(self.work_target)
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

    /// Les transporteurs courent : ils vont plus vite que les autres, à pied comme sous le contrôle du joueur.
    fn speed_factor(&self, cfg: &Config) -> f32 {
        if self.role == Role::Transporter {
            cfg.transporter_speed_factor
        } else {
            1.0
        }
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
    pub(crate) fn update(&mut self, env: &Env, control: Option<Vec2>, rng: &mut Rng) -> Option<Outcome> {
        self.chop_delta = 0.0;
        self.outcome = None;
        if self.asleep {
            self.sleep_tick(env); // dort, qu'il soit incarné ou non
            return None;
        }
        match control {
            Some(dir) => self.drive(dir, env),
            None => self.run_ai(env, rng),
        }
        let done = self.advance_work(env, control.is_some()).map(Outcome::Work);
        self.tick_fatigue(env);
        done.or(self.outcome.take())
    }

    // ------------------------------------------------------------ sommeil

    pub(crate) fn lie_down(&mut self) {
        self.deliver_first = self.cargo.is_some(); // il garde sa cargaison et la livrera à son réveil
        self.pos = self.home.center();
        self.asleep = true;
        self.activity = Activity::Sleeping;
        self.facing = Facing::Down;
        self.work = None;
        self.approach = None;
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
    pub(crate) fn start_work(&mut self, kind: WorkKind, tile: TilePos, secs: f32) -> bool {
        if kind.role() != self.role || self.asleep {
            return false;
        }
        self.work = Some(Work { kind, tile, progress: 0.0, secs });
        self.activity = Activity::Working;
        true
    }

    pub(crate) fn cancel_work(&mut self) {
        self.work = None;
    }

    /// Part seul vers le poste de forge de `anvil` par `path` (voir `drive`). L'enclume est réservée en route.
    pub(crate) fn start_approach(&mut self, anvil: TilePos, path: VecDeque<TilePos>) {
        self.approach = Some(anvil);
        self.path = path;
        self.work_target = Some(anvil);
    }

    pub(crate) fn cancel_approach(&mut self) {
        self.approach = None;
        self.path.clear();
        self.work_target = None;
        self.stuck = 0.0;
    }

    /// Est-il en route vers un poste de forge, à la demande du joueur ?
    pub fn is_approaching(&self) -> bool {
        self.approach.is_some()
    }

    /// Part d'arbre que ce bûcheron a abattue pendant le dernier pas (0 s'il ne coupe pas).
    pub(crate) fn chop_delta(&self) -> f32 {
        self.chop_delta
    }

    /// Affiche l'avancement partagé de l'arbre sur la barre de ce bûcheron.
    pub(crate) fn set_work_progress(&mut self, progress: f32) {
        if let Some(work) = self.work.as_mut() {
            work.progress = progress;
        }
    }

    /// Fait avancer le travail en cours. Le joueur travaille plus vite que l'IA ; la fatigue ralentit.
    fn advance_work(&mut self, env: &Env, by_player: bool) -> Option<WorkDone> {
        let efficiency = self.efficiency();
        let work = self.work.as_mut()?;
        let speed = if by_player { env.cfg.player_work_speed } else { 1.0 };
        work.progress += env.dt / work.secs * speed * efficiency;
        if work.kind == WorkKind::Chop {
            // un arbre se coupe à plusieurs : `Game` additionne les coups de tous et décide de la chute
            self.chop_delta = env.dt / work.secs * speed * efficiency;
            work.progress = 0.0;
            self.activity = Activity::Working;
            return None;
        }
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
            } else if delta.y > 0.0 {
                // contre le socle d'une enclume : les pieds s'y posent exactement, sans s'arrêter un pas avant
                if let Some(y) = map.anvil_contact(next, BODY_HALF).filter(|&y| y > self.pos.y) {
                    let contact = Vec2::new(self.pos.x, y);
                    if !map.is_box_blocked(contact, BODY_HALF) {
                        self.pos = contact;
                    }
                }
            }
        }
    }

    /// Contrôle direct par un joueur. Bouger interrompt le travail en cours.
    fn drive(&mut self, dir: Vec2, env: &Env) {
        let dir = dir.normalized_or_zero();
        if let Some(anvil) = self.approach {
            if dir == Vec2::ZERO {
                // conduit à son poste de forge : le chemin jusqu'à la case, puis le dernier pas jusqu'au point exact ;
                // à l'arrivée, c'est `Game` qui lance la forge
                if !self.path.is_empty() {
                    self.follow_path(env);
                } else if !beside_forge_post(self.tile(), anvil) {
                    self.cancel_approach(); // chemin abandonné en route (bloqué) : il s'arrête là
                    self.activity = Activity::Idle;
                } else if self.step_to_point(forge_post(anvil), env) {
                    self.approach = None;
                    self.work_target = None;
                    self.activity = Activity::Idle;
                    self.facing = Facing::Down;
                    self.outcome = Some(Outcome::Arrived { anvil });
                }
                return;
            }
            self.cancel_approach(); // le joueur reprend la main
        }
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
        let speed = PLAYER_SPEED * self.speed_factor(env.cfg) * self.efficiency();
        self.try_move(dir * (speed * env.dt), env.map);
        self.facing = Facing::from_dir(dir); // même contre un obstacle : on le regarde
        self.activity = Activity::Walking;
    }

    /// Le joueur quitte ce personnage : l'IA reprend depuis l'endroit où il se trouve.
    /// S'il dormait, il continue de dormir ; s'il travaillait, il termine son travail.
    pub(crate) fn release_to_ai(&mut self) {
        self.approach = None;
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
        if self.path.is_empty() {
            return false;
        }
        let was_walking = self.activity == Activity::Walking;
        self.activity = Activity::Walking;

        // Virage : on coupe le coin dès qu'on en est proche, si le segment vers la case suivante est libre.
        // Le dernier point n'est jamais coupé : on arrive exactement sur le but.
        while self.path.len() > 1
            && self.pos.distance(self.path[0].center()) < CORNER_RADIUS
            && env.map.is_segment_clear(self.pos, self.path[1].center(), BODY_HALF)
        {
            self.path.pop_front();
        }

        let goal = self.path[0].center();
        let to_goal = goal - self.pos;
        let dist = to_goal.length();
        let step = AI_SPEED * self.speed_factor(env.cfg) * self.efficiency() * env.dt;

        if dist <= step.max(0.03) {
            self.pos = goal;
            self.path.pop_front();
            self.stuck = 0.0;
            return self.path.is_empty();
        }

        // Le cap tourne progressivement vers le but : les changements de direction deviennent des courbes.
        // Près du but, ou au départ, on vise droit pour ne pas tourner autour.
        let desired = to_goal * (1.0 / dist);
        let steered = (self.heading + (desired - self.heading) * (TURN_RATE * env.dt).min(1.0)).normalized_or_zero();
        let dir = if !was_walking || dist < CORNER_RADIUS || steered == Vec2::ZERO { desired } else { steered };
        self.heading = dir;
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

    /// Dernier pas, en ligne droite, jusqu'à `target` dans la case où il se trouve déjà (sans obstacle à contourner).
    /// Renvoie `true` à l'arrivée.
    fn step_to_point(&mut self, target: Vec2, env: &Env) -> bool {
        let to_target = target - self.pos;
        let dist = to_target.length();
        let step = AI_SPEED * self.speed_factor(env.cfg) * self.efficiency() * env.dt;
        if dist <= step.max(0.005) {
            self.pos = target;
            return true;
        }
        let dir = to_target * (1.0 / dist);
        self.pos = self.pos + dir * step;
        self.facing = Facing::from_dir(dir);
        self.activity = Activity::Walking;
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
            Role::Miner => self.mine_routine(env, rng),
            Role::Lumberjack => self.chop_routine(env, rng),
            Role::Transporter => self.transport_routine(env, rng),
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
            self.path = env.path(self.tile(), self.home);
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
        // le blé récolté se porte à la main jusqu'au coffre du champ (voir `deliver`)
        let carried = self.cargo.map_or(0, |(_, n)| n);
        let can_harvest = carried + env.cfg.wheat_per_harvest <= env.cfg.farmer_carry;
        let chest_has_room = env.room(self.container, Resource::Wheat) > 0;
        // mains pleines (ou réveil avec du blé) : on livre, si le coffre peut recevoir ; sinon on continue de cultiver
        if carried > 0 && chest_has_room && (self.deliver_first || !can_harvest) {
            self.deliver(env);
            return;
        }
        if let Some(kind) = env.farm.work_at(here).filter(|&k| k != WorkKind::Harvest || can_harvest) {
            if !env.claimed.contains(&here) {
                self.start_work(kind, here, kind.duration_secs(env.cfg)); // enchaîne sur place : récolte, labour, semis sans marcher
                return;
            }
        }
        if let Some(target) = env.farm.pick_target_with(here, env.claimed, can_harvest) {
            self.path = env.path(here, target);
            if self.path.is_empty() {
                self.wait = 1.0; // parcelle inaccessible : on réessaiera
            } else {
                self.goal = Some(target);
                self.work_target = Some(target);
            }
            self.activity = Activity::Idle;
            return;
        }
        if carried > 0 && chest_has_room {
            self.deliver(env); // rien à cultiver : on livre ce qu'on porte
            return;
        }
        self.wander_routine(env, rng); // rien à faire : tout pousse
    }

    /// Le forgeron : tant qu'il y a du fer, va se placer au poste d'une enclume libre (au nord, contre son socle), regarde
    /// vers l'enclume et forge.
    /// Sans fer, il traîne dans sa forge en attendant.
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
        // la forge a besoin de fer dans son coffre et de place pour y ranger l'outil
        let enough_iron = env.stored(self.container, Resource::Iron) >= env.cfg.iron_per_tool
            && env.room(self.container, Resource::Tools) >= env.cfg.tools_per_forge;
        if enough_iron {
            let anvils: Vec<TilePos> = env
                .map
                .find_tiles(Tile::Anvil)
                .into_iter()
                .filter(|a| !env.claimed.contains(a))
                .collect();

            // à un dernier pas du poste d'une enclume libre : on se place au point exact, on la regarde et on forge
            if let Some(&anvil) = anvils.iter().find(|&&a| beside_forge_post(here, a)) {
                if !self.step_to_point(forge_post(anvil), env) {
                    self.work_target = Some(anvil);
                    return;
                }
                self.facing = Facing::Down;
                self.start_work(WorkKind::Forge, anvil, WorkKind::Forge.duration_secs(env.cfg));
                return;
            }

            // sinon : on marche jusqu'au poste de forge libre le plus proche
            let best = anvils
                .iter()
                .map(|&a| (a, forge_spot(a)))
                .filter_map(|(a, n)| {
                    let (path, cost) = env.path_cost(here, n)?;
                    (!path.is_empty()).then(|| (cost, a, path))
                })
                .min_by(|a, b| a.0.total_cmp(&b.0));
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

    /// Le mineur : va se placer sur la case libre la plus proche de son filon, le regarde et mine.
    /// La case réservée (`work_target`) est celle où il se tient, pour que les mineurs d'un même filon
    /// ne se marchent pas dessus. Sans filon attitré, il traîne.
    fn mine_routine(&mut self, env: &Env, rng: &mut Rng) {
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
        let Some(vein) = self.vein.and_then(|i| env.veins.get(i)) else {
            self.wander_routine(env, rng);
            return;
        };
        let here = self.tile();

        // déjà contre le filon, sur une case que personne d'autre n'occupe : on le regarde et on mine
        if !env.claimed.contains(&here) {
            if let Some(ore) = vein.tiles().into_iter().find(|&t| is_adjacent(here, t)) {
                self.facing = Facing::from_dir(ore.center() - self.pos);
                if env.room(self.container, vein.kind.resource()) == 0 {
                    self.activity = Activity::Idle; // le coffre de la mine est plein : on attend sur place
                    return;
                }
                let secs = work_secs(WorkKind::Mine, ore, env.cfg, env.veins);
                self.start_work(WorkKind::Mine, ore, secs);
                return;
            }
        }

        // sinon : on marche jusqu'à la case libre la plus proche
        let best = vein
            .access_tiles(env.map)
            .into_iter()
            .filter(|p| !env.claimed.contains(p))
            .filter_map(|p| {
                let (path, cost) = env.path_cost(here, p)?;
                (!path.is_empty()).then(|| (cost, p, path))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0));
        if let Some((_, post, path)) = best {
            self.goal = Some(post);
            self.path = path;
            self.work_target = Some(post);
            self.activity = Activity::Idle;
            return;
        }
        self.wander_routine(env, rng); // tous les postes sont pris ou inaccessibles
    }

    /// Le bûcheron : va se placer sur la case libre la plus proche d'un arbre de sa forêt, le regarde et
    /// l'abat. Un arbre déjà attaqué par un autre n'est pas évité (on coupe à plusieurs), seule la case
    /// où l'on se tient est réservée. Sans arbre à portée, il traîne près de sa forêt en attendant la repousse.
    fn chop_routine(&mut self, env: &Env, rng: &mut Rng) {
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
        let Some(forest) = self.forest.and_then(|i| env.forests.get(i)) else {
            self.wander_routine(env, rng);
            return;
        };
        let here = self.tile();
        let is_tree = |t: TilePos| forest.contains(t) && env.map.get(t) == Tile::Tree;
        let carried = self.cargo.map_or(0, |(_, n)| n);
        let room_for_a_tree = carried + env.cfg.logs_per_tree <= env.cfg.lumberjack_carry;
        // mains pleines (ou réveil avec du bois) : on livre avant d'abattre quoi que ce soit
        if carried > 0 && (self.deliver_first || !room_for_a_tree) {
            self.deliver(env);
            return;
        }

        // déjà contre un arbre de la forêt, sur une case que personne d'autre n'occupe : on le regarde et on coupe
        if room_for_a_tree && !env.claimed.contains(&here) {
            let next_to = [(0, -1), (1, 0), (0, 1), (-1, 0)]
                .iter()
                .map(|&(dx, dy)| TilePos::new(here.x + dx, here.y + dy))
                .find(|&t| is_tree(t));
            if let Some(tree) = next_to {
                self.facing = Facing::from_dir(tree.center() - self.pos);
                self.start_work(WorkKind::Chop, tree, WorkKind::Chop.duration_secs(env.cfg));
                return;
            }
        }

        // sinon : on marche vers la case libre la plus proche d'un arbre (cases triées par distance, on prend
        // la première joignable pour ne pas lancer une recherche de chemin par arbre)
        let mut posts: Vec<TilePos> = forest
            .trees(env.map)
            .into_iter()
            .flat_map(|t| env.map.walkable_neighbors(t))
            .filter(|p| !env.claimed.contains(p))
            .collect();
        posts.sort_by_key(|p| ((p.x - here.x).abs() + (p.y - here.y).abs(), p.x, p.y));
        posts.dedup();
        for post in posts {
            let path = env.path(here, post);
            if !path.is_empty() {
                self.goal = Some(post);
                self.path = path;
                self.work_target = Some(post);
                self.activity = Activity::Idle;
                return;
            }
        }
        if carried > 0 {
            self.deliver(env); // plus d'arbre à portée : on livre ce qu'on porte
            return;
        }
        self.wander_routine(env, rng); // plus d'arbre à portée : on attend la repousse
    }

    /// Va se placer à côté du coffre posé sur `crate_tile` (la case voisine la plus proche) et le regarde.
    /// Renvoie `true` quand on y est déjà : l'appelant peut alors prendre ou déposer.
    fn approach_crate(&mut self, env: &Env, crate_tile: TilePos) -> bool {
        let here = self.tile();
        if is_adjacent(here, crate_tile) {
            self.facing = Facing::from_dir(crate_tile.center() - self.pos);
            self.activity = Activity::Idle;
            return true;
        }
        let best = env
            .map
            .walkable_neighbors(crate_tile)
            .into_iter()
            .filter_map(|n| {
                let (path, cost) = env.path_cost(here, n)?;
                (!path.is_empty()).then(|| (cost, n, path))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0));
        match best {
            Some((_, n, path)) => {
                self.goal = Some(n);
                self.path = path;
            }
            None => self.wait = 1.0, // coffre inaccessible : on réessaiera
        }
        self.activity = Activity::Idle;
        false
    }

    /// Un bûcheron livre son bois au coffre de sa cabane : il y va, dépose ce qui rentre, et si le coffre
    /// est plein il attend là, avec le reste en main.
    fn deliver(&mut self, env: &Env) {
        let Some(ci) = self.container else { return };
        let Some(container) = env.containers.get(ci) else { return };
        if self.approach_crate(env, container.tile) {
            self.outcome = Some(Outcome::Drop { container: ci });
            self.deliver_first = false;
            self.wait = 1.0; // si le coffre est plein, on laisse passer un moment avant de réessayer
        }
    }

    /// Le transporteur : choisit tout seul son voyage, va chercher la marchandise, la porte, et recommence.
    /// Sa cargaison reste en main la nuit : il la livre à son réveil avant de choisir autre chose.
    fn transport_routine(&mut self, env: &Env, rng: &mut Rng) {
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
        let here = self.tile();

        // les mains pleines : on livre
        if let Some((resource, _)) = self.cargo {
            let planned = self.job.filter(|j| env.containers.get(j.to).map_or(false, |c| c.room(resource) > 0));
            let to = planned.map(|j| j.to).or_else(|| {
                best_destination(env.containers, env.reservations, self.id, resource, here)
            });
            let Some(to) = to else {
                self.wait = 1.0; // personne ne peut la recevoir : on attend avec la cargaison
                return;
            };
            if let Some(job) = self.job.as_mut() {
                job.to = to;
            }
            if self.approach_crate(env, env.containers[to].tile) {
                self.outcome = Some(Outcome::Drop { container: to });
                self.wait = 0.2;
            }
            return;
        }

        // les mains vides, un voyage décidé : on va chercher la marchandise
        if let Some(job) = self.job {
            if self.approach_crate(env, env.containers[job.from].tile) {
                self.outcome = Some(Outcome::Take { container: job.from, resource: job.resource, amount: job.amount });
                self.wait = 0.2;
            }
            return;
        }

        // rien en cours : on cherche le meilleur voyage, sinon on attend à l'entrepôt central
        match best_trip(env.containers, env.reservations, self.id, env.cfg.transport_load, here) {
            Some(job) => self.job = Some(job),
            None => self.wander_routine(env, rng),
        }
    }

    /// `Game` : la prise est faite, le personnage porte maintenant `taken` unités. Rien de pris : le voyage tombe.
    pub(crate) fn loaded(&mut self, resource: Resource, taken: u32) {
        if taken == 0 {
            self.job = None;
            return;
        }
        self.cargo = Some((resource, taken));
        if let Some(job) = self.job.as_mut() {
            job.amount = taken;
        }
    }

    /// `Game` : un dépôt vient d'avoir lieu ; `left` unités n'ont pas pu entrer et restent en main.
    pub(crate) fn unloaded(&mut self, left: u32) {
        self.cargo = match self.cargo {
            Some((resource, _)) if left > 0 => Some((resource, left)),
            _ => None,
        };
        if self.cargo.is_none() {
            self.job = None;
        }
    }

    /// `Game` : ajoute à ce qu'on porte (le bois d'un arbre abattu).
    pub(crate) fn pick_up(&mut self, resource: Resource, amount: u32) {
        let held = match self.cargo {
            Some((r, n)) if r == resource => n,
            _ => 0,
        };
        self.cargo = Some((resource, held + amount));
    }

    /// Ce que ce transporteur a promis de prendre et d'apporter, pour que les autres n'en tiennent pas pour acquis.
    pub(crate) fn reservations(&self) -> Vec<Reservation> {
        let Some(job) = self.job else { return Vec::new() };
        let mut list = Vec::new();
        match self.cargo {
            None => {
                list.push(Reservation { owner: self.id, container: job.from, resource: job.resource, amount: job.amount, incoming: false });
                list.push(Reservation { owner: self.id, container: job.to, resource: job.resource, amount: job.amount, incoming: true });
            }
            Some((resource, amount)) => {
                list.push(Reservation { owner: self.id, container: job.to, resource, amount, incoming: true });
            }
        }
        list
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
            self.path = env.path(self.tile(), target);
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
            self.path = env.path(self.tile(), target);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::PathMode;

    const DT: f32 = 0.05;

    fn tp(x: i32, y: i32) -> TilePos {
        TilePos::new(x, y)
    }

    fn walker(map: &TileMap, at: TilePos, home: TilePos) -> Character {
        let _ = map;
        Character::new(CharacterId(0), "Test".into(), Role::Farmer, home, false, vec![at])
    }

    /// Fait marcher `c` le long de son trajet et renvoie les positions successives.
    fn walk(c: &mut Character, map: &TileMap, night: bool, max_steps: usize) -> Vec<Vec2> {
        let farm = Farm::from_map(map);
        let cfg = Config::default();
        let mut trail = vec![c.pos];
        for _ in 0..max_steps {
            let env = Env {
                map,
                farm: &farm,
                containers: &[],
                reservations: &[],
                cfg: &cfg,
                night,
                dt: DT,
                dt_hours: 0.0,
                claimed: &[],
                veins: &[],
                forests: &[],
            };
            if night {
                c.go_to_bed(&env);
                if c.asleep {
                    break;
                }
            } else if c.follow_path(&env) {
                break;
            }
            trail.push(c.pos);
        }
        trail
    }

    fn turn_degrees(trail: &[Vec2]) -> f32 {
        let mut worst: f32 = 0.0;
        for w in trail.windows(3) {
            let (a, b) = (w[1] - w[0], w[2] - w[1]);
            if a.length() < 1e-4 || b.length() < 1e-4 {
                continue;
            }
            let cos = ((a.x * b.x + a.y * b.y) / (a.length() * b.length())).clamp(-1.0, 1.0);
            worst = worst.max(cos.acos().to_degrees());
        }
        worst
    }

    #[test]
    fn a_walker_cuts_across_open_ground_on_the_diagonal_and_arrives() {
        let map = TileMap::new(20, 20, Tile::Grass);
        let mut c = walker(&map, tp(2, 2), tp(2, 2));
        c.path = map.find_path_mode(tp(2, 2), tp(12, 12), PathMode::Direct);
        let trail = walk(&mut c, &map, false, 600);
        assert!(c.path.is_empty(), "arrivé");
        assert_eq!(c.tile(), tp(12, 12));
        assert!(trail.len() < 400, "pas de blocage sur la diagonale : {} pas", trail.len());
        // en diagonale : bien plus court que l'escalier de 20 cases
        let length: f32 = trail.windows(2).map(|w| w[0].distance(w[1])).sum();
        assert!(length < 15.5, "trajet {length}");
    }

    #[test]
    fn the_route_to_a_chest_off_the_road_is_a_smooth_curve() {
        let mut map = TileMap::new(30, 16, Tile::Grass);
        map.fill_rect(0, 12, 30, 1, Tile::Path);
        map.set(tp(20, 5), Tile::Crate);
        let mut c = walker(&map, tp(2, 12), tp(2, 12));
        c.pos = tp(2, 12).center();
        c.path = map.find_path_mode(tp(2, 12), tp(20, 6), PathMode::Roads(3.0));
        let trail = walk(&mut c, &map, false, 2000);
        assert!(c.path.is_empty());
        assert_eq!(c.tile(), tp(20, 6));
        assert!(trail.iter().all(|p| !map.is_box_blocked(*p, BODY_HALF)), "aucun point dans un obstacle");
        let worst = turn_degrees(&trail);
        assert!(worst < 25.0, "virage trop vif : {worst}°");
        let on_road = trail.iter().filter(|p| map.get(p.tile()) == Tile::Path).count();
        assert!(on_road * 2 > trail.len(), "il suit la route");
    }

    #[test]
    fn a_walker_goes_round_a_wall_corner_without_crossing_it() {
        let mut map = TileMap::new(20, 20, Tile::Grass);
        map.fill_rect(8, 4, 1, 12, Tile::Wall);
        let mut c = walker(&map, tp(4, 10), tp(4, 10));
        c.path = map.find_path_mode(tp(4, 10), tp(14, 10), PathMode::Direct);
        let trail = walk(&mut c, &map, false, 3000);
        assert_eq!(c.tile(), tp(14, 10));
        assert!(trail.iter().all(|p| !map.is_box_blocked(*p, BODY_HALF)));
    }

    #[test]
    fn the_direct_path_cuts_across_where_the_ordinary_path_follows_the_road() {
        let mut map = TileMap::new(30, 30, Tile::Grass);
        map.fill_rect(0, 2, 28, 1, Tile::Path);
        map.fill_rect(27, 2, 1, 26, Tile::Path);
        let farm = Farm::from_map(&map);
        let cfg = Config::default();
        let env = Env {
            map: &map,
            farm: &farm,
            containers: &[],
            reservations: &[],
            cfg: &cfg,
            night: false,
            dt: DT,
            dt_hours: 0.0,
            claimed: &[],
            veins: &[],
            forests: &[],
        };
        let on_road = |p: &VecDeque<TilePos>| p.iter().filter(|t| map.get(**t) == Tile::Path).count();
        let chase = env.direct_path(tp(0, 2), tp(27, 27));
        let patrol = env.path(tp(0, 2), tp(27, 27));
        assert!(chase.len() < patrol.len());
        assert!(on_road(&chase) < on_road(&patrol), "la poursuite coupe, la ronde suit la route");
    }

    #[test]
    fn going_to_bed_takes_the_road() {
        let mut map = TileMap::new(30, 16, Tile::Grass);
        map.fill_rect(0, 12, 30, 1, Tile::Path);
        let home = tp(25, 12);
        let mut c = walker(&map, tp(3, 5), home);
        let trail = walk(&mut c, &map, true, 4000);
        assert!(c.asleep, "il est allé se coucher");
        let on_road = trail.iter().filter(|p| map.get(p.tile()) == Tile::Path).count();
        assert!(on_road > trail.len() / 2, "il rejoint la route puis la suit : {on_road}/{}", trail.len());
    }
}
