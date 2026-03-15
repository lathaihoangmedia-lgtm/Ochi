//! 8 Bát Quái routing modules.

pub mod can;
pub mod khon;
pub mod chan;
pub mod ton;
pub mod kham;
pub mod ly;
pub mod can_gua;
pub mod doai;

/// Go Agents placeholder (scaffold for future FFI integration)
pub mod go_agents;

pub use go_agents::{BatQuaiAgent, BatQuaiRouter, GoAgentResponse};
