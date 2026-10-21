//! Petit générateur pseudo-aléatoire déterministe (xorshift64).
//! Déterministe = même graine, même partie : indispensable pour les tests et, plus tard, pour la coop.

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed })
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Indice aléatoire dans `0..len` (len > 0).
    pub fn index(&mut self, len: usize) -> usize {
        (self.next_u64() % len as u64) as usize
    }

    /// Flottant aléatoire dans `[min, max)`.
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        let t = (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32;
        min + t * (max - min)
    }
}
