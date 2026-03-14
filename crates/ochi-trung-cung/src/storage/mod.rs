//! Storage layer for Trung Cung.

#[cfg(feature = "duckdb")]
pub mod duckdb;

#[cfg(feature = "duckdb")]
pub use duckdb::DuckDbStore;

#[cfg(feature = "duckdb")]
pub use duckdb::TaskFlowLog;
