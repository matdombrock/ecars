# Cellular Automaton WASM & CLI

This project implements an elementary cellular automaton engine in Rust, with both a command-line interface (CLI) and a WebAssembly-powered web UI for interactive exploration and image generation.

## Features

- **Core Logic:** Deterministic automaton simulation with customizable rule, random distribution, width, generations, colors, and seed.
- **CLI:** Generate automaton images, pretty-print generations, and control all parameters from the terminal.
- **Web UI:** Interactive web interface to set parameters, visualize automaton, randomize settings, and reproduce results with a seed.

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
- `--random_distribution <float>`: Probability for random initial state (0.0–1.0), or 'none' for single center cell
- `--width <usize>`: Automaton width (default: 64)
- `--generations <usize>`: Number of generations (default: 32)
- `--seed <u64>`: Random seed (optional, for reproducibility)
- `--pretty_print`: Pretty print generations (default: true)
- `--shape <shape>`: Shape to use for cells in PNG output (`square`, `circle`, `triangle-up`, `triangle-down`, `triangle-left`, `triangle-right`, `triangle-r-up`, `triangle-r-down`, `triangle-r-a`, `triangle-r-b`, `triangle-r-c`, `triangle-r-d`). Default: `square`
- `--links`: Draw links between cells
- `--scale <usize>`: Scale factor for PNG output (default: 1)
- `--output <file>`: Output PNG file (optional)
- `--bg_from <hex>`: Background color start (default: #ffaaff)
- `--bg_to <hex>`: Background color end (default: #000000)
- `--fg_from <hex>`: Foreground color start (default: #000000)
- `--fg_to <hex>`: Foreground color end (default: #aaffff)

Example:

```bash
./target/release/ca --rule 110 --random_distribution 0.5 --width 128 --generations 64 --seed 123456 --output automaton.png --bg_from "#ffaaff" --bg_to "#000000" --fg_from "#000000" --fg_to "#aaffff"
```

---

## Web UI

Build the WASM package:

```bash
./build.wasm.sh
```

Open the web interface:

- Open `webui/index.html` in your browser.

### Features

- Set rule, random distribution, width, generations, scale, circle mode, colors, and seed.
- Click **Randomize** to generate new parameters and a random seed (results are reproducible).
- Click **Generate** to run the automaton and view the image.
- All parameters are adjustable; the seed ensures reproducibility.

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
