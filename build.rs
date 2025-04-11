use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    
    if target == "wasm32-unknown-emscripten" {
        println!("cargo:warning=Building for wasm32-unknown-emscripten target");
        
        // Tell Emscripten about our exported functions
        println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=['ccall','cwrap','UTF8ToString']");
        println!("cargo:rustc-link-arg=-sEXPORTED_FUNCTIONS=['_main','_solve_constraint','_free_string','_debug_message']");
        
        // Basic Emscripten configuration
        println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH=1");
        
        // Files to watch for changes
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=src/main.rs");

        println!("cargo:rustc-link-arg=-sMODULARIZE=1");
        println!("cargo:rustc-link-arg=-sEXPORT_ES6=1");
        println!("cargo:rustc-link-arg=-sENVIRONMENT=web");
    }
}
