//! Ochi Core Library
//!
//! Core functionality for the Ochi Next workspace

/// Core error types
pub mod error;

/// Core utilities
pub mod utils;

/// AI/ML - GGUF Inference with CUDA
#[cfg(feature = "ai")]
pub mod ai;

/// Hardware Detection & Auto-Tuning
#[cfg(feature = "ai")]
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

#[cfg(feature = "ai")]
pub use ai::model::{GGUFModel, GGUFConfig};

#[cfg(feature = "ai")]
pub use hardware::{HardwareInfo, AutoTuner};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
