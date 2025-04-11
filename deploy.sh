#!/bin/bash
# deploy.sh

# Create the dist directory if it doesn't exist
mkdir -p dist

# Copy the WebAssembly files 
cp target/wasm32-unknown-emscripten/release/pumpkin_web.js dist/
cp target/wasm32-unknown-emscripten/release/pumpkin_web.wasm dist/

# Copy the HTML file
cp index.html dist/

echo "Files copied to dist/ directory"
echo "To test, start a web server in the dist directory:"
echo "  cd dist && python3 -m http.server 8000"