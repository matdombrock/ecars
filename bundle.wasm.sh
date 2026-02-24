#!/usr/bin/env bash
set -e

./build.wasm.sh

web_path="./webui"

# Zip the entire webui directory into a single file
zip -r ecars.web.zip "${web_path}"
