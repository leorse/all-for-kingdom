//! La caméra : quelle partie de la carte est à l'écran. Logique pure (pas de macroquad),
//! donc testable : les tailles d'écran et de carte lui sont passées en paramètres.
//!
//! Tout est en tuiles. Le zoom est fixe : la zone visible vaut `taille de la fenêtre / TILE_PX`.

use afk_core::Vec2;

/// Taille d'une tuile à l'écran, en pixels. Fixe : la carte peut dépasser la fenêtre.
pub const TILE_PX: f32 = 32.0;
/// Durée du glissement vers un personnage qu'on vient d'incarner, en secondes réelles.
pub const SLIDE_SECS: f32 = 0.35;
/// Part de l'écran, de chaque côté, où le personnage peut bouger sans faire défiler la carte.
pub const FOLLOW_MARGIN: f32 = 0.25;

pub struct Camera {
    /// Point du monde affiché au centre de l'écran.
    center: Vec2,
    slide: Option<Slide>,
    placed: bool,
}

struct Slide {
    from: Vec2,
    elapsed: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera { center: Vec2::ZERO, slide: None, placed: false }
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    /// Le glissement vers un nouveau personnage est en cours.
    #[cfg(test)]
    pub fn is_sliding(&self) -> bool {
        self.slide.is_some()
    }

    /// Avance la caméra d'une image. `view` = taille de la zone visible, `map` = taille de la carte (tuiles).
    /// `recenter` : le joueur vient de changer de personnage, la caméra glisse jusqu'à lui.
    pub fn update(&mut self, target: Vec2, view: Vec2, map: Vec2, recenter: bool, dt: f32) {
        let goal = clamp_center(target, view, map);
        if !self.placed {
            // première image : déjà en place, sans animation
            self.placed = true;
            self.center = goal;
            return;
        }
        if recenter {
            self.slide = Some(Slide { from: self.center, elapsed: 0.0 });
        }

        if let Some(slide) = &mut self.slide {
            slide.elapsed += dt;
            let u = (slide.elapsed / SLIDE_SECS).min(1.0);
            let k = 1.0 - (1.0 - u) * (1.0 - u) * (1.0 - u); // part vite, freine à l'arrivée
            self.center = slide.from + (goal - slide.from) * k;
            if u >= 1.0 {
                self.center = goal;
                self.slide = None;
            }
            return;
        }

        // suivi : la carte ne défile que si le personnage sort de la zone centrale
        let dead = Vec2::new(view.x * (0.5 - FOLLOW_MARGIN), view.y * (0.5 - FOLLOW_MARGIN));
        let mut c = self.center;
        c.x = c.x.clamp(target.x - dead.x, target.x + dead.x);
        c.y = c.y.clamp(target.y - dead.y, target.y + dead.y);
        self.center = clamp_center(c, view, map);
    }
}

/// Garde la zone visible dans la carte ; une carte plus petite que l'écran est centrée sur l'axe concerné.
pub fn clamp_center(center: Vec2, view: Vec2, map: Vec2) -> Vec2 {
    let axis = |c: f32, v: f32, m: f32| if m <= v { m / 2.0 } else { c.clamp(v / 2.0, m - v / 2.0) };
    Vec2::new(axis(center.x, view.x, map.x), axis(center.y, view.y, map.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIEW: Vec2 = Vec2::new(32.0, 20.0);
    const MAP: Vec2 = Vec2::new(64.0, 40.0);

    fn placed_at(target: Vec2) -> Camera {
        let mut cam = Camera::new();
        cam.update(target, VIEW, MAP, false, 0.016);
        cam
    }

    #[test]
    fn first_frame_is_centered_without_animation() {
        let cam = placed_at(Vec2::new(30.0, 20.0));
        assert_eq!(cam.center(), Vec2::new(30.0, 20.0));
        assert!(!cam.is_sliding());
    }

    #[test]
    fn camera_is_clamped_to_the_map() {
        let cam = placed_at(Vec2::new(2.0, 1.0));
        assert_eq!(cam.center(), Vec2::new(16.0, 10.0));
        let cam = placed_at(Vec2::new(63.0, 39.0));
        assert_eq!(cam.center(), Vec2::new(48.0, 30.0));
    }

    #[test]
    fn a_small_map_is_centered() {
        let c = clamp_center(Vec2::new(3.0, 3.0), VIEW, Vec2::new(20.0, 10.0));
        assert_eq!(c, Vec2::new(10.0, 5.0));
    }

    #[test]
    fn moving_in_the_dead_zone_does_not_scroll() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.update(Vec2::new(35.0, 22.0), VIEW, MAP, false, 0.016);
        assert_eq!(cam.center(), Vec2::new(32.0, 20.0));
    }

    #[test]
    fn leaving_the_dead_zone_scrolls_and_keeps_the_target_on_screen() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        let target = Vec2::new(45.0, 20.0); // 13 tuiles à droite : au-delà de la marge (8)
        cam.update(target, VIEW, MAP, false, 0.016);
        assert!((target.x - cam.center().x - 8.0).abs() < 1e-4, "{:?}", cam.center());
        assert!(target.x - cam.center().x < VIEW.x / 2.0);
    }

    #[test]
    fn recenter_slides_then_lands_exactly_on_the_target() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        let target = Vec2::new(56.0, 6.0);
        cam.update(target, VIEW, MAP, true, 0.05);
        assert!(cam.is_sliding());
        let mid = cam.center();
        assert!(mid.x > 32.0 && mid.x < 48.0, "doit être en chemin : {mid:?}");
        for _ in 0..40 {
            cam.update(target, VIEW, MAP, false, 0.016);
        }
        assert!(!cam.is_sliding());
        assert_eq!(cam.center(), clamp_center(target, VIEW, MAP));
    }

    #[test]
    fn a_new_recenter_during_a_slide_starts_from_the_current_position() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.update(Vec2::new(56.0, 6.0), VIEW, MAP, true, 0.1);
        let before = cam.center();
        cam.update(Vec2::new(10.0, 30.0), VIEW, MAP, true, 0.0);
        assert_eq!(cam.center(), before, "pas de saut au redémarrage");
    }

    #[test]
    fn a_moving_target_is_reached_during_the_slide() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.update(Vec2::new(40.0, 20.0), VIEW, MAP, true, 0.0);
        let final_target = Vec2::new(44.0, 22.0);
        for _ in 0..40 {
            cam.update(final_target, VIEW, MAP, false, 0.016);
        }
        assert_eq!(cam.center(), final_target);
    }
}
