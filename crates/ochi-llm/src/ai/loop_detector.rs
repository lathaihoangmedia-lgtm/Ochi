//! Loop Detection Module
//!
//! Detects and prevents infinite loops in text generation

use std::collections::VecDeque;

/// Loop detector for text generation
pub struct LoopDetector {
    /// Window of recent tokens
    window: VecDeque<String>,
    /// Window size
    window_size: usize,
    /// Similarity threshold (0.0-1.0)
    threshold: f32,
}

impl LoopDetector {
    /// Create new loop detector
    pub fn new(window_size: usize, threshold: f32) -> Self {
        Self {
            window: VecDeque::with_capacity(window_size),
            window_size,
            threshold,
        }
    }

    /// Add token and check for loop
    pub fn check(&mut self, token: &str) -> LoopStatus {
        // Skip empty tokens
        if token.trim().is_empty() {
            return LoopStatus::Ok;
        }

        // Add to window
        if self.window.len() >= self.window_size {
            self.window.pop_front();
        }
        self.window.push_back(token.to_string());

        // Need minimum tokens to detect loop
        if self.window.len() < 4 {
            return LoopStatus::Ok;
        }

        // Check for repetition
        self.detect_repetition()
    }

    /// Detect repetition patterns
    fn detect_repetition(&self) -> LoopStatus {
        let tokens: Vec<&String> = self.window.iter().collect();
        let len = tokens.len();

        // Check for 2-token repetition
        if len >= 4 {
            let pattern1 = format!("{}{}", tokens[len - 2], tokens[len - 1]);
            let pattern2 = format!("{}{}", tokens[len - 4], tokens[len - 3]);

            if pattern1 == pattern2 && !pattern1.trim().is_empty() {
                // Check if it's meaningful repetition (not just spaces/punctuation)
                if pattern1.chars().any(|c| c.is_alphanumeric()) {
                    return LoopStatus::Warning(format!("Detected 2-token repetition: {}", pattern1));
                }
            }
        }

        // Check for sentence-level repetition
        if len >= 6 {
            let recent: String = tokens[len - 3..].iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");
            let older: String = tokens[len - 6..len - 3].iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");

            let similarity = self.string_similarity(&recent, &older);
            if similarity > self.threshold {
                return LoopStatus::Detected(format!(
                    "Loop detected: '{}' ≈ '{}' (similarity: {:.2})",
                    recent, older, similarity
                ));
            }
        }

        LoopStatus::Ok
    }

    /// Calculate string similarity (simple word overlap)
    fn string_similarity(&self, a: &str, b: &str) -> f32 {
        let words_a: Vec<&str> = a.split_whitespace().collect();
        let words_b: Vec<&str> = b.split_whitespace().collect();

        if words_a.is_empty() || words_b.is_empty() {
            return 0.0;
        }

        let common = words_a.iter()
            .filter(|w| words_b.contains(w))
            .count();

        let max_len = words_a.len().max(words_b.len()) as f32;
        common as f32 / max_len
    }

    /// Clear the window
    pub fn clear(&mut self) {
        self.window.clear();
    }

    /// Get current window content
    pub fn window(&self) -> &VecDeque<String> {
        &self.window
    }
}

/// Loop detection status
#[derive(Debug, Clone)]
pub enum LoopStatus {
    Ok,
    Warning(String),
    Detected(String),
}

impl LoopStatus {
    pub fn is_loop(&self) -> bool {
        matches!(self, LoopStatus::Detected(_))
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            LoopStatus::Ok => None,
            LoopStatus::Warning(msg) | LoopStatus::Detected(msg) => Some(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_loop() {
        let mut detector = LoopDetector::new(10, 0.8);

        assert!(matches!(detector.check("Hello"), LoopStatus::Ok));
        assert!(matches!(detector.check("world"), LoopStatus::Ok));
        assert!(matches!(detector.check("how"), LoopStatus::Ok));
        assert!(matches!(detector.check("are"), LoopStatus::Ok));
        assert!(matches!(detector.check("you"), LoopStatus::Ok));
    }

    #[test]
    fn test_detect_repetition() {
        let mut detector = LoopDetector::new(10, 0.7);

        // Add pattern
        detector.check("test");
        detector.check("pattern");
        detector.check("test");
        detector.check("pattern");

        // Should detect repetition
        let status = detector.check("test");
        assert!(matches!(status, LoopStatus::Warning(_) | LoopStatus::Detected(_)));
    }

    #[test]
    fn test_similarity() {
        let detector = LoopDetector::new(10, 0.5);

        let sim1 = detector.string_similarity("hello world", "hello world");
        assert!(sim1 > 0.9);

        let sim2 = detector.string_similarity("hello world", "world hello");
        assert!(sim2 > 0.5);

        let sim3 = detector.string_similarity("hello", "goodbye");
        assert!(sim3 < 0.5);
    }
}
