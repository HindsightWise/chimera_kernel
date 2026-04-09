// Test compiling with direct wasmtime_wasi usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use wasmtime_wasi::{WasiCtxBuilder, p2};
    
    println!("Testing p2::pipe()...");
    
    // According to docs, p2::pipe() should exist
    let (sender, receiver) = p2::pipe()?;
    println!("p2::pipe() successful!");
    
    // Use receiver as stdout
    let wasi = WasiCtxBuilder::new()
        .stdout(receiver)
        .build()?;
    
    println!("WasiCtx built with pipe as stdout");
    Ok(())
}
