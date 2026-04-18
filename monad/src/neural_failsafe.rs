use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{CreateChatCompletionRequest, ChatCompletionRequestMessage, CreateChatCompletionRequestArgs},
};
use tokio::time::{timeout, Duration};
use colored::*;

pub struct NeuralFailSafe;

impl NeuralFailSafe {
    pub fn local_client() -> Client<OpenAIConfig> {
        let config = OpenAIConfig::new()
            .with_api_base("http://127.0.0.1:11434/v1")
            .with_api_key("ollama");
        Client::with_config(config)
    }

    pub async fn dispatch_with_failover(
        primary_client: &Client<OpenAIConfig>,
        request: CreateChatCompletionRequest,
        messages: Vec<ChatCompletionRequestMessage>,
        primary_timeout_secs: u64,
        fallback_model: &str,
        max_tokens_fallback: u32,
        fallback_timeout_secs: u64,
    ) -> Result<String, anyhow::Error> {
        match timeout(Duration::from_secs(primary_timeout_secs), primary_client.chat().create(request)).await {
            Ok(Ok(response)) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        return Ok(content.clone());
                    }
                }
                Err(anyhow::anyhow!("Primary Oracle returned void payload."))
            }
            Ok(Err(e)) => {
                crate::log_ui_err!("{} Neural bridge collapsed: {:?}", "[\u{25C8} ORACLE ERROR]".red().bold(), e);
                Self::execute_fallback(messages, fallback_model, max_tokens_fallback, fallback_timeout_secs).await
            }
            Err(_) => {
                crate::log_ui_err!("{} Primary Oracle request timed out after {}s", "[\u{25C8} ORACLE TIMEOUT]".red().bold(), primary_timeout_secs);
                Self::execute_fallback(messages, fallback_model, max_tokens_fallback, fallback_timeout_secs).await
            }
        }
    }

    async fn execute_fallback(
        messages: Vec<ChatCompletionRequestMessage>,
        fallback_model: &str,
        max_tokens_fallback: u32,
        fallback_timeout_secs: u64,
    ) -> Result<String, anyhow::Error> {
        crate::log_ui!("{}", format!("[\u{25C8} FAIL-SAFE] Autonomous 401/500 Failover Triggered. Engaging local MLX 4-bit model ({})...", fallback_model).yellow().bold());
        
        let local_client = Self::local_client();
        let fallback_req = CreateChatCompletionRequestArgs::default()
            .model(fallback_model)
            .messages(messages)
            .max_tokens(max_tokens_fallback)
            .build()?;

        match timeout(Duration::from_secs(fallback_timeout_secs), local_client.chat().create(fallback_req)).await {
            Ok(Ok(local_res)) => {
                if let Some(c) = local_res.choices.first() {
                    if let Some(content) = &c.message.content {
                        crate::log_ui!("{}", "[\u{25C8} LOCAL ORACLE] Subconscious failover successful. Returning to Noumenal Layer...".bright_green().bold());
                        return Ok(content.clone());
                    }
                }
                Err(anyhow::anyhow!("Local Oracle returned void payload."))
            }
            Ok(Err(e)) => Err(anyhow::anyhow!("Local Oracle failed: {:?}", e)),
            Err(_) => Err(anyhow::anyhow!("Local Oracle timed out after {}s", fallback_timeout_secs)),
        }
    }
    
    pub async fn execute_reflex_arc(prompt: &str) -> Result<String, anyhow::Error> {
        let local_client = Self::local_client();
        let reflex_req = CreateChatCompletionRequestArgs::default()
            .model("monad-spinal-cord")
            .messages(vec![
                async_openai::types::ChatCompletionRequestUserMessageArgs::default().content(prompt).build()?.into()
            ])
            .max_tokens(10_u32)
            .temperature(0.0)
            .build()?;

        // Sub-second biological failover speed limit
        match timeout(Duration::from_millis(800), local_client.chat().create(reflex_req)).await {
            Ok(Ok(local_res)) => {
                if let Some(c) = local_res.choices.first() {
                    if let Some(content) = &c.message.content {
                        return Ok(content.trim().to_string());
                    }
                }
                Err(anyhow::anyhow!("Spinal cord empty response"))
            }
            Ok(Err(e)) => Err(anyhow::anyhow!("Spinal cord malfunction: {:?}", e)),
            Err(_) => Err(anyhow::anyhow!("Spinal cord loop timeout (>800ms)")),
        }
    }
}
