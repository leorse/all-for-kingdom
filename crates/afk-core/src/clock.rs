//! L'horloge de jeu : heure, jour, nuit, luminosité.

use crate::config::Config;

pub struct Clock {
    day_length: f32,
    start_hour: f32,
    dawn: f32,
    dusk: f32,
    /// Secondes réelles écoulées (f64 : reste précis sur de très longues parties AFK).
    elapsed: f64,
}

impl Clock {
    pub fn new(cfg: &Config) -> Clock {
        Clock {
            day_length: cfg.day_length_secs.max(1.0),
            start_hour: cfg.start_hour,
            dawn: cfg.dawn_hour,
            dusk: cfg.dusk_hour,
            elapsed: 0.0,
        }
    }

    pub fn advance(&mut self, dt: f32) {
        self.elapsed += dt as f64;
    }

    /// Heures de jeu écoulées par seconde réelle.
    pub fn hours_per_second(&self) -> f32 {
        24.0 / self.day_length
    }

    fn total_hours(&self) -> f64 {
        self.start_hour as f64 + self.elapsed * self.hours_per_second() as f64
    }

    /// Heure du jour, dans `[0, 24)`.
    pub fn hour(&self) -> f32 {
        self.total_hours().rem_euclid(24.0) as f32
    }

    /// Numéro du jour, à partir de 1.
    pub fn day(&self) -> u32 {
        (self.total_hours() / 24.0).floor() as u32 + 1
    }

    /// Nuit au sens du gameplay : tout le monde doit être couché.
    pub fn is_night(&self) -> bool {
        let h = self.hour();
        h >= self.dusk || h < self.dawn
    }

    /// Luminosité pour l'affichage : 0 = nuit noire, 1 = plein jour.
    /// Transition douce d'une heure avant à une heure après l'aube et le crépuscule.
    pub fn daylight(&self) -> f32 {
        let h = self.hour();
        if h < self.dawn - 1.0 || h >= self.dusk + 1.0 {
            0.0
        } else if h < self.dawn + 1.0 {
            (h - (self.dawn - 1.0)) / 2.0
        } else if h < self.dusk - 1.0 {
            1.0
        } else {
            1.0 - (h - (self.dusk - 1.0)) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clock_at(hour: f32) -> Clock {
        let cfg = Config { start_hour: hour, ..Config::default() };
        Clock::new(&cfg)
    }

    #[test]
    fn default_day_lasts_ten_minutes() {
        let mut c = clock_at(7.0);
        for _ in 0..(25.0 * 60.0) as usize {
            c.advance(1.0 / 60.0); // 25 s = 1 h de jeu
        }
        assert!((c.hour() - 8.0).abs() < 0.01, "heure : {}", c.hour());
        assert_eq!(c.day(), 1);
    }

    #[test]
    fn day_length_is_configurable() {
        let cfg = Config { day_length_secs: 10.0, start_hour: 0.0, ..Config::default() };
        let mut c = Clock::new(&cfg);
        for _ in 0..(10.0 * 60.0) as usize + 1 {
            c.advance(1.0 / 60.0);
        }
        assert_eq!(c.day(), 2, "une journée de 10 s doit déjà être finie");
    }

    #[test]
    fn night_boundaries() {
        assert!(clock_at(23.0).is_night());
        assert!(clock_at(3.0).is_night());
        assert!(clock_at(20.0).is_night());
        assert!(!clock_at(6.0).is_night());
        assert!(!clock_at(12.0).is_night());
        assert!(!clock_at(19.9).is_night());
    }

    #[test]
    fn daylight_curve() {
        assert_eq!(clock_at(12.0).daylight(), 1.0);
        assert_eq!(clock_at(0.0).daylight(), 0.0);
        assert!((clock_at(6.0).daylight() - 0.5).abs() < 1e-4);
        assert!((clock_at(20.0).daylight() - 0.5).abs() < 1e-4);
    }

    #[test]
    fn day_counter_rolls_over_at_midnight() {
        let mut c = clock_at(23.0);
        assert_eq!(c.day(), 1);
        for _ in 0..(2.0 * 25.0 * 60.0) as usize {
            c.advance(1.0 / 60.0); // +2 h
        }
        assert_eq!(c.day(), 2);
        assert!((c.hour() - 1.0).abs() < 0.02);
    }
}
