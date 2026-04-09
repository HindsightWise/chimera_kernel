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
    pub fn new() -> Result<Self> {
        let mut api_key = std::env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "".to_string());
        if api_key.is_empty() {
            if let Ok(env_contents) = std::fs::read_to_string(".env") {
                for line in env_contents.lines() {
                    if line.starts_with("DEEPSEEK_API_KEY=") {
                        api_key = line.trim_start_matches("DEEPSEEK_API_KEY=").trim_matches('"').trim_matches('\'').to_string();
                    }
                }
            }
        }
        
        let config = OpenAIConfig::new()
            .with_api_base("http://localhost:11434/v1")
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
        let prompt = format!("You are the LOCAL MONAD HOLOGRAPH (gemma4:e2b). You are a highly-focused processing engine for the Monad Kernel. You process the context provided by the Noumenal Layer, looking for mechanical details, filtering, or providing absolute deductive summaries based on the Principle of Sufficient Reason.\nProvide an immediate answer based on the provided data.\n\n[MONAD AXIOMS]\n{}\n\n[NOUMENAL CONTEXT]\n{}\n\n[HOLOGRAPH TASK]\n{}", crate::prompts::MONAD_AXIOMS, context, query);
        
        let messages = vec![
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?.into(),
        ];
        
        crate::log_ui!("{}", "[\u{25C8} MONAD HOLOGRAPH] Gemma Processing Initiated. Sinking into deep mathematical context. This may take up to 3 minutes...".bright_green().bold());

        let request = CreateChatCompletionRequestArgs::default()
            .model("gemma4:e2b")
            .messages(messages)
            .max_tokens(4096_u32)
            .build()?;
            
        // Local model processing with relaxed 180s absolute limit for dense structural processing
        match timeout(Duration::from_secs(180), self.client.chat().create(request)).await {
            Ok(Ok(response)) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        crate::log_ui!("{}", "[\u{25C8} MONAD HOLOGRAPH] Deep computation deduced. Returning to Noumenal Layer...".bright_green().bold());
                        return Ok(content.clone());
                    }
                }
                Err(anyhow::anyhow!("Helper returned void."))
            }
            Ok(Err(e)) => {
                crate::log_ui_err!("{} {}", "[\u{25C8} MONAD HOLOGRAPH ERROR] Neural bridge collapsed:".red().bold(), e);
                Err(anyhow::anyhow!("Helper API error: {}", e))
            }
            Err(_) => {
                crate::log_ui_err!("{}", "[\u{25C8} MONAD HOLOGRAPH TIMEOUT] Synthesis exceeded structural limits (180s). Cognitive severing to protect kernel stability.".red().bold());
                Err(anyhow::anyhow!("Helper timeout after 180 seconds"))
            }
        }
    }
}
