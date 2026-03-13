//! Model Auto-Configuration
//!
//! Automatically configures models based on hardware and model characteristics

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::ai::model::CandleConfig;
use crate::hardware::HardwareInfo;

/// Auto-configuration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoConfigResult {
    pub model_name: String,
    pub model_path: String,
    pub config: CandleConfig,
    pub recommendations: Vec<String>,
    pub performance_estimate: PerformanceEstimate,
}

/// Performance estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEstimate {
    pub estimated_speed_tps: f32,
    pub estimated_vram_mb: u32,
    pub estimated_ram_mb: u32,
    pub quality_rating: u8,
}

/// Model auto-configurator
pub struct AutoConfigurator {
    hardware: HardwareInfo,
}

impl AutoConfigurator {
    /// Create new auto-configurator
    pub fn new(hardware: HardwareInfo) -> Self {
        Self { hardware }
    }

    /// Auto-configure model with optimal settings
    pub fn auto_configure<P: AsRef<Path>>(&self, model_path: P) -> AutoConfigResult {
        let path = model_path.as_ref();
        let model_name = self.extract_model_name(path);

        let mut recommendations = Vec::new();

        // Start with balanced config
        let mut config = CandleConfig::balanced(path.to_string_lossy().to_string());

        // Apply hardware optimizations
        self.apply_hardware_optimizations(&mut config, &mut recommendations);

        // Model-specific recommendations
        self.add_model_recommendations(&model_name, &mut recommendations);

        // Estimate performance
        let perf = self.estimate_performance(&model_name, &config);

        AutoConfigResult {
            model_name,
            model_path: path.to_string_lossy().to_string(),
            config,
            recommendations,
            performance_estimate: perf,
        }
    }

    /// Extract model name from path
    fn extract_model_name(&self, path: &Path) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Apply hardware-based optimizations
    fn apply_hardware_optimizations(
        &self,
        config: &mut CandleConfig,
        recommendations: &mut Vec<String>,
    ) {
        // Adjust context size based on RAM
        let ram_gb = self.hardware.memory.available as f32;
        
        if ram_gb < 8.0 {
            config.context_size = 2048;
            recommendations.push("Limited RAM detected: reduced context size to 2048".to_string());
        } else if ram_gb > 16.0 {
            config.context_size = 8192;
            recommendations.push("Ample RAM available: increased context size to 8192".to_string());
        }

        // CPU threads
        config.n_threads = Some(self.hardware.cpu.threads);
        recommendations.push(format!(
            "Using {} CPU threads for inference",
            self.hardware.cpu.threads
        ));
    }

    /// Add model-specific recommendations
    fn add_model_recommendations(
        &self,
        model_name: &str,
        recommendations: &mut Vec<String>,
    ) {
        // Qwen models
        if model_name.contains("qwen") {
            recommendations.push(
                "Qwen models work best with temperature 0.7-0.8 for balanced output".to_string()
            );
        }

        // Llama models
        if model_name.contains("llama") {
            recommendations.push(
                "Llama models: use repetition penalty 1.1 to prevent loops".to_string()
            );
        }

        // Small models (< 3B)
        if model_name.contains("0.5b") || model_name.contains("1b") || model_name.contains("3b") {
            recommendations.push(
                "Small model: increase temperature (0.8+) for more creative output".to_string()
            );
        }
    }

    /// Estimate performance
    fn estimate_performance(&self, model_name: &str, config: &CandleConfig) -> PerformanceEstimate {
        // Estimate model size from name
        let model_size_b = self.estimate_model_size(model_name);
        
        // Candle performance estimates (CPU inference)
        // Rough estimate: 1-5 tokens/s for CPU, depends on model size
        let base_speed = if config.cpu_only {
            2.0  // CPU tokens/s
        } else {
            20.0  // GPU tokens/s (if CUDA available)
        };
        
        // Speed decreases with model size
        let speed = base_speed / model_size_b;
        
        // VRAM/RAM estimates (Q4_K_M quantization)
        let vram_per_b: f32 = 700.0;  // ~0.7GB per 1B params
        let estimated_vram = (model_size_b * vram_per_b) as u32;
        let estimated_ram = estimated_vram * 2;  // System RAM for context

        PerformanceEstimate {
            estimated_speed_tps: speed,
            estimated_vram_mb: estimated_vram,
            estimated_ram_mb: estimated_ram,
            quality_rating: if model_size_b >= 7.0 { 8 } else { 6 },
        }
    }

    /// Estimate model size from name (in billions of parameters)
    fn estimate_model_size(&self, model_name: &str) -> f32 {
        if model_name.contains("70b") || model_name.contains("70b") {
            70.0
        } else if model_name.contains("34b") {
            34.0
        } else if model_name.contains("13b") {
            13.0
        } else if model_name.contains("7b") {
            7.0
        } else if model_name.contains("3b") {
            3.0
        } else if model_name.contains("1b") {
            1.0
        } else if model_name.contains("0.5b") {
            0.5
        } else {
            3.0  // Default assumption
        }
    }
}
