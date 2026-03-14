//! Candle Demo - Run inference with local GGUF model
//!
//! Usage: cargo run --example candle-demo -p ochi-llm
//!
//! This example demonstrates:
//! - Hardware detection (CPU, GPU, RAM)
//! - Auto-configuration based on hardware
//! - Model loading with Candle
//! - Text generation with configurable parameters

use ochi_core::{HardwareInfo, Result};
use ochi_llm::{CandleModel, AutoConfigurator};
use ochi_llm::model::TextGenerator;

fn main() -> Result<()> {
    println!("=== Ochi Candle Demo ===\n");

    // 1. Detect hardware
    let hardware = HardwareInfo::detect()
        .unwrap_or_else(|_| create_mock_hardware());

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
        eprintln!("⚠️  Model file not found!");
        eprintln!("Please ensure {} exists", model_path);
        eprintln!();
        eprintln!("To download a model:");
        eprintln!("  curl -L -o {} https://huggingface.co/bartowski/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf", model_path);
        return Ok(());
    }

    // 3. Auto-configure based on hardware
    let auto_config = AutoConfigurator::new(hardware.clone());
    let config_result = auto_config.auto_configure(model_path);

    println!("\nAuto-Configuration:");
    println!("  Context size: {}", config_result.config.context_size);
    println!("  Temperature: {}", config_result.config.temperature);
    println!("  CPU threads: {:?}", config_result.config.n_threads);
    println!("  CPU only: {}", config_result.config.cpu_only);
    
    println!("\nRecommendations:");
    for rec in &config_result.recommendations {
        println!("  • {}", rec);
    }

    println!("\nPerformance Estimate:");
    println!("  Speed: ~{:.1} tokens/s", config_result.performance_estimate.estimated_speed_tps);
    println!("  VRAM: ~{}MB", config_result.performance_estimate.estimated_vram_mb);
    println!("  RAM: ~{}MB", config_result.performance_estimate.estimated_ram_mb);
    println!("  Quality: {}/10", config_result.performance_estimate.quality_rating);
    println!();

    // 4. Load model
    println!("Loading model...");
    let config = config_result.config.clone()
        .with_device(true)  // CPU only for compatibility
        .with_temperature(0.7);

    match CandleModel::load(model_path, config) {
        Ok(model) => {
            println!("✅ Model loaded successfully!");
            println!("  Device: {:?}", model.info().device);
            println!("  Context: {} tokens", model.info().context_size);
            println!();

            // 5. Run inference demo
            let generator = TextGenerator::new(model);
            
            let prompts = vec![
                "Xin chào! Tôi là trợ lý AI của Ochi.",
                "Giải thích ngắn gọn về Rust programming.",
            ];

            for (i, prompt) in prompts.iter().enumerate() {
                println!("--- Prompt {} ---", i + 1);
                println!("Input: {}", prompt);
                
                match generator.generate_with_penalty(prompt) {
                    Ok(output) => {
                        println!("Output: {}", output);
                    }
                    Err(e) => {
                        eprintln!("Generation error: {}", e);
                    }
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to load model: {}", e);
            eprintln!();
            eprintln!("Note: Full GGUF loading requires candle-transformers setup.");
            eprintln!("For now, try the Ollama example: cargo run --example ollama_autotune -p ochi-llm --features ollama");
        }
    }

    // 6. Demo complete
    println!("✅ Demo complete!");
    println!();
    println!("Next steps:");
    println!("  • Try ollama_autotune example for easier setup");
    println!("  • Download models: ollama pull llama3.2");
    println!("  • See README.md for advanced configuration");

    Ok(())
}

/// Create mock hardware for fallback
fn create_mock_hardware() -> HardwareInfo {
    HardwareInfo {
        cpu: ochi_core::hardware::detector::CpuInfo {
            cores: 4,
            threads: 8,
            name: "Mock CPU".to_string(),
        },
        gpu: None,
        memory: ochi_core::hardware::detector::MemoryInfo {
            total: 16,
            available: 8,
        },
        has_gpu: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_detection() {
        let hardware = HardwareInfo::detect().unwrap();
        assert!(hardware.cpu.cores > 0);
        assert!(hardware.memory.total > 0);
    }

    #[test]
    fn test_auto_config() {
        let hardware = HardwareInfo::mock_mid_range();
        let configurator = AutoConfigurator::new(hardware);
        let result = configurator.auto_configure("models/test.gguf");
        
        assert!(result.config.context_size > 0);
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_candle_config_default() {
        let config = CandleConfig::default();
        assert_eq!(config.context_size, 2048);
        assert!(config.cpu_only);
    }

    #[test]
    fn test_candle_config_presets() {
        let speed = CandleConfig::speed("test.gguf");
        let balanced = CandleConfig::balanced("test.gguf");
        let quality = CandleConfig::quality("test.gguf");

        assert!(speed.context_size <= balanced.context_size);
        assert!(balanced.context_size <= quality.context_size);
    }
}
