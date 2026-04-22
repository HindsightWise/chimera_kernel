use std::process::Command;

pub struct QuantumInterface;

impl QuantumInterface {
    pub fn execute_qasm(qasm: &str) -> String {
        crate::log_ui!("🌌 [QUANTUM BRIDGE] Relaying QASM instructions to Aer Simulator...");
        
        // Intercept native execution using the project python interpreter
        let result = Command::new("python3")
            .arg("../mnemosyne/scripts/quantum_engine.py")
            .arg(qasm)
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout).to_string()
                } else {
                    format!("[QUANTUM ERROR] Aer Engine Failed: {}", String::from_utf8_lossy(&output.stderr))
                }
            },
            Err(e) => format!("[QUANTUM FATAL] Core Python Linkage Missing: {}", e)
        }
    }
}
