//! Basic Chat Example - Interactive chat with Ollama models
//!
//! Usage:
//!   cargo run --example basic-chat -p ochi-llm --features ollama
//!   cargo run --example basic-chat -p ochi-llm --features ollama -- llama3.2:1b
//!
//! This example demonstrates:
//! - Interactive chat loop
//! - Conversation history management
//! - Auto-tuned parameters for chat

use ochi_core::{HardwareInfo, Result};
use ochi_llm::{OllamaAutoTuner, OllamaClient};

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
    println!("=== Ochi Basic Chat ===\n");

    // 1. Get model from args
    let model = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "qwen2.5:3b".to_string());

    // 2. Detect hardware and auto-tune
    let hardware = HardwareInfo::detect()
        .unwrap_or_else(|_| create_mock_hardware());

    let options = OllamaAutoTuner::recommend(&model, &hardware);

    println!("Model: {}", model);
    println!("Using auto-tuned parameters for optimal performance");
    println!();

    // 3. Check Ollama connection
    let client = OllamaClient::new();

    if !client.is_running().await {
        eprintln!("❌ Ollama server not running!");
        eprintln!("Start with: ollama serve");
        return Ok(());
    }

    println!("✅ Connected to Ollama at {}", client.url());
    println!("Type 'quit' or 'exit' to end conversation");
    println!("Type 'clear' to clear conversation history");
    println!();

    // 4. Chat loop
    let mut conversation_history = Vec::new();
    let system_prompt = "You are a helpful AI assistant. Respond in Vietnamese when appropriate.";
    conversation_history.push(system_prompt.to_string());

    loop {
        print!("You: ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return Ok(());
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("\nGoodbye! 👋");
                return Ok(());
            }
            "clear" => {
                conversation_history.clear();
                conversation_history.push(system_prompt.to_string());
                println!("Conversation cleared.\n");
                continue;
            }
            _ => {}
        }

        // Build prompt with context
        let prompt = format!("{}\n\nUser: {}\nAssistant:", 
            conversation_history.join("\n"), 
            input
        );

        // Generate response
        match client.generate(&model, &prompt, options.clone()).await {
            Ok(response) => {
                println!("Assistant: {}", response.trim());
                println!();

                // Update history
                conversation_history.push(format!("User: {}", input));
                conversation_history.push(format!("Assistant: {}", response.trim()));

                // Limit history to avoid context overflow
                if conversation_history.len() > 10 {
                    conversation_history.drain(1..3);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("Try: ollama pull {}", model);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_limit() {
        let mut history = vec!["system".to_string()];
        
        for i in 0..20 {
            history.push(format!("User: {}", i));
            history.push(format!("Assistant: {}", i));
            
            if history.len() > 10 {
                history.drain(1..3);
            }
        }
        
        assert!(history.len() <= 12);
    }
}
