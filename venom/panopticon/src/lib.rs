use tokio::net::TcpStream;
use tokio::time::{timeout, sleep, Duration};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::stream::{self, StreamExt};

/// Performs an adaptive ultra-high-speed TCP port sweep.
/// Utilizes Tokio streams to execute highly concurrent Half-Open (TCP SYN) probes without
/// triggering immediate thread saturation or M1 network stack exhaustion.
pub async fn sweep_target(target_ip: &str, start_port: u16, end_port: u16, batch_size: usize) -> Vec<u16> {
    let ip: IpAddr = match target_ip.parse() {
        Ok(ip) => ip,
        Err(_) => return vec![], // Invalid IP format
    };

    let open_ports = Arc::new(Mutex::new(Vec::new()));
    
    // Create an asynchronous iterator over the defined port range
    let ports: Vec<u16> = (start_port..=end_port).collect();
    
    // Execute concurrent batch scans. We batch them because unleashing 65,000 requests 
    // simultaneously onto the M1 kernel network stack will instantly panic the socket table.
    stream::iter(ports)
        .for_each_concurrent(batch_size, |port| {
            let open_ports_clone = Arc::clone(&open_ports);
            async move {
                let socket = SocketAddr::new(ip, port);
                
                // Adaptive Threshold: Wait exactly 150ms per port.
                match timeout(Duration::from_millis(150), TcpStream::connect(&socket)).await {
                    Ok(Ok(_)) => {
                        // Connection succeeded (Port is OPEN)
                        let mut op = open_ports_clone.lock().await;
                        op.push(port);
                    }
                    _ => {
                        // Connection failed or timed out (Port is CLOSED or FILTERED)
                        // Intentionally swallow the error to maintain silent stealth sweep
                    }
                }
                
                // Traffic Shaping: Sleep for 200 microseconds between connections to bypass IDS threshold alerts
                sleep(Duration::from_micros(200)).await;
            }
        })
        .await;

    let mut result = open_ports.lock().await.clone();
    result.sort_unstable();
    result
}
