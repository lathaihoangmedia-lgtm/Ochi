//! Utility functions for Ochi Core

/// Generate a unique ID
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Get current timestamp
pub fn timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert!(!id.is_empty());
    }
}
