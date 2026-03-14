//! Ollama types and client interface.

use serde::{Deserialize, Serialize};

#[cfg(feature = "ollama")]
pub mod client;

pub mod auto_tune;

/// Ollama model metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub size: String,
    pub digest: String,
    pub modified_at: String,
}

/// Ollama generation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub options: OllamaOptions,
}

/// Ollama generation options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaOptions {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub repeat_penalty: Option<f32>,
    pub num_predict: Option<u32>,
}

impl OllamaOptions {
    pub fn new() -> Self {
        Self {
            temperature: None,
            top_p: None,
            top_k: None,
            repeat_penalty: None,
            num_predict: None,
        }
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn with_repeat_penalty(mut self, repeat_penalty: f32) -> Self {
        self.repeat_penalty = Some(repeat_penalty);
        self
    }

    pub fn with_num_predict(mut self, num_predict: u32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }
}

impl Default for OllamaOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Ollama response placeholder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
}
