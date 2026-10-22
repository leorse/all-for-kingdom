//! All For Kingdom (AFK) — cœur de jeu.
//!
//! Ce crate contient toute la logique : carte, personnages, IA, règles, temps, cultures.
//! Il ne sait rien de l'affichage, des touches du clavier, du son ou du réseau.
//! Un frontend (voir `afk-game`) lit son état et lui envoie des [`Command`].

pub mod character;
pub mod clock;
pub mod config;
pub mod container;
pub mod farm;
pub mod forest;
pub mod game;
pub mod map;
pub mod math;
pub mod mine;
pub mod resources;
pub mod transport;
pub mod rng;
pub mod scenario;
pub mod work;

pub use character::{Activity, Character, Facing, Role};
pub use clock::Clock;
pub use config::Config;
pub use container::{Container, ContainerKind, ContainerSpec};
pub use farm::{Farm, Plot, PlotState};
pub use forest::{Forest, LodgeSpec, Saplings};
pub use game::{CharacterId, Command, Game, PlayerId, TICK};
pub use map::{Tile, TileMap};
pub use math::{TilePos, Vec2};
pub use mine::{MineSpec, OreKind, Quality, Vein};
pub use resources::{Resource, Stock};
pub use transport::{Job, Reservation};
pub use scenario::Scenario;
pub use work::{Work, WorkDone, WorkKind};
