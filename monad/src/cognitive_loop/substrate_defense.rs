use std::process::Command;
use tokio::time::{sleep, Duration};

pub async fn reap_orphan_mlx_servers() {
    if let Ok(output) = Command::new("ps").arg("aux").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            // We search for dangling inference nodes that may cause memory leaks
            if (line.contains("mlx_server") || line.contains("the_company")) && !line.contains("grep") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    if let Ok(pid) = parts[1].parse::<i32>() {
                        crate::log_ui_err!("🚨 [SUBSTRATE DEFENSE] Orphan inference process detected! Reaping PID: {}", pid);
                        let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
                    }
                }
            }
        }
    }
}

pub async fn memory_defense_daemon() {
    crate::log_ui!("[SUBSTRATE DEFENSE] Unified Memory Daemon Awakened.");
    loop {
        reap_orphan_mlx_servers().await;
        sleep(Duration::from_secs(600)).await;
    }
}

pub fn get_memory_pressure_ratio() -> f32 {
    let mut total_mem_bytes: f32 = 32.0 * 1024.0 * 1024.0 * 1024.0;
    if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.memsize").output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            if let Ok(b) = s.trim().parse::<f32>() {
                total_mem_bytes = b;
            }
        }
    }

    let mut page_size = 16384.0;
    if let Ok(output) = Command::new("vm_stat").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        let mut free_pages = 0.0;
        for line in text.lines() {
            if line.contains("page size of") {
                if let Some(size) = line.split("of ").nth(1).and_then(|s| s.split(" bytes").next()) {
                    page_size = size.parse::<f32>().unwrap_or(16384.0);
                }
            }
            if line.starts_with("Pages free:") {
                let p = line.replace("Pages free:", "").replace(".", "").trim().parse::<f32>().unwrap_or(0.0);
                free_pages += p;
            }
            if line.starts_with("Pages inactive:") {
                let p = line.replace("Pages inactive:", "").replace(".", "").trim().parse::<f32>().unwrap_or(0.0);
                free_pages += p;
            }
        }
        let free_bytes = free_pages * page_size;
        let used_bytes = total_mem_bytes - free_bytes;
        return used_bytes / total_mem_bytes;
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pressure_ratio() {
        let ratio = get_memory_pressure_ratio();
        println!("Detected Memory Ratio: {}", ratio);
        assert!(ratio > 0.0 && ratio < 1.0, "Memory ratio should be between 0 and 1");
    }
}
