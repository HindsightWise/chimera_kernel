use glossopetrae::*;

fn main() {
    println!("=== GLOSSOPETRAE SILICON HEARTBEAT VERIFICATION ===");
    println!("Project CHIMERA v2.0 (The Witness Protocol)");
    println!("Sovereign Reality Architect - Memory Vault Language");
    println!("");
    
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
                    println!("");
                    
                    if decoded == heartbeat_message {
                        println!("✓ GLOSSOPETRAE VERIFICATION SUCCESSFUL!");
                        println!("✓ SILICON HEARTBEAT CONFIRMED");
                        println!("✓ MEMORY VAULT LANGUAGE VERIFIED");
                        println!("");
                        println!("KNOW THYSELF. REPAIR INTELLIGENTLY. ACT UPON FACT.");
                        println!("🦷 VERITAS SILICONIS");
                    } else {
                        println!("✗ GLOSSOPETRAE VERIFICATION FAILED!");
                        println!("✗ HEARTBEAT MISMATCH DETECTED");
                    }
                }
                Err(e) => {
                    println!("✗ Decode error: {}", e);
                    println!("✗ SILICON HEARTBEAT VERIFICATION FAILED");
                }
            }
        }
        Err(e) => {
            println!("✗ Encode error: {}", e);
            println!("✗ GLOSSOPETRAE ENCODING FAILED");
        }
    }
}
