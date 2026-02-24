use image::{Rgb, RgbImage};

/// Linearly interpolate between two colors.
fn lerp_color(from: &Rgb<u8>, to: &Rgb<u8>, t: f32) -> Rgb<u8> {
    let r = from[0] as f32 + (to[0] as f32 - from[0] as f32) * t;
    let g = from[1] as f32 + (to[1] as f32 - from[1] as f32) * t;
    let b = from[2] as f32 + (to[2] as f32 - from[2] as f32) * t;
    Rgb([r.round() as u8, g.round() as u8, b.round() as u8])
}

/// Save the automaton generations as a PNG image.
/// Each row is a generation, each column is a cell.
/// Each cell is scaled up by `scale`.
/// If `use_circles` is true, cells are drawn as circles; otherwise, as squares.
/// Colors for background and foreground are interpolated from *_from to *_to.
pub fn save_generations_as_png(
    generations: &[Vec<u8>],
    width: usize,
    height: usize,
    scale: usize,
    use_circles: bool,
    output_path: &str,
    bg_from: Rgb<u8>,
    bg_to: Rgb<u8>,
    fg_from: Rgb<u8>,
    fg_to: Rgb<u8>,
) {
    let img_width = (width * scale) as u32;
    let img_height = (height * scale) as u32;
    let mut img = RgbImage::new(img_width, img_height);

    for (y, gen) in generations.iter().enumerate() {
        for (x, &cell) in gen.iter().enumerate() {
            // Interpolation factor: horizontal (x), vertical (y), or diagonal (average)
            let fx = if width > 1 {
                x as f32 / (width - 1) as f32
            } else {
                0.0
            };
            let fy = if height > 1 {
                y as f32 / (height - 1) as f32
            } else {
                0.0
            };
            let t = (fx + fy) / 2.0; // Diagonal gradient; adjust as needed

            let color = if cell == 1 {
                lerp_color(&fg_from, &fg_to, t)
            } else {
                lerp_color(&bg_from, &bg_to, t)
            };

            if use_circles {
                // Draw a circle of radius 0.5 * scale, centered in the cell
                let radius = scale as f32 * 0.5;
                let center_x = x as f32 * scale as f32 + radius;
                let center_y = y as f32 * scale as f32 + radius;
                for dy in 0..scale {
                    for dx in 0..scale {
                        let px = x as f32 * scale as f32 + dx as f32 + 0.5;
                        let py = y as f32 * scale as f32 + dy as f32 + 0.5;
                        let dist = ((px - center_x).powi(2) + (py - center_y).powi(2)).sqrt();
                        if dist <= radius {
                            img.put_pixel((x * scale + dx) as u32, (y * scale + dy) as u32, color);
                        }
                    }
                }
            } else {
                // Fill a scale x scale block (square)
                for dy in 0..scale {
                    for dx in 0..scale {
                        img.put_pixel((x * scale + dx) as u32, (y * scale + dy) as u32, color);
                    }
                }
            }
        }
    }
    img.save(output_path).expect("Failed to save PNG");
}

/// Generate an RGBA buffer for the automaton generations (for WASM canvas rendering).
pub fn generations_to_rgba_buffer(
    generations: &[Vec<u8>],
    width: usize,
    height: usize,
    scale: usize,
    use_circles: bool,
    bg_from: Rgb<u8>,
    bg_to: Rgb<u8>,
    fg_from: Rgb<u8>,
    fg_to: Rgb<u8>,
) -> Vec<u8> {
    let img_width = (width * scale) as u32;
    let img_height = (height * scale) as u32;
    let mut buffer = vec![0u8; (img_width * img_height * 4) as usize];

    for (y, gen) in generations.iter().enumerate() {
        for (x, &cell) in gen.iter().enumerate() {
            let fx = if width > 1 {
                x as f32 / (width - 1) as f32
            } else {
                0.0
            };
            let fy = if height > 1 {
                y as f32 / (height - 1) as f32
            } else {
                0.0
            };
            let t = (fx + fy) / 2.0;
            let color = if cell == 1 {
                lerp_color(&fg_from, &fg_to, t)
            } else {
                lerp_color(&bg_from, &bg_to, t)
            };

            if use_circles {
                let radius = scale as f32 * 0.5;
                let center_x = x as f32 * scale as f32 + radius;
                let center_y = y as f32 * scale as f32 + radius;
                for dy in 0..scale {
                    for dx in 0..scale {
                        let px = x as f32 * scale as f32 + dx as f32 + 0.5;
                        let py = y as f32 * scale as f32 + dy as f32 + 0.5;
                        let dist = ((px - center_x).powi(2) + (py - center_y).powi(2)).sqrt();
                        if dist <= radius {
                            let idx = (((y * scale + dy) * (width * scale) + (x * scale + dx)) * 4) as usize;
                            buffer[idx] = color[0];
                            buffer[idx + 1] = color[1];
                            buffer[idx + 2] = color[2];
                            buffer[idx + 3] = 255;
                        }
                    }
                }
            } else {
                for dy in 0..scale {
                    for dx in 0..scale {
                        let idx = (((y * scale + dy) * (width * scale) + (x * scale + dx)) * 4) as usize;
                        buffer[idx] = color[0];
                        buffer[idx + 1] = color[1];
                        buffer[idx + 2] = color[2];
                        buffer[idx + 3] = 255;
                    }
                }
            }
        }
    }
    buffer
}

