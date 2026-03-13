//! Auto-Tuner
//!
//! Automatically tunes model configuration based on hardware

use crate::ai::model::GGUFConfig;
use super::detector::HardwareInfo;

/// Auto-tuner for optimal model configuration
pub struct AutoTuner {
    hardware: HardwareInfo,
}

/// Tuning profile
#[derive(Debug, Clone)]
pub enum TuningProfile {
    /// Maximum speed, lower quality
    Speed,
    /// Balanced speed and quality
    Balanced,
    /// Maximum quality, slower
    Quality,
    /// Custom configuration
    Custom {
        context_size: usize,
        n_gpu_layers: usize,
        temperature: f32,
    },
}

impl AutoTuner {
    /// Create new auto-tuner with detected hardware
    pub fn new() -> Result<Self, String> {
        let hardware = HardwareInfo::detect()?;
        Ok(Self { hardware })
    }
    
    /// Create with existing hardware info
    pub fn with_hardware(hardware: HardwareInfo) -> Self {
        Self { hardware }
    }
    
    /// Get hardware info
    pub fn hardware(&self) -> &HardwareInfo {
        &self.hardware
    }
    
    /// Auto-tune configuration for a specific model
    pub fn tune(&self, model_params_b: f32, profile: TuningProfile) -> GGUFConfig {
        match profile {
            TuningProfile::Speed => self.tune_for_speed(model_params_b),
            TuningProfile::Balanced => self.tune_balanced(model_params_b),
            TuningProfile::Quality => self.tune_for_quality(model_params_b),
            TuningProfile::Custom {
                context_size,
                n_gpu_layers,
                temperature,
            } => GGUFConfig {
                model_path: String::new(),
                context_size,
                n_gpu_layers,
                temperature,
                max_tokens: 512,
            },
        }
    }
    
    /// Tune for maximum speed
    fn tune_for_speed(&self, model_params_b: f32) -> GGUFConfig {
        let n_gpu_layers = if self.hardware.has_gpu {
            self.hardware.recommended_gpu_layers(model_params_b)
        } else {
            0
        };
        
        GGUFConfig {
            model_path: String::new(),
            context_size: 1024,  // Small context for speed
            n_gpu_layers,
            temperature: 0.5,  // Lower temp for faster sampling
            max_tokens: 256,
        }
    }
    
    /// Tune for balanced performance
    fn tune_balanced(&self, model_params_b: f32) -> GGUFConfig {
        let n_gpu_layers = if self.hardware.has_gpu {
            self.hardware.recommended_gpu_layers(model_params_b)
        } else {
            0
        };
        
        GGUFConfig {
            model_path: String::new(),
            context_size: self.hardware.recommended_context(),
            n_gpu_layers,
            temperature: 0.7,  // Default temperature
            max_tokens: 512,
        }
    }
    
    /// Tune for maximum quality
    fn tune_for_quality(&self, model_params_b: f32) -> GGUFConfig {
        let n_gpu_layers = if self.hardware.has_gpu {
            self.hardware.recommended_gpu_layers(model_params_b)
        } else {
            0
        };
        
        GGUFConfig {
            model_path: String::new(),
            context_size: self.hardware.recommended_context().min(8192),
            n_gpu_layers,
            temperature: 0.8,  // Higher temp for creativity
            max_tokens: 1024,
        }
    }
    
    /// Get recommended model path based on hardware
    pub fn recommend_model(&self) -> &'static str {
        let max_params = self.hardware.recommended_model_size();
        
        if max_params >= 70.0 {
            "models/yi-34b.Q4_K_M.gguf"
        } else if max_params >= 34.0 {
            "models/mistral-nemo-12b.Q4_K_M.gguf"
        } else if max_params >= 13.0 {
            "models/mistral-7b.Q4_K_M.gguf"
        } else if max_params >= 7.0 {
            "models/llama-3-8b.Q4_K_M.gguf"
        } else if max_params >= 3.0 {
            "models/phi-3-mini.Q4_K_M.gguf"
        } else {
            "models/qwen3.5-0.8b.gguf"  // Ultra-light
        }
    }
    
    /// Print hardware summary
    pub fn print_summary(&self) {
        println!("\n=== Hardware Detection ===");
        println!("CPU: {} ({} cores, {} threads)", 
                 self.hardware.cpu.name, 
                 self.hardware.cpu.cores, 
                 self.hardware.cpu.threads);
        println!("RAM: {}GB total, {}GB available", 
                 self.hardware.memory.total, 
                 self.hardware.memory.available);
        
        if let Some(gpu) = &self.hardware.gpu {
            println!("GPU: {} ({} CUDA cores, {}MB VRAM)", 
                     gpu.name, gpu.cuda_cores, gpu.vram_total);
            println!("→ GPU acceleration: ENABLED");
        } else {
            println!("GPU: Not detected");
            println!("→ GPU acceleration: DISABLED");
        }
        
        println!("\n=== Recommendations ===");
        println!("Max model size: ~{}B parameters", 
                 self.hardware.recommended_model_size());
        println!("Recommended context: {}", 
                 self.hardware.recommended_context());
        println!("Recommended model: {}", 
                 self.recommend_model());
        println!("=========================\n");
    }
}

impl Default for AutoTuner {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            hardware: HardwareInfo {
                cpu: crate::hardware::detector::CpuInfo {
                    cores: 4,
                    threads: 8,
                    name: "Unknown CPU".to_string(),
                },
                gpu: None,
                memory: crate::hardware::detector::MemoryInfo {
                    total: 8,
                    available: 4,
                },
                has_gpu: false,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auto_tuner() {
        let tuner = AutoTuner::default();
        tuner.print_summary();
        
        let config = tuner.tune(0.8, TuningProfile::Balanced);
        println!("Config for 0.8B model: {:?}", config);
    }
}
