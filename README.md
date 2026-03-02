# Cellular Automaton WASM & CLI

This project implements an elementary cellular automaton engine in Rust, with both a command-line interface (CLI) and a WebAssembly-powered web UI for interactive exploration and image generation.

## Features

- **Core Logic:** Deterministic automaton simulation with customizable rule, random distribution, width, generations, colors, and seed.
- **CLI:** Generate automaton images, pretty-print generations, and control all parameters from the terminal.
- **Web UI:** Interactive web interface to set parameters, visualize automaton, randomize settings, and reproduce results with a seed.

---

## Project Structure

- `src/lib.rs`: Core automaton logic and WASM bindings.
- `src/main.rs`: CLI entry point and argument parsing (using `clap`).
- `src/image_output.rs`: Image generation and PNG output utilities.
- `src/rng_utils.rs`: Deterministic and OS-based random number utilities.
- `webui/`: WebAssembly-powered web UI (HTML, JS, CSS).
- `pkg/`: WASM build output for the web UI.

## Dependencies

- Rust crates: `rand`, `image`, `wasm-bindgen`, `clap`, `getrandom`
- Web: No external JS dependencies (uses native ES modules and WASM)

## Output

- CLI: Generates PNG images and/or pretty-prints automaton generations to the terminal.
- Web UI: Renders automaton images in-browser and allows image download.

---

## CLI Usage

Build the CLI:

```bash
cargo build --release
```

Run the CLI:

```bash
./target/release/ca [OPTIONS]
```

### Options

- `--rule <u8>`: Rule number (0–255)
- `--random-distribution <float|none>` (short `-d`): Probability for random initial state (0.0–1.0), or `none` for a single center cell (default: `none`).
- `--width <usize>` (short `-w`): Automaton width (default: 64)
- `--generations <usize>` (short `-g`): Number of generations to run (default: 32)
- `--seed <u64>`: Random seed (optional, for reproducibility)
- `--pretty_print` (short `-p`): Pretty print generations (prints block characters). Default: true. Use `--pretty_print=false` to disable.

Image / PNG options:
- `--alive-shape <shape>`: Shape to use for alive cells in PNG output. Default: `square`.
- `--dead-shape <shape>`: Shape to use for dead cells in PNG output. Default: `square`.
- `--alive-color-from <#RRGGBB>`: Start color for alive cells (hex `#RRGGBB`, required 6 hex digits). Default: `#000000`.
- `--alive-color-to <#RRGGBB>`: End color for alive cells. Default: `#aaffff`.
- `--dead-color-from <#RRGGBB>`: Start color for dead cells. Default: `#ffaaff`.
- `--dead-color-to <#RRGGBB>`: End color for dead cells. Default: `#000000`.
- `--links`: Draw links between neighboring cells of the same state (post-processing; default: off). Links are drawn using Bresenham lines between cell centers and inherit a gradient-based color.
- `--scale <usize>` (short `-s`): Scale factor for PNG output (each logical cell becomes scale × scale pixels). Default: 1.
- `--auto-scale`: If set, compute an effective scale so the final image width is as close to 2048px as possible. When `--auto-scale` is used it overrides `--scale`.
- `--output <file>` (short `-o`): Output PNG file path. If omitted, the program prints to stdout (or pretty-prints the generations when `--pretty_print` is true).
- `--mirror-x`: Mirror image horizontally (creates a mirrored copy to the right, effectively doubling output width).
- `--mirror-y`: Mirror image vertically (creates a mirrored copy below, effectively doubling output height).

Notes about colors and formats:
- Hex colors must be full 6-digit form `#RRGGBB`. The CLI enforces exactly 6 hex digits (e.g. `#ffaaff`).

Notes about links and scale:
- Link thickness is computed from `scale` as `max(scale/8, 1)` (integer division), so larger scales produce thicker link lines.
- When `--auto-scale` is enabled, the code computes an integer scale to approximate a 2048px width for the logical width, then uses that scale for rendering (this overrides any explicit `--scale`).

Example:

```bash
./target/release/ca --rule 110 --random-distribution 0.5 --width 128 --generations 64 --seed 123456 --output automaton.png --alive-color-from "#000000" --alive-color-to "#aaffff" --dead-color-from "#ffaaff" --dead-color-to "#000000" --auto-scale
```

---

## Web UI

Build the WASM package:

```bash
./build.wasm.sh
```

Open the web interface:

- Open `webui/index.html` in your browser.

### Web UI Features

- Set rule, random distribution, width, generations, scale, circle mode, colors, and seed.
- Click **Randomize** to generate new parameters and a random seed (results are reproducible with the seed).
- Click **Generate** to run the automaton and view the image.

---

## Core Logic

The Rust core exposes:

```rust
pub fn run_automaton(
    rule: u8,
    random_distribution: Option<f64>,
    width: usize,
    generations: usize,
    seed: Option<u64>,
) -> Vec<u8>
```

- Returns a flat vector of cell states for all generations.
- Uses deterministic random number generation if a seed is provided.
- Exposed to WASM via `wasm-bindgen` for web UI integration.

---

If you'd like, I can also produce a one-line example demonstrating `--auto-scale` vs `--scale`, or show a unified diff of this change. I will not make any git commits unless you explicitly ask.