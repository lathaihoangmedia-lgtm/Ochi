//! Ochi Core Library
//!
//! Core functionality for the Ochi Next workspace

/// Core error types
pub mod error;

/// Core utilities
pub mod utils;

/// AI/ML - Candle Inference + Ollama
pub mod ai;

/// Hardware Detection & Auto-Tuning
pub mod hardware;

/// Test utilities
#[cfg(test)]
pub mod test_utils;

/// Re-exports
pub use error::{Error, Result};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the core library
pub fn init() -> Result<()> {
    tracing::info!("Initializing Ochi Core v{}", VERSION);
    Ok(())
}

pub use ai::model::CandleModel;

#[cfg(feature = "ollama")]
pub use ai::OllamaClient;

pub use hardware::{HardwareInfo, AutoTuner};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
