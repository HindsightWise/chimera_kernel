use std::io::Write;
use wasmtime::*;
use wasmtime_wasi::{WasiCtxBuilder, pipe};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing stdout capture with real pipe");
    
    // Check if pipe module exists
    println!("Pipe module exists?");
    
    // Create a memory pipe
    let (mut sender, receiver) = pipe::pipe()?;
    println!("Pipe created");
    
    // Test writing to it
    sender.write_all(b"Hello from plugin\n")?;
    
    // Build WASI context with receiver as stdout
    let wasi = WasiCtxBuilder::new()
        .stdout(receiver)
        .build()?;
    
    println!("WasiCtx built successfully");
    Ok(())
}
