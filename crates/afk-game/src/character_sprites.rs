//! Le gabarit commun des personnages dessinés : la cellule, le point d'ancrage, la disposition de la planche et les
//! animations (nombre d'images, durées). Partagé par le jeu (choix de l'image à afficher) et par l'outil `spritegen`
//! (contrôle et génération des planches) : aucune dépendance à macroquad ni à resvg. Voir `knowledge/graphisme/direction-artistique.md`.

use afk_core::{Activity, Facing};

/// Côté d'une cellule, en pixels de dessin : 2×2 tuiles de 32 pixels.
pub const CELL: u32 = 64;
/// Le point de la cellule où sont les pieds, posé sur la position du personnage dans le monde.
pub const ANCHOR: (f32, f32) = (32.0, 58.0);
/// Colonnes de la planche : assez pour l'animation la plus longue.
pub const COLUMNS: u32 = 6;

/// Les quatre directions, dans l'ordre des colonnes de la ligne « debout ».
pub const DIRECTIONS: [&str; 4] = ["bas", "haut", "droite", "gauche"];

/// Une animation : sa ligne dans la planche, la durée de chacune de ses images (en secondes) et son nom, d'où viennent
/// les identifiants des images dans le SVG (`<nom>_<n>`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Anim {
    pub name: &'static str,
    pub row: u32,
    pub durations: &'static [f32],
}

impl Anim {
    pub fn frames(&self) -> u32 {
        self.durations.len() as u32
    }

    pub fn cycle(&self) -> f32 {
        self.durations.iter().sum()
    }

    /// L'image affichée au temps `t` (en secondes), l'animation tournant en boucle.
    pub fn frame_at(&self, t: f32) -> u32 {
        let mut local = t.rem_euclid(self.cycle());
        for (i, d) in self.durations.iter().enumerate() {
            if local < *d {
                return i as u32;
            }
            local -= d;
        }
        self.frames() - 1
    }

    /// L'instant (dans le cycle) où commence l'image `frame`.
    pub fn start_of(&self, frame: u32) -> f32 {
        self.durations[..frame as usize].iter().sum()
    }
}

const WALK: &[f32] = &[0.125; 4]; // 8 images/s : contact gauche, croisé, contact droit, croisé
const STILL: &[f32] = &[1.0];

/// Debout : une image par direction, sur la même ligne (colonne = indice dans `DIRECTIONS`).
pub const STAND: Anim = Anim { name: "debout", row: 0, durations: STILL };
pub const WALK_DOWN: Anim = Anim { name: "marche_bas", row: 1, durations: WALK };
pub const WALK_UP: Anim = Anim { name: "marche_haut", row: 2, durations: WALK };
pub const WALK_RIGHT: Anim = Anim { name: "marche_droite", row: 3, durations: WALK };
/// Produite par miroir de la marche vers la droite, sauf si le SVG la dessine.
pub const WALK_LEFT: Anim = Anim { name: "marche_gauche", row: 4, durations: WALK };
/// Repos, face vers le bas : neutre, inspiration, neutre, clignement (bref).
pub const IDLE: Anim = Anim { name: "repos", row: 5, durations: &[0.9, 0.9, 0.9, 0.15] };
/// Couché tête en haut, au centre de la cellule : deux temps de respiration.
pub const SLEEP: Anim = Anim { name: "sommeil", row: 6, durations: &[1.2, 1.2] };
/// La ligne de l'animation de travail, propre au métier.
pub const WORK_ROW: u32 = 7;
/// La ligne des objets fixes propres au métier (l'enclume du forgeron...).
pub const EXTRAS_ROW: u32 = 8;

/// Les animations communes à tous les personnages, dans l'ordre des lignes.
pub const COMMON: [Anim; 7] = [STAND, WALK_DOWN, WALK_UP, WALK_RIGHT, WALK_LEFT, IDLE, SLEEP];

/// Ce qui distingue la planche d'un métier : son animation de travail et ses objets.
#[derive(Clone, Copy, Debug)]
pub struct SheetSpec {
    /// Nom du fichier, sans extension, dans `assets/characters/`.
    pub name: &'static str,
    pub work: Anim,
    /// Objets fixes, une cellule chacun sur la ligne `EXTRAS_ROW`.
    pub extras: &'static [&'static str],
}

impl SheetSpec {
    pub fn size(&self) -> (u32, u32) {
        (COLUMNS * CELL, (EXTRAS_ROW + 1) * CELL)
    }

    /// Les identifiants de toutes les images, avec leur cellule (colonne, ligne).
    pub fn cells(&self) -> Vec<(String, u32, u32)> {
        let mut out = Vec::new();
        for (col, dir) in DIRECTIONS.iter().enumerate() {
            out.push((format!("{}_{dir}", STAND.name), col as u32, STAND.row));
        }
        for anim in COMMON.iter().skip(1).chain(std::iter::once(&self.work)) {
            for f in 0..anim.frames() {
                out.push((format!("{}_{f}", anim.name), f, anim.row));
            }
        }
        for (col, extra) in self.extras.iter().enumerate() {
            out.push((extra.to_string(), col as u32, EXTRAS_ROW));
        }
        out
    }

    /// La cellule d'un objet fixe.
    pub fn extra(&self, name: &str) -> Option<Frame> {
        let col = self.extras.iter().position(|e| *e == name)?;
        Some(Frame { col: col as u32, row: EXTRAS_ROW })
    }
}

/// Les images qui peuvent manquer au SVG : l'outil les obtient par miroir de leur pendant (image vers la droite).
pub fn mirror_source(id: &str) -> Option<String> {
    let base = id.strip_prefix("marche_gauche_").map(|n| format!("marche_droite_{n}"));
    base.or_else(|| (id == "debout_gauche").then(|| "debout_droite".to_string()))
}

/// Le forgeron : 6 images de forge (marteau bas, lever, haut, descente, impact, rebond) et son enclume.
pub const BLACKSMITH: SheetSpec = SheetSpec {
    name: "forgeron",
    work: Anim { name: "travail", row: WORK_ROW, durations: &[0.16, 0.12, 0.16, 0.07, 0.14, 0.12] },
    extras: &["enclume"],
};
/// L'image de la forge où le marteau touche le métal : c'est là que jaillissent les étincelles.
pub const FORGE_IMPACT: u32 = 4;

/// Toutes les planches connues.
pub const SHEETS: [SheetSpec; 1] = [BLACKSMITH];

/// Une cellule de la planche.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frame {
    pub col: u32,
    pub row: u32,
}

/// Décalage de temps propre à un personnage, pour que deux personnages ne bougent pas en même temps.
pub fn time_offset(id: usize) -> f32 {
    id as f32 * 0.37
}

fn walk_anim(facing: Facing) -> Anim {
    match facing {
        Facing::Down => WALK_DOWN,
        Facing::Up => WALK_UP,
        Facing::Right => WALK_RIGHT,
        Facing::Left => WALK_LEFT,
    }
}

fn direction_col(facing: Facing) -> u32 {
    match facing {
        Facing::Down => 0,
        Facing::Up => 1,
        Facing::Right => 2,
        Facing::Left => 3,
    }
}

/// L'image à afficher pour un personnage, d'après ce qu'il fait, où il regarde et l'heure (en secondes).
/// `working` : il a un travail en cours.
pub fn frame_for(sheet: &SheetSpec, activity: Activity, facing: Facing, working: bool, t: f32, id: usize) -> Frame {
    let t = t + time_offset(id);
    let anim = match activity {
        Activity::Sleeping => SLEEP,
        _ if working => sheet.work,
        Activity::Walking => walk_anim(facing),
        _ if facing == Facing::Down => IDLE,
        _ => return Frame { col: direction_col(facing), row: STAND.row },
    };
    Frame { col: anim.frame_at(t), row: anim.row }
}

/// Depuis combien de temps (en secondes) le marteau a touché le métal pour la dernière fois, dans la boucle de forge
/// d'un personnage. Sert à caler les étincelles sur l'image d'impact.
pub fn since_impact(sheet: &SheetSpec, t: f32, id: usize) -> f32 {
    let work = sheet.work;
    (t + time_offset(id) - work.start_of(FORGE_IMPACT)).rem_euclid(work.cycle())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walking_uses_the_row_of_its_direction() {
        for (facing, anim) in [(Facing::Down, WALK_DOWN), (Facing::Up, WALK_UP), (Facing::Right, WALK_RIGHT), (Facing::Left, WALK_LEFT)] {
            let f = frame_for(&BLACKSMITH, Activity::Walking, facing, false, 0.3, 0);
            assert_eq!(f.row, anim.row);
        }
    }

    #[test]
    fn idle_facing_down_breathes_and_other_directions_stand() {
        assert_eq!(frame_for(&BLACKSMITH, Activity::Idle, Facing::Down, false, 0.0, 0).row, IDLE.row);
        let up = frame_for(&BLACKSMITH, Activity::Idle, Facing::Up, false, 5.0, 0);
        assert_eq!(up, Frame { col: 1, row: STAND.row }, "debout, vu de dos");
        assert_eq!(frame_for(&BLACKSMITH, Activity::Idle, Facing::Left, false, 5.0, 0).col, 3);
    }

    #[test]
    fn work_and_sleep_have_their_own_rows() {
        assert_eq!(frame_for(&BLACKSMITH, Activity::Working, Facing::Down, true, 0.0, 0).row, WORK_ROW);
        assert_eq!(frame_for(&BLACKSMITH, Activity::Sleeping, Facing::Down, false, 0.0, 0).row, SLEEP.row);
    }

    #[test]
    fn frames_loop_within_bounds() {
        for anim in COMMON.iter().chain(std::iter::once(&BLACKSMITH.work)) {
            let mut seen = vec![false; anim.frames() as usize];
            for i in 0..2000 {
                let f = anim.frame_at(i as f32 * 0.01 - 3.0);
                assert!(f < anim.frames());
                seen[f as usize] = true;
            }
            assert!(seen.iter().all(|s| *s), "{} : toutes les images passent", anim.name);
        }
    }

    #[test]
    fn two_characters_are_not_in_step() {
        let a = frame_for(&BLACKSMITH, Activity::Walking, Facing::Down, false, 1.0, 0);
        let b = frame_for(&BLACKSMITH, Activity::Walking, Facing::Down, false, 1.0, 1);
        assert_ne!(a, b);
    }

    #[test]
    fn the_layout_fits_the_sheet_and_names_every_cell_once() {
        let cells = BLACKSMITH.cells();
        let (w, h) = BLACKSMITH.size();
        let mut ids = std::collections::HashSet::new();
        let mut spots = std::collections::HashSet::new();
        for (id, col, row) in &cells {
            assert!((col + 1) * CELL <= w && (row + 1) * CELL <= h, "{id}");
            assert!(ids.insert(id.clone()) && spots.insert((*col, *row)), "{id} en double");
        }
        assert!(ids.contains("marche_gauche_3") && ids.contains("travail_5") && ids.contains("enclume"));
        assert_eq!(mirror_source("marche_gauche_2").as_deref(), Some("marche_droite_2"));
        assert_eq!(mirror_source("debout_gauche").as_deref(), Some("debout_droite"));
        assert_eq!(mirror_source("repos_0"), None);
    }

    #[test]
    fn sparks_start_at_the_impact_frame() {
        let work = BLACKSMITH.work;
        let t = work.start_of(FORGE_IMPACT) + 0.01;
        assert_eq!(work.frame_at(t), FORGE_IMPACT);
        assert!(since_impact(&BLACKSMITH, t, 0) < 0.02);
    }
}
