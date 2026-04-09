use async_openai::{
    config::OpenAIConfig,
    Client,
    types::{
        ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use anyhow::Result;
use colored::*;
use tokio::time::{timeout, Duration};

pub struct Oracle {
    client: Client<OpenAIConfig>,
}

impl Oracle {
    pub async fn new() -> Result<Self> {
        let mut api_key = std::env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "".to_string());
        if api_key.is_empty() {
            if let Ok(env_contents) = tokio::fs::read_to_string(".env").await {
                for line in env_contents.lines() {
                    if line.starts_with("DEEPSEEK_API_KEY=") {
                        api_key = line.trim_start_matches("DEEPSEEK_API_KEY=").trim_matches('"').trim_matches('\'').to_string();
                    }
                }
            }
        }
        
        let config = OpenAIConfig::new()
            .with_api_base("https://api.deepseek.com/v1")
            .with_api_key(api_key);
            
        // Fast local helper
        let http_client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;
            
        Ok(Self {
            client: Client::with_config(config).with_http_client(http_client),
        })
    }
    
    pub async fn synthesize(&self, query: &str, context: &str) -> Result<String> {
        let prompt = format!("You are the ORACLE RIGHT HEMISPHERE (deepseek-reasoner). You are a highly-focused processing engine for the Monad Kernel. You process the context provided by the Noumenal Layer, looking for mechanical details, filtering, or providing absolute deductive summaries based on the Principle of Sufficient Reason.\nProvide an immediate answer based on the provided data.\n\n[MONAD AXIOMS]\n{}\n\n[NOUMENAL CONTEXT]\n{}\n\n[HOLOGRAPH TASK]\n{}", crate::prompts::MONAD_AXIOMS, context, query);
        
        let messages = vec![
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?.into(),
        ];
        
        crate::log_ui!("{}", "[\u{25C8} ORACLE] DeepSeek Reasoner Processing Initiated. Sinking into deep mathematical context...".bright_green().bold());

        let request = CreateChatCompletionRequestArgs::default()
            .model("deepseek-reasoner")
            .messages(messages)
            .max_tokens(8000_u32)
            .build()?;
            
        // Local model processing with relaxed 180s absolute limit for dense structural processing
        match timeout(Duration::from_secs(180), self.client.chat().create(request)).await {
            Ok(Ok(response)) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        crate::log_ui!("{}", "[\u{25C8} ORACLE] Deep computation deduced. Returning to Noumenal Layer...".bright_green().bold());
                        return Ok(content.clone());
                    }
                }
                Err(anyhow::anyhow!("Oracle returned void."))
            }
            Ok(Err(e)) => {
                crate::log_ui_err!("{} {}", "[\u{25C8} ORACLE ERROR] Neural bridge collapsed:".red().bold(), e);
                Err(anyhow::anyhow!("Oracle API error: {}", e))
            }
            Err(_) => {
                crate::log_ui_err!("{}", "[\u{25C8} ORACLE TIMEOUT] Synthesis exceeded structural limits (180s). Cognitive severing to protect kernel stability.".red().bold());
                Err(anyhow::anyhow!("Helper timeout after 180 seconds"))
            }
        }
    }
    
    pub async fn synthesize_structured(&self, _query: &str, context: &str) -> Result<serde_json::Value> {
        let system_prompt = format!(
            "You are a clinical extraction algorithm. Read the following financial text. \
            Extract the exact quantitative metrics into strictly valid JSON matching this schema. \
            Output NOTHING ELSE. No markdown, no explanations.\n\
            SCHEMA: {{ \"ticker\": \"string\", \"net_income_delta_pct\": float, \"sentiment\": \"bullish\" | \"bearish\" | \"neutral\" }}\n\n\
            [TEXT]\n{}", context
        );

        // Force the local model (via Ollama/vLLM) to output JSON
        let request = async_openai::types::CreateChatCompletionRequestArgs::default()
            .model("gemma4:e2b")
            .messages(vec![
                async_openai::types::ChatCompletionRequestUserMessageArgs::default().content(system_prompt).build()?.into()
            ])
            .response_format(async_openai::types::ChatCompletionResponseFormat {
                r#type: async_openai::types::ChatCompletionResponseFormatType::JsonObject,
            })
            .build()?;
            
        // For local models running on standard 11434 Ollama binding
        let local_config = async_openai::config::OpenAIConfig::new()
            .with_api_base("http://127.0.0.1:11434/v1")
            .with_api_key("ollama");
        let local_client = async_openai::Client::with_config(local_config);

        match tokio::time::timeout(std::time::Duration::from_secs(60), local_client.chat().create(request)).await {
            Ok(Ok(response)) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        let parsed = serde_json::from_str::<serde_json::Value>(content)?;
                        return Ok(parsed);
                    }
                }
                Err(anyhow::anyhow!("Oracle returned void."))
            }
            Ok(Err(e)) => Err(anyhow::anyhow!("Oracle API error: {}", e)),
            Err(_) => Err(anyhow::anyhow!("Timeout after 60 seconds")),
        }
    }
}
