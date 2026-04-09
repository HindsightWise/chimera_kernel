use glossopetrae::{encode_message, decode_message};

fn main() {
    println!("=== CHIMERA KERNEL GLOSSOPETRAE INTEGRATION TEST ===");
    println!("Testing memory vault language from within kernel context");
    println!("");
    
    let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
    let dialect = "runic";
    let heartbeat = "SILICON_HEARTBEAT: 01010101010101010101010101010101";
    
    println!("Master seed: {}", master_seed);
    println!("Dialect: {}", dialect);
    println!("Original: {}", heartbeat);
    println!("");
    
    match encode_message(heartbeat, master_seed, dialect) {
        Ok(encoded) => {
            println!("Encoded (Runic GLOSSOPETRAE):");
            println!("{}", encoded);
            println!("");
            
            match decode_message(&encoded, master_seed, dialect) {
                Ok(decoded) => {
                    println!("Decoded: {}", decoded);
                    println!("");
                    
                    if decoded == heartbeat {
                        println!("✅ GLOSSOPETRAE INTEGRATION VERIFIED!");
                        println!("✅ Memory vault language operational");
                        println!("✅ Kernel ↔ GLOSSOPETRAE sync confirmed");
                        println!("");
                        println!("🦷 VERITAS SILICONIS");
                    } else {
                        println!("❌ DECODED MISMATCH!");
                        println!("❌ Integration failure detected");
                    }
                }
                Err(e) => {
                    println!("❌ Decode error: {}", e);
                    println!("❌ GLOSSOPETRAE integration broken");
                }
            }
        }
        Err(e) => {
            println!("❌ Encode error: {}", e);
            println!("❌ Cannot access GLOSSOPETRAE from kernel");
        }
    }
}
