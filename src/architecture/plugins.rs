use tokio::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasi_common::pipe::WritePipe;
use async_openai::types::{ChatCompletionTool, FunctionObject};

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub wasm_file: String,
}

pub struct PluginManager {
    pub engine: Engine,
    pub plugins: Vec<PluginManifest>,
    pub plugins_dir: String,
}

impl PluginManager {
    pub async fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(false); // We use standard core WASM modules for simplicity
        
        let engine = Engine::new(&config).expect("Failed to initialize WASM engine for testing ground.");
        let plugins_dir = "plugins".to_string();
        
        let mut manager = Self {
            engine,
            plugins: Vec::new(),
            plugins_dir,
        };
        manager.reload_plugins().await;
        manager
    }

    pub async fn reload_plugins(&mut self) {
        self.plugins.clear();
        let path = Path::new(&self.plugins_dir);
        if !path.exists() {
            let _ = fs::create_dir_all(path).await;
            return;
        }

        if let Ok(mut entries) = fs::read_dir(path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&p).await {
                        if let Ok(manifest) = serde_json::from_str::<PluginManifest>(&content) {
                            self.plugins.push(manifest);
                        }
                    }
                }
            }
        }
    }

    pub fn get_tools(&self) -> Vec<ChatCompletionTool> {
        let mut tools = Vec::new();
        for p in &self.plugins {
            if p.parameters.is_object() {
                tools.push(ChatCompletionTool {
                    r#type: async_openai::types::ChatCompletionToolType::Function,
                    function: FunctionObject {
                        name: p.name.clone(),
                        description: Some(p.description.clone()),
                        parameters: Some(p.parameters.clone()),
                    },
                });
            } else {
                crate::log_ui_err!("[WASM REGISTRY ERROR] Invalid parameters schema for {}", p.name);
            }
        }
        tools
    }

    pub async fn execute(&self, name: &str, args: serde_json::Value) -> String {
        let Some(manifest) = self.plugins.iter().find(|p| p.name == name) else {
            return format!("[ERROR] Plugin {} not found in memory.", name);
        };

        let wasm_path = Path::new(&self.plugins_dir).join(&manifest.wasm_file);
        if !wasm_path.exists() {
            return format!("[ERROR] WASM file missing: {}", wasm_path.display());
        }

        let Ok(module) = Module::from_file(&self.engine, &wasm_path) else {
            return format!("[ERROR] WASM compilation failed.");
        };

        let mut linker = Linker::new(&self.engine);
        if let Err(e) = wasmtime_wasi::add_to_linker(&mut linker, |s| s) {
            return format!("[ERROR] Failed to link WASI capabilities: {}", e);
        }

        // We use a temporary file approach to capture stdout safely across environments
        let output_log_path = format!("plugins/{}_output.log", name);
        let _ = tokio::fs::remove_file(&output_log_path).await;

        let args_str = serde_json::to_string(&args).unwrap_or_default();
        
        let stdout = WritePipe::new_in_memory();
        
        // Build restricted WASI Sandbox Environment
        let mut builder = WasiCtxBuilder::new();
        // Use single-statement builder modifications to avoid 'unwrap()' or return type trait issues
        let _ = builder.arg(name);
        let _ = builder.arg(&args_str);
        let _ = builder.stdout(Box::new(stdout.clone()));
        
        let wasi = builder.build();

        let mut store = Store::new(&self.engine, wasi);

        let Ok(instance) = linker.instantiate(&mut store, &module) else {
            return format!("[ERROR] Instantiation Trap inside Sandbox");
        };

        let Ok(func) = instance.get_typed_func::<(), ()>(&mut store, "_start") else {
            return format!("[ERROR] Sandbox Execution Hook Missing (needs rust main function)");
        };

        crate::log_ui!("[WASM TESTING GROUND] Igniting bytecode engine for '{}'...", name);
        if let Err(trap) = func.call(&mut store, ()) {
            // proc_exit is acceptable if it exited successfully, but we'll return the trap as error just in case it panicked.
            let trap_msg = trap.to_string();
            if !trap_msg.contains("exit status 0") {
                return format!("[WASM SANDBOX PANIC TRAPPED] {}", trap_msg);
            }
        }
        
        // Drop store so we can unwrap stdout cleanly, or just call try_into_inner
        drop(store);
        
        let bytes = stdout.try_into_inner().unwrap().into_inner();
        let output = String::from_utf8_lossy(&bytes).into_owned();

        if output.trim().is_empty() {
             "[WASM EXECUTION COMPLETE] (No stdout output detected)".to_string()
        } else {
             format!("[WASM STDOUT]\n{}", output)
        }
    }
}
