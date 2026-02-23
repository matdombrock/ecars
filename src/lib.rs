use rand::Rng;
use wasm_bindgen::prelude::*;
mod rng_utils;
use rng_utils::seeded_small_rng;

#[wasm_bindgen]
pub fn run_automaton(
    rule: u8,
    random_distribution: Option<f64>,
    width: usize,
    generations: usize,
    seed: Option<u64>,
) -> Vec<u8> {
    let mut rng = seeded_small_rng(seed);

    let mut current = vec![0u8; width];
    if let Some(p) = random_distribution {
        if p > 0.0 {
            for cell in current.iter_mut() {
                // Use next_u32 for randomness, convert to [0,1)
                let rand_val = (rng.next_u32() as f64) / (u32::MAX as f64 + 1.0);
                *cell = if rand_val < p { 1 } else { 0 };
            }
        } else {
            current[width / 2] = 1;
        }
    } else {
        current[width / 2] = 1;
    }
    let mut generations_vec = Vec::with_capacity(generations * width);
    generations_vec.extend_from_slice(&current);
    for _ in 1..generations {
        let mut next = vec![0u8; width];
        for i in 0..width {
            let left = if i == 0 { 0 } else { current[i - 1] };
            let center = current[i];
            let right = if i == width - 1 { 0 } else { current[i + 1] };
            let idx = (left << 2) | (center << 1) | right;
            next[i] = (rule >> idx) & 1;
        }
        generations_vec.extend_from_slice(&next);
        current = next;
    }
    generations_vec
}

