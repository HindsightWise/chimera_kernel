use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use reqwest::Client;
use roxmltree::Document;
use std::time::Duration;
use tokio::time::sleep;

pub async fn run_omniscient_sweep(categories: Vec<&str>, max_results: usize, fetch_new_only: bool) {
    let client = Client::new();

    for category in categories {
        let sort_by = if fetch_new_only {
            "submittedDate"
        } else {
            "relevance"
        };
        let url = format!(
            "http://export.arxiv.org/api/query?search_query=cat:{}&sortBy={}&max_results={}",
            category, sort_by, max_results
        );

        crate::log_ui!("[OMNISCIENCE DAEMON] Scraping ArXiv category: {}", category);

        if let Ok(response) = client.get(&url).send().await {
            if let Ok(xml_body) = response.text().await {
                if let Ok(doc) = Document::parse(&xml_body) {
                    for entry in doc.descendants().filter(|n| n.has_tag_name("entry")) {
                        let mut title = String::new();
                        let mut summary = String::new();

                        for child in entry.children() {
                            if child.has_tag_name("title") {
                                title = child.text().unwrap_or("").to_string();
                            } else if child.has_tag_name("summary") {
                                summary = child.text().unwrap_or("").to_string();
                            }
                        }

                        if !title.is_empty() && !summary.is_empty() {
                            let _ = refine_and_ingest(category, &title, &summary).await;
                        }
                    }
                }
            }
        }

        // Strict rate limit compliance (ArXiv allows max 1 req per 3-4s without ban)
        sleep(Duration::from_secs(4)).await;
    }
    crate::log_ui!("[OMNISCIENCE DAEMON] Category sweep complete.");
}

async fn refine_and_ingest(
    _category: &str,
    title: &str,
    summary: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let local_client = crate::neural_failsafe::NeuralFailSafe::local_client();

    let fallback_model =
        std::env::var("FAILOVER_MODEL").unwrap_or_else(|_| "gemma4:e2b".to_string());

    let instructions = "You are the Local Refinery. Read this arXiv abstract. Filter out noise. Extract the core mathematical or systemic breakthrough. Output strictly as a JSON object: {\"category\": \"string\", \"core_innovation\": \"string\", \"mathematical_basis\": \"string\"}.";

    let user_msg = format!("Title: {}\nAbstract: {}", title, summary);

    let request = match CreateChatCompletionRequestArgs::default()
        .model(fallback_model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(instructions)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_msg)
                .build()?
                .into(),
        ])
        .temperature(0.1)
        .build()
    {
        Ok(r) => r,
        Err(_) => return Ok(()),
    };

    if let Ok(response) = tokio::time::timeout(
        tokio::time::Duration::from_secs(120),
        local_client.chat().create(request),
    )
    .await
    {
        if let Ok(res) = response {
            if let Some(choice) = res.choices.first() {
                if let Some(content) = &choice.message.content {
                    // Send directly to the Subconscious Pipeline natively via the global anchor!
                    if let Some(mem_pipeline) = crate::GLOBAL_MEM_PIPELINE.get() {
                        let mut mp = mem_pipeline.lock().await;
                        let _ = mp.store_working(content.to_string(), 0.8, 0.2, false).await;
                        // Avoid overloading the rendering UI with massive text chunks, log quietly
                    }
                }
            }
        }
    }

    Ok(())
}
