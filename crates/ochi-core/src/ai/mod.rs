//! AI/ML Module - Candle Inference + Ollama Integration
//!
//! Provides:
//! - Candle: Pure Rust model inference (GGUF, Safetensors)
//! - Ollama: Easy model switching and management
//! - Loop detection and auto-recovery

pub mod model;
pub mod loop_detector;
pub mod auto_config;

#[cfg(feature = "ollama")]
pub mod ollama;

#[cfg(feature = "ollama")]
pub mod client;

pub use model::CandleModel;
pub use loop_detector::{LoopDetector, LoopStatus};
pub use auto_config::{AutoConfigurator, AutoConfigResult};

#[cfg(feature = "ollama")]
pub use ollama::{OllamaModel, OllamaRequest, OllamaOptions, OllamaResponse};

#[cfg(feature = "ollama")]
pub use client::OllamaClient;
