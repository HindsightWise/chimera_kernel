// Quick test to see what's in wasmtime_wasi
fn main() {
    // Check sync module
    println!("Testing wasmtime_wasi imports");
    
    // Try to use WasiCtxBuilder to see what methods are available
    use wasmtime_wasi::sync::WasiCtxBuilder;
    let builder = WasiCtxBuilder::new();
    println!("WasiCtxBuilder created");
    
    // Check if stdout method exists
    println!("Builder methods should include .stdout()");
}
