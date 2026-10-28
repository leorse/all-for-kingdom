//! Le pixel art du jeu : une palette commune, des sprites dessinés sur une grille de 16 pixels par tuile, et leur
//! affichage net (sans lissage) à une échelle entière.
//!
//! Un sprite est une grille de caractères : chaque lettre est une couleur de la palette (`.` = transparent).
//! La partie « données » (palette, grille, rotation) est pure et testée ; la partie « textures » ne sert qu'à
//! l'affichage et suppose une fenêtre macroquad ouverte. Voir `docs/art-direction.md`.

use macroquad::prelude::*;

/// La palette commune, tirée de l'image de référence `docs/images/image_maison.jpg`.
pub const PALETTE: &[(char, [u8; 4])] = &[
    ('.', [0, 0, 0, 0]),
    ('k', [0x2a, 0x1a, 0x12, 255]), // contour sombre
    ('W', [0xd9, 0xa5, 0x57, 255]), // bois clair
    ('w', [0xb2, 0x7a, 0x3c, 255]), // bois moyen
    ('d', [0x7a, 0x4a, 0x22, 255]), // bois foncé
    ('D', [0x4a, 0x2c, 0x14, 255]), // ombre de bois
    ('T', [0xd8, 0xb0, 0x4a, 255]), // chaume clair
    ('t', [0xb8, 0x8a, 0x2e, 255]), // chaume moyen
    ('h', [0x8a, 0x64, 0x20, 255]), // chaume foncé
    ('S', [0x9a, 0x9a, 0x9c, 255]), // pierre claire
    ('s', [0x72, 0x72, 0x76, 255]), // pierre moyenne
    ('z', [0x4a, 0x4a, 0x50, 255]), // pierre foncée
    ('R', [0xc8, 0x32, 0x3c, 255]), // rouge de la couverture
    ('r', [0x8e, 0x1f, 0x2a, 255]), // rouge foncé
    ('F', [0xf0, 0xea, 0xd8, 255]), // blanc cassé
    ('f', [0xcb, 0xbf, 0xa3, 255]), // crème
    ('G', [0x4c, 0x9a, 0x3c, 255]), // vert
    ('g', [0x2f, 0x6b, 0x2a, 255]), // vert foncé
    ('B', [0x4a, 0x86, 0xc8, 255]), // eau
    ('b', [0x8c, 0xc0, 0xec, 255]), // eau claire
    ('M', [0x8a, 0x8f, 0x9a, 255]), // métal
    ('m', [0x5a, 0x5f, 0x6a, 255]), // métal foncé
    ('u', [0xa9, 0xd2, 0xec, 255]), // vitre de jour
    ('Y', [0xff, 0xe9, 0xa0, 255]), // vitre éclairée
    ('y', [0xf5, 0xc2, 0x5a, 255]), // lueur ambrée
    ('O', [0xff, 0x9a, 0x2a, 255]), // flamme orange
    ('o', [0xff, 0xd2, 0x3c, 255]), // flamme jaune
    ('e', [0xd8, 0x54, 0x1a, 255]), // braise
    ('L', [0x3a, 0x5f, 0xa8, 255]), // livre bleu
    ('N', [0xa8, 0x40, 0x3a, 255]), // livre brun-rouge
    ('P', [0x8a, 0x4a, 0xb0, 255]), // flacon violet
    ('Q', [0x4c, 0xb0, 0x5a, 255]), // flacon vert
    ('C', [0x8a, 0x8a, 0x3a, 255]), // tapis olive
    ('c', [0x5e, 0x5e, 0x26, 255]), // tapis olive foncé
];

/// La couleur RGBA d'une lettre de la palette.
pub fn color_of(ch: char) -> Option<[u8; 4]> {
    PALETTE.iter().find(|&&(c, _)| c == ch).map(|&(_, rgba)| rgba)
}

/// Un sprite en pixels, sans texture : testable sans fenêtre.
#[derive(Clone, Debug, PartialEq)]
pub struct PixelSprite {
    pub w: usize,
    pub h: usize,
    /// Les lettres de la palette, ligne par ligne.
    pub cells: Vec<char>,
}

impl PixelSprite {
    /// Construit un sprite depuis ses lignes. Refuse une grille irrégulière ou une couleur hors palette.
    pub fn parse(rows: &[String]) -> Result<PixelSprite, String> {
        let h = rows.len();
        let w = rows.first().map_or(0, |r| r.chars().count());
        if w == 0 || h == 0 {
            return Err("sprite vide".into());
        }
        let mut cells = Vec::with_capacity(w * h);
        for (y, row) in rows.iter().enumerate() {
            if row.chars().count() != w {
                return Err(format!("la ligne {y} fait {} pixels au lieu de {w}", row.chars().count()));
            }
            for (x, ch) in row.chars().enumerate() {
                if color_of(ch).is_none() {
                    return Err(format!("couleur « {ch} » hors palette en ({x}, {y})"));
                }
                cells.push(ch);
            }
        }
        Ok(PixelSprite { w, h, cells })
    }

    pub fn at(&self, x: usize, y: usize) -> char {
        self.cells[y * self.w + x]
    }

    /// Le sprite tourné d'un quart de tour dans le sens des aiguilles d'une montre.
    pub fn rotated_cw(&self) -> PixelSprite {
        let (w, h) = (self.h, self.w);
        let mut cells = vec!['.'; w * h];
        for y in 0..self.h {
            for x in 0..self.w {
                // (x, y) devient (h-1-y, x) : la colonne de gauche passe en haut
                cells[x * w + (self.h - 1 - y)] = self.at(x, y);
            }
        }
        PixelSprite { w, h, cells }
    }

    /// Le sprite tourné de `turns` quarts de tour dans le sens des aiguilles d'une montre.
    pub fn rotated(&self, turns: u8) -> PixelSprite {
        let mut sprite = self.clone();
        for _ in 0..turns % 4 {
            sprite = sprite.rotated_cw();
        }
        sprite
    }

    /// Les octets RGBA, prêts pour une texture.
    pub fn rgba(&self) -> Vec<u8> {
        self.cells.iter().flat_map(|&c| color_of(c).unwrap_or([0, 0, 0, 0])).collect()
    }
}

/// Une toile pour dessiner un sprite au pixel près avec des rectangles, puis le valider.
pub struct Canvas {
    w: usize,
    h: usize,
    cells: Vec<char>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        Canvas { w, h, cells: vec!['.'; w * h] }
    }

    pub fn px(&mut self, x: i32, y: i32, ch: char) -> &mut Canvas {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            self.cells[y as usize * self.w + x as usize] = ch;
        }
        self
    }

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, ch: char) -> &mut Canvas {
        for yy in y..y + h {
            for xx in x..x + w {
                self.px(xx, yy, ch);
            }
        }
        self
    }

    /// Entoure d'un contour les pixels non transparents, sur les cases voisines encore vides.
    pub fn outline(&mut self, ch: char) -> &mut Canvas {
        let before = self.cells.clone();
        for y in 0..self.h as i32 {
            for x in 0..self.w as i32 {
                if before[y as usize * self.w + x as usize] != '.' {
                    continue;
                }
                let near = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|&(dx, dy)| {
                    let (nx, ny) = (x + dx, y + dy);
                    nx >= 0
                        && ny >= 0
                        && (nx as usize) < self.w
                        && (ny as usize) < self.h
                        && before[ny as usize * self.w + nx as usize] != '.'
                });
                if near {
                    self.px(x, y, ch);
                }
            }
        }
        self
    }

    pub fn finish(&self) -> PixelSprite {
        let rows: Vec<String> = (0..self.h).map(|y| self.cells[y * self.w..(y + 1) * self.w].iter().collect()).collect();
        PixelSprite::parse(&rows).expect("un sprite dessiné avec la palette est valide")
    }
}

// ---------------------------------------------------------------- les dessins

/// Le sol de planches : quatre variantes (longueur des planches et position des joints).
pub fn floor(variant: usize) -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    c.rect(0, 0, 16, 16, 'w');
    for plank in 0..4 {
        let y = plank * 4;
        c.rect(0, y, 16, 1, 'd'); // joint horizontal
        c.rect(0, y + 1, 16, 1, 'W'); // arête claire de la planche
        let joint = (variant * 5 + plank as usize * 7) % 16;
        c.rect(joint as i32, y + 1, 1, 3, 'd'); // joint vertical
        c.px(((joint + 7) % 16) as i32, y + 2, 'd'); // un nœud
        c.px(((joint + 3) % 16) as i32, y + 3, 'W');
    }
    c.finish()
}

/// Le mur nord : une bande de chaume sur des rondins vus de face.
pub fn wall_north() -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    // le chaume : brins serrés
    c.rect(0, 0, 16, 6, 't');
    for x in 0..16 {
        c.px(x, 0, if x % 3 == 0 { 'T' } else { 't' });
        c.px(x, 2, if x % 4 == 1 { 'T' } else { 't' });
        c.px(x, 3, if x % 3 == 2 { 'h' } else { 'T' });
        c.px(x, 4, if x % 2 == 0 { 't' } else { 'T' });
        c.px(x, 5, 'h'); // l'ombre de l'avant-toit
    }
    // les rondins : deux bandes
    for log in 0..2 {
        let y = 6 + log * 5;
        c.rect(0, y, 16, 5, 'w');
        c.rect(0, y, 16, 1, 'W');
        c.rect(0, y + 4, 16, 1, 'd');
        c.px(3 + log * 7, y + 2, 'D'); // un nœud
        c.px(11 - log * 5, y + 2, 'd');
    }
    c.rect(0, 15, 16, 1, 'D');
    c.finish()
}

/// Le mur nord percé de la porte : une ouverture sombre cadrée de bois.
pub fn wall_north_door() -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    c.rect(0, 0, 16, 16, '.');
    let wall = wall_north();
    for y in 0..16 {
        for x in 0..16 {
            c.px(x as i32, y as i32, wall.at(x, y));
        }
    }
    c.rect(3, 6, 10, 10, 'd'); // le cadre
    c.rect(4, 7, 8, 9, 'D'); // l'ouverture
    c.rect(4, 7, 8, 1, 'k');
    c.finish()
}

/// Le mur sud, bas : une poutre vue de dessus puis les rondins.
pub fn wall_south() -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    c.rect(0, 0, 16, 16, 'w');
    c.rect(0, 0, 16, 3, 'W'); // la poutre
    c.rect(0, 3, 16, 1, 'd');
    for log in 0..2 {
        let y = 4 + log * 6;
        c.rect(0, y, 16, 1, 'W');
        c.rect(0, y + 5, 16, 1, 'd');
        c.px(4 + log * 6, y + 3, 'D');
    }
    c.rect(0, 15, 16, 1, 'D');
    c.finish()
}

/// Le mur sud avec la porte : le sol se voit entre deux montants.
pub fn wall_south_door(variant: usize) -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    let floor = floor(variant);
    for y in 0..16 {
        for x in 0..16 {
            c.px(x as i32, y as i32, floor.at(x, y));
        }
    }
    c.rect(0, 0, 3, 16, 'd');
    c.rect(13, 0, 3, 16, 'd');
    c.rect(0, 0, 3, 2, 'W');
    c.rect(13, 0, 3, 2, 'W');
    c.rect(0, 0, 16, 2, 'W'); // le linteau
    c.rect(0, 2, 16, 1, 'D');
    c.finish()
}

/// Un mur latéral : des planches verticales. `east` : le bord intérieur est à gauche.
pub fn wall_side(east: bool) -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    c.rect(0, 0, 16, 16, 'w');
    for x in [3, 8, 13] {
        c.rect(x, 0, 1, 16, 'd');
        c.rect(x + 1, 0, 1, 16, 'W');
    }
    for y in [2, 9, 13] {
        c.px(5, y, 'D');
        c.px(11, y + 2, 'D');
    }
    // le bord intérieur, du côté de la pièce, plus sombre
    let inner = if east { 0 } else { 13 };
    c.rect(inner, 0, 3, 16, 'd');
    c.rect(if east { 0 } else { 15 }, 0, 1, 16, 'D');
    c.finish()
}

/// Un mur latéral percé de la porte : on voit le sol entre une poutre en haut et une en bas.
pub fn wall_side_door(east: bool, variant: usize) -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    let floor = floor(variant);
    for y in 0..16 {
        for x in 0..16 {
            c.px(x as i32, y as i32, floor.at(x, y));
        }
    }
    c.rect(0, 0, 16, 2, 'd');
    c.rect(0, 14, 16, 2, 'd');
    c.rect(if east { 13 } else { 0 }, 0, 3, 16, 'd'); // le montant côté extérieur
    c.finish()
}

/// Une fenêtre à poser sur la partie « rondins » du mur nord : 8×7 pixels. `lit` : éclairée (la nuit).
pub fn window(lit: bool) -> PixelSprite {
    let mut c = Canvas::new(8, 7);
    c.rect(0, 0, 8, 7, 'D');
    c.rect(1, 1, 6, 5, 'd');
    let glass = if lit { 'Y' } else { 'u' };
    c.rect(1, 1, 3, 2, glass);
    c.rect(4, 1, 3, 2, glass);
    c.rect(1, 3, 3, 3, glass);
    c.rect(4, 3, 3, 3, glass);
    if lit {
        c.px(1, 1, 'F');
        c.px(4, 3, 'y');
    } else {
        c.px(1, 1, 'F');
    }
    c.finish()
}

/// La cheminée de pierre : le conduit dépasse du toit (4 pixels au-dessus de la tuile), puis le foyer.
/// 16×20 : les 4 premières lignes sont au-dessus de la tuile. `frame` : 0 à 2, l'image de la flamme.
pub fn fireplace(frame: usize) -> PixelSprite {
    let mut c = Canvas::new(16, 20);
    // le conduit, dans le chaume
    c.rect(5, 0, 6, 10, 'S');
    for y in [1, 4, 7] {
        c.rect(5, y, 6, 1, 's');
    }
    for (x, y) in [(7, 0), (9, 3), (6, 6), (8, 8)] {
        c.px(x, y, 's');
    }
    c.rect(5, 0, 1, 10, 's');
    c.rect(10, 0, 1, 10, 'z');
    // la hotte et le manteau
    c.rect(2, 10, 12, 3, 'S');
    c.rect(2, 12, 12, 1, 's');
    c.rect(1, 13, 14, 2, 'd');
    c.rect(1, 13, 14, 1, 'W');
    // le foyer
    c.rect(2, 15, 12, 5, 'S');
    c.rect(2, 15, 1, 5, 's');
    c.rect(13, 15, 1, 5, 'z');
    c.rect(4, 15, 8, 5, 'k');
    c.rect(4, 19, 8, 1, 'e');
    // la flamme : trois images
    let (tip, wide) = match frame % 3 {
        0 => (16, 3),
        1 => (15, 4),
        _ => (16, 4),
    };
    c.rect(5, 17, 6, 3, 'e');
    c.rect(6, 17, 4, 2, 'O');
    c.rect(7 - (wide - 3) / 2, tip + 1, wide, 3, 'O');
    c.rect(7, tip + 2, 2, 2, 'o');
    c.px(8, tip, 'o');
    c.px(if frame % 2 == 0 { 6 } else { 9 }, 17, 'o');
    c.finish()
}

/// Un lit vertical, tête en haut (16×32) : cadre de bois, oreiller blanc, couverture rouge.
pub fn bed_head_up() -> PixelSprite {
    let mut c = Canvas::new(16, 32);
    c.rect(1, 0, 14, 32, 'd'); // le cadre
    c.rect(1, 0, 14, 4, 'w'); // la tête de lit
    c.rect(1, 0, 14, 1, 'W');
    c.rect(1, 29, 14, 3, 'd'); // le pied de lit
    c.rect(1, 29, 14, 1, 'w');
    c.rect(0, 0, 2, 4, 'D'); // montants
    c.rect(14, 0, 2, 4, 'D');
    c.rect(0, 29, 2, 3, 'D');
    c.rect(14, 29, 2, 3, 'D');
    // le drap, l'oreiller, la couverture
    c.rect(3, 4, 10, 25, 'F');
    c.rect(4, 5, 8, 6, 'f');
    c.rect(4, 5, 8, 5, 'F');
    c.rect(3, 12, 10, 17, 'R');
    c.rect(3, 12, 10, 2, 'r');
    c.rect(3, 22, 10, 1, 'r');
    c.rect(3, 28, 10, 1, 'r');
    c.px(7, 17, 'r');
    c.px(9, 25, 'r');
    c.finish()
}

/// Une table vue de dessus, 2×1 tuiles (32×16), avec une assiette et une tasse.
pub fn table() -> PixelSprite {
    let mut c = Canvas::new(32, 16);
    c.rect(1, 1, 30, 14, 'W');
    c.rect(1, 1, 30, 1, 'w');
    for x in [9, 21] {
        c.rect(x, 2, 1, 12, 'w'); // joints des planches
    }
    c.rect(0, 14, 32, 2, 'D'); // l'ombre du bord
    c.rect(1, 13, 30, 1, 'd');
    // l'assiette
    c.rect(18, 5, 6, 5, 'f');
    c.rect(19, 6, 4, 3, 'F');
    // la tasse
    c.rect(7, 5, 4, 4, 'F');
    c.rect(8, 6, 2, 2, 'd');
    c.px(11, 6, 'F');
    c.finish()
}

/// Une étagère de face, 2×1 tuiles (32×16) : livres et flacons en haut, deux portes basses.
pub fn shelf() -> PixelSprite {
    let mut c = Canvas::new(32, 16);
    c.rect(0, 0, 32, 16, 'd');
    c.rect(1, 1, 14, 9, 'D'); // le fond des deux travées
    c.rect(17, 1, 14, 9, 'D');
    // les livres de la travée de gauche
    for (i, color) in ['L', 'N', 'G', 'R', 'L', 'N'].iter().enumerate() {
        c.rect(2 + i as i32 * 2, 2 + (i as i32 % 2), 2, 7 - (i as i32 % 2), *color);
    }
    c.rect(1, 9, 14, 1, 'W'); // la planche
    // les flacons de la travée de droite
    c.rect(18, 4, 3, 5, 'Q');
    c.rect(19, 2, 1, 2, 'F');
    c.rect(22, 5, 3, 4, 'B');
    c.rect(23, 3, 1, 2, 'F');
    c.rect(26, 4, 3, 5, 'P');
    c.rect(27, 2, 1, 2, 'F');
    c.rect(17, 9, 14, 1, 'W');
    // les portes basses
    c.rect(1, 10, 14, 5, 'w');
    c.rect(17, 10, 14, 5, 'w');
    c.rect(1, 10, 14, 1, 'W');
    c.rect(17, 10, 14, 1, 'W');
    c.rect(1, 14, 14, 1, 'd');
    c.rect(17, 14, 14, 1, 'd');
    c.rect(6, 11, 4, 3, 'd');
    c.rect(22, 11, 4, 3, 'd');
    c.px(8, 12, 'M');
    c.px(24, 12, 'M');
    c.rect(15, 0, 2, 16, 'D'); // le montant du milieu
    c.finish()
}

/// Un baquet : un tonneau d'eau cerclé de métal, vu de dessus, avec sa louche (16×16).
pub fn trough() -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    // les douelles, en cercle
    let spans = [(5, 6), (3, 10), (2, 12), (1, 14), (1, 14), (0, 16), (0, 16), (0, 16), (0, 16), (1, 14), (1, 14), (2, 12), (3, 10), (5, 6)];
    for (i, &(x0, w)) in spans.iter().enumerate() {
        c.rect(x0, i as i32 + 1, w, 1, 'd');
    }
    for (i, &(x0, w)) in spans.iter().enumerate() {
        if w > 4 {
            c.rect(x0 + 1, i as i32 + 1, w - 2, 1, 'w');
        }
    }
    // le cerclage
    for &(x0, w, y) in &[(3, 10, 3), (1, 14, 12)] {
        c.rect(x0, y, w, 1, 'M');
    }
    c.rect(2, 7, 1, 2, 'M');
    c.rect(13, 7, 1, 2, 'M');
    // l'eau
    c.rect(4, 5, 8, 6, 'B');
    c.rect(5, 4, 6, 1, 'B');
    c.rect(5, 11, 6, 1, 'B');
    c.rect(5, 6, 3, 1, 'b');
    c.rect(9, 8, 2, 1, 'b');
    // la louche
    c.rect(9, 3, 5, 1, 'W');
    c.rect(13, 3, 2, 2, 'w');
    c.rect(14, 5, 1, 1, 'd');
    c.outline('k');
    c.finish()
}

/// Le paillasson devant la porte : un petit tapis olive bordé (16×16).
pub fn doormat() -> PixelSprite {
    let mut c = Canvas::new(16, 16);
    c.rect(2, 3, 12, 10, 'c');
    c.rect(3, 4, 10, 8, 'C');
    c.rect(4, 5, 8, 6, 'c');
    c.rect(5, 6, 6, 4, 'C');
    c.px(8, 7, 'W');
    c.px(8, 8, 'W');
    c.finish()
}

/// Tous les sprites de base, nommés : sert à les vérifier tous d'un coup dans les tests.
#[cfg(test)]
pub fn all_pixel_sprites() -> Vec<(&'static str, PixelSprite)> {
    let mut sprites = vec![
        ("wall_north", wall_north()),
        ("wall_north_door", wall_north_door()),
        ("wall_south", wall_south()),
        ("wall_side_west", wall_side(false)),
        ("wall_side_east", wall_side(true)),
        ("window_day", window(false)),
        ("window_lit", window(true)),
        ("bed_head_up", bed_head_up()),
        ("table", table()),
        ("shelf", shelf()),
        ("trough", trough()),
        ("doormat", doormat()),
    ];
    for v in 0..4 {
        sprites.push(("floor", floor(v)));
        sprites.push(("wall_south_door", wall_south_door(v)));
        sprites.push(("wall_side_door_west", wall_side_door(false, v)));
        sprites.push(("wall_side_door_east", wall_side_door(true, v)));
    }
    for f in 0..3 {
        sprites.push(("fireplace", fireplace(f)));
    }
    sprites
}

// ---------------------------------------------------------------- les textures

/// Un sprite prêt à être affiché : sa texture, sans lissage.
pub struct Sprite {
    texture: Texture2D,
    pub w: usize,
    pub h: usize,
}

impl Sprite {
    pub fn new(pixels: &PixelSprite) -> Sprite {
        let texture = Texture2D::from_rgba8(pixels.w as u16, pixels.h as u16, &pixels.rgba());
        texture.set_filter(FilterMode::Nearest); // jamais de lissage : les pixels restent nets
        Sprite { texture, w: pixels.w, h: pixels.h }
    }

    /// Dessine le sprite avec son coin haut-gauche en `(x, y)` pixels écran, chaque pixel du dessin devenant un carré
    /// de `scale × scale` pixels écran (`scale` entier).
    pub fn draw(&self, x: f32, y: f32, scale: f32) {
        let (x, y) = snap(x, y);
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams { dest_size: Some(vec2(self.w as f32 * scale, self.h as f32 * scale)), ..Default::default() },
        );
    }
}

/// Une position écran arrondie à l'entier : un pixel du dessin ne tombe jamais sur un demi-pixel d'écran.
pub fn snap(x: f32, y: f32) -> (f32, f32) {
    (x.round(), y.round())
}

/// Un sprite en quatre rotations (0° à 270°), calculées une fois sur les pixels (rotation exacte, sans lissage).
pub struct Turns {
    by_turn: [Sprite; 4],
}

impl Turns {
    /// `base` : le sprite dans sa pose de référence ; `base_turns` : à combien de quarts de tour (sens horaire)
    /// de la pose de base du jeu (0°) il est dessiné.
    pub fn new(base: &PixelSprite, base_turns: u8) -> Turns {
        // le sprite dessiné à `base_turns` quarts de tour : la rotation 0° est donc `4 - base_turns` quarts plus loin
        let make = |turn: u8| Sprite::new(&base.rotated((turn + 4 - base_turns % 4) % 4));
        Turns { by_turn: [make(0), make(1), make(2), make(3)] }
    }

    pub fn get(&self, quarter_turns: u8) -> &Sprite {
        &self.by_turn[(quarter_turns % 4) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sprite_uses_only_palette_colors_and_has_a_regular_grid() {
        for (name, sprite) in all_pixel_sprites() {
            assert_eq!(sprite.cells.len(), sprite.w * sprite.h, "{name}");
            assert!(sprite.cells.iter().all(|&c| color_of(c).is_some()), "{name} utilise une couleur hors palette");
        }
    }

    #[test]
    fn sizes_are_multiples_of_the_16_pixel_base_tile() {
        let expect = |name: &str, w: usize, h: usize| {
            let sprite = all_pixel_sprites().into_iter().find(|(n, _)| *n == name).unwrap().1;
            assert_eq!((sprite.w, sprite.h), (w, h), "{name}");
        };
        expect("floor", 16, 16);
        expect("wall_north", 16, 16);
        expect("wall_south", 16, 16);
        expect("doormat", 16, 16);
        expect("trough", 16, 16);
        expect("bed_head_up", 16, 32);
        expect("table", 32, 16);
        expect("shelf", 32, 16);
        expect("fireplace", 16, 20);
        expect("window_day", 8, 7);
    }

    #[test]
    fn a_color_outside_the_palette_is_refused() {
        let rows = vec!["kW".to_string(), "?W".to_string()];
        assert!(PixelSprite::parse(&rows).unwrap_err().contains("hors palette"));
    }

    #[test]
    fn an_irregular_grid_is_refused() {
        let rows = vec!["kW".to_string(), "k".to_string()];
        assert!(PixelSprite::parse(&rows).unwrap_err().contains("pixels au lieu de"));
        assert!(PixelSprite::parse(&[]).is_err());
    }

    #[test]
    fn a_quarter_turn_moves_the_top_left_pixel_to_the_top_right_and_four_turns_are_the_identity() {
        let rows = vec!["kW.".to_string(), "dD.".to_string()];
        let sprite = PixelSprite::parse(&rows).unwrap();
        let turned = sprite.rotated_cw();
        assert_eq!((turned.w, turned.h), (2, 3));
        assert_eq!(turned.at(1, 0), 'k', "le coin haut-gauche passe en haut à droite");
        assert_eq!(turned.at(0, 0), 'd');
        assert_eq!(sprite.rotated(4), sprite);
        assert_eq!(sprite.rotated(2).at(2, 1), 'k');
    }

    #[test]
    fn the_bed_drawn_head_up_keeps_its_pillow_at_the_head_in_every_rotation() {
        // l'oreiller blanc est près de la tête : en haut, à droite, en bas, à gauche selon le quart de tour
        let bed = bed_head_up();
        let pillow = |s: &PixelSprite| -> (f32, f32) {
            let (mut sx, mut sy, mut n) = (0.0, 0.0, 0.0);
            for y in 0..s.h {
                for x in 0..s.w {
                    if s.at(x, y) == 'F' {
                        sx += x as f32 / s.w as f32;
                        sy += y as f32 / s.h as f32;
                        n += 1.0;
                    }
                }
            }
            (sx / n, sy / n)
        };
        let (_, up_y) = pillow(&bed);
        assert!(up_y < 0.5, "l'oreiller est en haut quand le lit regarde en haut");
        let (right_x, _) = pillow(&bed.rotated(1));
        assert!(right_x > 0.5, "un quart de tour : la tête à droite");
        let (_, down_y) = pillow(&bed.rotated(2));
        assert!(down_y > 0.5);
        let (left_x, _) = pillow(&bed.rotated(3));
        assert!(left_x < 0.5);
    }

    #[test]
    fn screen_positions_are_snapped_to_whole_pixels() {
        assert_eq!(snap(10.4, 20.6), (10.0, 21.0));
        assert_eq!(snap(-0.4, 0.5), (-0.0, 1.0));
    }

    #[test]
    fn floor_variants_differ_so_the_planks_do_not_repeat_visibly() {
        assert_ne!(floor(0), floor(1));
        assert_ne!(floor(1), floor(2));
    }
}
