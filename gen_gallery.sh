#!/bin/bash

# gen_gallery.sh - Generate a gallery for the current images in /tmp/ecars/rand using Thumbsup

# Directory containing images
INPUT_DIR="/tmp/ecars/rand"
# Output directory for the gallery
OUTPUT_DIR="/tmp/gallery"
# Theme for the gallery
THEME="cards"

# Ensure the output directory is cleared before regeneration
rm -rf "$OUTPUT_DIR" && mkdir -p "$OUTPUT_DIR"

# Run Thumbsup gallery generation
thumbsup --input "$INPUT_DIR" --output "$OUTPUT_DIR" --theme "$THEME"

# Notify the user
echo "Gallery generated successfully in $OUTPUT_DIR"