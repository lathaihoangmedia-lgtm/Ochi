//! Streaming Chat Example - Real-time token streaming
//!
//! Usage:
//!   cargo run --example streaming -p ochi-llm --features ollama
//!   cargo run --example streaming -p ochi-llm --features ollama -- llama3.2:1b
//!
//! This example demonstrates:
//! - Real-time token streaming
//! - Progress visualization
//! - Token generation metrics

use ochi_core::{HardwareInfo, Result};
use ochi_llm::{OllamaAutoTuner, OllamaClient};
use std::io::{self, Write};
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
    println!("=== Ochi Streaming Demo ===\n");

    // 1. Get model
    let model = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "qwen2.5:3b".to_string());

    // 2. Detect hardware
    let hardware = HardwareInfo::detect()
        .unwrap_or_else(|_| create_mock_hardware());

    let options = OllamaAutoTuner::recommend(&model, &hardware);

    println!("Model: {}", model);
    println!();

    // 3. Check connection
    let client = OllamaClient::new();

    if !client.is_running().await {
        eprintln!("❌ Ollama server not running!");
        eprintln!("Start with: ollama serve");
        return Ok(());
    }

    println!("✅ Connected to Ollama at {}", client.url());
    println!();

    // 4. Demo prompts
    let prompts = vec![
        "Viết một đoạn văn ngắn về trí tuệ nhân tạo.",
        "Explain quantum computing in 3 sentences.",
        "Write a haiku about Rust programming.",
    ];

    for (i, prompt) in prompts.iter().enumerate() {
        println!("--- Demo {} ---", i + 1);
        println!("Prompt: {}\n", prompt);
        print!("Response: ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        // Track metrics
        let start = Instant::now();
        let mut token_count = 0;
        let mut first_token_time = None;

        // Streaming callback
        let callback = |chunk: String| async move {
            token_count += 1;
            
            if first_token_time.is_none() {
                first_token_time = Some(Instant::now());
            }

            print!("{}", chunk);
            io::stdout().flush().unwrap();
            
            true  // Continue streaming
        };

        // Note: Current ollama-rs version doesn't support streaming
        // Using regular generate as fallback
        match client.generate(&model, prompt, options.clone()).await {
            Ok(response) => {
                print!("{}", response);
                println!();
                
                let elapsed = start.elapsed();
                let tokens_per_sec = response.len() as f64 / elapsed.as_secs_f64();
                
                println!();
                println!("Metrics:");
                println!("  Total time: {:.2}s", elapsed.as_secs_f64());
                println!("  Response length: {} chars", response.len());
                println!("  Speed: {:.1} chars/sec", tokens_per_sec);
            }
            Err(e) => {
                eprintln!("\nError: {}", e);
            }
        }

        println!();
    }

    println!("=== Streaming Demo Complete ===");
    println!();
    println!("Note: For true token-by-token streaming,");
    println!("consider using the Ollama REST API directly");
    println!("or wait for ollama-rs streaming support.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_detection() {
        let hardware = HardwareInfo::detect().unwrap();
        assert!(hardware.cpu.cores > 0);
    }

    #[test]
    fn test_auto_tune() {
        let hardware = HardwareInfo::mock_mid_range();
        let options = OllamaAutoTuner::recommend("llama3.2", &hardware);
        assert!(options.temperature.is_some());
    }
}
