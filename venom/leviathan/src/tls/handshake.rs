use std::sync::Arc;
use rustls_pki_types::ServerName;
use tokio_rustls::{TlsConnector, client::TlsStream};
use tokio::net::TcpStream;
use crate::error::LeviathanError;
use crate::network::TcpConnection;
use super::ja3::build_chrome_config;

pub struct SpoofedTlsStream {
    pub inner: TlsStream<TcpStream>,
}

pub enum Fingerprint {
    Chrome,
}

impl SpoofedTlsStream {
    pub async fn upgrade(conn: TcpConnection, domain: &str, _fp: Fingerprint) -> Result<Self, LeviathanError> {
        let config = build_chrome_config();
        let connector = TlsConnector::from(Arc::new(config));
        
        let server_name = ServerName::try_from(domain)
            .map_err(|_| LeviathanError::ConnectionFailed(format!("Invalid DNS name: {}", domain)))?
            .to_owned();

        let stream = connector.connect(server_name, conn.stream).await?;

        Ok(Self { inner: stream })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::TcpConnection;

    #[tokio::test]
    async fn test_tls_handshake_cloudflare() {
        // Hitting cloudflare's core public HTTPS endpoint
        let conn = TcpConnection::connect("cloudflare.com", 443).await.unwrap();
        let spoofed = SpoofedTlsStream::upgrade(conn, "cloudflare.com", Fingerprint::Chrome).await;
        
        assert!(spoofed.is_ok(), "TLS Handshake explicitly failed.");
        let stream = spoofed.unwrap();
        
        let alpn = stream.inner.get_ref().1.alpn_protocol();
        let alpn_str = alpn.map(|s| String::from_utf8_lossy(s).to_string()).unwrap_or_else(|| "none".to_string());
        println!("Successfully negotiated TLS with ALPN: {}", alpn_str);
    }
}
