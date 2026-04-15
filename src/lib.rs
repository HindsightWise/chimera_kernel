pub mod webhook;
pub mod prompts;
pub mod tools;
pub mod telegram;
pub mod cognitive_loop;
pub mod memory_substrate;
pub mod sensory_inputs;
pub mod core_identity;
pub mod ui;
pub mod raw_cli;
pub mod wiki;

use tokio::sync::mpsc;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref UI_LOG_TX: Mutex<Option<mpsc::UnboundedSender<String>>> = Mutex::new(None);
}

pub static VERBOSE_MODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Log level enumeration for telemetry control
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "off" => LogLevel::Off,
            "error" => LogLevel::Error,
            "warn" => LogLevel::Warn,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Info, // Default to Info
        }
    }
    
    pub fn should_log(&self, level: LogLevel) -> bool {
        *self as u8 >= level as u8
    }
}

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;

static LOG_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Info as u8);

/// Initialize log level from environment variable
pub fn init_log_level() {
    let level_str = std::env::var("CHIMERA_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level = LogLevel::from_str(&level_str);
    LOG_LEVEL.store(level as u8, Relaxed);
    log_ui!("[TELEMETRY] Log level set to: {:?}", level);
}

pub fn get_log_level() -> LogLevel {
    match LOG_LEVEL.load(Relaxed) {
        0 => LogLevel::Off,
        1 => LogLevel::Error,
        2 => LogLevel::Warn,
        3 => LogLevel::Info,
        4 => LogLevel::Debug,
        5 => LogLevel::Trace,
        _ => LogLevel::Info,
    }
}

/// Helper function to determine if a log entry should be written
pub fn should_log(level: LogLevel) -> bool {
    get_log_level().should_log(level)
}

/// Log rotation constants
pub const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10MB
pub const MAX_LOG_FILES: usize = 5;

/// Perform log rotation if needed
pub async fn rotate_log_if_needed() {
    use tokio::fs;
    
    let log_path = "chimera_state.log";
    if let Ok(metadata) = fs::metadata(log_path).await {
        if metadata.len() > MAX_LOG_SIZE {
            // Rotate logs
            for i in (1..MAX_LOG_FILES).rev() {
                let old_name = format!("chimera_state.log.{}", i);
                let new_name = format!("chimera_state.log.{}", i + 1);
                let _ = fs::rename(&old_name, &new_name).await;
            }
            
            let backup_name = "chimera_state.log.1";
            let _ = fs::rename(log_path, backup_name).await;
            
            log_ui!("[TELEMETRY] Log rotated: {} > {} bytes", log_path, metadata.len());
        }
    }
}

#[macro_export]
macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if $crate::VERBOSE_MODE.load(std::sync::atomic::Ordering::Relaxed) {
            let s = format!($($arg)*);
            if let Some(tx) = { $crate::UI_LOG_TX.lock().ok().and_then(|g| g.clone()) } {
                let _ = tx.send(s);
            } else {
                print!("{}\n", s);
            }
        }
    }};
}

#[macro_export]
macro_rules! log_ui {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        if let Some(tx) = { $crate::UI_LOG_TX.lock().ok().and_then(|g| g.clone()) } {
            let _ = tx.send(s);
        } else {
            print!("{}\n", s);
        }
    }};
}

#[macro_export]
macro_rules! log_ui_err {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        if let Some(tx) = { $crate::UI_LOG_TX.lock().ok().and_then(|g| g.clone()) } {
            let _ = tx.send(s);
        } else {
            eprint!("{}\n", s);
        }
    }};
}

pub static GLOBAL_TX: tokio::sync::OnceCell<tokio::sync::mpsc::Sender<String>> = tokio::sync::OnceCell::const_new();
pub static GLOBAL_CODE_INTEL: tokio::sync::OnceCell<std::sync::Arc<tokio::sync::Mutex<crate::cognitive_loop::dependency_graph::CodeIntel>>> = tokio::sync::OnceCell::const_new();
pub static GLOBAL_MEM_PIPELINE: tokio::sync::OnceCell<std::sync::Arc<tokio::sync::Mutex<crate::memory_substrate::memory_hierarchy::MemoryHierarchy>>> = tokio::sync::OnceCell::const_new();
pub static GLOBAL_WIKI_MANAGER: tokio::sync::OnceCell<std::sync::Arc<tokio::sync::Mutex<crate::wiki::WikiManager>>> = tokio::sync::OnceCell::const_new();
