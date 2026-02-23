use getrandom::getrandom;
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// Returns a seeded SmallRng using OS randomness.
pub fn seeded_small_rng() -> SmallRng {
    let mut seed = [0u8; 32];
    getrandom(&mut seed).expect("Failed to get randomness from OS");
    SmallRng::from_seed(seed)
}
