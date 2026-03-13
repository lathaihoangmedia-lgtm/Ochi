//! Example: Auto-tune và inference với Qwen3.5-0.8B

use ochi_core::{AutoTuner, GGUFModel, GGUFConfig, TuningProfile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Ochi Core AI Demo ===\n");
    
    // 1. Detect hardware
    println!("[1/3] Detecting hardware...");
    let tuner = AutoTuner::new()?;
    tuner.print_summary();
    
    // 2. Auto-tune config
    println!("[2/3] Auto-tuning config for Qwen3.5-0.8B...");
    let config = tuner.tune(0.8, TuningProfile::Balanced);
    println!("Config: {:?}", config);
    
    // 3. Load model
    println!("[3/3] Loading model...");
    let model_path = "models/qwen3.5-0.8b.gguf";
    
    if !std::path::Path::new(model_path).exists() {
        eprintln!("Model not found: {}", model_path);
        eprintln!("Please download the model first!");
        return Err("Model not found".into());
    }
    
    let model = GGUFModel::load(model_path, config)?;
    println!("Model loaded successfully!");
    println!("Info: {:?}", model.info());
    
    // 4. Test inference
    println!("\n=== Test Inference ===\n");
    
    let prompts = vec![
        "Hello, how are you?",
        "What is AI?",
        "Explain quantum computing in 1 sentence.",
    ];
    
    for prompt in prompts {
        println!("Prompt: {}", prompt);
        let output = model.generate(prompt)?;
        println!("Response: {}\n", output);
    }
    
    Ok(())
}
