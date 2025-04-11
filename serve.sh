#!/bin/bash

# Exit on error
set -e

echo "Building Pumpkin Solver WebAssembly Demo"
echo "========================================"

# 1. Build the project for Emscripten target
echo "Step 1: Building for wasm32-unknown-emscripten target..."
cargo build --target wasm32-unknown-emscripten --release

# 2. Run the post-build script
echo "Step 2: Running post-build script to copy files..."
cargo run --bin post-build

# 3. Start a web server in the www directory
echo "Step 3: Starting web server in www directory..."
echo "Press Ctrl+C to stop the server"
echo ""
echo "Open http://localhost:8000 in your browser"
echo ""

cd www && python3 -m http.server 8000
