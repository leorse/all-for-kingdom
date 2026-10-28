//! Convertit les dessins SVG des personnages (`assets/characters/*.svg`) en planches PNG pour le jeu.
//!
//!   sprites.bat           régénère chaque planche et son aperçu (`target/sprites-preview/<nom>.html`)
//!   sprites.bat --check   vérifie seulement que les PNG commités correspondent à leur SVG

#[allow(dead_code)]
#[path = "../character_sprites.rs"]
mod character_sprites;
#[allow(dead_code)]
#[path = "../sprites.rs"]
mod sprites;
#[path = "../spritegen.rs"]
mod spritegen;

use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let check = std::env::args().any(|a| a == "--check");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dir = root.join("assets/characters");
    let preview = root.join("../../target/sprites-preview");
    let mut failed = false;

    for spec in character_sprites::SHEETS {
        let svg_path = dir.join(format!("{}.svg", spec.name));
        let png_path = dir.join(format!("{}.png", spec.name));
        let result = std::fs::read_to_string(&svg_path)
            .map_err(|e| format!("{} : {e}", svg_path.display()))
            .and_then(|svg| spritegen::render_sheet(&svg, &spec));
        let sheet = match result {
            Ok(sheet) => sheet,
            Err(e) => {
                eprintln!("ERREUR {e}");
                failed = true;
                continue;
            }
        };
        if check {
            let same = std::fs::read(&png_path)
                .ok()
                .and_then(|bytes| spritegen::Sheet::decode_png(&bytes).ok())
                .is_some_and(|committed| committed == sheet);
            if same {
                println!("ok      {}", spec.name);
            } else {
                eprintln!("ERREUR {} : le PNG ne correspond plus au SVG, relancer sprites.bat", spec.name);
                failed = true;
            }
            continue;
        }
        std::fs::write(&png_path, sheet.encode_png()).expect("écriture du PNG");
        std::fs::create_dir_all(&preview).expect("dossier d'aperçu");
        std::fs::copy(&png_path, preview.join(format!("{}.png", spec.name))).expect("copie de l'aperçu");
        std::fs::write(preview.join(format!("{}.html", spec.name)), spritegen::preview_html(&spec, &format!("{}.png", spec.name)))
            .expect("écriture de l'aperçu");
        println!("généré  {}  (aperçu : target/sprites-preview/{}.html)", png_path.display(), spec.name);
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
