//! La caméra : quelle partie de la carte est à l'écran. Logique pure (pas de macroquad),
//! donc testable : les tailles d'écran et de carte lui sont passées en paramètres.
//!
//! Tout est en tuiles. La zone visible vaut `taille de la fenêtre / taille d'une tuile` ; la taille d'une tuile
//! est l'un des paliers de `ZOOM_LEVELS` (le joueur zoome à la molette, voir `step_zoom`).

use afk_core::Vec2;

/// Les paliers de zoom : taille d'une tuile à l'écran, en pixels. Des entiers, pour éviter les coutures entre
/// rectangles au dessin. À 16 px la carte 64×40 tient dans la fenêtre par défaut (1024×640).
pub const ZOOM_LEVELS: [f32; 7] = [16.0, 20.0, 24.0, 32.0, 40.0, 48.0, 64.0];
/// Le palier de départ (32 px par tuile).
pub const DEFAULT_ZOOM: usize = 3;
/// Durée du glissement vers un personnage qu'on vient d'incarner, en secondes réelles.
pub const SLIDE_SECS: f32 = 0.35;
/// Part de l'écran, de chaque côté, où le personnage peut bouger sans faire défiler la carte.
pub const FOLLOW_MARGIN: f32 = 0.25;

pub struct Camera {
    /// Point du monde affiché au centre de l'écran.
    center: Vec2,
    slide: Option<Slide>,
    placed: bool,
    /// Vue libre : le joueur a déplacé la carte à la souris, la caméra ne suit plus le personnage.
    free: bool,
}

struct Slide {
    from: Vec2,
    elapsed: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera { center: Vec2::ZERO, slide: None, placed: false, free: false }
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    /// La vue est libre (déplacée à la souris) : le suivi est suspendu jusqu'au prochain `recenter`.
    pub fn is_free(&self) -> bool {
        self.free
    }

    /// Déplace la vue de `delta` tuiles (borné par la carte) : suspend le suivi et annule un glissement.
    pub fn pan(&mut self, delta: Vec2, view: Vec2, map: Vec2) {
        self.free = true;
        self.slide = None;
        self.center = clamp_center(self.center + delta, view, map);
    }

    /// Place la caméra (borné par la carte) sans toucher au suivi : sert au zoom.
    pub fn set_center(&mut self, center: Vec2, view: Vec2, map: Vec2) {
        self.center = clamp_center(center, view, map);
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
        if recenter {
            self.free = false; // le joueur reprend la main sur son personnage : le suivi reprend
        }
        if self.free {
            // vue libre : la caméra reste où le joueur l'a mise (re-bornée si la fenêtre ou le zoom a changé)
            self.center = clamp_center(self.center, view, map);
            return;
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

/// Le palier de zoom après `steps` crans de molette (positif : on zoome), borné aux deux bouts.
pub fn step_zoom(level: usize, steps: i32) -> usize {
    (level as i32 + steps).clamp(0, ZOOM_LEVELS.len() as i32 - 1) as usize
}

/// Nouveau centre de caméra pour zoomer en gardant sous le même pixel le point du monde qui est sous le
/// curseur. `offset` : position du curseur par rapport au centre de l'écran, en pixels. Non borné.
pub fn zoom_center(center: Vec2, offset: Vec2, old_tile_px: f32, new_tile_px: f32) -> Vec2 {
    let under_cursor = center + offset * (1.0 / old_tile_px);
    under_cursor - offset * (1.0 / new_tile_px)
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
    fn panning_moves_the_view_by_the_delta_and_stops_following() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.pan(Vec2::new(5.0, -3.0), VIEW, MAP);
        assert!(cam.is_free());
        assert_eq!(cam.center(), Vec2::new(37.0, 17.0));
        // le personnage sort de la vue : la caméra ne bouge pas
        cam.update(Vec2::new(5.0, 35.0), VIEW, MAP, false, 0.016);
        assert_eq!(cam.center(), Vec2::new(37.0, 17.0));
        assert!(cam.is_free());
    }

    #[test]
    fn panning_is_bounded_by_the_map() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.pan(Vec2::new(100.0, 100.0), VIEW, MAP);
        assert_eq!(cam.center(), Vec2::new(48.0, 30.0));
        cam.pan(Vec2::new(-200.0, -200.0), VIEW, MAP);
        assert_eq!(cam.center(), Vec2::new(16.0, 10.0));
    }

    #[test]
    fn panning_cancels_a_slide_in_progress() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.update(Vec2::new(56.0, 6.0), VIEW, MAP, true, 0.1);
        assert!(cam.is_sliding());
        cam.pan(Vec2::new(1.0, 0.0), VIEW, MAP);
        assert!(!cam.is_sliding());
    }

    #[test]
    fn recenter_leaves_the_free_view_and_slides_back_to_the_target() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.pan(Vec2::new(10.0, 8.0), VIEW, MAP);
        let target = Vec2::new(32.0, 20.0);
        cam.update(target, VIEW, MAP, true, 0.05);
        assert!(!cam.is_free());
        assert!(cam.is_sliding());
        for _ in 0..40 {
            cam.update(target, VIEW, MAP, false, 0.016);
        }
        assert_eq!(cam.center(), target);
        // et le suivi a repris
        cam.update(Vec2::new(45.0, 20.0), VIEW, MAP, false, 0.016);
        assert!((45.0 - cam.center().x - 8.0).abs() < 1e-4);
    }

    #[test]
    fn a_free_view_is_rebounded_when_the_visible_area_grows() {
        let mut cam = placed_at(Vec2::new(32.0, 20.0));
        cam.pan(Vec2::new(16.0, 10.0), VIEW, MAP); // collé au coin bas droit
        let bigger = Vec2::new(48.0, 30.0);
        cam.update(Vec2::new(0.0, 0.0), bigger, MAP, false, 0.016);
        assert_eq!(cam.center(), Vec2::new(40.0, 25.0));
    }

    #[test]
    fn zoom_steps_move_one_level_and_stop_at_both_ends() {
        let d = DEFAULT_ZOOM;
        assert_eq!(ZOOM_LEVELS[d], 32.0);
        assert_eq!(step_zoom(d, 1), d + 1);
        assert_eq!(step_zoom(d, -1), d - 1);
        assert_eq!(step_zoom(ZOOM_LEVELS.len() - 1, 1), ZOOM_LEVELS.len() - 1);
        assert_eq!(step_zoom(0, -1), 0);
        assert!(ZOOM_LEVELS.windows(2).all(|w| w[0] < w[1]));
        assert!(ZOOM_LEVELS.iter().all(|z| z.fract() == 0.0));
    }

    #[test]
    fn the_whole_map_fits_the_default_window_at_the_smallest_zoom() {
        let (w, h) = (1024.0, 640.0);
        assert!(MAP.x * ZOOM_LEVELS[0] <= w && MAP.y * ZOOM_LEVELS[0] <= h);
        assert!(MAP.x * ZOOM_LEVELS[1] > w || MAP.y * ZOOM_LEVELS[1] > h, "le palier suivant ne tient plus");
    }

    #[test]
    fn zooming_keeps_the_point_under_the_cursor_at_the_same_pixel() {
        let center = Vec2::new(32.0, 20.0);
        let offset = Vec2::new(200.0, -100.0); // curseur à droite et au-dessus du centre de l'écran
        let (old, new) = (32.0, 48.0);
        let new_center = zoom_center(center, offset, old, new);
        let world_before = center + offset * (1.0 / old);
        let world_after = new_center + offset * (1.0 / new);
        assert!((world_before.x - world_after.x).abs() < 1e-4 && (world_before.y - world_after.y).abs() < 1e-4);
        // zoomer vers un point à droite décale le centre vers la droite
        assert!(new_center.x > center.x && new_center.y < center.y);
        // curseur au centre : le centre ne bouge pas
        assert_eq!(zoom_center(center, Vec2::ZERO, old, new), center);
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
