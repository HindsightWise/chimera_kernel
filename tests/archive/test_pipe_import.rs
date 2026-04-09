// Test different import patterns
use wasmtime_wasi::sync;

fn main() {
    println!("Testing imports");
    
    // Check if pipe exists in sync
    println!("Checking sync::pipe");
    
    // Try to import pipe
    #[allow(unused_imports)]
    use sync::pipe;
    println!("sync::pipe exists");
    
    // Try to create a WritePipe
    let pipe = pipe::WritePipe::new_in_memory();
    println!("Created WritePipe: {:?}", pipe);
}
