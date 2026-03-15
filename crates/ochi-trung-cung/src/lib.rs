//! Trung Cung - Âm Dương smart router and 8 Bát Quái routing modules.
//!
//! Modules:
//! - am_duong: Yin-Yang smart router
//! - bat_quai: 8 Trigram routing agents (Go integration via FFI)
//! - nlp: Hybrid 3-layer NLP (Dictionary + Rust NLP + LLM)
//! - qwen_code: AI code generation with Qwen models

pub mod am_duong;
pub mod bat_quai;
pub mod nlp;
pub mod qwen_code;

pub use am_duong::{AmDuongRouter, RouteDecision, RouteRequest, TaskEnvelope};
pub use nlp::{HybridNLPProcessor, NLPIntent, Operation, ParamDef};
pub use qwen_code::{QwenCodeConfig, QwenCodeGenerator, LLMProvider};
