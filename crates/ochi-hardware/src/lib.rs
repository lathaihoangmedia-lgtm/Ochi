//! Hardware Module - Detection & Auto-Tuning
//!
//! Detects CPU, GPU, RAM and provides optimal model configurations
//! Uses sysinfo for cross-platform support (no NVML required)

pub mod detector;

pub use detector::HardwareInfo;

/// Auto-tuner for optimal model configuration
pub struct AutoTuner {
    hardware: HardwareInfo,
}

impl AutoTuner {
    pub fn new() -> Self {
        let hardware = HardwareInfo::detect().unwrap_or_else(|_| {
            // Fallback if detection fails
            HardwareInfo {
                cpu: detector::CpuInfo {
                    cores: 4,
                    threads: 8,
                    name: "Unknown CPU".to_string(),
                },
                gpu: None,
                memory: detector::MemoryInfo {
                    total: 16,
                    available: 8,
                },
                has_gpu: false,
            }
        });
        
        Self { hardware }
    }

    /// Get recommended configuration for model
    pub fn recommend(&self, model_size_b: f32) -> ModelConfig {
        let gpu_layers = self.hardware.recommended_gpu_layers(model_size_b);
        let context_size = self.hardware.recommended_context();
        
        ModelConfig {
            gpu_layers,
            context_size,
            n_threads: Some(self.hardware.cpu.threads),
        }
    }

    /// Get hardware info
    pub fn hardware(&self) -> &HardwareInfo {
        &self.hardware
    }
}

impl Default for AutoTuner {
    fn default() -> Self {
        Self::new()
    }
}

/// Recommended model configuration
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub gpu_layers: usize,
    pub context_size: usize,
    pub n_threads: Option<usize>,
}
