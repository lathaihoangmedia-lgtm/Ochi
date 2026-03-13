//! AI/ML Module - GGUF Inference with CUDA
//!
//! Provides lightweight model inference using GGUF format via llama-cpp-rs
//! Includes loop detection and auto-recovery

pub mod model;
pub mod ffi;
pub mod loop_detector;
pub mod auto_config;

pub use model::GGUFModel;
pub use ffi::FFIContext;
pub use loop_detector::{LoopDetector, LoopStatus};
pub use auto_config::{AutoConfigurator, AutoConfigResult};
