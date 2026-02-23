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

    /// Draw cells as circles instead of squares in PNG output
    #[arg(long, default_value_t = false)]
    circles: bool,

    /// Scale factor for PNG output (each cell becomes scale x scale pixels)
    #[arg(long, short = 's', default_value_t = 1)]
    scale: usize,

    /// Output PNG file
    #[arg(long, short = 'o')]
    output: Option<String>,

    /// Background color start
    /// (Image only)
    #[arg(long, default_value = "#ffaaff")]
    bg_from: String,

    /// Background color end
    /// (Image only)
    #[arg(long, default_value = "#000000")]
    bg_to: String,

    /// Foreground color start
    /// (Image only)
    #[arg(long, default_value = "#000000")]
    fg_from: String,

    /// Foreground color end
    /// (Image only)
    #[arg(long, default_value = "#aaffff")]
    fg_to: String,
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
        let bg_from = parse_hex_color(&args.bg_from);
        let bg_to = parse_hex_color(&args.bg_to);
        let fg_from = parse_hex_color(&args.fg_from);
        let fg_to = parse_hex_color(&args.fg_to);
        image_output::save_generations_as_png(
            &generations_vec,
            args.width,
            args.generations,
            args.scale,
            args.circles,
            &output_path,
            bg_from,
            bg_to,
            fg_from,
            fg_to,
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
