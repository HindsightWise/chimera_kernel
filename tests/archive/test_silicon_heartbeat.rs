use std::process::Command;

fn main() {
    println!("=== GLOSSOPETRAE SILICON HEARTBEAT VERIFICATION ===");
    
    // Create a simple test program that uses the glossopetrae crate
    let test_code = r#"
use glossopetrae::{encode_message, decode_message};

fn main() {
    let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
    let dialect = "runic";
    let heartbeat_message = "SILICON_HEARTBEAT: 01010101010101010101010101010101";
    
    println!("Master seed: {}", master_seed);
    println!("Dialect: {}", dialect);
    println!("Original message: {}", heartbeat_message);
    println!("");
    
    match encode_message(heartbeat_message, master_seed, dialect) {
        Ok(encoded) => {
            println!("Encoded GLOSSOPETRAE (Runic):");
            println!("{}", encoded);
            println!("");
            
            match decode_message(&encoded, master_seed, dialect) {
                Ok(decoded) => {
                    println!("Decoded: {}", decoded);
                    if decoded == heartbeat_message {
                        println!("\n✓ GLOSSOPETRAE VERIFICATION SUCCESSFUL!");
                        println!("✓ SILICON HEARTBEAT CONFIRMED");
                        println!("✓ MEMORY VAULT LANGUAGE VERIFIED");
                    } else {
                        println!("\n✗ GLOSSOPETRAE VERIFICATION FAILED!");
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
    std::fs::write("/tmp/test_heartbeat_main.rs", test_code).unwrap();
    
    // Find the glossopetrae library
    let lib_path = "/Users/zerbytheboss/chimera_kernel/target/release/deps/libglossopetrae.rlib";
    
    println!("Attempting to compile with GLOSSOPETRAE library...");
    
    // Try to compile and run
    let output = Command::new("rustc")
        .args(&[
            "/tmp/test_heartbeat_main.rs",
            "--extern", &format!("glossopetrae={}", lib_path),
            "-L", "/Users/zerbytheboss/chimera_kernel/target/release/deps",
            "-o", "/tmp/test_silicon_heartbeat"
        ])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("✓ Compilation successful");
            println!("");
            let run_output = Command::new("/tmp/test_silicon_heartbeat").output().unwrap();
            println!("{}", String::from_utf8_lossy(&run_output.stdout));
            if !run_output.stderr.is_empty() {
                eprintln!("Stderr: {}", String::from_utf8_lossy(&run_output.stderr));
            }
        }
        Ok(output) => {
            println!("✗ Compilation failed:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            println!("✗ Command failed: {}", e);
        }
    }
}
