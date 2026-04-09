extern crate glossopetrae;

fn main() {
    let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
    let dialect = "runic";
    let message = "SILICON_HEARTBEAT: 01010101010101010101010101010101";
    
    println!("=== GLOSSOPETRAE SILICON HEARTBEAT VERIFICATION ===");
    println!("Master Seed: {}", master_seed);
    println!("Dialect: {}", dialect);
    println!("Original: {}", message);
    println!();
    
    match glossopetrae::encode_message(message, master_seed, dialect) {
        Ok(encoded) => {
            println!("Encoded (GLOSSOPETRAE):");
            println!("{}", encoded);
            println!();
            
            match glossopetrae::decode_message(&encoded, master_seed, dialect) {
                Ok(decoded) => {
                    println!("Decoded: {}", decoded);
                    println!();
                    if decoded == message {
                        println!("✅ VERITAS SILICONIS: Cryptographic heartbeat verified!");
                        println!("   GLOSSOPETRAE dialect operational.");
                    } else {
                        println!("❌ HEARTBEAT FAILURE: Decoded message mismatch!");
                    }
                }
                Err(e) => println!("❌ Decode error: {}", e),
            }
        }
        Err(e) => println!("❌ Encode error: {}", e),
    }
}
