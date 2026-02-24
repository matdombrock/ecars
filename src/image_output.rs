use image::Rgb;
use image::{ImageBuffer, Rgba};

/// Linearly interpolate between two colors.
fn lerp_color(from: &Rgb<u8>, to: &Rgb<u8>, t: f32) -> Rgb<u8> {
    let r = from[0] as f32 + (to[0] as f32 - from[0] as f32) * t;
    let g = from[1] as f32 + (to[1] as f32 - from[1] as f32) * t;
    let b = from[2] as f32 + (to[2] as f32 - from[2] as f32) * t;
    Rgb([r.round() as u8, g.round() as u8, b.round() as u8])
}

#[allow(dead_code)] // NOTE: This is not dead, compiler issue
pub fn save_generations_as_png(
    generations: &[Vec<u8>],
    width: usize,
    height: usize,
    scale: usize,
    use_circles: bool,
    use_links: bool,
    output_path: &str,
    bg_from: Rgb<u8>,
    bg_to: Rgb<u8>,
    fg_from: Rgb<u8>,
    fg_to: Rgb<u8>,
) {
    let buffer = generations_to_rgba_buffer(
        generations,
        width,
        height,
        scale,
        use_circles,
        use_links,
        bg_from,
        bg_to,
        fg_from,
        fg_to,
    );
    let img_width = (width * scale) as u32;
    let img_height = (height * scale) as u32;
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(img_width, img_height, buffer)
        .expect("Failed to create image buffer");
    img.save(output_path).expect("Failed to save PNG");
}

/// Generate an RGBA buffer for the automaton generations (for WASM canvas rendering).
pub fn generations_to_rgba_buffer(
    generations: &[Vec<u8>],
    width: usize,
    height: usize,
    scale: usize,
    use_circles: bool,
    use_links: bool,
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
                            let idx = (((y * scale + dy) * (width * scale) + (x * scale + dx)) * 4)
                                as usize;
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
                        let idx =
                            (((y * scale + dy) * (width * scale) + (x * scale + dx)) * 4) as usize;
                        buffer[idx] = color[0];
                        buffer[idx + 1] = color[1];
                        buffer[idx + 2] = color[2];
                        buffer[idx + 3] = 255;
                    }
                }
            }
        }
    }
    // Draw links if requested (post-processing)
    if use_links {
        draw_links_bresenham_rgba(
            &mut buffer,
            generations,
            width,
            height,
            scale,
            fg_from,
            fg_to,
            bg_from,
            bg_to,
        );
    }
    buffer
}

/// Draw links between neighboring "on" cells in an RGBA buffer using Bresenham's algorithm
fn draw_links_bresenham_rgba(
    buffer: &mut [u8],
    generations: &[Vec<u8>],
    width: usize,
    height: usize,
    scale: usize,
    fg_from: Rgb<u8>,
    fg_to: Rgb<u8>,
    bg_from: Rgb<u8>,
    bg_to: Rgb<u8>,
) {
    let thickness = ((scale as i32) / 8).max(1);
    let img_width = (width * scale) as i32;
    let img_height = (height * scale) as i32;
    let neighbor_offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for y in 0..height - 1 {
        for x in 0..width {
            let cell_val = generations[y][x];
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
            let (cx, cy) = (
                (x as i32 * scale as i32 + scale as i32 / 2),
                (y as i32 * scale as i32 + scale as i32 / 2),
            );
            for &(dx, dy) in &neighbor_offsets {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if ny == y as isize + 1
                    && nx >= 0
                    && nx < width as isize
                    && ny >= 0
                    && ny < height as isize
                {
                    let neighbor_val = generations[ny as usize][nx as usize];
                    if neighbor_val == cell_val {
                        let (ncx, ncy) = (
                            (nx as i32 * scale as i32 + scale as i32 / 2),
                            (ny as i32 * scale as i32 + scale as i32 / 2),
                        );
                        let debug_color = if cell_val == 1 {
                            // Invert FG gradient for alive links
                            lerp_color(&fg_to, &fg_from, t)
                        } else {
                            // Invert BG gradient for dead links
                            lerp_color(&bg_to, &bg_from, t)
                        };
                        draw_line_bresenham_rgba(
                            buffer,
                            img_width,
                            img_height,
                            cx,
                            cy,
                            ncx,
                            ncy,
                            debug_color,
                            thickness,
                        );
                    }
                }
            }
        }
    }
}

fn draw_line_bresenham_rgba(
    buffer: &mut [u8],
    img_width: i32,
    img_height: i32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Rgb<u8>,
    thickness: i32,
) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        // Draw a filled square of size thickness centered at (x0, y0)
        let half = thickness / 2;
        for dy in -half..=half {
            for dx in -half..=half {
                let px = x0 + dx;
                let py = y0 + dy;
                if px >= 0 && py >= 0 && px < img_width && py < img_height {
                    let idx = ((py * img_width + px) * 4) as usize;
                    buffer[idx] = color[0];
                    buffer[idx + 1] = color[1];
                    buffer[idx + 2] = color[2];
                    buffer[idx + 3] = 255;
                }
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
