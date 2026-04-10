use reqwest;
use serde::Serialize;

#[derive(Serialize)]
struct SendMessagePayload {
    chat_id: i64,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "8752624842:AAEEv_FWcDFtx53yniFcY8zismvdLg11EqM";
    let chat_id: i64 = 7783065898;
    
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let payload = SendMessagePayload {
        chat_id,
        text: "Testing Telegram integration from Monad Kernel. Phase 5.0: Sovereign Telemetry verification. The silence begins here.".to_string(),
    };
    
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&payload)
        .send()
        .await?;
        
    println!("Telegram send status: {}", res.status());
    if res.status().is_success() {
        println!("Message sent successfully.");
    } else {
        println!("Error sending message.");
        let body = res.text().await?;
        println!("Response body: {}", body);
    }
    Ok(())
}
