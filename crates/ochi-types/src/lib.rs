//! Transitional compatibility crate for Phase 3 rename.
//!
//! During the crate/workspace rename window, this crate re-exports all public
//! items from `openfang-types` so downstream crates can migrate incrementally.

pub use openfang_types::*;
