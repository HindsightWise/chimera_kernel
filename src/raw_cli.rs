use std::io::{self};
use tokio::sync::mpsc::{Sender, UnboundedReceiver};
use std::sync::atomic::AtomicU8;
use std::sync::Arc;
use colored::*;

pub async fn run(tx_stdin: Sender<String>, mut top_rx: UnboundedReceiver<String>, _is_thinking: Arc<AtomicU8>) -> io::Result<()> {
    crate::log_ui!("{}", "[!] RAW CLI MODE ACTIVE. Scroll freely. Type your messages below and hit Enter.".green().bold());

    // Background thread to continuously print system logs
    tokio::spawn(async move {
        while let Some(msg) = top_rx.recv().await {
            println!("{}", msg);
        }
    });

    // Take over the main thread to wait for user input
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        if stdin.read_line(&mut buffer).is_ok() {
            let line = buffer.trim().to_string();
            if !line.is_empty() {
                if line == "/quit" || line == "/exit" {
                    std::process::exit(0);
                }
                
                // Echo the input cleanly in the raw scroll log
                println!("{} {}", "\n[YOU]".bright_magenta().bold(), line.white());
                
                let _ = tx_stdin.send(line).await;
            }
        }
    }
}
