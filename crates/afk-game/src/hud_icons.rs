//! Les pictogrammes du HUD : des grilles de 10x10 pixels, dessinées au code (comme le reste du thème), agrandies d'un
//! entier pour rester nets. Un caractère = une couleur de `PALETTE` ; `.` est transparent.

use afk_core::{Resource, WorkKind};
use macroquad::prelude::*;

/// Côté d'une grille, en pixels.
pub const GRID: usize = 10;

#[derive(Clone, Copy)]
pub enum Icon {
    Res(Resource),
    Work(WorkKind),
    Crown,
    Bed,
    Sun,
    Moon,
    Chest,
    Sack,
    Up,
    Down,
    Cross,
    Bang,
    Pause,
    Fast,
}

const PALETTE: [(char, (u8, u8, u8)); 32] = [
    ('k', (44, 30, 22)),    // contour sombre
    ('w', (245, 240, 225)), // blanc
    ('a', (232, 200, 70)),  // blé
    ('A', (190, 150, 40)),  // blé foncé, or foncé
    ('y', (228, 188, 52)),  // or
    ('Y', (250, 226, 110)), // or clair
    ('b', (140, 92, 50)),   // bois
    ('B', (96, 60, 34)),    // bois foncé
    ('l', (190, 140, 84)),  // bois clair
    ('g', (132, 138, 150)), // fer
    ('G', (196, 202, 214)), // fer clair
    ('h', (80, 84, 96)),    // fer foncé
    ('c', (188, 108, 56)),  // cuivre
    ('C', (226, 148, 90)),  // cuivre clair
    ('D', (130, 70, 36)),   // cuivre foncé
    ('s', (196, 190, 170)), // pierre
    ('S', (146, 140, 122)), // pierre foncée
    ('t', (196, 160, 92)),  // graine
    ('T', (140, 104, 50)),  // graine foncée
    ('u', (110, 150, 200)), // outil
    ('U', (170, 200, 240)), // outil clair
    ('r', (210, 60, 50)),   // rouge
    ('R', (150, 36, 30)),   // rouge foncé
    ('n', (80, 175, 70)),   // vert
    ('N', (46, 110, 46)),   // vert foncé
    ('x', (255, 160, 40)),  // rayon de soleil
    ('m', (205, 212, 255)), // lune
    ('M', (140, 150, 210)), // lune dans l'ombre
    ('p', (150, 60, 190)),  // pourpre
    ('f', (240, 200, 160)), // peau
    ('F', (200, 150, 110)), // peau ombrée
    ('o', (60, 40, 30)),    // ombre
];

fn color(ch: char) -> Option<Color> {
    PALETTE.iter().find(|(c, _)| *c == ch).map(|(_, (r, g, b))| Color::from_rgba(*r, *g, *b, 255))
}

const WHEAT: [&str; GRID] = [
    "....kk....",
    "...kaak...",
    ".kkkaAkkk.",
    "kaAkaAkaAk",
    ".kkkaAkkk.",
    "...kaAk...",
    "..kkbbkk..",
    "...kNNk...",
    "...kNNk...",
    "....kk....",
];
const SEEDS: [&str; GRID] = [
    "..........",
    ".kk....kk.",
    "knnk..knnk",
    "kNnnkknnNk",
    ".kkNNNNkk.",
    "...kNNk...",
    ".kk...kk..",
    "ktTk.ktTk.",
    ".kk...kk..",
    "..........",
];
const WOOD: [&str; GRID] = [
    "..........",
    "..........",
    "..kkkkkkk.",
    ".kllbbbbbk",
    "klBlbbbbbk",
    "klBlbbbbbk",
    ".kllbbbbbk",
    "..kkkkkkk.",
    "..........",
    "..........",
];
const IRON: [&str; GRID] = [
    "..........",
    "..........",
    "..kkkkkkk.",
    ".kGGGGGGgk",
    "kkkkkkkkkk",
    "kGGggggggk",
    "kgggggghhk",
    "kkkkkkkkkk",
    "..........",
    "..........",
];
const COPPER: [&str; GRID] = [
    "..........",
    "..........",
    "..kkkkkkk.",
    ".kCCCCCCck",
    "kkkkkkkkkk",
    "kCCcccccck",
    "kccccccDDk",
    "kkkkkkkkkk",
    "..........",
    "..........",
];
const GOLD: [&str; GRID] = [
    "..........",
    "..........",
    "..kkkkkkk.",
    ".kYYYYYYyk",
    "kkkkkkkkkk",
    "kYYyyyyyyk",
    "kyyyyyyAAk",
    "kkkkkkkkkk",
    "..........",
    "..........",
];
const STONE: [&str; GRID] = [
    "..........",
    "..kkkkk...",
    ".kssssSkk.",
    "ksssssSSSk",
    "kssskSSSSk",
    "ksssSkSSSk",
    "kSSSSSSSSk",
    ".kkkkkkkk.",
    "..........",
    "..........",
];
const TOOLS: [&str; GRID] = [
    ".kkkk...uu",
    "kGGGgk.uu.",
    "kggggkuu..",
    ".kkbbuu...",
    "....bb....",
    "...uubb...",
    "..uu..bb..",
    ".uu....bb.",
    "uu......bb",
    "U.........",
];
const CROWN: [&str; GRID] = [
    "..........",
    "k..kkkk..k",
    "kykkyykkyk",
    "kyyyyyyyyk",
    "kyyyrryyyk",
    "kyyyyyyyyk",
    "kAAAAAAAAk",
    "kkkkkkkkkk",
    "..........",
    "..........",
];
const BED: [&str; GRID] = [
    "..........",
    "kkk.......",
    "klkkkkkkk.",
    "klwwwrrrrk",
    "klwwwrrrRk",
    "kbbbbbbbbk",
    "kkkkkkkkkk",
    "kbk....kbk",
    "kkk....kkk",
    "..........",
];
const SUN: [&str; GRID] = [
    "....xx....",
    ".x..xx..x.",
    "...kkkk...",
    "..kYYyyk..",
    "xxkYyyAkxx",
    "xxkyyyAkxx",
    "..kyyAAk..",
    "...kkkk...",
    ".x..xx..x.",
    "....xx....",
];
const MOON: [&str; GRID] = [
    "..kkkk....",
    ".kmmmmk...",
    "kmmmkkk...",
    "kmmk......",
    "kmmk......",
    "kmmmk.....",
    "kmmmmkkk..",
    ".kmmmmMMk.",
    "..kkkkkkk.",
    "..........",
];
const CHEST: [&str; GRID] = [
    "..........",
    ".kkkkkkkk.",
    "kllllllllk",
    "kbbbbbbbbk",
    "kkkkyykkkk",
    "kbbbyybbbk",
    "kbbbbbbbbk",
    "kBBBBBBBBk",
    ".kkkkkkkk.",
    "..........",
];
const SACK: [&str; GRID] = [
    "..........",
    "...kkkk...",
    "...kbbk...",
    "..kkyykk..",
    ".kllllllk.",
    "kllllllllk",
    "kllllllbbk",
    "kllllbbbbk",
    ".kbbbbbbk.",
    "..kkkkkk..",
];
const UP: [&str; GRID] = [
    "....ww....",
    "...wwww...",
    "..wwwwww..",
    ".wwwwwwww.",
    "...wwww...",
    "...wwww...",
    "...wwww...",
    "...wwww...",
    "..........",
    "..........",
];
const CROSS: [&str; GRID] = [
    "rr......rr",
    "rrr....rrr",
    ".rrr..rrr.",
    "..rrrrrr..",
    "...rrrr...",
    "...rrrr...",
    "..rrrrrr..",
    ".rrr..rrr.",
    "rrr....rrr",
    "rr......rr",
];
const BANG: [&str; GRID] = [
    "...kkkk...",
    "..krrrrk..",
    ".krrwwrrk.",
    "krrrwwrrrk",
    "krrrwwrrrk",
    "krrrwwrrrk",
    "krrrrrrrrk",
    ".krrwwrrk.",
    "..krrrrk..",
    "...kkkk...",
];
const PAUSE: [&str; GRID] = [
    "..........",
    "..ww..ww..",
    "..ww..ww..",
    "..ww..ww..",
    "..ww..ww..",
    "..ww..ww..",
    "..ww..ww..",
    "..ww..ww..",
    "..ww..ww..",
    "..........",
];
const FAST: [&str; GRID] = [
    "..........",
    "ww..ww....",
    ".ww..ww...",
    "..ww..ww..",
    "...ww..ww.",
    "...ww..ww.",
    "..ww..ww..",
    ".ww..ww...",
    "ww..ww....",
    "..........",
];
const HOE: [&str; GRID] = [
    "kk........",
    "lbk.......",
    "klbk......",
    ".klbk.....",
    "..klbk....",
    "...klbkkk.",
    "....kbgGGk",
    ".....kkGgk",
    ".......Ggk",
    ".......kkk",
];
const SICKLE: [&str; GRID] = [
    "..gggg....",
    ".gGGGGg...",
    "gGg..gGg..",
    "gg....gg..",
    "g.....gg..",
    "......g...",
    ".....bb...",
    "....bb....",
    "...bb.....",
    "..bb......",
];
const ANVIL: [&str; GRID] = [
    "..........",
    "kkkkkkkkk.",
    "kGGGGGGggk",
    ".kkgggggk.",
    "...kgggk..",
    "...kgggk..",
    "..kkgggkk.",
    ".kghhhhhgk",
    ".kkkkkkkkk",
    "..........",
];
const PICKAXE: [&str; GRID] = [
    "..gggggg..",
    ".gg.bb.gg.",
    "gg..bb..gg",
    "g...bb...g",
    "....bb....",
    "....bb....",
    "....bb....",
    "....bb....",
    "....bb....",
    "....bb....",
];
const AXE: [&str; GRID] = [
    "..........",
    ".ggg.bb...",
    "gGGggbb...",
    "gGGGgbb...",
    "gGGGgbb...",
    ".ggg.bb...",
    ".....bb...",
    ".....bb...",
    ".....bb...",
    ".....bb...",
];
const HAMMER: [&str; GRID] = [
    "..........",
    ".kkkkkkkk.",
    "kGGGGGGggk",
    "kgggggghhk",
    ".kkkbbkkk.",
    "...klbk...",
    "...klbk...",
    "...klbk...",
    "...klbk...",
    "...kkkk...",
];


impl Icon {
    /// La grille, et si elle se lit à l'envers (la flèche vers le bas est la flèche vers le haut retournée).
    fn rows(self) -> (&'static [&'static str; GRID], bool) {
        let rows = match self {
            Icon::Res(Resource::Wheat) => &WHEAT,
            Icon::Res(Resource::Seeds) | Icon::Work(WorkKind::Plant) => &SEEDS,
            Icon::Res(Resource::Wood) => &WOOD,
            Icon::Res(Resource::Iron) => &IRON,
            Icon::Res(Resource::Copper) => &COPPER,
            Icon::Res(Resource::Gold) => &GOLD,
            Icon::Res(Resource::Stone) => &STONE,
            Icon::Res(Resource::Tools) => &TOOLS,
            Icon::Work(WorkKind::Till) => &HOE,
            Icon::Work(WorkKind::Harvest) => &SICKLE,
            Icon::Work(WorkKind::Forge) => &ANVIL,
            Icon::Work(WorkKind::Mine) => &PICKAXE,
            Icon::Work(WorkKind::Chop) => &AXE,
            Icon::Work(WorkKind::Build) => &HAMMER,
            Icon::Crown => &CROWN,
            Icon::Bed => &BED,
            Icon::Sun => &SUN,
            Icon::Moon => &MOON,
            Icon::Chest => &CHEST,
            Icon::Sack => &SACK,
            Icon::Up | Icon::Down => &UP,
            Icon::Cross => &CROSS,
            Icon::Bang => &BANG,
            Icon::Pause => &PAUSE,
            Icon::Fast => &FAST,
        };
        (rows, matches!(self, Icon::Down))
    }
}

/// Dessine un pictogramme, coin haut-gauche en `(x, y)`, chaque pixel de la grille faisant `px` pixels d'écran.
/// `alpha` l'estompe (0 = invisible).
pub fn draw(icon: Icon, x: f32, y: f32, px: f32, alpha: f32) {
    let (rows, flipped) = icon.rows();
    for (j, row) in rows.iter().enumerate() {
        let j = if flipped { GRID - 1 - j } else { j };
        for (i, ch) in row.chars().enumerate() {
            if let Some(c) = color(ch) {
                draw_rectangle(x + i as f32 * px, y + j as f32 * px, px, px, Color::new(c.r, c.g, c.b, alpha));
            }
        }
    }
}

/// Couleur d'un nombre en manque, sur parchemin.
pub const SHORT: Color = Color::new(0.65, 0.12, 0.10, 1.0);

/// Une ligne de coût : pour chaque `(ressource, nombre, en manque)`, son pictogramme (échelle 2, les mêmes que le HUD) et
/// le nombre, calé sur le milieu `center_y` de la ligne. Un nombre en manque est rouge, suivi d'une croix : la couleur
/// ne porte pas seule le sens. Renvoie le `x` où la ligne finit.
pub fn draw_cost(x: f32, center_y: f32, items: &[(Resource, u32, bool)], size: f32, ink: Color) -> f32 {
    cost_row(x, center_y, items, size, ink, true)
}

/// La largeur qu'occuperait `draw_cost`.
pub fn cost_width(items: &[(Resource, u32, bool)], size: f32) -> f32 {
    cost_row(0.0, 0.0, items, size, WHITE, false)
}

fn cost_row(x: f32, center_y: f32, items: &[(Resource, u32, bool)], size: f32, ink: Color, paint: bool) -> f32 {
    const PX: f32 = 2.0;
    let icon = GRID as f32 * PX;
    let mut cx = x;
    for &(resource, amount, short) in items {
        let label = amount.to_string();
        if paint {
            draw(Icon::Res(resource), cx, center_y - icon / 2.0, PX, 1.0);
            draw_text(&label, cx + icon + 3.0, center_y + size * 0.33, size, if short { SHORT } else { ink });
        }
        cx += icon + 3.0 + measure_text(&label, None, size as u16, 1.0).width;
        if short {
            if paint {
                draw(Icon::Cross, cx + 2.0, center_y - GRID as f32 / 2.0, 1.0, 1.0);
            }
            cx += GRID as f32 + 2.0;
        }
        cx += 8.0;
    }
    cx
}


#[cfg(test)]
mod tests {
    use super::*;

    fn all_icons() -> Vec<Icon> {
        [
            Icon::Crown, Icon::Bed, Icon::Sun, Icon::Moon, Icon::Chest, Icon::Sack, Icon::Up, Icon::Down, Icon::Cross, Icon::Bang,
            Icon::Pause, Icon::Fast,
        ]
        .into_iter()
        .chain(Resource::ALL.into_iter().map(Icon::Res))
        .chain(
            [WorkKind::Till, WorkKind::Plant, WorkKind::Harvest, WorkKind::Forge, WorkKind::Mine, WorkKind::Chop, WorkKind::Build]
                .into_iter()
                .map(Icon::Work),
        )
        .collect()
    }

    #[test]
    fn every_icon_is_a_square_grid_of_palette_colours() {
        for icon in all_icons() {
            let (rows, _) = icon.rows();
            for row in rows {
                assert_eq!(row.chars().count(), GRID, "ligne « {row} »");
                for ch in row.chars() {
                    assert!(ch == '.' || color(ch).is_some(), "caractère « {ch} » hors palette dans « {row} »");
                }
            }
        }
    }

    fn crc32(data: &[u8]) -> u32 {
        let mut crc = 0xFFFF_FFFFu32;
        for &b in data {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
            }
        }
        !crc
    }

    fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
        out.extend((data.len() as u32).to_be_bytes());
        let mut body = kind.to_vec();
        body.extend(data);
        out.extend(&body);
        out.extend(crc32(&body).to_be_bytes());
    }

    /// PNG RGB non compressé (blocs « stored »), sans fenêtre ni dépendance.
    fn write_png(path: &str, w: usize, h: usize, rgb: &[u8]) {
        let mut raw = Vec::new();
        for y in 0..h {
            raw.push(0);
            raw.extend(&rgb[y * w * 3..(y + 1) * w * 3]);
        }
        let mut z = vec![0x78, 0x01];
        let blocks: Vec<&[u8]> = raw.chunks(65535).collect();
        for (i, b) in blocks.iter().enumerate() {
            z.push(if i + 1 == blocks.len() { 1 } else { 0 });
            z.extend((b.len() as u16).to_le_bytes());
            z.extend((!(b.len() as u16)).to_le_bytes());
            z.extend(*b);
        }
        let (mut a, mut s) = (1u32, 0u32);
        for &b in &raw {
            a = (a + b as u32) % 65521;
            s = (s + a) % 65521;
        }
        z.extend(((s << 16) | a).to_be_bytes());
        let mut png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        let mut ihdr = Vec::new();
        ihdr.extend((w as u32).to_be_bytes());
        ihdr.extend((h as u32).to_be_bytes());
        ihdr.extend([8, 2, 0, 0, 0]);
        chunk(&mut png, b"IHDR", &ihdr);
        chunk(&mut png, b"IDAT", &z);
        chunk(&mut png, b"IEND", &[]);
        std::fs::write(path, png).unwrap();
    }

    /// Planche de toutes les pictogrammes (échelles 4 puis 2) sur bois sombre puis sur parchemin :
    /// `ICONS_OUT=planche.png cargo test -p afk-game icon_sheet -- --ignored`.
    #[test]
    #[ignore]
    fn icon_sheet() {
        let Ok(path) = std::env::var("ICONS_OUT") else { return };
        let icons = all_icons();
        let per_row = 13usize;
        let cols = per_row;
        let cell = 48usize;
        let rows_per_bg = 2 * icons.len().div_ceil(per_row);
        let (w, h) = (cols * cell, rows_per_bg * 2 * cell);
        let mut rgb: Vec<u8> = (0..w * h).flat_map(|i| if i / w < h / 2 { [58u8, 40, 30] } else { [226, 208, 170] }).collect();
        let backgrounds = [(58u8, 40u8, 30u8), (226, 208, 170)];
        for (bg_i, bg) in backgrounds.iter().enumerate() {
            for (scale_i, scale) in [4usize, 2].iter().enumerate() {
                for (n, icon) in icons.iter().enumerate() {
                    let (rows, flipped) = icon.rows();
                    let cx = (n % per_row) * cell;
                    let band = icons.len().div_ceil(per_row);
                    let cy = (bg_i * 2 * band + scale_i * band + n / per_row) * cell;
                    for y in 0..cell {
                        for x in 0..cell {
                            let o = ((cy + y) * w + cx + x) * 3;
                            rgb[o..o + 3].copy_from_slice(&[bg.0, bg.1, bg.2]);
                        }
                    }
                    let (ox, oy) = (cx + (cell - GRID * scale) / 2, cy + (cell - GRID * scale) / 2);
                    for (j, row) in rows.iter().enumerate() {
                        let j = if flipped { GRID - 1 - j } else { j };
                        for (i, ch) in row.chars().enumerate() {
                            if let Some((_, (r, g, b))) = PALETTE.iter().find(|(c, _)| *c == ch) {
                                for dy in 0..*scale {
                                    for dx in 0..*scale {
                                        let o = ((oy + j * scale + dy) * w + ox + i * scale + dx) * 3;
                                        rgb[o..o + 3].copy_from_slice(&[*r, *g, *b]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        write_png(&path, w, h, &rgb);
    }
}
