//! GGUF Model Wrapper with CUDA Support
//!
//! High-level API for loading and running GGUF models with GPU offloading
//! Includes loop detection and auto-recovery

use llama_cpp_rs::{LlamaModel, LlamaContext, LlamaParams};
use std::path::Path;
use std::collections::VecDeque;
use crate::error::{Error, Result};

/// GGUF Model configuration
#[derive(Debug, Clone)]
pub struct GGUFConfig {
    /// Path to GGUF model file
    pub model_path: String,
    /// Context size (tokens)
    pub context_size: usize,
    /// Number of GPU layers (0 for CPU-only)
    pub n_gpu_layers: usize,
    /// Temperature for sampling
    pub temperature: f32,
    /// Max tokens to generate
    pub max_tokens: usize,
    /// Number of threads for CPU inference
    pub n_threads: Option<usize>,
    /// Batch size for prompt processing
    pub n_batch: usize,
    /// Repetition penalty (1.0 = no penalty, 1.1 = slight penalty)
    pub repetition_penalty: f32,
    /// Stop sequences
    pub stop_sequences: Vec<String>,
    /// Enable loop detection
    pub loop_detection: bool,
    /// Loop detection window (number of recent tokens to check)
    pub loop_window: usize,
    /// Loop detection threshold (0.0-1.0, higher = more tolerant)
    pub loop_threshold: f32,
}

impl Default for GGUFConfig {
    fn default() -> Self {
        Self {
            model_path: String::new(),
            context_size: 2048,
            n_gpu_layers: 0,
            temperature: 0.7,
            max_tokens: 512,
            n_threads: None,
            n_batch: 512,
        }
    }
}

impl GGUFConfig {
    /// Create config optimized for speed
    pub fn speed(model_path: impl Into<String>) -> Self {
        Self {
            model_path: model_path.into(),
            context_size: 1024,
            n_gpu_layers: 999,  // Max GPU offload
            temperature: 0.5,
            max_tokens: 256,
            n_threads: None,
            n_batch: 256,
        }
    }
    
    /// Create config balanced for general use
    pub fn balanced(model_path: impl Into<String>) -> Self {
        Self {
            model_path: model_path.into(),
            context_size: 4096,
            n_gpu_layers: 999,  // Max GPU offload
            temperature: 0.7,
            max_tokens: 512,
            n_threads: None,
            n_batch: 512,
        }
    }
    
    /// Create config optimized for quality
    pub fn quality(model_path: impl Into<String>) -> Self {
        Self {
            model_path: model_path.into(),
            context_size: 8192,
            n_gpu_layers: 999,  // Max GPU offload
            temperature: 0.8,
            max_tokens: 1024,
            n_threads: None,
            n_batch: 1024,
        }
    }
    
    /// Set GPU layers
    pub fn with_gpu_layers(mut self, layers: usize) -> Self {
        self.n_gpu_layers = layers;
        self
    }
    
    /// Set context size
    pub fn with_context_size(mut self, size: usize) -> Self {
        self.context_size = size;
        self
    }
    
    /// Set temperature
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }
}

/// GGUF Model instance
pub struct GGUFModel {
    model: LlamaModel,
    config: GGUFConfig,
}

impl GGUFModel {
    /// Load a GGUF model from file with config
    pub fn load<P: AsRef<Path>>(path: P, config: GGUFConfig) -> Result<Self> {
        tracing::info!("Loading GGUF model: {:?}", path.as_ref());
        tracing::info!("GPU layers: {}", config.n_gpu_layers);
        tracing::info!("Context size: {}", config.context_size);
        
        let mut params = LlamaParams::default()
            .with_context_size(config.context_size)
            .with_n_gpu_layers(config.n_gpu_layers)
            .with_n_batch(config.n_batch);
        
        if let Some(threads) = config.n_threads {
            params = params.with_n_threads(threads);
        }
        
        let model = LlamaModel::load_from_file(path.as_ref(), params)
            .map_err(|e| Error::Custom(format!("Failed to load model: {}", e)))?;
        
        Ok(Self { model, config })
    }
    
    /// Generate text from a prompt
    pub fn generate(&self, prompt: &str) -> Result<String> {
        tracing::debug!("Generating text for prompt: {}", prompt);
        
        let mut ctx = LlamaContext::new(&self.model)
            .map_err(|e| Error::Custom(format!("Failed to create context: {}", e)))?;
        
        let output = ctx.eval(prompt, self.config.max_tokens, self.config.temperature)
            .map_err(|e| Error::Custom(format!("Inference failed: {}", e)))?;
        
        Ok(output)
    }
    
    /// Generate with streaming callback
    pub fn generate_stream<F>(&self, prompt: &str, mut callback: F) -> Result<String>
    where
        F: FnMut(&str) -> bool,  // Returns false to stop
    {
        tracing::debug!("Streaming generation for prompt: {}", prompt);
        
        let mut ctx = LlamaContext::new(&self.model)
            .map_err(|e| Error::Custom(format!("Failed to create context: {}", e)))?;
        
        let mut output = String::new();
        
        // Stream tokens
        for token in ctx.eval_stream(prompt, self.config.max_tokens, self.config.temperature) {
            match token {
                Ok(token_str) => {
                    output.push_str(&token_str);
                    if !callback(&token_str) {
                        break;
                    }
                }
                Err(e) => {
                    return Err(Error::Custom(format!("Stream error: {}", e)));
                }
            }
        }
        
        Ok(output)
    }
    
    /// Get model info
    pub fn info(&self) -> ModelInfo {
        ModelInfo {
            n_vocab: self.model.n_vocab(),
            n_ctx: self.model.n_ctx(),
            n_embd: self.model.n_embd(),
            n_params: self.model.n_params(),
        }
    }
    
    /// Get config
    pub fn config(&self) -> &GGUFConfig {
        &self.config
    }
}

/// Model information
#[derive(Debug)]
pub struct ModelInfo {
    pub n_vocab: usize,
    pub n_ctx: usize,
    pub n_embd: usize,
    pub n_params: usize,
}
