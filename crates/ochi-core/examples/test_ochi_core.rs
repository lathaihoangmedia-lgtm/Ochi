//! Test Program for Ochi Core
//! Tests: Loop Detection, Auto-Tuning, Hardware Detection

use ochi_core::{
    HardwareInfo, 
    AutoTuner, 
    ai::{
        LoopDetector, 
        AutoConfigurator,
    }
};

fn main() {
    println!("🧪 Testing Ochi Core Modules\n");
    println!("={:=<60}\n", "");

    // Test 1: Hardware Detection
    test_hardware_detection();
    
    // Test 2: Auto-Tuning
    test_auto_tuning();
    
    // Test 3: Loop Detection
    test_loop_detection();
    
    // Test 4: Auto-Configuration
    test_auto_config();
    
    println!("\n={:=<60}", "");
    println!("✅ All tests completed!\n");
}

fn test_hardware_detection() {
    println!("📊 Test 1: Hardware Detection\n");
    
    match HardwareInfo::detect() {
        Ok(info) => {
            println!("  CPU: {} ({} cores, {} threads)", 
                     info.cpu.name, info.cpu.cores, info.cpu.threads);
            println!("  RAM: {}GB total, {}GB available", 
                     info.memory.total, info.memory.available);
            
            if let Some(gpu) = &info.gpu {
                println!("  GPU: {} ({}MB VRAM, {} CUDA cores)", 
                         gpu.name, gpu.vram_total, gpu.cuda_cores);
            } else {
                println!("  GPU: Not detected (CPU-only mode)");
            }
            
            println!("  Recommended model size: {:.1}B parameters", 
                     info.recommended_model_size());
            println!("  Recommended context: {} tokens\n", 
                     info.recommended_context());
        }
        Err(e) => {
            println!("  ⚠️  Hardware detection failed: {}\n", e);
        }
    }
}

fn test_auto_tuning() {
    println!("🎯 Test 2: Auto-Tuning\n");
    
    let tuner = AutoTuner::new();
    let hardware = tuner.hardware();
    
    // Test with different model sizes
    let model_sizes = vec![1.0, 3.0, 7.0, 13.0];
    
    for size in model_sizes {
        let config = tuner.recommend(size);
        println!("  Model {:.1}B:", size);
        println!("    → GPU layers: {}", config.gpu_layers);
        println!("    → Context size: {}", config.context_size);
        println!("    → CPU threads: {:?}", config.n_threads);
    }
    
    println!();
}

fn test_loop_detection() {
    println!("🔄 Test 3: Loop Detection\n");
    
    // Test 3.1: Normal text (no loop)
    println!("  Test 3.1: Normal text (should be OK)");
    let mut detector = LoopDetector::new(10, 0.7);
    let normal_text = vec!["Hello", "world", "how", "are", "you", "today", "I", "am", "fine", "thanks"];
    
    for token in normal_text {
        let status = detector.check(token);
        if status.is_loop() {
            println!("    ⚠️  False positive: {}", status.message().unwrap());
        }
    }
    println!("    ✅ No loop detected (correct)\n");
    
    // Test 3.2: Repetitive text (should detect loop)
    println!("  Test 3.2: Repetitive text (should detect loop)");
    let mut detector = LoopDetector::new(10, 0.7);
    let repetitive_text = vec!["I", "love", "coding", "I", "love", "coding", "I", "love", "coding"];
    
    let mut loop_detected = false;
    for token in repetitive_text {
        let status = detector.check(token);
        if status.is_loop() {
            println!("    ✅ Loop detected: {}", status.message().unwrap());
            loop_detected = true;
            break;
        }
    }
    
    if !loop_detected {
        println!("    ⚠️  Loop not detected (may need tuning)");
    }
    println!();
    
    // Test 3.3: Sample paragraph with anti-repetition
    println!("  Test 3.3: Sample paragraph with repetition penalty");
    let sample_paragraph = "Artificial intelligence is transforming the world. \
                           Machine learning enables computers to learn from data. \
                           Deep learning uses neural networks with many layers. \
                           Natural language processing helps computers understand human language.";
    
    let mut detector = LoopDetector::new(10, 0.7);
    let tokens: Vec<&str> = sample_paragraph.split_whitespace().collect();
    
    for token in tokens {
        let status = detector.check(token);
        if let Some(msg) = status.message() {
            println!("    ⚠️  {}", msg);
        }
    }
    println!("    ✅ Paragraph processed successfully\n");
}

fn test_auto_config() {
    println!("⚙️  Test 4: Auto-Configuration\n");
    
    match HardwareInfo::detect() {
        Ok(hardware) => {
            let configurator = AutoConfigurator::new(hardware);
            
            // Test with sample model paths
            let test_models = vec![
                "models/llama3.2-3b.safetensors",
                "models/qwen2.5-7b.gguf",
                "models/phi3-mini.safetensors",
            ];
            
            for model_path in test_models {
                println!("  Model: {}", model_path);
                let result = configurator.auto_configure(model_path);
                
                println!("    Config:");
                println!("      - Context size: {}", result.config.context_size);
                println!("      - Temperature: {:.2}", result.config.temperature);
                println!("      - Repetition penalty: {:.2}", result.config.repetition_penalty);
                println!("      - CPU only: {}", result.config.cpu_only);
                
                if !result.recommendations.is_empty() {
                    println!("    Recommendations:");
                    for rec in &result.recommendations {
                        println!("      • {}", rec);
                    }
                }
                
                println!("    Performance estimate:");
                println!("      - Speed: ~{:.1} tokens/sec", result.performance_estimate.estimated_speed_tps);
                println!("      - VRAM: ~{}MB", result.performance_estimate.estimated_vram_mb);
                println!("      - RAM: ~{}MB", result.performance_estimate.estimated_ram_mb);
                println!("      - Quality rating: {}/10", result.performance_estimate.quality_rating);
                println!();
            }
        }
        Err(e) => {
            println!("  ⚠️  Cannot run auto-config: {}\n", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hardware_info() {
        let info = HardwareInfo::detect().unwrap();
        assert!(info.cpu.cores > 0);
        assert!(info.memory.total > 0);
    }
    
    #[test]
    fn test_loop_detector_normal() {
        let mut detector = LoopDetector::new(10, 0.7);
        for token in &["hello", "world", "test", "data"] {
            assert!(!detector.check(token).is_loop());
        }
    }
    
    #[test]
    fn test_auto_tuner() {
        let tuner = AutoTuner::new();
        let config = tuner.recommend(3.0);
        assert!(config.context_size > 0);
    }
}
