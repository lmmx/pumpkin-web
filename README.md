# Pumpkin Solver with Emscripten

A minimal demo showing how to use the Pumpkin constraint solver with WebAssembly via Emscripten.

## Prerequisites

To build this project, you'll need:

1. **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/)
2. **Emscripten SDK**: Follow the [installation instructions](https://emscripten.org/docs/getting_started/downloads.html)
3. **A local copy of pumpkin-solver**: This should be compiled with the wasm32-unknown-emscripten target

## Project Structure

```
pumpkin-web-emscripten/
├── Cargo.toml                # Rust dependencies and config
├── build.rs                  # Rust build script
├── post-build.rs             # Script to copy build artifacts
├── serve.sh                  # Convenience script for building and serving
├── index.html                # HTML page for the demo
├── src/
│   └── main.rs               # Rust code with Emscripten exports
└── www/                      # Output directory for compiled files
```

## Building the Project

### Option 1: Using the convenience script (recommended)

1. **Activate Emscripten SDK environment**:
   ```bash
   # Assuming emsdk is installed at ~/emsdk
   source ~/emsdk/emsdk_env.sh
   ```

2. **Make the script executable and run it**:
   ```bash
   ./serve.sh
   ```
   
   This script will:
   - Build the project for the wasm32-unknown-emscripten target
   - Copy all necessary files to the www/ directory
   - Start a web server on port 8000

3. **Open the demo** in your browser at http://localhost:8000

### Option 2: Manual steps

1. **Activate Emscripten SDK environment**:
   ```bash
   # Assuming emsdk is installed at ~/emsdk
   source ~/emsdk/emsdk_env.sh
   ```

2. **Add the wasm32-unknown-emscripten target to Rust**:
   ```bash
   rustup target add wasm32-unknown-emscripten
   ```

3. **Build the project for Emscripten target**:
   ```bash
   cargo build --target wasm32-unknown-emscripten --release
   ```

4. **Run the post-build script to copy files**:
   ```bash
   cargo run --bin post-build
   ```
   
   This will:
   - Copy the generated .js and .wasm files from the target directory
   - Copy the index.html file to the www/ directory

5. **Serve the application**:
   ```bash
   cd www
   python3 -m http.server 8000
   ```

6. **Open the demo** in your browser at http://localhost:8000

## How It Works

This demo provides a simple web interface to a Rust constraint solver using Emscripten:

1. **Rust code**: Defines C-compatible functions for Emscripten to expose to JavaScript
2. **Emscripten**: Compiles the Rust code to WebAssembly and generates JavaScript bindings
3. **HTML/JavaScript**: Provides a UI and calls the exported functions

The demo solves a simple constraint problem: `x + y = 12` with user-defined domains for x and y.

## Notes on Interoperability

The Rust code exports three functions:
- `solve_constraint`: Takes domain boundaries and returns a JSON string with results
- `free_string`: Frees memory allocated for strings in Rust
- `debug_message`: For sending debug messages to the JavaScript console

JavaScript calls these functions using the Emscripten-generated Module interface.

## Troubleshooting

- **Emscripten environment not detected**: Make sure you've sourced the emsdk_env.sh script
- **Build errors**: Check that pumpkin-solver is compatible with the wasm32-unknown-emscripten target
- **Runtime errors**: Look at the browser's JavaScript console for detailed error messages

## Extending the Demo

To extend this demo:
- Add more constraints in `src/main.rs`
- Update the UI in `index.html`
- Add more exported functions for more complex scenarios
