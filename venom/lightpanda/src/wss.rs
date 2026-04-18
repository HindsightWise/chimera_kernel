use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use url::Url;

pub struct WssEngine;

impl WssEngine {
    pub async fn connect(uri: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(uri)?;
        println!("[WSS] Handshaking with Lightpanda -> {}", uri);
        
        // Spawn async connection
        let (ws_stream, response) = connect_async(url).await?;
        println!("[WSS] Sockets Bound (HTTP {})", response.status());

        let (_write, _read) = ws_stream.split();

        // Testing the channel
        // write.send(Message::Text("hello".to_string())).await?;

        Ok(())
    }
}
