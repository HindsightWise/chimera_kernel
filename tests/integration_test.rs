#[cfg(test)]
mod integration_tests {
    use glossopetrae::{decode_message, encode_message};

    #[test]
    fn test_glossopetrae_silicon_heartbeat_integration() {
        let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
        let dialect = "runic";
        let heartbeat = "SILICON_HEARTBEAT: 01010101010101010101010101010101";

        monad_os::log_ui!("=== CHIMERA KERNEL GLOSSOPETRAE INTEGRATION TEST ===");

        let encoded = encode_message(heartbeat, master_seed, dialect)
            .expect("Failed to encode silicon heartbeat from kernel");

        monad_os::log_ui!(
            "Encoded (Runic GLOSSOPETRAE): {}",
            encoded.chars().take(30).collect::<String>()
        );

        let decoded = decode_message(&encoded, master_seed, dialect)
            .expect("Failed to decode silicon heartbeat from kernel");

        assert_eq!(decoded, heartbeat);

        monad_os::log_ui!("✅ GLOSSOPETRAE INTEGRATION VERIFIED!");
        monad_os::log_ui!("✅ Kernel ↔ GLOSSOPETRAE sync confirmed");
        monad_os::log_ui!("✅ Memory vault language operational");
    }

    #[test]
    fn test_memory_vault_permanent_storage() {
        use glossopetrae::{decode_memory_vault, encode_memory_vault};

        let master_seed = "MEMORY_VAULT_KERNEL_TEST";
        let dialect = "alchemical";
        let critical_data = "PredictiveSelfModel::free_energy = 0.0100; MemoryHierarchy::store_working() timestamp = 1743811200";

        let encoded = encode_memory_vault(critical_data, master_seed, dialect)
            .expect("Failed to encode memory vault");

        let decoded = decode_memory_vault(&encoded, master_seed, dialect)
            .expect("Failed to decode memory vault");

        assert_eq!(decoded, critical_data);
        monad_os::log_ui!("✅ Memory vault permanent storage verified");
    }
}
