//! Trung Cung - Âm Dương smart router and 8 Bát Quái routing modules.

pub mod am_duong;
pub mod bat_quai;
pub mod storage;

pub use am_duong::AmDuongRouter;
pub use storage::DuckDbStore;
