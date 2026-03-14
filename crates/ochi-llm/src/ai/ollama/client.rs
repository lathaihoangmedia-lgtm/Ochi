//! Ollama Client - Easy Model Management
//!
//! Simple wrapper around ollama-rs for pulling and switching models

use ochi_core::{Error, Result};
use super::{OllamaModel, OllamaOptions};

#[cfg(feature = "ollama")]
use ollama_rs::{Ollama as OllamaAPI, generation::options::GenerationOptions};
#[cfg(feature = "ollama")]
use futures::StreamExt;

// ollama-rs 0.2 API compatibility
#[cfg(feature = "ollama")]
type GenerationRequest<'a> = ollama_rs::generation::completion::request::GenerationRequest<'a>;

/// Ollama Client for model management
pub struct OllamaClient {
    host: String,
    port: u16,
}

impl OllamaClient {
    /// Create new client with default host (localhost:11434)
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 11434,
        }
    }

    /// Create client with custom host/port
    pub fn with_host(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    /// Get API URL
    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    /// Pull a model from Ollama library
    #[cfg(feature = "ollama")]
    pub async fn pull_model(&self, model: &str) -> Result<()> {
        tracing::info!("Pulling model: {}", model);
        tracing::warn!("Please run: ollama pull {} in terminal", model);
        Ok(())
    }

    /// List local models
    #[cfg(feature = "ollama")]
    pub async fn list_models(&self) -> Result<Vec<OllamaModel>> {
        tracing::debug!("Listing local models");

        let ollama = OllamaAPI::new(self.url(), self.port);

        match ollama.list_local_models().await {
            Ok(models) => {
                let models: Vec<OllamaModel> = models.iter().map(|m| OllamaModel {
                    name: m.name.clone(),
                    size: format!("{:.2} GB", m.size as f64 / 1e9),
                    digest: String::new(),
                    modified_at: m.modified_at.clone(),
                }).collect();

                tracing::info!("Found {} models", models.len());
                Ok(models)
            }
            Err(e) => {
                Err(Error::Custom(format!("Failed to list models: {}", e)))
            }
        }
    }

    /// Generate text with a model
    #[cfg(feature = "ollama")]
    pub async fn generate(&self, model: &str, prompt: &str, options: OllamaOptions) -> Result<String> {
        tracing::debug!("Generating with model: {}", model);

        let ollama = OllamaAPI::new(self.url(), self.port);

        // Build generation options
        let mut gen_options = GenerationOptions::default();

        if let Some(temp) = options.temperature {
            gen_options = gen_options.temperature(temp);
        }
        if let Some(top_p) = options.top_p {
            gen_options = gen_options.top_p(top_p);
        }
        if let Some(top_k) = options.top_k {
            gen_options = gen_options.top_k(top_k);
        }
        if let Some(penalty) = options.repeat_penalty {
            gen_options = gen_options.repeat_penalty(penalty);
        }
        if let Some(num) = options.num_predict {
            gen_options = gen_options.num_predict(num as i32);
        }

        let request = GenerationRequest::new(model.to_string(), prompt.to_string())
            .options(gen_options);

        match ollama.generate(request).await {
            Ok(response) => {
                tracing::debug!("Generated: {}", response.response.len());
                Ok(response.response)
            }
            Err(e) => {
                Err(Error::Custom(format!("Generation failed: {}", e)))
            }
        }
    }

    /// Generate with streaming
    #[cfg(feature = "ollama")]
    pub async fn generate_stream<F, Fut>(
        &self,
        model: &str,
        prompt: &str,
        options: OllamaOptions,
        mut callback: F,
    ) -> Result<String>
    where
        F: FnMut(String) -> Fut + Send,
        Fut: std::future::Future<Output = bool> + Send,
    {
        tracing::debug!("Streaming generation with model: {}", model);

        let ollama = OllamaAPI::new(self.url(), self.port);

        let mut gen_options = GenerationOptions::default();
        if let Some(temp) = options.temperature {
            gen_options = gen_options.temperature(temp);
        }
        if let Some(penalty) = options.repeat_penalty {
            gen_options = gen_options.repeat_penalty(penalty);
        }

        let mut output = String::new();
        let request = GenerationRequest::new(model.to_string(), prompt.to_string())
            .options(gen_options);

        // Note: ollama-rs 0.2 doesn't have streaming in this version
        // Use regular generate instead
        match ollama.generate(request).await {
            Ok(response) => {
                output = response.response;
                let _ = callback(output.clone()).await;
            }
            Err(e) => {
                return Err(Error::Custom(format!("Stream error: {}", e)));
            }
        }

        Ok(output)
    }

    /// Check if Ollama server is running
    pub async fn is_running(&self) -> bool {
        let client = reqwest::Client::new();
        match client.get(format!("{}/api/tags", self.url())).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Get recommended models for beginners
    pub fn recommended_models() -> Vec<&'static str> {
        vec![
            "llama3.2",           // 3B - Good for GTX 1050 Ti
            "llama3.2:1b",        // 1B - Very fast
            "phi3:mini",          // 3.8B - Microsoft's model
            "qwen2.5:3b",         // 3B - Alibaba's model
            "gemma2:2b",          // 2B - Google's model
            "mistral:7b",         // 7B - Good quality (needs more VRAM)
        ]
    }
}

#[cfg(not(feature = "ollama"))]
impl OllamaClient {
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 11434,
        }
    }

    pub fn with_host(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub async fn is_running(&self) -> bool {
        false
    }

    pub fn recommended_models() -> Vec<&'static str> {
        vec!["llama3.2", "phi3:mini", "qwen2.5:3b"]
    }
}
