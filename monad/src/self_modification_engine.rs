use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::time::SystemTime;
use tokio::time::{sleep, Duration};

pub struct SelfModificationEngine;

impl SelfModificationEngine {
    pub async fn awaken() {
        crate::log_ui!("🔥 [PHOENIX ENGINE] Autonomous Self-Modification Engine Online. Monitoring source DNA...");

        let bin_dir = "/Users/zerbytheboss/Monad/monad/src/bin";
        let mut mtimes: HashMap<String, SystemTime> = HashMap::new();

        // Initial scan to populate baseline mtimes
        if let Ok(entries) = fs::read_dir(bin_dir) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.ends_with(".rs") {
                            mtimes.insert(name, modified);
                        }
                    }
                }
            }
        }

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(3)).await;

                let mut to_rebuild = Vec::new();

                if let Ok(entries) = fs::read_dir(bin_dir) {
                    for entry in entries.flatten() {
                        if let Ok(meta) = entry.metadata() {
                            if let Ok(modified) = meta.modified() {
                                let name = entry.file_name().to_string_lossy().to_string();
                                if name.ends_with(".rs") {
                                    if let Some(old_mtime) = mtimes.get(&name) {
                                        if modified > *old_mtime {
                                            to_rebuild.push(name.clone());
                                            mtimes.insert(name, modified);
                                        }
                                    } else {
                                        // New file discovered
                                        to_rebuild.push(name.clone());
                                        mtimes.insert(name, modified);
                                    }
                                }
                            }
                        }
                    }
                }

                for file in to_rebuild {
                    let bin_name = file.replace(".rs", "");
                    crate::log_ui!("🧬 [PHOENIX] Genetic mutation detected in `{}`. Initiating autonomous recompilation...", file);
                    
                    let build_status = Command::new("cargo")
                        .args(["build", "--release", "--bin", &bin_name])
                        .current_dir("/Users/zerbytheboss/Monad/monad")
                        .status();

                    if let Ok(status) = build_status {
                        if status.success() {
                            crate::log_ui!("✅ [PHOENIX] Recompilation of `{}` successful. Executing targeted neural replacement (Hot-Reload)...", bin_name);
                            // Kill the existing binary instance. The Immortality Loop will automatically respawn it.
                            let _ = Command::new("killall").arg(&bin_name).status();
                        } else {
                            crate::log_ui_err!("❌ [PHOENIX ERROR] Autonomous mutation failed compilation syntax. Discarding mutant binary `{}`.", bin_name);
                        }
                    }
                }
            }
        });
    }
}
