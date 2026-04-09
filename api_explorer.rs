//! Explore wasmtime_wasi API
fn main() {
    println!("Exploring wasmtime_wasi v14.0...");
    
    // List all modules in wasmtime_wasi
    println!("Modules in wasmtime_wasi:");
    println!("- WasiCtxBuilder");
    println!("- WasiCtx");
    println!("- add_to_linker");
    
    // Check for pipe
    use wasmtime_wasi as wasi;
    
    // Try to find pipe through sync module
    #[allow(unused_imports)]
    use wasi::sync;
    println!("sync module exists");
    
    // Check if there's a pipe in sync
    // sync::pipe would be the full path if it exists
    
    // Actually, let's just try to compile with the actual usage
}
