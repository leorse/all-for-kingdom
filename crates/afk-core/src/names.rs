//! Les prénoms de tous les habitants : médiévaux et fantasy, neutres (ils vont à tout le monde, aucun ne dit un sexe).

pub const FIRST_NAMES: &[&str] = &[
    "Aldwyn", "Alaric", "Elowen", "Thorne", "Ysolde", "Kaelen", "Brannoc", "Merrin", "Rowan", "Ashlin",
    "Corwyn", "Eldric", "Fenwick", "Galen", "Hawthorn", "Isolde", "Jorund", "Kestrel", "Lorcan", "Maelis",
    "Nimue", "Orwin", "Perrin", "Quill", "Rhosyn", "Sorrel", "Tavish", "Ulric", "Vesper", "Wynne",
    "Ambrose", "Briar", "Caelum", "Dunstan", "Edda", "Falk", "Garrick", "Hollis", "Ivor", "Jasper",
    "Keldan", "Lyra", "Morwen", "Nyx", "Oberon", "Pip", "Quentin", "Rune", "Selwyn", "Tamsin",
    "Uther", "Vaelin", "Wystan", "Yarrow", "Zephyr", "Aethel", "Branwen", "Cadoc", "Dagny", "Eira",
    "Fael", "Gwydion", "Hild", "Idris", "Jocasta", "Kerrin", "Leofric", "Marrow", "Neve", "Osric",
    "Peregrine", "Rhiannon", "Sigrun", "Talon", "Una", "Vigdis", "Willow", "Xan", "Ysmay", "Zora",
    "Arden", "Bryn", "Cedar", "Dwyn", "Emrys", "Fiora", "Gawain", "Heloise", "Isen", "Jory",
    "Kell", "Lunet", "Mordred", "Nerys", "Oriel", "Percival", "Ravenna", "Sable", "Torin", "Umber",
    "Valdis", "Wren", "Ysabeau", "Zinnia", "Alder", "Bevan", "Cinder", "Drystan", "Elspeth", "Frey",
    "Garrow", "Hesper", "Ilar", "Jarl", "Kyra", "Lioness", "Maren", "Nolwenn", "Ottoline", "Piers",
    "Rook", "Seren", "Thistle", "Ulla", "Vance", "Wolfram", "Yrsa", "Zeru", "Ansel", "Brynja",
    "Cael", "Dorian", "Elric", "Faelan", "Gorm", "Halvard", "Ingrith", "Jules", "Kieran", "Liriel",
    "Mirelle", "Nairn", "Ondine", "Pelleas", "Quillon", "Rhys", "Soren", "Tristane", "Ursin", "Vael",
    "Wilmot", "Xylia", "Yvaine", "Zerin", "Aeron", "Bellamy", "Corin", "Dessa", "Eldon", "Farrin",
    "Gwenllian", "Haldor", "Ismay", "Jareth", "Kaede", "Lothar", "Maeve", "Neldor", "Orla", "Pryce",
    "Raegan", "Sylas", "Tolliver", "Ulfric", "Verdan", "Wilder", "Yoran", "Zaffre", "Ashby", "Bodil",
    "Aubrey", "Beowyn", "Cormac", "Dilys", "Eamon", "Fenna", "Godwin", "Hartwin", "Ilse", "Jovan",
    "Kelda", "Lysander", "Merewyn", "Nuala", "Osgar", "Petra", "Rafe", "Saskia", "Tiernan", "Vilja",
];

/// Le prénom numéro `pick` (modulo la liste) qui n'est pas dans `taken` ; si tous le sont, le premier choisi suivi d'un
/// numéro de lignée (« Aubin II »).
pub fn pick_name(pick: usize, taken: &[&str]) -> String {
    let n = FIRST_NAMES.len();
    for k in 0..n {
        let name = FIRST_NAMES[(pick + k) % n];
        if !taken.contains(&name) {
            return name.to_string();
        }
    }
    let base = FIRST_NAMES[pick % n];
    let mut ordinal = 2;
    while taken.iter().any(|t| *t == format!("{base} {ordinal}")) {
        ordinal += 1;
    }
    format!("{base} {ordinal}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_hundred_distinct_names() {
        let mut sorted = FIRST_NAMES.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 200);
        assert!(FIRST_NAMES.iter().all(|n| !n.is_empty() && !n.contains(' ')));
    }

    #[test]
    fn a_taken_name_is_skipped() {
        assert_eq!(pick_name(0, &[FIRST_NAMES[0]]), FIRST_NAMES[1]);
        assert_eq!(pick_name(5 + FIRST_NAMES.len(), &[]), FIRST_NAMES[5]);
        assert_eq!(pick_name(0, FIRST_NAMES), format!("{} 2", FIRST_NAMES[0]));
    }
}
