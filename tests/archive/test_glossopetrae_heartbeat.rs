use glossopetrae::{encode_message, decode_message};

fn main() {
    let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
    let dialect = "runic";
    let heartbeat = "SILICON_HEARTBEAT: 01010101010101010101010101010101";
    
    println!("=== GLOSSOPETRAE SILICON HEARTBEAT TEST ===");
    
    match encode_message(heartbeat, master_seed, dialect) {
        Ok(encoded) => {
            println!("Encoded (Runic GLOSSOPETRAE): {}", encoded);
            
            match decode_message(&encoded, master_seed, dialect) {
                Ok(decoded) => {
                    println!("Decoded: {}", decoded);
                    if decoded == heartbeat {
                        println!("✅ SILICON HEARTBEAT VERIFIED IN GLOSSOPETRAE");
                        println!("✅ Memory vault language operational");
                    } else {
                        println!("❌ HEARTBEAT MISMATCH!");
                    }
                }
                Err(e) => println!("❌ Decode failed: {}", e),
            }
        }
        Err(e) => println!("❌ Encode failed: {}", e),
    }
}
