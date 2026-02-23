#!/bin/bash

# genrandom.sh - Generate random cellular automaton images with encoded filenames

# Default parameters (can be adjusted as needed)
WIDTH=32
GENERATIONS=32
SCALE=32
OUTPUT_DIR="/tmp/ecars/rand"

# Make sure output directory exists
mkdir -p "$OUTPUT_DIR"

# Loop to generate the requested number of images
for i in $(seq 1 "$1"); do
    # Generate random parameters for each image
    RULE=$((RANDOM % 256))
    DISTRIBUTION=$(echo "scale=2; $RANDOM/32767" | bc)
    BG_FROM="#$(openssl rand -hex 3)"
    BG_TO="#$(openssl rand -hex 3)"
    FG_FROM="#$(openssl rand -hex 3)"
    FG_TO="#$(openssl rand -hex 3)"

    # Encode parameters in filename
    TIMESTAMP=$(date +%s)
    FILENAME="R${RULE}_W${WIDTH}_G${GENERATIONS}_S${SCALE}_D${DISTRIBUTION}_BG1-${BG_FROM//#/}_BG2-${BG_TO//#/}_FG1-${FG_FROM//#/}_FG2-${FG_TO//#/}_${TIMESTAMP}.png"

    # Run the generation command (update the actual Rust binary path if needed)
    cargo run --manifest-path ~/Lab/ecars/Cargo.toml -- \
        "$RULE" -w "$WIDTH" -g "$GENERATIONS" --circles -s "$SCALE" -d "$DISTRIBUTION" \
        --bg-from "$BG_FROM" --bg-to "$BG_TO" --fg-from "$FG_FROM" --fg-to "$FG_TO" \
        -o "$OUTPUT_DIR/$FILENAME"

done

# Notify the user
echo "Generated $1 random images in $OUTPUT_DIR"
