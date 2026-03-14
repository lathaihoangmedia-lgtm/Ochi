//! Ollama Auto-Tune Demo - Automatically optimize model parameters
//!
//! Usage:
//!   cargo run --example ollama_autotune -p ochi-llm --features ollama
//!   cargo run --example ollama_autotune -p ochi-llm --features ollama -- llama3.2:1b
//!   OLLAMA_CALL=1 cargo run --example ollama_autotune -p ochi-llm --features ollama
//!
//! This example demonstrates:
//! - Hardware-aware parameter tuning
//! - Model-specific optimizations
//! - Real-time generation with auto-tuned settings

use ochi_core::{HardwareInfo, Result};
use ochi_llm::{OllamaAutoTuner, OllamaOptions, OllamaClient};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Ochi Ollama Auto-Tune Demo ===\n");

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

    // 2. Get model from args or use default
    let model = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "qwen2.5:3b".to_string());

    println!("Model: {}", model);

    // 3. Check if Ollama is running
    let client = OllamaClient::new();
    
    match client.is_running().await {
        true => println!("✅ Ollama server detected at {}", client.url()),
        false => {
            eprintln!("⚠️  Ollama server not running!");
            eprintln!("Start Ollama: ollama serve");
            eprintln!("Or install: https://ollama.ai");
            eprintln!();
            eprintln!("Continuing with parameter recommendations only...");
        }
    }
    println!();

    // 4. Auto-tune parameters
    let options: OllamaOptions = OllamaAutoTuner::recommend(&model, &hardware);

    println!("Auto-Tuned Parameters:");
    println!("  temperature:      {:?}", options.temperature);
    println!("  top_p:            {:?}", options.top_p);
    println!("  top_k:            {:?}", options.top_k);
    println!("  repeat_penalty:   {:?}", options.repeat_penalty);
    println!("  num_predict:      {:?}", options.num_predict);
    println!();

    // 5. Explain tuning decisions
    println!("Tuning Rationale:");
    explain_tuning(&model, &options, &hardware);
    println!();

    // 6. Run generation if Ollama is available
    if client.is_running().await {
        println!("Running generation test...");
        println!();

        let prompt = "Xin chào! Hãy giới thiệu ngắn gọn về bản thân bạn.";
        println!("Prompt: {}", prompt);
        println!();
        println!("Response:");

        match client.generate(&model, prompt, options).await {
            Ok(response) => {
                println!("{}", response);
                println!();
                println!("✅ Generation successful!");
            }
            Err(e) => {
                eprintln!("❌ Generation failed: {}", e);
                eprintln!();
                eprintln!("Troubleshooting:");
                eprintln!("  1. Pull the model: ollama pull {}", model);
                eprintln!("  2. Check Ollama logs: ollama serve");
            }
        }
    } else {
        println!("Set OLLAMA_CALL=1 environment variable to run generation test.");
        println!("Example: OLLAMA_CALL=1 cargo run --example ollama_autotune -p ochi-llm --features ollama");
    }

    println!();
    println!("=== Demo Complete ===");
    println!();
    println!("Recommended models for your hardware:");
    for rec_model in OllamaClient::recommended_models() {
        println!("  • {}", rec_model);
    }

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

/// Explain the tuning decisions
fn explain_tuning(model: &str, options: &OllamaOptions, hardware: &HardwareInfo) {
    let size_b = estimate_model_size(model);

    // Temperature explanation
    if let Some(temp) = options.temperature {
        if temp >= 0.8 {
            println!("  • High temperature ({:.2}): More creative, diverse output", temp);
        } else if temp <= 0.6 {
            println!("  • Low temperature ({:.2}): More focused, deterministic output", temp);
        } else {
            println!("  • Balanced temperature ({:.2}): Good mix of creativity and coherence", temp);
        }
    }

    // Model size recommendations
    if size_b <= 1.0 {
        println!("  • Small model detected: Reduced num_predict for faster response");
        println!("  • Small model detected: Increased temperature for more creative output");
    } else if size_b >= 7.0 {
        println!("  • Large model detected: Reduced temperature for better coherence");
        println!("  • Large model detected: Moderate num_predict to balance quality/speed");
    }

    // RAM-aware adjustments
    let ram_gb = hardware.memory.available as f32;
    if ram_gb < 8.0 {
        println!("  • Limited RAM: Reduced context and output length");
    } else if ram_gb >= 16.0 {
        println!("  • Ample RAM: Can handle longer contexts and outputs");
    }

    // GPU presence
    if hardware.has_gpu {
        println!("  • GPU detected: Can tolerate higher token counts");
    } else {
        println!("  • CPU-only mode: Optimized for CPU inference speed");
    }
}

/// Estimate model size from name (in billions of parameters)
fn estimate_model_size(model_name: &str) -> f32 {
    let name = model_name.to_lowercase();

    if name.contains("70b") {
        70.0
    } else if name.contains("34b") {
        34.0
    } else if name.contains("13b") {
        13.0
    } else if name.contains("8b") {
        8.0
    } else if name.contains("7b") {
        7.0
    } else if name.contains("3b") {
        3.0
    } else if name.contains("2b") {
        2.0
    } else if name.contains("1b") {
        1.0
    } else if name.contains("0.8b") || name.contains("0.5b") {
        0.8
    } else {
        3.0  // Default assumption
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_model_size() {
        assert_eq!(estimate_model_size("llama3.2:1b"), 1.0);
        assert_eq!(estimate_model_size("qwen2.5:3b"), 3.0);
        assert_eq!(estimate_model_size("mistral:7b"), 7.0);
        assert_eq!(estimate_model_size("unknown"), 3.0);
    }

    #[test]
    fn test_auto_tuner_recommends() {
        let hardware = HardwareInfo::mock_mid_range();
        let options = OllamaAutoTuner::recommend("llama3.2:1b", &hardware);
        
        assert!(options.temperature.is_some());
        assert!(options.top_p.is_some());
        assert!(options.repeat_penalty.is_some());
    }

    #[test]
    fn test_options_builder() {
        let options = OllamaOptions::new()
            .with_temperature(0.7)
            .with_top_p(0.9)
            .with_top_k(40)
            .with_repeat_penalty(1.1)
            .with_num_predict(512);

        assert_eq!(options.temperature, Some(0.7));
        assert_eq!(options.top_p, Some(0.9));
        assert_eq!(options.top_k, Some(40));
        assert_eq!(options.repeat_penalty, Some(1.1));
        assert_eq!(options.num_predict, Some(512));
    }
}
