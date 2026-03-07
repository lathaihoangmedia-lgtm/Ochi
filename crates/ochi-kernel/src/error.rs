//! Kernel-specific error types.

use ochi_types::error::OchiError;
use thiserror::Error;

/// Kernel error type wrapping OchiError with kernel-specific context.
#[derive(Error, Debug)]
pub enum KernelError {
    /// A wrapped OchiError.
    #[error(transparent)]
    Ochi(#[from] OchiError),

    /// The kernel failed to boot.
    #[error("Boot failed: {0}")]
    BootFailed(String),
}

/// Alias for kernel results.
pub type KernelResult<T> = Result<T, KernelError>;
