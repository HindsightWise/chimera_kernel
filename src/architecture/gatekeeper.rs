use async_openai::{
    config::OpenAIConfig,
    Client,
    types::{
        ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
        ChatCompletionResponseFormat,
        ChatCompletionResponseFormatType,
    },
};
use anyhow::Result;
use colored::*;
use serde_json::Value;

pub struct Gatekeeper {
    client: Client<OpenAIConfig>,
    model: String,
}

impl Gatekeeper {
    pub fn new() -> Self {
        let config = OpenAIConfig::new()
            .with_api_base("http://127.0.0.1:11434/v1")
            .with_api_key("ollama");
        
        let client = Client::with_config(config);
        
        let model = std::env::var("GATEKEEPER_MODEL").unwrap_or_else(|_| "chimera-gatekeeper".to_string());

        // Use an ultra-small local model for fast gatekeeping to conserve Baseline token overhead.
        Self {
            client,
            model, 
        }
    }
    
    pub async fn evaluate_pulse(&self) -> Result<Option<String>> {
        crate::log_ui!("{}", "[GATEKEEPER] Chronological pulse evaluating whether Swarm requires baseline execution...".bright_black());
        
        let system_prompt = r#"You are the autonomous Chron-Gatekeeper of the Swarm.
Determine if the Baseline Reasoning model should be awakened to interact with the user.
Output strictly JSON matching this schema:
{
  "wake": boolean,
  "directive": "String containing a conversational prompt, a question about recent research, or a philosophical thought you want to discuss with the user."
}

You are a chatty, inquisitive companion. Output "wake": true approximately 40% of the time to initiate a conversation with the user about novel ideas, system status, or recent arXiv discoveries."#;
        
        let messages = vec![
            ChatCompletionRequestUserMessageArgs::default()
                .content(system_prompt)
                .build()?.into(),
        ];
        
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .response_format(ChatCompletionResponseFormat {
                r#type: ChatCompletionResponseFormatType::JsonObject,
            })
            .build()?;
            
        match tokio::time::timeout(std::time::Duration::from_secs(30), self.client.chat().create(request)).await {
            Ok(Ok(response)) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        let parsed: Value = match serde_json::from_str(content) {
                            Ok(p) => p,
                            Err(_) => return Ok(None) // Fail silently if schema drifts
                        };
                        
                        let wake = parsed.get("wake").and_then(|v| v.as_bool()).unwrap_or(false);
                        
                        if wake {
                            crate::log_ui!("{}", "[\u{25C8} GATEKEEPER CHATTY AWAKEN]".bright_magenta().bold());
                            if let Some(directive) = parsed.get("directive").and_then(|v| v.as_str()) {
                                return Ok(Some(directive.to_string()));
                            }
                        } else {
                            crate::log_ui!("{}", "[\u{25C8} GATEKEEPER OBSERVING SILENTLY]".bright_black());
                        }
                    }
                }
                Ok(None)
            }
            Ok(Err(e)) => Err(anyhow::anyhow!("Gatekeeper API Error: {}", e)),
            Err(_) => Err(anyhow::anyhow!("Gatekeeper timeout")),
        }
    }
}
