//! Trung Cung - Âm Dương smart router and 8 Bát Quái routing modules.

pub mod am_duong;
pub mod bat_quai;

#[cfg(feature = "duckdb")]
pub mod storage;

pub use am_duong::{AmDuongRouter, RouteDecision, RouteRequest, TaskEnvelope};

#[cfg(feature = "duckdb")]
pub use storage::DuckDbStore;
