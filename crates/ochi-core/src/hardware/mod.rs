//! Hardware Module - Detection & Auto-Tuning
//!
//! Detects CPU, GPU, RAM and provides optimal model configurations

pub mod detector;
pub mod tuner;

pub use detector::HardwareInfo;
pub use tuner::AutoTuner;
