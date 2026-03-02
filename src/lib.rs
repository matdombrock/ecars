use rand::Rng;
use wasm_bindgen::prelude::*;
mod image_output;
mod rng_utils;
use image::Rgb;
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

/// Helper to parse hex color string (e.g. "#ffaaff") to Rgb<u8>
fn parse_hex_color(s: &str) -> Rgb<u8> {
    let s = s.strip_prefix('#').unwrap_or(s);
    assert!(s.len() == 6, "Color must be in format #RRGGBB");
    let r = u8::from_str_radix(&s[0..2], 16).expect("Invalid hex color");
    let g = u8::from_str_radix(&s[2..4], 16).expect("Invalid hex color");
    let b = u8::from_str_radix(&s[4..6], 16).expect("Invalid hex color");
    Rgb([r, g, b])
}

/// WASM-exported: Generate RGBA buffer for automaton image (for canvas rendering)
#[wasm_bindgen]
pub fn generate_automaton_image(
    rule: u8,
    random_distribution: Option<f64>,
    width: usize,
    generations: usize,
    seed: Option<u64>,
    scale: usize,
    alive_shape: &str,
    dead_shape: &str,
    use_links: bool,
    bg_from: &str,
    bg_to: &str,
    fg_from: &str,
    fg_to: &str,
    mirror_x: bool,
    mirror_y: bool,
    mirror_share_center: bool,
) -> Vec<u8> {
    let mut rng = seeded_small_rng(seed);
    let mut current = vec![0u8; width];
    if let Some(p) = random_distribution {
        if p > 0.0 {
            for cell in current.iter_mut() {
                let rand_val = (rng.next_u32() as f64) / (u32::MAX as f64 + 1.0);
                *cell = if rand_val < p { 1 } else { 0 };
            }
        } else {
            current[width / 2] = 1;
        }
    } else {
        current[width / 2] = 1;
    }
    let mut generations_vec = Vec::with_capacity(generations);
    generations_vec.push(current.clone());
    for _ in 1..generations {
        let mut next = vec![0u8; width];
        for i in 0..width {
            let left = if i == 0 { 0 } else { current[i - 1] };
            let center = current[i];
            let right = if i == width - 1 { 0 } else { current[i + 1] };
            let idx = (left << 2) | (center << 1) | right;
            next[i] = (rule >> idx) & 1;
        }
        generations_vec.push(next.clone());
        current = next;
    }
    let dead_from = parse_hex_color(bg_from);
    let dead_to = parse_hex_color(bg_to);
    let alive_from = parse_hex_color(fg_from);
    let alive_to = parse_hex_color(fg_to);
    {
        let (buf, w, h) = image_output::generations_to_rgba_buffer(
            &generations_vec,
            width,
            generations,
            scale,
            alive_shape,
            dead_shape,
            use_links,
            dead_from,
            dead_to,
            alive_from,
            alive_to,
            mirror_x,
            mirror_y,
            mirror_share_center,
        );
        // Prepend width and height as 8 bytes (little-endian u32,u32) for the JS caller
        let mut out = Vec::with_capacity(8 + buf.len());
        out.extend_from_slice(&w.to_le_bytes());
        out.extend_from_slice(&h.to_le_bytes());
        out.extend_from_slice(&buf);
        out
    }
}
