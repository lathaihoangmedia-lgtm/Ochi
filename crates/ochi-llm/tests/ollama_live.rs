use ochi_core::HardwareInfo;
use ochi_llm::{OllamaAutoTuner, OllamaClient, OllamaOptions};

#[tokio::test]
#[ignore]
async fn live_ollama_autotune_generate() {
    if std::env::var("OLLAMA_CALL").ok().as_deref() != Some("1") {
        eprintln!("Set OLLAMA_CALL=1 to run live Ollama test");
        return;
    }

    let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen3.5:0.8b".to_string());
    let prompt = std::env::var("OLLAMA_PROMPT")
        .unwrap_or_else(|_| "Xin chao, hay tra loi ngan gon.".to_string());

    let hardware = HardwareInfo::detect().unwrap_or_else(|_| {
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
    });

    let options: OllamaOptions = OllamaAutoTuner::recommend(&model, &hardware);

    eprintln!("Model: {}", model);
    eprintln!("Auto-tuned options:");
    eprintln!("  temperature: {:?}", options.temperature);
    eprintln!("  top_p: {:?}", options.top_p);
    eprintln!("  top_k: {:?}", options.top_k);
    eprintln!("  repeat_penalty: {:?}", options.repeat_penalty);
    eprintln!("  num_predict: {:?}", options.num_predict);

    let client = OllamaClient::new();
    let output = client.generate(&model, &prompt, options).await;
    match output {
        Ok(text) => {
            eprintln!("\n=== OLLAMA OUTPUT ===\n{}", text);
        }
        Err(err) => {
            panic!("Ollama generate failed: {}", err);
        }
    }
}
