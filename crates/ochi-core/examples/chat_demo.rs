//! Interactive Chat Demo with Qwen3.5-0.8B
//! 
//! Run this to chat with the model (simulation mode)

use ochi_core::{
    HardwareInfo, 
    AutoTuner, 
    ai::{
        LoopDetector,
        model::CandleConfig,
    }
};
use std::io::{self, BufRead, Write};

fn main() {
    println!("🤖 Ochi Chat - Qwen3.5-0.8B Interactive Demo\n");
    println!("={:=<70}\n", "");
    
    // Initialize
    let config = initialize();
    
    println!("💬 Chat mode started!");
    println!("   Type your message and press Enter");
    println!("   Type 'quit' or 'exit' to stop\n");
    
    let mut detector = LoopDetector::new(10, 0.6);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        print!("You: ");
        stdout.flush().unwrap();
        
        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() {
            break;
        }
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("\n👋 Goodbye!");
            break;
        }
        
        // Check input for loops (user might paste repetitive text)
        let status = check_for_loops(&mut detector, input);
        if status {
            println!("⚠️  Warning: Repetitive input detected. Please rephrase.\n");
            continue;
        }
        
        // Generate response (simulation)
        let response = generate_response(input, &config);
        
        println!("\nAI: {}\n", response);
    }
    
    println!("\n={:=<70}", "");
    println!("Session ended.\n");
}

fn initialize() -> CandleConfig {
    println!("⚙️  Initializing Qwen3.5-0.8B...\n");
    
    let config = match HardwareInfo::detect() {
        Ok(hardware) => {
            println!("  CPU: {} cores, {} threads", hardware.cpu.cores, hardware.cpu.threads);
            println!("  RAM: {}GB available", hardware.memory.available);
            
            if let Some(gpu) = &hardware.gpu {
                println!("  GPU: {} ({}MB VRAM)", gpu.name, gpu.vram_total);
            }
            
            // Auto-tune
            let tuner = AutoTuner::new();
            let rec = tuner.recommend(0.8);
            
            CandleConfig {
                model_path: "models/qwen3.5-0.8b.safetensors".to_string(),
                context_size: rec.context_size,
                temperature: 0.75,
                max_tokens: 512,
                top_p: 0.9,
                top_k: 40,
                repetition_penalty: 1.15,
                cpu_only: !hardware.has_gpu,
                n_threads: rec.n_threads,
            }
        }
        Err(_) => {
            println!("  ⚠️  Hardware detection failed, using defaults");
            CandleConfig::balanced("models/qwen3.5-0.8b.safetensors")
        }
    };
    
    println!("\n  Configuration:");
    println!("    - Context: {} tokens", config.context_size);
    println!("    - Temperature: {:.2}", config.temperature);
    println!("    - Repetition penalty: {:.2}", config.repetition_penalty);
    println!("    - Device: {}", if config.cpu_only { "CPU" } else { "GPU" });
    println!();
    
    config
}

fn check_for_loops(detector: &mut LoopDetector, text: &str) -> bool {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    
    for token in tokens {
        let status = detector.check(token);
        if status.is_loop() {
            return true;
        }
    }
    
    false
}

fn generate_response(input: &str, _config: &CandleConfig) -> String {
    // SIMULATION MODE - Replace with actual model inference
    // let model = CandleModel::load(...)?;
    // model.generate(input)?;
    
    // Simple pattern matching for demo
    let input_lower = input.to_lowercase();
    
    if input_lower.contains("xin chào") || input_lower.contains("hello") {
        return "Xin chào! Tôi là Qwen3.5-0.8B. Tôi có thể giúp gì cho bạn hôm nay?".to_string();
    }
    
    if input_lower.contains("giới thiệu") || input_lower.contains("giới thiệu") {
        return "Tôi là mô hình ngôn ngữ Qwen3.5-0.8B, được tối ưu cho việc chạy trên \
               CPU với hiệu suất cao. Tôi có thể trả lời câu hỏi, viết code, phân tích \
               văn bản, và hỗ trợ nhiều tác vụ khác.".to_string();
    }
    
    if input_lower.contains("code") || input_lower.contains("lập trình") {
        return "Tôi có thể giúp bạn viết code! Hãy cho tôi biết:\n\
               - Ngôn ngữ lập trình (Python, Rust, JavaScript...)\n\
               - Chức năng bạn cần\n\
               - Ví dụ về input/output (nếu có)".to_string();
    }
    
    if input_lower.contains("tạm biệt") || input_lower.contains("bye") {
        return "Tạm biệt! Hẹn gặp lại bạn lần sau. Nếu cần giúp đỡ, cứ quay lại nhé!".to_string();
    }
    
    if input_lower.contains("?") {
        return format!("Đây là câu hỏi: \"{}\". Trong chế độ simulation, tôi chưa thể \
                       trả lời chính xác. Khi có model thật, tôi sẽ phân tích và trả lời \
                       chi tiết cho bạn!", input);
    }
    
    // Default response
    format!("Bạn nói: \"{}\". Đây là phản hồi mẫu từ Qwen3.5-0.8B simulation. \
            Để có phản hồi thực tế, bạn cần tải model và kết nối với Candle inference engine.", input)
}
