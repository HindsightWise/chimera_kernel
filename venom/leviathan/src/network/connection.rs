use crate::error::LeviathanError;
use tokio::net::TcpStream;

pub struct TcpConnection {
    pub stream: TcpStream,
}

impl TcpConnection {
    pub async fn connect(host: &str, port: u16) -> Result<Self, LeviathanError> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&addr).await?;
        Ok(Self { stream })
    }

    pub async fn connect_via_proxy(_host: &str, _port: u16, _proxy: &str) -> Result<Self, LeviathanError> {
        // Stub for future JA3 proxy implementation
        Err(LeviathanError::ConnectionFailed("Proxy connect not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_raw_tcp_connection() {
        // 1.1.1.1 is Cloudflare's public DNS resolver, typically open on port 80
        match TcpConnection::connect("1.1.1.1", 80).await {
            Ok(conn) => {
                println!("Successfully connected to {:?}", conn.stream.peer_addr().unwrap());
            }
            Err(e) => {
                panic!("Failed to connect: {}", e);
            }
        }
    }
}
