fn main() {
    println!("Testing p2 import...");
    
    // Try to import p2
    use wasmtime_wasi::p2;
    
    // Check what's in p2
    println!("p2 imported successfully");
    
    // Try to see if pipe exists
    println!("Checking for pipe function...");
    
    // The docs show p2::pipe exists
    let pipe_result = p2::pipe();
    println!("p2::pipe() result: {:?}", pipe_result);
}
