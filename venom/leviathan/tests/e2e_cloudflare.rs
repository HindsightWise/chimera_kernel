use leviathan::network::LeviathanClient;
use leviathan::dom::LeviathanDocument;
use hyper::Request;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;

#[tokio::test]
async fn test_full_stealth_pipeline() {
    // Phase 1: Substrate Hijacking
    println!("[E2E] Bootstrapping Strict H2 Fingerprint Pipeline against Cloudflare...");
    let mut client = LeviathanClient::build_h2_session("cloudflare.com").await.expect("Failed to build H2 session");

    let request = Request::builder()
        .method("GET")
        .uri("https://cloudflare.com/")
        .header("Host", "cloudflare.com")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.9")
        .body(Empty::<Bytes>::new())
        .unwrap();

    let response = client.request(request).await.expect("H2 Request violently blocked or failed");
    
    // Captcha Bypass validation
    println!("[E2E] Received HTTP Status: {}", response.status());
    assert!(response.status().is_success(), "Response was blocked by Captcha/WAF (Status: {})", response.status());

    // Phase 2: Native DOM Parsing Integration
    println!("[E2E] Extracting raw TCP/H2 payload buffer...");
    let bytes = response.into_body().collect().await.expect("Failed body collection").to_bytes();
    
    let raw_html = String::from_utf8_lossy(&bytes);
    println!("[E2E] Ripped {} bytes at protocol-level. Passing to LeviathanDocument...", raw_html.len());
    
    let document = LeviathanDocument::parse(&raw_html);
    
    // Scraper CSS Query validation
    let titles = document.css_select_text("title").expect("CSS Selection failed");
    assert!(!titles.is_empty(), "Failed to read <title> from extracted DOM");
    println!("[E2E] Extracted Native DOM Node (Title): {}", titles[0]);
    
    // Explicit assertion ensuring we did not land on a turnstile page
    let challenge = document.css_select_text("#challenge-running").unwrap_or_default();
    assert!(challenge.is_empty(), "CRITICAL FAILURE: CAPTCHA Challenge layer detected.");
    
    println!("[E2E] Cloudflare Native Stealth Verification Passed.");
}
