//! Candle Demo - Run inference with local GGUF model
//! 
//! Usage: cargo run --example candle-demo -p ochi-llm --features ollama

use ochi_core::{HardwareInfo, Result};
use ochi_llm::{CandleModel, CandleConfig, AutoConfigurator};

fn main() -> Result<()> {
    println!("=== Ochi Candle Demo ===\n");

    // 1. Detect hardware
    let hardware = HardwareInfo::detect()
        .unwrap_or_else(|_| HardwareInfo::mock_mid_range());
    
    println!("Hardware:");
    println!("  CPU: {} ({} cores, {} threads)", 
        hardware.cpu.name, hardware.cpu.cores, hardware.cpu.threads);
    println!("  RAM: {}GB total, {}GB available", 
        hardware.memory.total, hardware.memory.available);
    if hardware.has_gpu {
        if let Some(gpu) = &hardware.gpu {
            println!("  GPU: {} ({}MB VRAM)", gpu.name, gpu.vram_total);
        }
    }
    println!();

    // 2. Configure model
    let model_path = "models/qwen3.5-0.8b.gguf";
    println!("Model: {}", model_path);

    // Check if file exists
    if !std::path::Path::new(model_path).exists() {
        eprintln!("ERROR: Model file not found!");
        eprintln!("Please ensure {} exists", model_path);
        return Ok(());
    }

    // 3. Auto-configure based on hardware
    let auto_config = AutoConfigurator::new();
    let config_result = auto_config.auto_configure(model_path);
    
    println!("\nAuto-Configuration:");
    println!("  Context size: {}", config_result.config.context_size);
    println!("  Temperature: {}", config_result.config.temperature);
    println!("  CPU threads: {:?}", config_result.config.n_threads);
    println!("  CPU only: {}", config_result.config.cpu_only);
    println!();

    // 4. Load model (scaffold - full implementation in Phase 2)
    println!("Model loading (scaffold)...");
    println!("  Format: GGUF");
    println!("  Size: ~0.8B parameters");
    println!("  Expected speed: ~50-80 tok/s (CPU)");
    println!();

    // 5. Demo complete
    println!("✅ Demo complete!");
    println!();
    println!("Next steps:");
    println!("  1. Implement full GGUF loading in CandleModel::load()");
    println!("  2. Add tokenizer integration");
    println!("  3. Connect to ochi-runtime for agent inference");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_path_exists() {
        let path = std::path::Path::new("models/qwen3.5-0.8b.gguf");
        assert!(path.exists(), "Model file should exist");
    }

    #[test]
    fn test_hardware_detection() {
        let hardware = HardwareInfo::detect().unwrap();
        assert!(hardware.cpu.cores > 0);
        assert!(hardware.memory.total > 0);
    }
}
