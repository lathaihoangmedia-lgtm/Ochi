//! Multi-Model Comparison - Compare outputs from different models
//!
//! Usage:
//!   cargo run --example multi-model -p ochi-llm --features ollama
//!   cargo run --example multi-model -p ochi-llm --features ollama -- "Your prompt here"
//!
//! This example demonstrates:
//! - Running the same prompt across multiple models
//! - Comparing output quality and speed
//! - Model selection recommendations

use ochi_core::{HardwareInfo, Result};
use ochi_llm::{OllamaAutoTuner, OllamaClient};
use std::time::Instant;

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

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Ochi Multi-Model Comparison ===\n");

    // 1. Detect hardware
    let hardware = HardwareInfo::detect()
        .unwrap_or_else(|_| create_mock_hardware());

    println!("Hardware Profile:");
    println!("  CPU: {} ({} threads)", hardware.cpu.name, hardware.cpu.threads);
    println!("  RAM: {}GB available", hardware.memory.available);
    if hardware.has_gpu {
        println!("  GPU: Available");
    }
    println!();

    // 2. Check Ollama connection
    let client = OllamaClient::new();

    if !client.is_running().await {
        eprintln!("❌ Ollama server not running!");
        eprintln!("Start with: ollama serve");
        return Ok(());
    }

    println!("✅ Connected to Ollama at {}", client.url());
    println!();

    // 3. Get prompt from args or use default
    let prompt = std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");

    let prompt = if prompt.is_empty() {
        "Giải thích ngắn gọn về lợi ích của trí tuệ nhân tạo trong cuộc sống hàng ngày."
    } else {
        &prompt
    };

    println!("Prompt: {}\n", prompt);

    // 4. Models to compare
    let models = vec![
        ("qwen2.5:1b", "Small - Fast"),
        ("qwen2.5:3b", "Medium - Balanced"),
        ("llama3.2:1b", "Small - Meta"),
        ("phi3:mini", "Medium - Microsoft"),
    ];

    println!("Comparing {} models...\n", models.len());

    // 5. Run comparison
    let mut results = Vec::new();

    for (model, description) in &models {
        println!("Testing {} ({})...", model, description);

        let options = OllamaAutoTuner::recommend(model, &hardware);
        let start = Instant::now();

        match client.generate(model, prompt, options).await {
            Ok(response) => {
                let elapsed = start.elapsed();
                let speed = response.len() as f64 / elapsed.as_secs_f64();

                println!("  ✅ Success: {} chars in {:.2}s ({:.1} chars/s)", 
                    response.len(), elapsed.as_secs_f64(), speed);

                results.push(ModelResult {
                    model: model.to_string(),
                    description: description.to_string(),
                    response,
                    time: elapsed,
                    speed,
                });
            }
            Err(e) => {
                println!("  ❌ Error: {}", e);
                println!("  Hint: ollama pull {}", model);
            }
        }
        println!();
    }

    // 6. Show comparison table
    if !results.is_empty() {
        println!("=== Comparison Results ===\n");
        println!("{:<20} {:<12} {:<10} {:<12}", 
            "Model", "Chars", "Time (s)", "Speed (c/s)");
        println!("{}", "-".repeat(54));

        for result in &results {
            println!("{:<20} {:<12} {:<10.2} {:<12.1}", 
                format!("{} ({})", result.model, result.description),
                result.response.len(),
                result.time.as_secs_f64(),
                result.speed);
        }
        println!();

        // 7. Find best model
        if let Some(fastest) = results.iter().max_by(|a, b| {
            a.speed.partial_cmp(&b.speed).unwrap()
        }) {
            println!("🏆 Fastest: {} ({:.1} chars/s)", fastest.model, fastest.speed);
        }

        if let Some(longest) = results.iter().max_by(|a, b| {
            a.response.len().cmp(&b.response.len())
        }) {
            println!("📝 Most detailed: {} ({} chars)", longest.model, longest.response.len());
        }
    }

    println!();
    println!("=== Comparison Complete ===");
    println!();
    println!("Recommendation:");
    recommend_model(&hardware, &results);

    Ok(())
}

struct ModelResult {
    model: String,
    description: String,
    response: String,
    time: std::time::Duration,
    speed: f64,
}

/// Recommend best model based on hardware and results
fn recommend_model(hardware: &HardwareInfo, results: &[ModelResult]) {
    let ram_gb = hardware.memory.available as f32;

    if results.is_empty() {
        println!("  No models tested. Install models with: ollama pull <model>");
        return;
    }

    // Find fastest working model
    let fastest = results.iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();

    if ram_gb < 8.0 {
        println!("  For your hardware ({}GB RAM):", ram_gb as u32);
        println!("  Use small models: llama3.2:1b, qwen2.5:1b, phi3:mini");
    } else if ram_gb < 16.0 {
        println!("  For your hardware ({}GB RAM):", ram_gb as u32);
        println!("  Use medium models: qwen2.5:3b, llama3.2:3b, phi3:mini");
    } else {
        println!("  For your hardware ({}GB RAM):", ram_gb as u32);
        println!("  Can run larger models: llama3.2:7b, mistral:7b, qwen2.5:7b");
    }

    println!();
    println!("  Best tested: {} ({:.1} chars/s)", fastest.model, fastest.speed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_result_structure() {
        let result = ModelResult {
            model: "test".to_string(),
            description: "test".to_string(),
            response: "hello".to_string(),
            time: std::time::Duration::from_secs(1),
            speed: 5.0,
        };

        assert_eq!(result.response.len(), 5);
        assert_eq!(result.speed, 5.0);
    }

    #[test]
    fn test_auto_tune_different_models() {
        let hardware = HardwareInfo::mock_mid_range();
        
        let small = OllamaAutoTuner::recommend("llama3.2:1b", &hardware);
        let medium = OllamaAutoTuner::recommend("qwen2.5:3b", &hardware);
        let large = OllamaAutoTuner::recommend("mistral:7b", &hardware);

        // Small models should have higher temperature
        assert!(small.temperature.unwrap() >= medium.temperature.unwrap());
    }
}
