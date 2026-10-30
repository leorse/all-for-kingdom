//! La conversion d'un dessin SVG de personnage en planche PNG (fonctionnalité `spritegen`, voir `sprites.bat`).
//!
//! Le SVG est une planche de cellules de 64×64 (`character_sprites`) où chaque image est un groupe `<g id="...">`.
//! On le rend à l'échelle 1 sans lissage (`shape-rendering="crispEdges"`), on crée les images vers la gauche en miroir
//! de celles vers la droite quand le SVG ne les dessine pas, on ajoute le contour sombre extérieur, puis on refuse
//! toute couleur hors de la palette commune et toute demi-transparence.

use crate::character_sprites::{mirror_source, SheetSpec, CELL};
use crate::sprites::PALETTE;
use resvg::tiny_skia::{IntSize, Pixmap, Transform};
use resvg::usvg;

/// La couleur du contour ajouté autour des silhouettes.
const OUTLINE: char = 'k';

/// Une planche en pixels RGBA (non prémultipliés : chaque pixel est opaque ou entièrement transparent).
#[derive(Clone, Debug, PartialEq)]
pub struct Sheet {
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
}

impl Sheet {
    fn idx(&self, x: u32, y: u32) -> usize {
        ((y * self.w + x) * 4) as usize
    }

    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let i = self.idx(x, y);
        [self.rgba[i], self.rgba[i + 1], self.rgba[i + 2], self.rgba[i + 3]]
    }

    fn set(&mut self, x: u32, y: u32, p: [u8; 4]) {
        let i = self.idx(x, y);
        self.rgba[i..i + 4].copy_from_slice(&p);
    }

    fn cell_is_empty(&self, col: u32, row: u32) -> bool {
        (0..CELL).all(|y| (0..CELL).all(|x| self.pixel(col * CELL + x, row * CELL + y)[3] == 0))
    }

    pub fn encode_png(&self) -> Vec<u8> {
        // tous les pixels sont opaques ou nuls : les données sont déjà « prémultipliées »
        let pixmap = Pixmap::from_vec(self.rgba.clone(), IntSize::from_wh(self.w, self.h).unwrap()).unwrap();
        pixmap.encode_png().expect("encodage PNG")
    }

    pub fn decode_png(bytes: &[u8]) -> Result<Sheet, String> {
        let pixmap = Pixmap::decode_png(bytes).map_err(|e| e.to_string())?;
        Ok(Sheet { w: pixmap.width(), h: pixmap.height(), rgba: pixmap.data().to_vec() })
    }
}

/// Convertit le SVG d'un personnage en planche. L'erreur nomme ce qui ne va pas (image manquante, pixel hors palette).
pub fn render_sheet(svg: &str, spec: &SheetSpec) -> Result<Sheet, String> {
    let name = spec.name;
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|e| format!("{name} : SVG illisible ({e})"))?;
    let (w, h) = spec.size();
    let size = tree.size();
    if size.width() != w as f32 || size.height() != h as f32 {
        return Err(format!("{name} : la planche doit faire {w}×{h}, pas {}×{}", size.width(), size.height()));
    }

    // les images exigées par le gabarit : dessinées, ou obtenues par miroir
    let mut mirrors = Vec::new();
    for (id, col, row) in spec.cells() {
        if tree.node_by_id(&id).is_some() {
            continue;
        }
        let source = mirror_source(&id).filter(|s| tree.node_by_id(s).is_some());
        match source {
            Some(s) => {
                let (_, scol, srow) = spec.cells().into_iter().find(|(i, _, _)| *i == s).unwrap();
                mirrors.push(((scol, srow), (col, row)));
            }
            None => return Err(format!("{name} : image manquante « {id} »")),
        }
    }

    let mut pixmap = Pixmap::new(w, h).unwrap();
    resvg::render(&tree, Transform::identity(), &mut pixmap.as_mut());
    let mut sheet = Sheet { w, h, rgba: pixmap.data().to_vec() };
    check_palette(&sheet, name)?;

    for ((scol, srow), (col, row)) in mirrors {
        mirror_cell(&mut sheet, (scol, srow), (col, row));
    }
    add_outline(&mut sheet);

    for (id, col, row) in spec.cells() {
        if sheet.cell_is_empty(col, row) {
            return Err(format!("{name} : l'image « {id} » est vide"));
        }
    }
    Ok(sheet)
}

/// Chaque pixel est entièrement transparent, ou opaque et d'une couleur de la palette.
pub fn check_palette(sheet: &Sheet, name: &str) -> Result<(), String> {
    for y in 0..sheet.h {
        for x in 0..sheet.w {
            let p = sheet.pixel(x, y);
            let ok = p[3] == 0 || (p[3] == 255 && PALETTE.iter().any(|(_, c)| c[..3] == p[..3] && c[3] == 255));
            if !ok {
                return Err(format!(
                    "{name} : pixel ({x}, {y}) de couleur #{:02x}{:02x}{:02x} (alpha {}) hors palette, ou lissé",
                    p[0], p[1], p[2], p[3]
                ));
            }
        }
    }
    Ok(())
}

/// Copie une cellule dans une autre en miroir horizontal, autour de l'axe vertical de l'ancre (le milieu de la cellule).
fn mirror_cell(sheet: &mut Sheet, (scol, srow): (u32, u32), (col, row): (u32, u32)) {
    for y in 0..CELL {
        for x in 0..CELL {
            let p = sheet.pixel(scol * CELL + x, srow * CELL + y);
            sheet.set(col * CELL + (CELL - 1 - x), row * CELL + y, p);
        }
    }
}

/// Entoure chaque silhouette d'un pixel de contour sombre, sans sortir de sa cellule.
fn add_outline(sheet: &mut Sheet) {
    let color = PALETTE.iter().find(|(c, _)| *c == OUTLINE).unwrap().1;
    let src = sheet.clone();
    for y in 0..sheet.h {
        for x in 0..sheet.w {
            if src.pixel(x, y)[3] != 0 {
                continue;
            }
            let (cx, cy) = (x / CELL, y / CELL);
            let touches = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)].iter().any(|(dx, dy)| {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                nx >= 0
                    && ny >= 0
                    && (nx as u32) < src.w
                    && (ny as u32) < src.h
                    && nx as u32 / CELL == cx
                    && ny as u32 / CELL == cy
                    && src.pixel(nx as u32, ny as u32)[3] != 0
            });
            if touches {
                sheet.set(x, y, color);
            }
        }
    }
}

/// Un aperçu HTML : la planche agrandie ×4, sans lissage, avec la grille et le nom de chaque image. C'est là qu'on
/// relit le visage et l'alternance des jambes.
pub fn preview_html(spec: &SheetSpec, png_file: &str) -> String {
    let (w, h) = spec.size();
    let z = 4;
    let mut labels = String::new();
    for (id, col, row) in spec.cells() {
        labels.push_str(&format!(
            "<div class=c style=\"left:{}px;top:{}px\"><span>{id}</span></div>\n",
            col * CELL * z,
            row * CELL * z
        ));
    }
    format!(
        "<!doctype html><meta charset=utf-8><title>{name}</title>\n<style>\
body{{background:#3a2a20;color:#f0ead8;font:12px sans-serif;margin:16px}}\
.s{{position:relative;width:{pw}px;height:{ph}px;background:repeating-conic-gradient(#4a3a30 0 25%,#54443a 0 50%) 0 0/32px 32px}}\
.s img{{position:absolute;inset:0;width:100%;height:100%;image-rendering:pixelated}}\
.c{{position:absolute;width:{cw}px;height:{cw}px;box-sizing:border-box;border:1px solid #0006}}\
.c span{{background:#000a;padding:1px 3px}}\
.a{{position:absolute;width:6px;height:6px;margin:-3px;border-radius:3px;background:#f0f}}\
</style>\n<h1>{name}</h1><p>Ancre des pieds en magenta. Ligne par animation, colonne par image.</p>\n\
<div class=s><img src=\"{png_file}\">\n{labels}{anchors}</div>\n",
        name = spec.name,
        pw = w * z,
        ph = h * z,
        cw = CELL * z,
        anchors = spec
            .cells()
            .iter()
            .map(|(_, col, row)| {
                let (ax, ay) = crate::character_sprites::ANCHOR;
                format!(
                    "<div class=a style=\"left:{}px;top:{}px\"></div>",
                    (col * CELL) as f32 * z as f32 + ax * z as f32,
                    (row * CELL) as f32 * z as f32 + ay * z as f32
                )
            })
            .collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character_sprites::{Anim, SheetSpec, COMMON, DIRECTIONS, EXTRAS_ROW, WORK_ROW};

    const TINY: SheetSpec = SheetSpec {
        name: "essai",
        work: Anim { name: "travail", row: WORK_ROW, durations: &[1.0] },
        extras: &["objet"],
    };

    /// Un SVG d'essai : un carré de bois dans chaque cellule exigée, sauf celles qu'on veut omettre ; `extra` est
    /// ajouté tel quel.
    fn svg(omit: &[&str], extra: &str) -> String {
        let (w, h) = TINY.size();
        let mut s = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" shape-rendering=\"crispEdges\">"
        );
        for (id, col, row) in TINY.cells() {
            if omit.contains(&id.as_str()) {
                continue;
            }
            // un carré décentré (vers la droite de l'ancre) pour voir le miroir
            s.push_str(&format!(
                "<g id=\"{id}\" transform=\"translate({} {})\"><rect x=\"34\" y=\"20\" width=\"6\" height=\"30\" fill=\"#b27a3c\"/></g>",
                col * CELL,
                row * CELL
            ));
        }
        s.push_str(extra);
        s.push_str("</svg>");
        s
    }

    #[test]
    fn a_missing_frame_is_named() {
        let err = render_sheet(&svg(&["repos_2"], ""), &TINY).unwrap_err();
        assert!(err.contains("repos_2"), "{err}");
    }

    #[test]
    fn left_frames_are_mirrors_of_right_ones_when_missing() {
        let left: Vec<String> = (0..4).map(|n| format!("marche_gauche_{n}")).chain(["debout_gauche".to_string()]).collect();
        let omit: Vec<&str> = left.iter().map(|s| s.as_str()).collect();
        let sheet = render_sheet(&svg(&omit, ""), &TINY).unwrap();
        let (rrow, lrow) = (COMMON[3].row, COMMON[4].row);
        for y in 0..CELL {
            for x in 0..CELL {
                let right = sheet.pixel(x, rrow * CELL + y);
                let mirrored = sheet.pixel(CELL - 1 - x, lrow * CELL + y);
                assert_eq!(right, mirrored, "({x}, {y})");
            }
        }
        // le carré dessiné à droite de l'ancre se retrouve à sa gauche
        assert_eq!(sheet.pixel(36, rrow * CELL + 30)[3], 255);
        assert_eq!(sheet.pixel(CELL - 1 - 36, lrow * CELL + 30)[3], 255);
        assert_eq!(sheet.pixel(36, lrow * CELL + 30)[3], 0);
        assert_eq!(DIRECTIONS[3], "gauche");
    }

    #[test]
    fn a_colour_outside_the_palette_is_refused_with_its_position() {
        let extra = "<rect x=\"3\" y=\"5\" width=\"1\" height=\"1\" fill=\"#123456\"/>";
        let err = render_sheet(&svg(&[], extra), &TINY).unwrap_err();
        assert!(err.contains("(3, 5)") && err.contains("#123456"), "{err}");
    }

    #[test]
    fn the_outline_is_one_pixel_and_stays_in_its_cell() {
        let sheet = render_sheet(&svg(&[], ""), &TINY).unwrap();
        let k = PALETTE.iter().find(|(c, _)| *c == 'k').unwrap().1;
        // le carré va de x=34 à 39, y=20 à 49 : contour en x=33 et x=40, rien au-delà
        assert_eq!(sheet.pixel(33, 30), k);
        assert_eq!(sheet.pixel(40, 30), k);
        assert_eq!(sheet.pixel(32, 30)[3], 0);
        assert_eq!(sheet.pixel(41, 30)[3], 0);
        assert_eq!(sheet.pixel(36, 19), k);
        // une forme qui touche le bord de sa cellule n'en déborde pas dans la voisine
        let extra = format!("<rect x=\"{}\" y=\"10\" width=\"2\" height=\"2\" fill=\"#b27a3c\"/>", CELL - 2);
        let sheet = render_sheet(&svg(&[], &extra), &TINY).unwrap();
        assert_eq!(sheet.pixel(CELL, 10)[3], 0, "rien dans la cellule voisine");
    }

    #[test]
    fn an_empty_extra_cell_is_refused() {
        // présent mais vide : le groupe existe sans rien dedans
        let mut s = svg(&["objet"], "");
        s = s.replace("</svg>", &format!("<g id=\"objet\" transform=\"translate(0 {})\"/></svg>", EXTRAS_ROW * CELL));
        let err = render_sheet(&s, &TINY).unwrap_err();
        assert!(err.contains("objet") && err.contains("vide"), "{err}");
    }

    #[test]
    fn png_round_trip_keeps_every_pixel() {
        let sheet = render_sheet(&svg(&[], ""), &TINY).unwrap();
        assert_eq!(Sheet::decode_png(&sheet.encode_png()).unwrap(), sheet);
    }

    /// Les planches commitées correspondent à leur SVG : sinon, relancer `sprites.bat`.
    #[test]
    fn committed_sheets_match_their_svg() {
        for spec in crate::character_sprites::SHEETS {
            let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/characters");
            let svg = std::fs::read_to_string(dir.join(format!("{}.svg", spec.name))).unwrap();
            let png = std::fs::read(dir.join(format!("{}.png", spec.name))).unwrap();
            let expected = render_sheet(&svg, &spec).unwrap();
            assert!(
                Sheet::decode_png(&png).unwrap() == expected,
                "{} : le PNG ne correspond plus au SVG, relancer sprites.bat",
                spec.name
            );
        }
    }
}
