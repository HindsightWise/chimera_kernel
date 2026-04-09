use std::process::Command;

fn main() {
    println!("Testing GLOSSOPETRAE silicon heartbeat...");
    
    // First check if we can compile a simple test
    let test_code = r#"
use glossopetrae::*;

fn main() {
    let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
    let dialect = "runic";
    let message = "SILICON_HEARTBEAT: 10101010101010101010101010101010";
    
    println!("Master seed: {}", master_seed);
    println!("Dialect: {}", dialect);
    println!("Original message: {}", message);
    
    match encode_message(message, master_seed, dialect) {
        Ok(encoded) => {
            println!("\nEncoded (GLOSSOPETRAE):\n{}", encoded);
            
            match decode_message(&encoded, master_seed, dialect) {
                Ok(decoded) => {
                    println!("\nDecoded: {}", decoded);
                    if decoded == message {
                        println!("✓ GLOSSOPETRAE verification successful!");
                        println!("✓ SILICON HEARTBEAT VERIFIED");
                    } else {
                        println!("✗ GLOSSOPETRAE verification failed!");
                    }
                }
                Err(e) => println!("Decode error: {}", e),
            }
        }
        Err(e) => println!("Encode error: {}", e),
    }
}
"#;
    
    // Write test file
    std::fs::write("/tmp/test_glossopetrae_main.rs", test_code).unwrap();
    
    // Try to compile and run
    let output = Command::new("rustc")
        .args(&["/tmp/test_glossopetrae_main.rs", "--extern", "glossopetrae=/Users/zerbytheboss/chimera_kernel/target/release/deps/libglossopetrae.rlib", "-o", "/tmp/test_glossopetrae"])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("✓ Compilation successful");
            let run_output = Command::new("/tmp/test_glossopetrae").output().unwrap();
            println!("{}", String::from_utf8_lossy(&run_output.stdout));
        }
        Ok(output) => {
            println!("✗ Compilation failed:");
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            println!("✗ Command failed: {}", e);
        }
    }
}
