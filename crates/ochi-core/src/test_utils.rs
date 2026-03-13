//! Test Utilities for Ochi Core
//!
//! Common test helpers, mocks, and fixtures

#[cfg(test)]
pub mod mocks {
    use crate::hardware::{HardwareInfo, detector::{CpuInfo, GpuInfo, MemoryInfo}};

    /// Mock hardware info for testing
    impl HardwareInfo {
        /// Create mock hardware (low-end)
        pub fn mock_low_end() -> Self {
            Self {
                cpu: CpuInfo {
                    cores: 2,
                    threads: 4,
                    name: "Mock CPU".to_string(),
                },
                gpu: None,
                memory: MemoryInfo {
                    total: 4,
                    available: 2,
                },
                has_gpu: false,
            }
        }

        /// Create mock hardware (mid-range)
        pub fn mock_mid_range() -> Self {
            Self {
                cpu: CpuInfo {
                    cores: 4,
                    threads: 8,
                    name: "Mock CPU".to_string(),
                },
                gpu: Some(GpuInfo {
                    name: "Mock GPU".to_string(),
                    vram_total: 4096,
                    vram_available: 3072,
                    cuda_cores: 768,
                    supports_cuda: true,
                }),
                memory: MemoryInfo {
                    total: 16,
                    available: 12,
                },
                has_gpu: true,
            }
        }

        /// Create mock hardware (high-end)
        pub fn mock_high_end() -> Self {
            Self {
                cpu: CpuInfo {
                    cores: 16,
                    threads: 32,
                    name: "Mock CPU".to_string(),
                },
                gpu: Some(GpuInfo {
                    name: "Mock GPU".to_string(),
                    vram_total: 24576,
                    vram_available: 20480,
                    cuda_cores: 10496,
                    supports_cuda: true,
                }),
                memory: MemoryInfo {
                    total: 64,
                    available: 56,
                },
                has_gpu: true,
            }
        }
    }
}

#[cfg(test)]
pub mod fixtures {
    use crate::ai::model::CandleConfig;

    /// Get path to test model
    pub fn test_model_path() -> String {
        "test_fixtures/tiny-model.safetensors".to_string()
    }

    /// Create minimal test config
    pub fn test_config() -> CandleConfig {
        CandleConfig {
            model_path: test_model_path(),
            context_size: 256,
            temperature: 0.0,
            max_tokens: 32,
            top_p: 0.9,
            top_k: 40,
            repetition_penalty: 1.0,
            cpu_only: true,
            n_threads: Some(1),
        }
    }
}

#[cfg(test)]
pub mod assertions {
    /// Assert that a duration is within expected range
    #[macro_export]
    macro_rules! assert_duration {
        ($duration:expr, $min_ms:expr, $max_ms:expr) => {
            let ms = $duration.as_millis();
            assert!(
                ms >= $min_ms && ms <= $max_ms,
                "Duration {}ms not in range [{}ms, {}ms]",
                ms,
                $min_ms,
                $max_ms
            );
        };
    }

    /// Assert that a string looks like valid text (not empty, has content)
    pub fn assert_valid_text(text: &str, min_len: usize) {
        assert!(
            text.len() >= min_len,
            "Text too short: {} (min: {})",
            text.len(),
            min_len
        );
        assert!(
            !text.chars().all(|c| c.is_whitespace()),
            "Text is all whitespace"
        );
    }
}
