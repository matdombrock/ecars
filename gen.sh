#!/bin/bash

# Parameters for the automaton
WIDTH=256
GENERATIONS=256
RULE=30
CIRCLE_OPTION="--circles"
SCALE=8

# Output directory
OUTPUT_DIR="/tmp"
mkdir -p "$OUTPUT_DIR"

# Generate 10 images
for i in {1..10}; do
  RANDOM_DISTRIBUTION=$(awk -v seed="$RANDOM" 'BEGIN { srand(seed); print rand() }')
  OUTPUT_FILE="$OUTPUT_DIR/image_${i}_$(date +%s%N).png"

  echo "Generating $OUTPUT_FILE with random distribution $RANDOM_DISTRIBUTION"
  cargo run --manifest-path ~/Lab/ecars/Cargo.toml -- \
    $RULE \
    -w $WIDTH \
    -g $GENERATIONS \
    $CIRCLE_OPTION \
    -s $SCALE \
    -d $RANDOM_DISTRIBUTION \
    -o "$OUTPUT_FILE"
done

echo "All images generated in $OUTPUT_DIR/."