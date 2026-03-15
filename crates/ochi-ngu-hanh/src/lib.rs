//! Ngũ Hành agents and database scaffolding.

pub mod db;
pub mod kim;
pub mod moc;
pub mod thuy;
pub mod hoa;
pub mod tho;
pub mod embedding;

pub use db::DatabaseManager;
pub use kim::KimAgent;
pub use moc::MocAgent;
pub use thuy::ThuyAgent;
pub use hoa::HoaAgent;
pub use tho::{ThoAgent, MemorySearchResult, MemoryStats};
pub use embedding::{EmbeddingProvider, EmbeddingModel, create_provider, NoopEmbedding, MockEmbedding};
