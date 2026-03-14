use ochi_core::HardwareInfo;
use ochi_llm::{OllamaAutoTuner, OllamaOptions};

#[cfg(feature = "ollama")]
use ochi_llm::OllamaClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "qwen2.5:3b".to_string());

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

    println!("Model: {}", model);
    println!("Auto-tuned options:");
    println!("  temperature: {:?}", options.temperature);
    println!("  top_p: {:?}", options.top_p);
    println!("  top_k: {:?}", options.top_k);
    println!("  repeat_penalty: {:?}", options.repeat_penalty);
    println!("  num_predict: {:?}", options.num_predict);

    #[cfg(feature = "ollama")]
    {
        if std::env::var("OLLAMA_CALL").ok().as_deref() == Some("1") {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                let client = OllamaClient::new();
                let prompt = "Xin chao, tu dong toi uu tham so se ra sao?";
                let output = client.generate(&model, prompt, options).await?;
                println!("\n=== Ollama Output ===\n{}", output);
                Ok::<(), Box<dyn std::error::Error>>(())
            })?;
        } else {
            println!("\nSet OLLAMA_CALL=1 to run a real generation test.");
        }
    }

    Ok(())
}
