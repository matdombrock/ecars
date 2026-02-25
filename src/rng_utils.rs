use getrandom::getrandom;
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// Returns a seeded SmallRng using OS randomness or a provided u64 seed.
pub fn seeded_small_rng(seed: Option<u64>) -> SmallRng {
    let mut seed_arr = [0u8; 32];
    if let Some(s) = seed {
        // Fill the first 8 bytes with the u64 seed, rest zero
        seed_arr[..8].copy_from_slice(&s.to_le_bytes());
    } else {
        getrandom(&mut seed_arr).expect("Failed to get randomness from OS");
    }
    SmallRng::from_seed(seed_arr)
}

