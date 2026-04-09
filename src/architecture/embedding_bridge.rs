use tokio::process::Command;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use std::process::Stdio;
use tokio::sync::{mpsc, oneshot};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub struct IPCBridge {
    tx: mpsc::Sender<(String, oneshot::Sender<Option<String>>)>,
    connected: Arc<AtomicBool>,
}

impl IPCBridge {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel::<(String, oneshot::Sender<Option<String>>)>(100);
        let connected = Arc::new(AtomicBool::new(false));
        let connected_clone = connected.clone();
        
        tokio::spawn(async move {
            // Attempt to bind the child process to Mnemosyne's venv
            let mut child = match Command::new("../Mnemosyne-Substrate/mnemosyne/venv/bin/python3")
                .arg("-u") // Force unbuffered stdout/stdin
                .arg("src/tools/mnemosyne_ipc.py")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        crate::log_ui_err!("[IPC KERNEL PANIC] Failed to bind mnemosyne_ipc. Fallback required. {}", e);
                        connected_clone.store(false, Ordering::SeqCst);
                        // Exhaust loop natively dropping sender requests
                        while let Some((_, reply)) = rx.recv().await {
                            let _ = reply.send(None);
                        }
                        return;
                    }
                };
            
            connected_clone.store(true, Ordering::SeqCst);
            
            let mut stdin = child.stdin.take().expect("Failed to open IPC stdin");
            let stdout = child.stdout.take().expect("Failed to open IPC stdout");
            let mut reader = BufReader::new(stdout);
            
            if let Some(stderr) = child.stderr.take() {
                tokio::spawn(async move {
                    use tokio::io::AsyncBufReadExt;
                    let mut err_reader = tokio::io::BufReader::new(stderr);
                    let mut line = String::new();
                    while let Ok(bytes) = err_reader.read_line(&mut line).await {
                        if bytes == 0 { break; }
                        // Log Python IPC stderr silently into the chimera_state.log without breaking the TUI
                        if !line.trim().is_empty() {
                            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("chimera_state.log") {
                                use std::io::Write;
                                let _ = writeln!(file, "[MNEMOSYNE SERVER LOG]: {}", line.trim());
                            }
                        }
                        line.clear();
                    }
                });
            }
            
            while let Some((text, reply)) = rx.recv().await {
                // Remove raw newlines which would break IPC
                let clean_text = text.replace('\n', " ");
                let req = format!("{}\n", clean_text);
                
                if let Err(_) = stdin.write_all(req.as_bytes()).await {
                    let _ = reply.send(None);
                    continue;
                }
                
                let mut line = String::new();
                if let Ok(bytes) = reader.read_line(&mut line).await {
                    if bytes == 0 {
                        let _ = reply.send(None);
                        continue;
                    }
                    
                    let _ = reply.send(Some(line.trim().to_string()));
                } else {
                    let _ = reply.send(None);
                }
            }
            
            connected_clone.store(false, Ordering::SeqCst);
        });
        
        Self { tx, connected }
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }
    
    pub async fn dispatch_ipc(&self, json_payload: String) -> Option<String> {
        if !self.connected.load(Ordering::SeqCst) {
            return None;
        }
        
        let (reply_tx, reply_rx) = oneshot::channel();
        if self.tx.send((json_payload, reply_tx)).await.is_err() {
            return None;
        }
        
        reply_rx.await.unwrap_or(None)
    }
}
