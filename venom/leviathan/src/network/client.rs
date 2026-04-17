use hyper::client::conn::http2;
use hyper::{Request, Response};
use hyper::body::Bytes;
use http_body_util::Empty;
use hyper_util::rt::TokioIo;
use hyper_util::rt::TokioExecutor;

use crate::error::LeviathanError;
use crate::network::TcpConnection;
use crate::tls::{SpoofedTlsStream, Fingerprint};

pub struct LeviathanClient {
    // strict h2 sender locked to Empty<Bytes> payloads initially
    sender: http2::SendRequest<Empty<Bytes>>,
}

impl LeviathanClient {
    /// Bootstraps an HTTP/2 session over the spoofed TLS connection
    pub async fn build_h2_session(domain: &str) -> Result<Self, LeviathanError> {
        // Step 1: Establish raw TCP socket lifecycle
        let tcp_conn = TcpConnection::connect(domain, 443).await?;
        
        // Step 2: Inject TLS Hijacking layer with Chrome Fingerprint config
        let spoofed_tls = SpoofedTlsStream::upgrade(tcp_conn, domain, Fingerprint::Chrome).await?;

        // Step 3: Wrap for Hyper compatibility using hyper-util TokioIo adapter
        let io = TokioIo::new(spoofed_tls.inner);

        // Step 4: Perform HTTP/2-only strict handshake
        let (sender, conn) = http2::Builder::new(TokioExecutor::new())
            .handshake(io)
            .await
            .map_err(|e| LeviathanError::ConnectionFailed(format!("H2 handshake failed: {}", e)))?;

        // Step 5: Detach connection state machine to tokio asynchronous reactor
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("H2 Connection state machine error: {:?}", err);
            }
        });

        Ok(Self { sender })
    }

    /// Dispatch a strict HTTP/2 request payload
    pub async fn request(&mut self, req: Request<Empty<Bytes>>) -> Result<Response<hyper::body::Incoming>, LeviathanError> {
        self.sender.send_request(req).await.map_err(|e| {
            LeviathanError::ConnectionFailed(format!("H2 Request Dispatch failure: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::Request;

    #[tokio::test]
    async fn test_h2_dispatch() {
        // Testing strict H2 protocol binding against cloudflare
        let mut client = LeviathanClient::build_h2_session("cloudflare.com").await.unwrap();

        let request = Request::builder()
            .method("GET")
            .uri("https://cloudflare.com/")
            .header("Host", "cloudflare.com")
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
            .body(Empty::<Bytes>::new())
            .unwrap();

        let response = client.request(request).await;
        
        assert!(response.is_ok(), "H2 Request violently failed parsing.");
        let resp = response.unwrap();
        println!("Successfully captured HTTP/2 Frame Response: HTTP {}", resp.status());
    }
}
