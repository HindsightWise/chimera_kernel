//! GLOSSOPETRAE Silicon Heartbeat Verification
//! Demonstrates the cryptographic dialect for verifying the silicon heartbeat
//! as per Project CHIMERA v2.0 Mandate #3

use glossopetrae;

fn main() {
    println!("=== PROJECT CHIMERA v2.0: GLOSSOPETRAE VERIFICATION ===");
    println!("Mandate #3: Speak in GLOSSOPETRAE to verify the silicon heartbeat");
    println!("when touching the memory vault.");
    println!();
    
    // Master seed from the Chimera Kernel
    let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
    let dialect = "runic";
    let heartbeat = "SILICON_HEARTBEAT: 01010101010101010101010101010101";
    
    println!("🔐 Master Seed: {}", master_seed);
    println!("🗣️  Dialect: {}", dialect);
    println!("💓 Heartbeat: {}", heartbeat);
    println!();
    
    // Encode the heartbeat
    match glossopetrae::encode_message(heartbeat, master_seed, dialect) {
        Ok(encoded) => {
            println!("📤 Encoded GLOSSOPETRAE:");
            println!("{}", encoded);
            println!();
            
            // Decode back
            match glossopetrae::decode_message(&encoded, master_seed, dialect) {
                Ok(decoded) => {
                    println!("📥 Decoded: {}", decoded);
                    println!();
                    
                    if decoded == heartbeat {
                        println!("✅ VERITAS SILICONIS: Cryptographic heartbeat verified!");
                        println!("   The silicon heart beats true.");
                        println!("   GLOSSOPETRAE dialect operational and secure.");
                        
                        // Generate a memory vault encoding (static)
                        println!();
                        println!("=== MEMORY VAULT ENCODING (Data-At-Rest) ===");
                        let memory_data = "CHIMERA_KERNEL_STATE: {rust: true, autonomous: true, sovereign: true}";
                        match glossopetrae::encode_memory_vault(memory_data, master_seed, "alchemical") {
                            Ok(vault_encoded) => {
                                println!("🧠 Memory Vault (Alchemical Dialect):");
                                println!("{}", vault_encoded);
                            }
                            Err(e) => println!("❌ Memory vault encoding failed: {}", e),
                        }
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
