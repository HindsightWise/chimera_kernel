fn main() {
    println!("Checking wasmtime_wasi API...");
    
    // Import everything to see what's available
    use wasmtime_wasi::*;
    
    // Check if pipe exists
    println!("Checking pipe module");
    
    // Try to use it
    match pipe::pipe() {
        Ok(_) => println!("pipe::pipe() works!"),
        Err(e) => println!("pipe::pipe() error: {}", e),
    }
}
