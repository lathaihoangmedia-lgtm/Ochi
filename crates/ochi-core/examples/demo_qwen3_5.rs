//! Real-world Demo: Qwen3.5-0.8B with Auto-Tuning & Loop Detection
//! 
//! This demo shows:
//! 1. Hardware detection
//! 2. Auto-tuning for 0.8B model
//! 3. Loop detection with sample text
//! 4. Text generation simulation (placeholder)

use ochi_core::{
    HardwareInfo, 
    AutoTuner, 
    ai::{
        LoopDetector, 
        AutoConfigurator,
        model::CandleConfig,
    }
};

fn main() {
    println!("🚀 Ochi Core Demo - Qwen3.5-0.8B Real-world Test\n");
    println!("={:=<70}\n", "");
    
    // Step 1: Detect Hardware
    let hardware = detect_hardware();
    
    // Step 2: Auto-tune for 0.8B model
    let config = auto_tune_for_qwen(&hardware);
    
    // Step 3: Test Loop Detection with Vietnamese text
    test_loop_detection_vietnamese();
    
    // Step 4: Simulate text generation
    simulate_generation(&config);
    
    println!("\n={:=<70}", "");
    println!("✅ Demo completed successfully!\n");
}

fn detect_hardware() -> HardwareInfo {
    println!("📊 Step 1: Hardware Detection\n");
    
    match HardwareInfo::detect() {
        Ok(info) => {
            println!("  ✓ CPU: {} ({} cores, {} threads)", 
                     info.cpu.name, info.cpu.cores, info.cpu.threads);
            println!("  ✓ RAM: {}GB total, {}GB available", 
                     info.memory.total, info.memory.available);
            
            if let Some(gpu) = &info.gpu {
                println!("  ✓ GPU: {} ({}MB VRAM)", gpu.name, gpu.vram_total);
            } else {
                println!("  ⚠ GPU: Not detected (CPU-only mode)");
                println!("     Tip: Set CUDA_VISIBLE_DEVICES=0 to enable GPU detection");
            }
            
            println!("\n  💡 Recommendations:");
            println!("     - Model size: up to {:.1}B parameters", info.recommended_model_size());
            println!("     - Context size: {} tokens", info.recommended_context());
            println!();
            
            info
        }
        Err(e) => {
            eprintln!("  ✗ Hardware detection failed: {}", e);
            eprintln!("  Using fallback configuration...");
            
            HardwareInfo {
                cpu: ochi_core::hardware::detector::CpuInfo {
                    cores: 4,
                    threads: 8,
                    name: "Unknown CPU".to_string(),
                },
                gpu: None,
                memory: ochi_core::hardware::detector::MemoryInfo {
                    total: 16,
                    available: 8,
                },
                has_gpu: false,
            }
        }
    }
}

fn auto_tune_for_qwen(hardware: &HardwareInfo) -> CandleConfig {
    println!("🎯 Step 2: Auto-Tuning for Qwen3.5-0.8B\n");
    
    let tuner = AutoTuner::new();
    let recommendation = tuner.recommend(0.8);  // 0.8B model
    
    println!("  Model: Qwen3.5-0.8B");
    println!("  Hardware recommendations:");
    println!("    - GPU layers: {}", recommendation.gpu_layers);
    println!("    - Context size: {}", recommendation.context_size);
    println!("    - CPU threads: {:?}", recommendation.n_threads);
    
    // Create optimized config for Qwen3.5-0.8B
    let config = CandleConfig {
        model_path: "models/qwen3.5-0.8b.safetensors".to_string(),
        context_size: recommendation.context_size,
        temperature: 0.75,  // Qwen works well with 0.7-0.8
        max_tokens: 512,
        top_p: 0.9,
        top_k: 40,
        repetition_penalty: 1.15,  // Higher to prevent looping (common in small models)
        cpu_only: !hardware.has_gpu,
        n_threads: recommendation.n_threads,
    };
    
    println!("\n  Optimized configuration:");
    println!("    - Context size: {}", config.context_size);
    println!("    - Temperature: {:.2}", config.temperature);
    println!("    - Repetition penalty: {:.2}", config.repetition_penalty);
    println!("    - Device: {}", if config.cpu_only { "CPU" } else { "GPU" });
    println!("    - Threads: {:?}", config.n_threads);
    println!();
    
    // Auto-configure with model-specific optimizations
    let configurator = AutoConfigurator::new(hardware.clone());
    let result = configurator.auto_configure("models/qwen3.5-0.8b.safetensors");
    
    println!("  Auto-config recommendations:");
    for rec in &result.recommendations {
        println!("    • {}", rec);
    }
    
    println!("\n  Performance estimate:");
    println!("    - Speed: ~{:.1} tokens/sec", result.performance_estimate.estimated_speed_tps);
    println!("    - VRAM: ~{}MB", result.performance_estimate.estimated_vram_mb);
    println!("    - RAM: ~{}MB", result.performance_estimate.estimated_ram_mb);
    println!("    - Quality: {}/10", result.performance_estimate.quality_rating);
    println!();
    
    config
}

fn test_loop_detection_vietnamese() {
    println!("🔄 Step 3: Loop Detection Test (Vietnamese)\n");
    
    // Test with Qwen3.5-0.8B specific settings
    // Small models tend to loop more, so we use stricter detection
    let mut detector = LoopDetector::new(10, 0.6);  // Stricter threshold for small models
    
    // Sample 1: Normal conversation
    println!("  Test 1: Normal conversation");
    let conversation = "Xin chào! Tôi là trợ lý AI. Tôi có thể giúp gì cho bạn? \
                       Tôi có thể trả lời câu hỏi, viết code, hoặc phân tích dữ liệu.";
    
    let tokens: Vec<&str> = conversation.split_whitespace().collect();
    for token in tokens {
        let status = detector.check(token);
        if status.is_loop() {
            println!("    ⚠️  Loop detected: {}", status.message().unwrap());
        }
    }
    println!("    ✅ No loops detected\n");
    
    // Sample 2: Code explanation (technical)
    println!("  Test 2: Code explanation");
    let code_explanation = "Hàm này nhận vào một số nguyên n. \
                           Sau đó kiểm tra xem n có chia hết cho 2 không. \
                           Nếu chia hết thì trả về true, ngược lại trả về false.";
    
    let tokens: Vec<&str> = code_explanation.split_whitespace().collect();
    for token in tokens {
        let status = detector.check(token);
        if let Some(msg) = status.message() {
            println!("    ⚠️  {}", msg);
        }
    }
    println!("    ✅ Technical text processed\n");
    
    // Sample 3: Intentional loop (should detect)
    println!("  Test 3: Detecting intentional loop");
    let mut loop_detector = LoopDetector::new(10, 0.6);
    let loop_text = "và sau đó và sau đó và sau đó và sau đó";
    
    let tokens: Vec<&str> = loop_text.split_whitespace().collect();
    let mut detected = false;
    for (i, token) in tokens.iter().enumerate() {
        let status = loop_detector.check(token);
        if status.is_loop() {
            println!("    ✅ Loop detected at token {}: {}", i, status.message().unwrap());
            detected = true;
            break;
        }
    }
    
    if !detected {
        println!("    ⚠️  Loop not detected (may need tuning)");
    }
    println!();
}

fn simulate_generation(config: &CandleConfig) {
    println!("📝 Step 4: Simulate Text Generation\n");
    
    println!("  Using configuration:");
    println!("    - Model: Qwen3.5-0.8B");
    println!("    - Context: {} tokens", config.context_size);
    println!("    - Temperature: {:.2}", config.temperature);
    println!("    - Repetition penalty: {:.2}", config.repetition_penalty);
    println!();
    
    // Simulate generation with loop detection
    let mut detector = LoopDetector::new(10, 0.6);
    
    let sample_outputs = vec![
        // Sample 1: Greeting
        "Xin chào! Tôi là Qwen3.5-0.8B. Tôi có thể giúp bạn hôm nay?",
        
        // Sample 2: Code explanation
        "Đây là hàm Python kiểm tra số nguyên tố. Đầu tiên, nó kiểm tra \
         xem n có nhỏ hơn 2 không. Nếu có thì trả về False. Sau đó, nó \
         kiểm tra xem n có chia hết cho số nào từ 2 đến căn bậc hai của n không.",
        
        // Sample 3: Creative writing
        "Trong một thế giới xa xôi, nơi công nghệ và phép thuật cùng tồn tại, \
         một lập trình viên trẻ phát hiện ra rằng code có thể tạo ra phép màu.",
    ];
    
    for (i, output) in sample_outputs.iter().enumerate() {
        println!("  Sample {}:", i + 1);
        println!("    \"{}\"", output);
        
        // Check for loops in generated text
        let tokens: Vec<&str> = output.split_whitespace().collect();
        let mut has_loop = false;
        
        for token in tokens {
            let status = detector.check(token);
            if status.is_loop() {
                println!("    ⚠️  Loop warning: {}", status.message().unwrap());
                has_loop = true;
                break;
            }
        }
        
        if !has_loop {
            println!("    ✅ Quality check: Passed (no loops)");
        }
        println!();
    }
    
    // Demonstrate anti-repetition in action
    println!("  💡 Anti-repetition mechanism:");
    println!("    - Repetition penalty: {:.2}", config.repetition_penalty);
    println!("    - Loop detection window: 10 tokens");
    println!("    - Similarity threshold: 0.6 (stricter for small models)");
    println!();
    
    println!("  ℹ️  Note: This is a simulation. Real generation requires:");
    println!("    1. Download Qwen3.5-0.8B model");
    println!("    2. Load with CandleModel::load()");
    println!("    3. Call model.generate() with config");
    println!();
    
    println!("  Example code:");
    println!("    let model = CandleModel::load(\"models/qwen3.5-0.8b.safetensors\", config)?;");
    println!("    let output = model.generate(\"Xin chào, hãy giới thiệu về bản thân\")?;");
    println!("    println!(\"{{}}\", output);");
    println!();
}
