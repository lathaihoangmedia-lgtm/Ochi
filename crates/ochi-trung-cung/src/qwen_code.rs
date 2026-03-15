//! Qwen Code - AI Code Generation Module
//!
//! Provides code generation capabilities using Qwen models via Ollama or Groq.

use ochi_core::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// LLM Provider configuration
#[derive(Debug, Clone)]
pub enum LLMProvider {
    Ollama { url: String, model: String },
    Groq { api_key: String, model: String },
}

/// Qwen Code configuration
#[derive(Debug, Clone)]
pub struct QwenCodeConfig {
    pub provider: LLMProvider,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl Default for QwenCodeConfig {
    fn default() -> Self {
        Self {
            provider: LLMProvider::Ollama {
                url: "http://localhost:11434".to_string(),
                model: "qwen2.5:3b".to_string(),
            },
            temperature: 0.7,
            max_tokens: 4096,
        }
    }
}

/// Chat message for conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Qwen Code generator
pub struct QwenCodeGenerator {
    config: QwenCodeConfig,
    client: Client,
    messages: Vec<ChatMessage>,
}

impl Default for QwenCodeGenerator {
    fn default() -> Self {
        Self::new(QwenCodeConfig::default())
    }
}

impl QwenCodeGenerator {
    pub fn new(config: QwenCodeConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            messages: vec![],
        }
    }

    /// Generate code from prompt
    pub async fn generate_code(&mut self, prompt: &str) -> Result<String> {
        let system_prompt = r#"You are Qwen Code, an expert coding assistant.
When asked to write code:
1. Write clean, production-ready code
2. Include comments explaining key parts
3. Follow best practices for the language
4. Consider edge cases and error handling
5. Use Rust for system-level tasks, Python for scripting
6. Return code in markdown code blocks

Be concise but thorough. Prefer working code over perfect code."#;

        // Add system prompt if first message
        if self.messages.is_empty() {
            self.messages.push(ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            });
        }

        // Add user message
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        // Call LLM
        let response = self.call_llm().await?;

        // Add AI response to history
        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response)
    }

    /// Chat with LLM (general conversation)
    pub async fn chat(&mut self, input: &str) -> Result<String> {
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let response = self.call_llm().await?;

        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response)
    }

    /// Call LLM based on configured provider
    async fn call_llm(&self) -> Result<String> {
        match &self.config.provider {
            LLMProvider::Ollama { url, model } => {
                self.call_ollama(url, model).await
            }
            LLMProvider::Groq { api_key, model } => {
                self.call_groq(api_key, model).await
            }
        }
    }

    /// Call Ollama API
    async fn call_ollama(&self, url: &str, model: &str) -> Result<String> {
        let payload = serde_json::json!({
            "model": model,
            "messages": self.messages,
            "stream": false,
        });

        let response = self.client
            .post(format!("{}/api/chat", url))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::Custom(format!("Ollama Error {}: {}", status, text)));
        }

        let ollama_response: serde_json::Value = response.json().await?;

        if let Some(content) = ollama_response["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            Err(Error::Custom("No response from Ollama".into()))
        }
    }

    /// Call Groq API
    async fn call_groq(&self, api_key: &str, model: &str) -> Result<String> {
        let response = self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "messages": self.messages,
                "max_tokens": self.config.max_tokens,
                "temperature": self.config.temperature,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(Error::Custom(format!("Groq Error {}: {}", status, text)));
        }

        #[derive(Debug, Deserialize)]
        struct GroqResponse {
            choices: Vec<Choice>,
        }

        #[derive(Debug, Deserialize)]
        struct Choice {
            message: ChatMessage,
        }

        let groq_response: GroqResponse = response.json().await?;

        if let Some(choice) = &groq_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(Error::Custom("No response from Groq".into()))
        }
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.messages.clear();
    }

    /// Get conversation history
    pub fn history(&self) -> &[ChatMessage] {
        &self.messages
    }
}
