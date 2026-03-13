//! Error types for Ochi Core

use thiserror::Error;

/// Core error enumeration
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Custom error: {0}")]
    Custom(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
