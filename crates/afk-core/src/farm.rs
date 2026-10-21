//! Les cultures : chaque tuile de champ est une parcelle qui passe par plusieurs états.
//!
//!   en friche --labourer--> labourée --planter--> plantée --(temps)--> mûre --récolter--> en friche
//!
//! La pousse ne dépend que du temps de jeu : personne ne peut l'accélérer, joueur ou IA.

use crate::map::{Tile, TileMap};
use crate::math::TilePos;
use crate::work::WorkKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlotState {
    Fallow,
    Tilled,
    /// `growth` : heures de jeu écoulées depuis le semis.
    Planted { growth: f32 },
    Ripe,
}

impl PlotState {
    /// Le travail possible sur une parcelle dans cet état (aucun pendant la pousse).
    pub fn next_work(self) -> Option<WorkKind> {
        match self {
            PlotState::Fallow => Some(WorkKind::Till),
            PlotState::Tilled => Some(WorkKind::Plant),
            PlotState::Planted { .. } => None,
            PlotState::Ripe => Some(WorkKind::Harvest),
        }
    }

    /// Avancement de la pousse de 0.0 à 1.0 (`total_hours` : durée de pousse complète).
    pub fn growth_fraction(self, total_hours: f32) -> f32 {
        match self {
            PlotState::Planted { growth } => (growth / total_hours).clamp(0.0, 1.0),
            PlotState::Ripe => 1.0,
            _ => 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Plot {
    pub tile: TilePos,
    pub state: PlotState,
}

pub struct Farm {
    plots: Vec<Plot>,
}

/// Plus la valeur est basse, plus c'est prioritaire : on récolte avant de semer avant de labourer.
fn priority(kind: WorkKind) -> u8 {
    match kind {
        WorkKind::Harvest => 0,
        WorkKind::Plant => 1,
        WorkKind::Till => 2,
        WorkKind::Forge => 3, // pas un travail de ferme : n'arrive jamais ici
    }
}

impl Farm {
    /// Une parcelle (en friche) par tuile de champ de la carte.
    pub fn from_map(map: &TileMap) -> Farm {
        let mut plots = Vec::new();
        for y in 0..map.height() {
            for x in 0..map.width() {
                let tile = TilePos::new(x, y);
                if map.get(tile) == Tile::Field {
                    plots.push(Plot { tile, state: PlotState::Fallow });
                }
            }
        }
        Farm { plots }
    }

    pub fn plots(&self) -> &[Plot] {
        &self.plots
    }

    pub fn state_at(&self, tile: TilePos) -> Option<PlotState> {
        self.plots.iter().find(|p| p.tile == tile).map(|p| p.state)
    }

    /// Le travail possible sur cette tuile, s'il y en a un.
    pub fn work_at(&self, tile: TilePos) -> Option<WorkKind> {
        self.state_at(tile).and_then(|s| s.next_work())
    }

    /// La parcelle la plus utile où aller travailler : d'abord par priorité de travail
    /// (récolte, semis, labour), puis la plus proche de `from`.
    /// `claimed` : parcelles déjà visées ou travaillées par quelqu'un d'autre, à laisser tranquilles.
    pub fn pick_target(&self, from: TilePos, claimed: &[TilePos]) -> Option<TilePos> {
        self.plots
            .iter()
            .filter(|p| !claimed.contains(&p.tile))
            .filter_map(|p| p.state.next_work().map(|kind| (kind, p.tile)))
            .min_by_key(|&(kind, t)| (priority(kind), (t.x - from.x).abs() + (t.y - from.y).abs()))
            .map(|(_, t)| t)
    }

    /// Applique le résultat d'un travail terminé. Renvoie `false` si la parcelle n'était plus
    /// dans l'état attendu (rien ne change alors).
    pub(crate) fn apply(&mut self, kind: WorkKind, tile: TilePos) -> bool {
        let Some(plot) = self.plots.iter_mut().find(|p| p.tile == tile) else {
            return false;
        };
        plot.state = match (kind, plot.state) {
            (WorkKind::Till, PlotState::Fallow) => PlotState::Tilled,
            (WorkKind::Plant, PlotState::Tilled) => PlotState::Planted { growth: 0.0 },
            (WorkKind::Harvest, PlotState::Ripe) => PlotState::Fallow,
            _ => return false,
        };
        true
    }

    /// Fait pousser les cultures de `dt_hours` heures de jeu.
    pub(crate) fn tick(&mut self, dt_hours: f32, growth_hours: f32) {
        for plot in &mut self.plots {
            let mut ripe = false;
            if let PlotState::Planted { growth } = &mut plot.state {
                *growth += dt_hours;
                ripe = *growth >= growth_hours;
            }
            if ripe {
                plot.state = PlotState::Ripe;
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn set_state(&mut self, tile: TilePos, state: PlotState) {
        if let Some(p) = self.plots.iter_mut().find(|p| p.tile == tile) {
            p.state = state;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_farm() -> Farm {
        let mut map = TileMap::new(6, 3, Tile::Grass);
        map.fill_rect(1, 1, 4, 1, Tile::Field);
        Farm::from_map(&map)
    }

    #[test]
    fn one_plot_per_field_tile_all_fallow() {
        let farm = small_farm();
        assert_eq!(farm.plots().len(), 4);
        assert!(farm.plots().iter().all(|p| p.state == PlotState::Fallow));
        assert_eq!(farm.state_at(TilePos::new(0, 0)), None);
    }

    #[test]
    fn full_crop_cycle() {
        let mut farm = small_farm();
        let t = TilePos::new(2, 1);
        assert!(farm.apply(WorkKind::Till, t));
        assert_eq!(farm.state_at(t), Some(PlotState::Tilled));
        assert!(farm.apply(WorkKind::Plant, t));
        assert!(matches!(farm.state_at(t), Some(PlotState::Planted { .. })));
        assert_eq!(farm.work_at(t), None, "rien à faire pendant la pousse");
        farm.tick(11.9, 12.0);
        assert!(matches!(farm.state_at(t), Some(PlotState::Planted { .. })));
        farm.tick(0.2, 12.0);
        assert_eq!(farm.state_at(t), Some(PlotState::Ripe));
        assert!(farm.apply(WorkKind::Harvest, t));
        assert_eq!(farm.state_at(t), Some(PlotState::Fallow));
    }

    #[test]
    fn invalid_transitions_are_rejected() {
        let mut farm = small_farm();
        let t = TilePos::new(1, 1);
        assert!(!farm.apply(WorkKind::Plant, t), "on ne sème pas une friche");
        assert!(!farm.apply(WorkKind::Harvest, t), "on ne récolte pas une friche");
        assert!(!farm.apply(WorkKind::Till, TilePos::new(0, 0)), "pas de parcelle ici");
        assert_eq!(farm.state_at(t), Some(PlotState::Fallow));
    }

    #[test]
    fn pick_target_prefers_harvest_then_plant_then_till_then_nearest() {
        let mut farm = small_farm();
        let from = TilePos::new(1, 1);
        assert_eq!(farm.pick_target(from, &[]), Some(TilePos::new(1, 1)), "le plus proche en premier");
        assert_eq!(farm.pick_target(TilePos::new(4, 1), &[]), Some(TilePos::new(4, 1)));

        farm.set_state(TilePos::new(4, 1), PlotState::Tilled);
        assert_eq!(farm.pick_target(from, &[]), Some(TilePos::new(4, 1)), "semer avant de labourer");

        farm.set_state(TilePos::new(3, 1), PlotState::Ripe);
        assert_eq!(farm.pick_target(from, &[]), Some(TilePos::new(3, 1)), "récolter avant tout");

        for x in 1..5 {
            farm.set_state(TilePos::new(x, 1), PlotState::Planted { growth: 0.0 });
        }
        assert_eq!(farm.pick_target(from, &[]), None, "tout pousse : rien à faire");
    }

    #[test]
    fn pick_target_skips_claimed_plots() {
        let mut farm = small_farm();
        farm.set_state(TilePos::new(3, 1), PlotState::Ripe);
        let from = TilePos::new(3, 1);
        assert_eq!(farm.pick_target(from, &[]), Some(TilePos::new(3, 1)));
        // la récolte est réservée par quelqu'un d'autre : on se rabat sur un labour voisin
        let next = farm.pick_target(from, &[TilePos::new(3, 1)]);
        assert!(next.is_some() && next != Some(TilePos::new(3, 1)));
        // tout est réservé : rien à faire
        let all: Vec<TilePos> = farm.plots().iter().map(|p| p.tile).collect();
        assert_eq!(farm.pick_target(from, &all), None);
    }

    #[test]
    fn growth_fraction() {
        assert_eq!(PlotState::Fallow.growth_fraction(12.0), 0.0);
        assert!((PlotState::Planted { growth: 6.0 }.growth_fraction(12.0) - 0.5).abs() < 1e-6);
        assert_eq!(PlotState::Ripe.growth_fraction(12.0), 1.0);
    }
}
