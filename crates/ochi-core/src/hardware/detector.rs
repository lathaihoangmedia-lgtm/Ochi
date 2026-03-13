//! Hardware Detection
//!
//! Detects CPU, GPU, RAM, and VRAM information

use serde::{Deserialize, Serialize};

/// CPU Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub cores: usize,
    pub threads: usize,
    pub name: String,
}

/// GPU Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vram_total: usize,  // MB
    pub vram_available: usize,  // MB
    pub cuda_cores: usize,
    pub supports_cuda: bool,
}

/// System Memory Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: usize,  // GB
    pub available: usize,  // GB
}

/// Complete Hardware Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: CpuInfo,
    pub gpu: Option<GpuInfo>,
    pub memory: MemoryInfo,
    pub has_gpu: bool,
}

impl HardwareInfo {
    /// Detect current system hardware
    pub fn detect() -> Result<Self, String> {
        let cpu = detect_cpu();
        let memory = detect_memory();
        let gpu = detect_gpu();
        
        Ok(Self {
            cpu,
            gpu: gpu.clone(),
            memory,
            has_gpu: gpu.is_some(),
        })
    }
    
    /// Get recommended GPU layers for model offloading
    pub fn recommended_gpu_layers(&self, model_params_b: f32) -> usize {
        if let Some(gpu) = &self.gpu {
            // Estimate VRAM needed per billion parameters (Q4_K_M quant)
            let vram_per_b: f32 = 0.7;  // ~0.7GB per 1B params at Q4
            
            // Calculate how many layers we can fit
            let model_vram_needed = model_params_b * vram_per_b;
            let usable_vram = gpu.vram_available as f32 * 0.85;  // Use 85% of available VRAM
            
            if usable_vram >= model_vram_needed {
                // Can fit entire model
                999  // Max layers (will be clamped by model)
            } else {
                // Partial offload
                let ratio = usable_vram / model_vram_needed;
                (ratio * 100.0) as usize  // Percentage of layers
            }
        } else {
            0  // CPU-only
        }
    }
    
    /// Get recommended context size
    pub fn recommended_context(&self) -> usize {
        let ram_gb = self.memory.available as f32;
        
        if ram_gb > 16.0 {
            8192
        } else if ram_gb > 8.0 {
            4096
        } else {
            2048
        }
    }
    
    /// Get recommended model size (in billions of parameters)
    pub fn recommended_model_size(&self) -> f32 {
        let ram_gb = self.memory.available as f32;
        
        if self.has_gpu {
            // GPU + RAM combo
            if ram_gb > 24.0 {
                70.0  // Can run 70B with offload
            } else if ram_gb > 16.0 {
                34.0  // Can run 34B
            } else {
                13.0  // Safe for 13B
            }
        } else {
            // CPU-only
            if ram_gb > 24.0 {
                20.0
            } else if ram_gb > 12.0 {
                7.0
            } else {
                3.0
            }
        }
    }
}

/// Detect CPU information
fn detect_cpu() -> CpuInfo {
    let cores = num_cpus::get_physical();
    let threads = num_cpus::get();
    let name = get_cpu_name();
    
    CpuInfo {
        cores,
        threads,
        name,
    }
}

/// Detect system memory
fn detect_memory() -> MemoryInfo {
    let total = sysinfo::System::new_all().total_memory() / (1024 * 1024 * 1024);
    let available = sysinfo::System::new_all().available_memory() / (1024 * 1024 * 1024);
    
    MemoryInfo {
        total: total as usize,
        available: available as usize,
    }
}

/// Detect GPU information (NVIDIA via NVML)
fn detect_gpu() -> Option<GpuInfo> {
    #[cfg(feature = "cuda")]
    {
        use nvml_wrapper::Nvml;
        
        if let Ok(nvml) = Nvml::init() {
            if let Ok(device) = nvml.device_by_index(0) {
                let name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
                let memory_info = device.memory_info().ok()?;
                
                let vram_total = (memory_info.total / (1024 * 1024)) as usize;
                let vram_available = (memory_info.free / (1024 * 1024)) as usize;
                
                // Get CUDA cores (approximate based on GPU name)
                let cuda_cores = estimate_cuda_cores(&name);
                
                return Some(GpuInfo {
                    name,
                    vram_total,
                    vram_available,
                    cuda_cores,
                    supports_cuda: true,
                });
            }
        }
    }
    
    None
}

/// Get CPU name from registry (Windows) or /proc/cpuinfo (Linux)
fn get_cpu_name() -> String {
    #[cfg(target_os = "windows")]
    {
        // Try to get from WMI or registry
        // Fallback to generic name
        format!("CPU ({} cores)", num_cpus::get_physical())
    }
    
    #[cfg(target_os = "linux")]
    {
        // Read from /proc/cpuinfo
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("model name") {
                    if let Some(name) = line.split(':').nth(1) {
                        return name.trim().to_string();
                    }
                }
            }
        }
        format!("CPU ({} cores)", num_cpus::get_physical())
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        format!("CPU ({} cores)", num_cpus::get_physical())
    }
}

/// Estimate CUDA cores based on GPU name
fn estimate_cuda_cores(gpu_name: &str) -> usize {
    // Common NVIDIA GPUs
    if gpu_name.contains("GTX 1050") {
        768
    } else if gpu_name.contains("GTX 1060") {
        1280
    } else if gpu_name.contains("GTX 1070") {
        1920
    } else if gpu_name.contains("GTX 1080") {
        2560
    } else if gpu_name.contains("RTX 2060") {
        1920
    } else if gpu_name.contains("RTX 2070") {
        2304
    } else if gpu_name.contains("RTX 2080") {
        2944
    } else if gpu_name.contains("RTX 3060") {
        3584
    } else if gpu_name.contains("RTX 3070") {
        5888
    } else if gpu_name.contains("RTX 3080") {
        8704
    } else if gpu_name.contains("RTX 3090") {
        10496
    } else if gpu_name.contains("RTX 4060") {
        3072
    } else if gpu_name.contains("RTX 4070") {
        5888
    } else if gpu_name.contains("RTX 4080") {
        9728
    } else if gpu_name.contains("RTX 4090") {
        16384
    } else {
        0  // Unknown or integrated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_hardware() {
        let info = HardwareInfo::detect().unwrap();
        println!("CPU: {} ({} cores, {} threads)", 
                 info.cpu.name, info.cpu.cores, info.cpu.threads);
        println!("Memory: {}GB total, {}GB available", 
                 info.memory.total, info.memory.available);
        
        if let Some(gpu) = &info.gpu {
            println!("GPU: {} ({} CUDA cores, {}MB VRAM)", 
                     gpu.name, gpu.cuda_cores, gpu.vram_total);
        } else {
            println!("No GPU detected");
        }
    }
}
