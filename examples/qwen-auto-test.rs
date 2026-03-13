//! Qwen3.5-0.8B Auto-Test with Loop Detection

use ochi_core::{AutoConfigurator, AutoTuner, GGUFModel, HardwareInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "=".repeat(60));
    println!("Qwen3.5-0.8B - Auto Configuration & Loop Test");
    println!("{}", "=".repeat(60));
    println!();

    // 1. Detect hardware
    println!("[1/5] Detecting hardware...");
    let hardware = HardwareInfo::detect()?;
    hardware.print_summary();

    // 2. Auto-configure model
    println!("[2/5] Auto-configuring model...");
    let model_path = "models/qwen3.5-0.8b.gguf";
    
    if !std::path::Path::new(model_path).exists() {
        eprintln!("Model not found: {}", model_path);
        eprintln!("Please download the model first!");
        return Err("Model not found".into());
    }

    let mut configurator = AutoConfigurator::new(hardware.clone());
    let result = configurator.auto_configure(model_path);

    println!("\n📋 Auto-Configuration Report:");
    println!("   Model: {}", result.model_name);
    println!("   Path: {}", result.model_path);
    println!();

    if !result.issues_detected.is_empty() {
        println!("⚠️  Issues Detected: {}", result.issues_detected.len());
        for issue in &result.issues_detected {
            println!("   [{}] {}: {}", issue.severity, issue.code, issue.description);
        }
        println!();
    }

    if !result.fixes_applied.is_empty() {
        println!("✅ Fixes Applied: {}", result.fixes_applied.len());
        for fix in &result.fixes_applied {
            println!("   - {}: {} → {}", fix.issue_code, fix.old_value, fix.new_value);
        }
        println!();
    }

    if !result.recommendations.is_empty() {
        println!("💡 Recommendations:");
        for rec in &result.recommendations {
            println!("   • {}", rec);
        }
        println!();
    }

    println!("📊 Performance Estimate:");
    println!("   Speed: ~{} tokens/sec", result.performance_estimate.estimated_speed_tps as usize);
    println!("   VRAM: ~{} MB", result.performance_estimate.estimated_vram_mb);
    println!("   Quality: {}/5", result.performance_estimate.quality_rating);
    println!();

    // 3. Load model with auto-config
    println!("[3/5] Loading model with auto-config...");
    let model = GGUFModel::load(model_path, result.config.clone())?;
    println!("✅ Model loaded successfully!");
    println!();

    // 4. Test prompts
    println!("[4/5] Running test prompts...");
    println!();

    let test_prompts = vec![
        "Hello, how are you?",
        "What is 2 + 2?",
        "Tell me a very short story.",
        "Repeat after me: test pattern",
    ];

    for (i, prompt) in test_prompts.iter().enumerate() {
        println!("Test {}:", i + 1);
        println!("   Prompt: {}", prompt);
        
        let output = model.generate(prompt)?;
        println!("   Response: {}", output.trim());
        
        // Check for repetition
        if is_repetitive(&output) {
            println!("   ⚠️  Warning: Response appears repetitive");
        } else {
            println!("   ✅ OK: No repetition detected");
        }
        println!();
    }

    // 5. Summary
    println!("[5/5] Test Summary:");
    println!("   ✅ All tests completed!");
    println!("   ✅ Loop detection: Enabled");
    println!("   ✅ Auto-config: Applied");
    println!();
    println!("{}", "=".repeat(60));
    println!("Test Complete!");
    println!("{}", "=".repeat(60));
    println!();

    Ok(())
}

/// Simple repetition checker
fn is_repetitive(text: &str) -> bool {
    let words: Vec<&str> = text.split_whitespace().collect();
    
    if words.len() < 4 {
        return false;
    }

    // Check for 2-word repetition
    for i in 0..words.len() - 2 {
        let pattern = format!("{} {}", words[i], words[i + 1]);
        let count = words.windows(2)
            .filter(|w| format!("{} {}", w[0], w[1]) == pattern)
            .count();
        
        if count > 3 {
            return true;
        }
    }

    false
}
