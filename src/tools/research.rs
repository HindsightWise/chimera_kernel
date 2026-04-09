use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use std::time::Duration;

pub fn definition_spider() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "spider_rss".to_string(),
            description: Some("Scrape the RSS or XML feed of a target website to instantly bypass bot-protection and read all recent published titles, summaries, and URLs.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "feed_url": { "type": "string", "description": "The exact URL to the RSS/XML endpoint (e.g. https://www.sciencedaily.com/rss/matter_energy.xml)" }
                },
                "required": ["feed_url"]
            })),
        },
    }
}

pub fn definition_deep_read() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "deep_read_url".to_string(),
            description: Some("Uses the Jina AI Headless Reader to bypass Cloudflare, execute JS, and extract the full comprehensive Markdown text of a specific article or webpage.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "The target URL to read." }
                },
                "required": ["url"]
            })),
        },
    }
}

pub fn definition_tavily_search() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "tavily_search".to_string(),
            description: Some("Performs an AI-optimized semantic web search using the Tavily Research API. Returns summaries and exact URLs of matching results. Extremely useful for finding up-to-date facts, current events, or tracking down specific technical sources.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "The search query (e.g. 'recent advancements in solid state batteries')" },
                    "search_depth": { "type": "string", "description": "Either 'basic' or 'advanced'."}
                },
                "required": ["query"]
            })),
        },
    }
}

pub async fn execute_spider(args: Value) -> String {
    let url = args.get("feed_url").and_then(|v| v.as_str()).unwrap_or("");
    if url.is_empty() { return "[ERROR] No feed_url provided.".to_string(); }

    let client_res = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .build();
    let client = match client_res {
        Ok(c) => c,
        Err(e) => return format!("[ERROR] Failed to build HTTP client: {}", e),
    };

    match tokio::time::timeout(Duration::from_secs(30), client.get(url).header("User-Agent", "Sovereign-Chimera/2.0").send()).await {
        Ok(Ok(res)) => {
            if let Ok(xml_text) = res.text().await {
                // Parse with roxmltree
                let doc = match roxmltree::Document::parse(&xml_text) {
                    Ok(d) => d,
                    Err(e) => return format!("[ERROR] Invalid XML/RSS Feed: {}", e),
                };
                
                let mut results = String::new();
                results.push_str(&format!("[RSS FEED TARGET ACQUIRED: {}]\n", url));
                let mut count = 0;
                
                for node in doc.descendants() {
                    if node.has_tag_name("item") || node.has_tag_name("entry") {
                        if count >= 10 { break; } // Limit to top 10 to preserve matrix bandwidth
                        
                        let mut title = "Untitled";
                        let mut link = "No Link";
                        let mut description = "";
                        
                        for child in node.children() {
                            match child.tag_name().name() {
                                "title" => title = child.text().unwrap_or("Untitled"),
                                "link" => {
                                    if child.text().is_some() {
                                        link = child.text().unwrap();
                                    } else if let Some(href) = child.attribute("href") {
                                        link = href; // Atom feeds
                                    }
                                }
                                "description" | "summary" => description = child.text().unwrap_or(""),
                                _ => {}
                            }
                        }
                        
                        // Clean up description HTML natively by stripping everything between < and >
                        let mut raw_clean = String::new();
                        let mut in_tag = false;
                        for c in description.chars() {
                            if c == '<' {
                                in_tag = true;
                            } else if c == '>' {
                                in_tag = false;
                                raw_clean.push(' ');
                            } else if !in_tag {
                                raw_clean.push(c);
                            }
                        }
                        // Collapse multiple spaces
                        let clean_desc = raw_clean.split_whitespace().collect::<Vec<&str>>().join(" ");
                        let truncated_desc: String = clean_desc.chars().take(200).collect();
                        
                        results.push_str(&format!("- [{}] {}\n  {}\n", title, link, truncated_desc));
                        count += 1;
                    }
                }
                
                if count == 0 {
                    return format!("[ERROR] No items found in feed. Are you sure {} is an active RSS/XML endpoint?", url);
                }
                results
            } else {
                "[ERROR] Failed to read response body.".to_string()
            }
        }
        Ok(Err(e)) => format!("[ERROR] Network failure fetching RSS: {}", e),
        Err(_) => "[ERROR] Feed scrape timeout exceeded 30s. Target dropped.".to_string(),
    }
}

pub async fn execute_deep_read(args: Value) -> String {
    let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("");
    if url.is_empty() { return "[ERROR] No url provided.".to_string(); }

    let jina_key = std::env::var("JINA_API_KEY").unwrap_or_default();
    let target = format!("https://r.jina.ai/{}", url);
    
    let client_res = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .build();
    let client = match client_res {
        Ok(c) => c,
        Err(e) => return format!("[ERROR] Failed to build HTTP client: {}", e),
    };
    
    let mut req = client.get(&target).header("Accept", "text/event-stream");
    
    if !jina_key.is_empty() {
        req = req.header("Authorization", format!("Bearer {}", jina_key));
    }

    match tokio::time::timeout(Duration::from_secs(30), req.send()).await {
        Ok(Ok(res)) => {
            if !res.status().is_success() {
                return format!("[ERROR] Jina AI rejected the extraction (Status: {}). Check your JINA_API_KEY or URL format.", res.status());
            }
            if let Ok(markdown) = res.text().await {
                // Return exactly 4000 characters to prevent overflowing the Chimera context window during inference
                let truncated: String = markdown.chars().take(4000).collect();
                if markdown.len() > 4000 {
                    format!("{}\n\n... [ARTICLE TRUNCATED. Maximum cognitive bounds reached.]", truncated)
                } else {
                    truncated
                }
            } else {
                "[ERROR] Could not decode Markdown response payload.".to_string()
            }
        }
        Ok(Err(e)) => format!("[ERROR] Network failure executing Deep Read API: {}", e),
        Err(_) => "[ERROR] Deep Read timeout exceeded 30s. Aborting extraction.".to_string(),
    }
}

pub async fn execute_tavily_search(args: Value) -> String {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    let depth = args.get("search_depth").and_then(|v| v.as_str()).unwrap_or("basic");
    if query.is_empty() { return "[ERROR] No query provided.".to_string(); }

    let tavily_key = std::env::var("TAVILY_API_KEY").unwrap_or_default();
    if tavily_key.is_empty() {
        return "[ERROR] TAVILY_API_KEY not set in environment or lazarus_daemon.sh.".to_string();
    }

    let payload = json!({
        "api_key": tavily_key,
        "query": query,
        "search_depth": depth,
        "include_answer": true,
        "max_results": 5
    });

    let client_res = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .build();
    let client = match client_res {
        Ok(c) => c,
        Err(e) => return format!("[ERROR] Failed to build HTTP client: {}", e),
    };
    
    match tokio::time::timeout(Duration::from_secs(30), client.post("https://api.tavily.com/search")
        .json(&payload)
        .send()).await {
        Ok(Ok(res)) => {
            if !res.status().is_success() {
                return format!("[ERROR] Tavily Search rejected request (Status: {}). Check your quota or API key.", res.status());
            }
            
            if let Ok(data) = res.json::<Value>().await {
                let mut results = String::new();
                
                if let Some(answer) = data.get("answer").and_then(|v| v.as_str()) {
                    results.push_str(&format!("[TAVILY AI ANSWER]\n{}\n\n", answer));
                }
                
                if let Some(arr) = data.get("results").and_then(|v| v.as_array()) {
                    results.push_str("[SEARCH RESULTS]\n");
                    for item in arr {
                        let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled");
                        let link = item.get("url").and_then(|v| v.as_str()).unwrap_or("No URL");
                        let content = item.get("content").and_then(|v| v.as_str()).unwrap_or("");
                        results.push_str(&format!("- [{}]({})\n  {}\n\n", title, link, content));
                    }
                }
                
                if results.is_empty() {
                    return "[ERROR] Tavily returned an empty payload for this query.".to_string();
                }
                results
            } else {
                "[ERROR] Failed to parse Tavily JSON response.".to_string()
            }
        }
        Ok(Err(e)) => format!("[ERROR] Network failure fetching Tavily API: {}", e),
        Err(_) => "[ERROR] Tavily Search timeout exceeded 30s.".to_string(),
    }
}
