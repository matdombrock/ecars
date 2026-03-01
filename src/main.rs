use ca::run_automaton;
use clap::Parser;
mod image_output;

use image::Rgb;

/// Runs an elementary cellular automaton and prints the generations.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Rule number (0-255)
    rule: u8,

    /// Random seed (u64, optional)
    #[arg(long)]
    seed: Option<u64>,

    /// Probability for random initial state (0.0-1.0), or 'none' for single center cell
    #[arg(long, short = 'd', default_value = "none")]
    random_distribution: String,

    /// Width of the automaton
    #[arg(long, short = 'w', default_value_t = 64)]
    width: usize,

    /// Number of generations to run
    #[arg(long, short = 'g', default_value_t = 32)]
    generations: usize,

    /// Pretty print generations (1 as '#' and 0 as space)
    #[arg(long, short = 'p', default_value_t = true)]
    pretty_print: bool,

    /// Shape to use for alive cells in PNG output
    #[arg(long, default_value = "square")]
    alive_shape: String,

    /// Shape to use for dead cells in PNG output
    #[arg(long, default_value = "square")]
    dead_shape: String,

    /// Draw links between neighboring cells (post-processing)
    #[arg(long, default_value_t = false)]
    links: bool,

    /// Scale factor for PNG output (each cell becomes scale x scale pixels)
    #[arg(long, short = 's', default_value_t = 1)]
    scale: usize,

    /// Output PNG file
    #[arg(long, short = 'o')]
    output: Option<String>,

    /// Start color for dead cells
    #[arg(long, default_value = "#ffaaff")]
    dead_color_from: String,

    /// End color for dead cells
    #[arg(long, default_value = "#000000")]
    dead_color_to: String,

    /// Start color for alive cells
    #[arg(long, default_value = "#000000")]
    alive_color_from: String,

    /// End color for alive cells
    #[arg(long, default_value = "#aaffff")]
    alive_color_to: String,
    /// Mirror horizontally (flip left-right)
    #[arg(long, default_value_t = false)]
    mirror_x: bool,

    /// Mirror vertically (flip top-bottom)
    #[arg(long, default_value_t = false)]
    mirror_y: bool,
}

fn parse_hex_color(s: &str) -> Rgb<u8> {
    let s = s.strip_prefix('#').unwrap_or(s);
    assert!(s.len() == 6, "Color must be in format #RRGGBB");
    let r = u8::from_str_radix(&s[0..2], 16).expect("Invalid hex color");
    let g = u8::from_str_radix(&s[2..4], 16).expect("Invalid hex color");
    let b = u8::from_str_radix(&s[4..6], 16).expect("Invalid hex color");
    Rgb([r, g, b])
}

fn main() {
    let args = Args::parse();
    let random_distribution = match args.random_distribution.as_str() {
        "none" => None,
        s => Some(s.parse().expect("Invalid random_distribution")),
    };
    let seed = args.seed;
    let flat_vec = run_automaton(
        args.rule,
        random_distribution,
        args.width,
        args.generations,
        seed,
    );
    let generations_vec: Vec<Vec<u8>> = flat_vec
        .chunks(args.width)
        .map(|chunk| chunk.to_vec())
        .collect();

    if let Some(output_path) = args.output {
        let dead_from = parse_hex_color(&args.dead_color_from);
        let dead_to = parse_hex_color(&args.dead_color_to);
        let alive_from = parse_hex_color(&args.alive_color_from);
        let alive_to = parse_hex_color(&args.alive_color_to);
        image_output::save_generations_as_png(
            &generations_vec,
            args.width,
            args.generations,
            args.scale,
            &args.alive_shape,
            &args.dead_shape,
            args.links,
            &output_path,
            dead_from,
            dead_to,
            alive_from,
            alive_to,
            args.mirror_x,
            args.mirror_y,
        );
    } else if args.pretty_print {
        for gen in generations_vec {
            for cell in gen {
                print!("{}", if cell == 1 { '█' } else { ' ' });
            }
            println!();
        }
    } else {
        for gen in generations_vec {
            for cell in gen {
                print!("{}", cell);
            }
            println!();
        }
    }
}
