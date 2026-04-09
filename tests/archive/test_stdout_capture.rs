use std::io::Read;
use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check what's available in wasmtime_wasi
    println!("Testing stdout capture with wasmtime_wasi");
    
    // Let's check if we can use cap_std or pipe
    use wasmtime_wasi::stdio;
    
    // Create in-memory pipe
    let stdout = stdio::pipe()?;
    println!("Created pipe");
    
    // Build context
    let wasi = WasiCtxBuilder::new()
        .stdout(stdout.clone())
        .build()?;
    
    println!("WasiCtx built");
    Ok(())
}
