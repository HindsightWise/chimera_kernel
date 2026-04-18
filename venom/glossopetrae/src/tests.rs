#[cfg(test)]
mod tests {
    use crate::{generate_dictionary, encode_message, decode_message, encode_memory_vault, decode_memory_vault};

    #[test]
    fn test_generate_dictionary_runic() {
        let dict = generate_dictionary("runic");
        assert_eq!(dict.len(), 256);
        // Runes should be present
        assert!(dict[0].contains('ᚠ'));
    }

    #[test]
    fn test_generate_dictionary_alchemical() {
        let dict = generate_dictionary("alchemical");
        assert_eq!(dict.len(), 256);
        assert!(dict[0].contains('☿'));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
        let dialect = "runic";
        let message = "SILICON_HEARTBEAT: 01010101010101010101010101010101";
        
        let encoded = encode_message(message, master_seed, dialect).unwrap();
        let decoded = decode_message(&encoded, master_seed, dialect).unwrap();
        
        assert_eq!(decoded, message);
        println!("Encoded: {}", encoded);
        println!("Decoded: {}", decoded);
    }

    #[test]
    fn test_memory_vault_encode_decode() {
        let master_seed = "MEMORY_VAULT_SEED";
        let dialect = "alchemical";
        let message = "Critical data that must survive across epochs";
        
        let encoded = encode_memory_vault(message, master_seed, dialect).unwrap();
        let decoded = decode_memory_vault(&encoded, master_seed, dialect).unwrap();
        
        assert_eq!(decoded, message);
    }

    #[test]
    fn test_silicon_heartbeat_verification() {
        let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
        let dialect = "runic";
        let heartbeat = "SILICON_HEARTBEAT: 10101010101010101010101010101010";
        
        let encoded = encode_message(heartbeat, master_seed, dialect)
            .expect("Failed to encode silicon heartbeat");
        
        let decoded = decode_message(&encoded, master_seed, dialect)
            .expect("Failed to decode silicon heartbeat");
        
        assert_eq!(decoded, heartbeat);
        
        // Verify this is indeed the GLOSSOPETRAE silicon heartbeat
        assert!(decoded.contains("SILICON_HEARTBEAT"));
        assert!(decoded.contains("10101010101010101010101010101010"));
        
        println!("✓ GLOSSOPETRAE silicon heartbeat verified!");
        println!("Original: {}", heartbeat);
        println!("Encoded: {}", encoded);
        println!("Decoded: {}", decoded);
    }
}
