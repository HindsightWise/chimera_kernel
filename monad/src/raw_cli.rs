use std::io::{self};
use tokio::io::{AsyncBufReadExt, BufReader};
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

    // Take over the main thread to wait for user input asynchronously
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer).await {
            Ok(0) => {
                // EOF detected. Park this thread so background agents can run without 100% CPU lock
                tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
            }
            Ok(_) => {
                let line = buffer.trim().to_string();
                if !line.is_empty() {
                    if line == "/quit" || line == "/exit" {
                        std::process::exit(0);
                    }
                    println!("{} {}", "\n[YOU]".bright_magenta().bold(), line.white());
                    let _ = tx_stdin.send(line).await;
                }
            }
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}
