//! Le système de travail, commun au joueur et à l'IA.
//!
//! Un travail = une action sur une cible, qui prend du temps (barre de progression).
//! Joueur et IA utilisent exactement la même mécanique ; seuls changent celui qui décide de
//! l'action et la vitesse d'exécution (le joueur est plus rapide, la fatigue ralentit tout le monde).

use crate::character::Role;
use crate::config::Config;
use crate::math::TilePos;
use crate::mine::Vein;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkKind {
    /// Labourer une parcelle en friche.
    Till,
    /// Semer une parcelle labourée.
    Plant,
    /// Récolter une parcelle mûre.
    Harvest,
    /// Forger un outil sur une enclume (consomme du fer).
    Forge,
    /// Donner des coups de pioche sur un filon.
    Mine,
    /// Donner des coups de hache sur un arbre de forêt (à plusieurs : les coups s'additionnent).
    Chop,
}

impl WorkKind {
    /// Le métier qui sait faire ce travail.
    pub fn role(self) -> Role {
        match self {
            WorkKind::Till | WorkKind::Plant | WorkKind::Harvest => Role::Farmer,
            WorkKind::Forge => Role::Blacksmith,
            WorkKind::Mine => Role::Miner,
            WorkKind::Chop => Role::Lumberjack,
        }
    }

    /// Durée de base du travail, en secondes réelles.
    pub fn duration_secs(self, cfg: &Config) -> f32 {
        match self {
            WorkKind::Till => cfg.till_secs,
            WorkKind::Plant => cfg.plant_secs,
            WorkKind::Harvest => cfg.harvest_secs,
            WorkKind::Forge => cfg.forge_secs,
            WorkKind::Mine => cfg.strike_secs,
            WorkKind::Chop => cfg.chop_secs,
        }
    }
}

/// Durée d'un travail sur cette tuile. Miner dépend de la qualité du filon : nombre de coups × durée d'un coup.
pub fn work_secs(kind: WorkKind, tile: TilePos, cfg: &Config, veins: &[Vein]) -> f32 {
    match (kind, veins.iter().find(|v| v.contains(tile))) {
        (WorkKind::Mine, Some(vein)) => {
            let strikes = cfg.strikes_per_quality[vein.quality as usize];
            strikes as f32 * cfg.strike_secs
        }
        _ => kind.duration_secs(cfg),
    }
}

/// Un travail en cours sur un personnage.
#[derive(Clone, Copy, Debug)]
pub struct Work {
    pub kind: WorkKind,
    pub tile: TilePos,
    /// Avancement de 0.0 à 1.0.
    pub progress: f32,
    /// Durée totale de ce travail, en secondes réelles (pour l'IA en pleine forme).
    pub secs: f32,
}

/// Événement : un travail vient de se terminer. C'est `Game` qui en applique les effets au monde.
#[derive(Clone, Copy, Debug)]
pub struct WorkDone {
    pub kind: WorkKind,
    pub tile: TilePos,
}
