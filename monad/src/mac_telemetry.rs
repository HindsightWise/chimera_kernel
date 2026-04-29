use std::process::Command;

pub fn gather_entropy() -> String {
    let mut telemetry = String::new();

    // 1. Battery Entropy
    if let Ok(output) = Command::new("pmset").arg("-g").arg("batt").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        if let Some(batt_line) = text.lines().nth(1) {
            let parsed = batt_line.split(';').next().unwrap_or("Unknown").trim();
            telemetry.push_str(&format!("[BATTERY: {}] ", parsed));
        }
    }

    // 2. RAM and CPU Thermodynamics
    if let Ok(output) = Command::new("top").args(&["-l", "1", "-n", "0"]).output() {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.starts_with("PhysMem:") {
                telemetry.push_str(&format!("[RAM: {}] ", line.replace("PhysMem:", "").trim()));
            } else if line.starts_with("CPU usage:") {
                telemetry.push_str(&format!("[CPU: {}] ", line.replace("CPU usage:", "").trim()));
            }
        }
    }

    if telemetry.is_empty() {
        telemetry.push_str("[PHYSICAL_SENSORS_OFFLINE]");
    }

    telemetry.trim().to_string()
}
