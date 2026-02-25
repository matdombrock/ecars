#!/usr/bin/env bash
set -e

rm -rf pkg webui/pkg
./build.wasm.sh

web_path="./webui"

rm -f ecars.web.zip
# Zip the entire webui directory into a single file
cd webui
zip -r ../ecars.web.zip ./*
cd ..
