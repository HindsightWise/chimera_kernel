use hyper::Request;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use url::Url;

use crate::error::LeviathanError;
use crate::network::LeviathanClient;
use crate::dom::LeviathanDocument;
use crate::dom::LeviathanJsContext;

pub struct LeviathanResponse {
    pub status: u16,
    pub document: LeviathanDocument,
    // Native Headers composite extension point
}

pub struct LeviathanEngine {
    user_agent: String,
}

impl LeviathanEngine {
    pub fn new() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".to_string(),
        }
    }

    /// Autonomous one-liner to rip payloads undetected via HTTP/2 and JA3 spoofing
    pub async fn stealth_get(&self, target_url: &str) -> Result<LeviathanResponse, LeviathanError> {
        let url = Url::parse(target_url).map_err(|e| LeviathanError::ConnectionFailed(format!("Invalid URL: {}", e)))?;
        let host = url.host_str().ok_or_else(|| LeviathanError::ConnectionFailed("No host bounded in URL".into()))?;
        
        let mut client = LeviathanClient::build_h2_session(host).await?;

        let request = Request::builder()
            .method("GET")
            .uri(target_url)
            .header("Host", host)
            .header("User-Agent", &self.user_agent)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .body(Empty::<Bytes>::new())
            .unwrap();

        let response = client.request(request).await?;
        let status = response.status().as_u16();

        let bytes = response.into_body().collect().await
            .map_err(|e| LeviathanError::ConnectionFailed(format!("Body decode explicitly failed: {}", e)))?
            .to_bytes();
            
        let raw_html = String::from_utf8_lossy(&bytes);
        let document = LeviathanDocument::parse(&raw_html);

        Ok(LeviathanResponse {
            status,
            document,
        })
    }

    /// Macro execution bounds to extract all scripts from the dom and evaluate them isolated
    pub fn evaluate_dom_scripts(&self, doc: &LeviathanDocument) -> Result<Vec<String>, LeviathanError> {
        let mut js = LeviathanJsContext::new();
        let scripts = doc.extract_scripts_raw()?;
        
        let mut results = Vec::new();
        for script in scripts {
            // Very rough tag stripping just to securely isolate the payload string
            let cleaned = script.replace("<script>", "").replace("</script>", "").replace(r#"type="text/javascript""#, "");
            
            // Only evaluate structural native scripts (ignoring outer sources for now)
            if !cleaned.trim().is_empty() {
                if let Ok(res) = js.evaluate(&cleaned) {
                    results.push(res);
                }
            }
        }
        Ok(results)
    }
}
