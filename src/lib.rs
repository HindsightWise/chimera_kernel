pub mod agent;
pub mod webhook;
pub mod prompts;
pub mod tools;
pub mod telegram;
pub mod architecture;
pub mod ui;
pub mod raw_cli;

use tokio::sync::mpsc;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref UI_LOG_TX: Mutex<Option<mpsc::UnboundedSender<String>>> = Mutex::new(None);
}

pub static VERBOSE_MODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

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
