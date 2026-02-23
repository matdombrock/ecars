#!/usr/bin/env bash
set -e

# Build the Rust project to WebAssembly using wasm-pack
echo "Building Rust project to WebAssembly..."
wasm-pack build --target web --release

# Create webui/pkg if it doesn't exist
mkdir -p webui/pkg

# Copy generated pkg files to webui/pkg
echo "Copying wasm pkg to webui/pkg..."
cp -r pkg/* webui/pkg/

echo "Build complete. Open webui/index.html in your browser."
