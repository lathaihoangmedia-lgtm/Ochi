//! Ochi Gateway - AI-Powered Coding Assistant
//!
//! Pipeline: User → LLM (Ollama/Groq) → Ochi-NLP (skills) → Qwen Code → Execute
//!
//! Quick start:
//!   ochi chat              - Interactive chat mode with LLM
//!   ochi code "prompt"     - Generate code with Qwen
//!   ochi run "command"     - Run shell command
//!   ochi status            - Show gateway status

use clap::{Parser, Subcommand};
use ochi_core::Result;
use ochi_trung_cung::{QwenCodeGenerator, QwenCodeConfig, LLMProvider, HybridNLPProcessor};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::fs;

// ============== Gateway State ==============

struct GatewayState {
    qwen: QwenCodeGenerator,
    nlp: HybridNLPProcessor,
}

impl GatewayState {
    fn new(provider: LLMProvider) -> Self {
        let config = QwenCodeConfig {
            provider,
            temperature: 0.7,
            max_tokens: 4096,
        };

        Self {
            qwen: QwenCodeGenerator::new(config),
            nlp: HybridNLPProcessor::new(),
        }
    }

    /// Chat with LLM - main entry point
    async fn chat(&mut self, input: &str) -> Result<String> {
        // Step 1: Process with Hybrid NLP (3 layers)
        let intent = self.nlp.process(input);
        println!("📍 Operation: {} (layer: {}, confidence: {:.0}%)", 
                 intent.operation, intent.layer, intent.confidence * 100.0);

        // Step 2: Execute action based on operation
        match intent.operation.as_str() {
            "generate_code" | "code" => {
                return self.qwen.generate_code(&intent.description).await;
            }
            "run_command" => {
                let cmd = self.nlp.extract_command(&intent.description);
                return Self::execute_shell_command(&cmd);
            }
            _ => {
                // General chat or LLM fallback
            }
        }

        // Step 3: For general chat, just call LLM
        let response = self.qwen.chat(input).await?;

        Ok(response)
    }

    /// Generate code
    async fn generate_code(&mut self, prompt: &str) -> Result<String> {
        self.qwen.generate_code(prompt).await
    }

    /// Execute shell command
    fn execute_shell_command(command: &str) -> Result<String> {
        println!("🚀 Executing: {}\n", command);

        #[cfg(windows)]
        let output = std::process::Command::new("cmd")
            .arg("/c")
            .arg(command)
            .output();

        #[cfg(unix)]
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output();

        match output {
            Ok(output) => {
                let mut result = String::new();
                if !output.stdout.is_empty() {
                    result.push_str(&format!("📝 Output:\n{}\n", String::from_utf8_lossy(&output.stdout)));
                }
                if !output.stderr.is_empty() {
                    result.push_str(&format!("⚠️  Stderr:\n{}\n", String::from_utf8_lossy(&output.stderr)));
                }
                result.push_str(&format!("\n✅ Exit code: {:?}", output.status.code()));
                Ok(result)
            }
            Err(e) => Err(ochi_core::Error::Custom(format!("Command failed: {}", e))),
        }
    }

    fn clear_history(&mut self) {
        self.qwen.clear_history();
    }
}

// ============== CLI ==============

#[derive(Parser)]
#[command(name = "ochi")]
#[command(author = "Ochi Team")]
#[command(version = "0.1.0")]
#[command(about = "Ochi - AI Coding Assistant (LLM + NLP + Qwen Code)", long_about = None)]
struct Cli {
    /// LLM provider: "ollama" or "groq"
    #[arg(long, default_value = "ollama")]
    provider: String,

    /// Ollama base URL
    #[arg(long, default_value = "http://localhost:11434", env = "OLLAMA_HOST")]
    ollama_url: String,

    /// Model to use
    #[arg(long, default_value = "qwen2.5:3b")]
    model: String,

    /// Groq API key (for cloud models)
    #[arg(short, long, env = "GROQ_API_KEY", default_value = "")]
    api_key: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive chat mode with LLM
    Chat,

    /// Generate code from prompt
    Code {
        #[arg(required = true)]
        prompt: String,
    },

    /// Read and analyze a file
    Read {
        #[arg(required = true)]
        file: PathBuf,
    },

    /// Write content to a file (AI-generated)
    Write {
        #[arg(required = true)]
        file: PathBuf,
        #[arg(short, long, required = true)]
        prompt: String,
    },

    /// Edit a file with AI
    Edit {
        #[arg(required = true)]
        file: PathBuf,
        #[arg(short, long, required = true)]
        prompt: String,
    },

    /// Run a shell command
    Run {
        #[arg(required = true)]
        command: String,
    },

    /// Show gateway status
    Status,
}

fn print_banner() {
    println!(r#"
╔═══════════════════════════════════════════╗
║  🦀 Ochi - AI Coding Assistant             ║
║  Pipeline: LLM → NLP → Qwen Code → Execute ║
║  Powered by Trung Cung (ochi-trung-cung)   ║
╚═══════════════════════════════════════════╝

Commands:
  /help     - Show this help
  /clear    - Clear conversation history
  /status   - Show gateway status
  /exit     - Exit

Type your message or code request:
"#);
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Build LLM config
    let provider = if cli.api_key.is_empty() {
        LLMProvider::Ollama {
            url: cli.ollama_url.clone(),
            model: cli.model.clone(),
        }
    } else {
        LLMProvider::Groq {
            api_key: cli.api_key.clone(),
            model: cli.model.clone(),
        }
    };

    println!("🦀 Ochi v0.1.0");
    let provider_info = match &provider {
        LLMProvider::Ollama { model, .. } => format!("ollama ({})", model),
        LLMProvider::Groq { model, .. } => format!("groq ({})", model),
    };
    println!("   LLM: {}", provider_info);
    println!("   Pipeline: LLM → NLP → Qwen Code → Execute");
    println!("   Trung Cung: ochi-trung-cung crate");
    println!();

    // Initialize gateway
    let mut gateway = GatewayState::new(provider);

    match &cli.command {
        Some(Commands::Chat) => {
            run_chat_mode(&mut gateway).await?;
        }
        Some(Commands::Code { prompt }) => {
            run_code_mode(&mut gateway, prompt).await?;
        }
        Some(Commands::Read { file }) => {
            run_read_mode(&mut gateway, file).await?;
        }
        Some(Commands::Write { file, prompt }) => {
            run_write_mode(&mut gateway, file, prompt).await?;
        }
        Some(Commands::Edit { file, prompt }) => {
            run_edit_mode(&mut gateway, file, prompt).await?;
        }
        Some(Commands::Run { command }) => {
            run_command_mode(command)?;
        }
        Some(Commands::Status) => {
            run_status_mode(&gateway)?;
        }
        None => {
            // Default to chat mode
            run_chat_mode(&mut gateway).await?;
        }
    }

    Ok(())
}

async fn run_chat_mode(gateway: &mut GatewayState) -> Result<()> {
    print_banner();

    let stdin = io::stdin();

    loop {
        print!("👉 You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("/exit") {
            println!("👋 Goodbye!");
            break;
        }

        if input.eq_ignore_ascii_case("help") || input.eq_ignore_ascii_case("/help") {
            print_banner();
            continue;
        }

        if input.eq_ignore_ascii_case("clear") || input.eq_ignore_ascii_case("/clear") {
            gateway.clear_history();
            println!("✨ Conversation cleared\n");
            continue;
        }

        if input.is_empty() {
            continue;
        }

        println!("🤔 AI is thinking...\n");

        match gateway.chat(input).await {
            Ok(response) => {
                println!("\n🤖 AI: {}\n", response);
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
    }

    Ok(())
}

async fn run_code_mode(gateway: &mut GatewayState, prompt: &str) -> Result<()> {
    println!("🦀 Ochi Code Generator\n");
    println!("📝 Prompt: {}\n", prompt);
    println!("⏳ Generating code...\n");

    match gateway.generate_code(prompt).await {
        Ok(response) => {
            println!("\n{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn run_read_mode(gateway: &mut GatewayState, file: &PathBuf) -> Result<()> {
    println!("📖 Reading file: {:?}\n", file);

    let content = fs::read_to_string(file)
        .map_err(|e| ochi_core::Error::Custom(format!("Cannot read file: {}", e)))?;

    println!("📄 File content ({} bytes)\n", content.len());
    println!("⏳ Analyzing...\n");

    let prompt = format!(
        "Analyze this code file and provide:\n\
         1. Brief summary of what it does\n\
         2. Key functions/structures\n\
         3. Any issues or improvements\n\
         4. Dependencies\n\n\
         File:\n```rust\n{}\n```",
        content
    );

    match gateway.chat(&prompt).await {
        Ok(response) => {
            println!("\n{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn run_write_mode(gateway: &mut GatewayState, file: &PathBuf, prompt: &str) -> Result<()> {
    println!("📝 Writing to: {:?}\n", file);
    println!("🤖 Prompt: {}\n", prompt);
    println!("⏳ Generating content...\n");

    let write_prompt = format!(
        "Write content to the file '{}'. {}\n\n\
         IMPORTANT: Return ONLY the exact content that should be written.\n\
         Do NOT include markdown code blocks or explanations.\n\
         Just return the raw file content.\n\n\
         Request: {}",
        file.display(),
        if file.exists() { "Replace existing content." } else { "Create new file." },
        prompt
    );

    match gateway.qwen.chat(&write_prompt).await {
        Ok(content) => {
            // Clean up markdown code blocks if AI included them
            let clean_content = content
                .trim()
                .strip_prefix("```")
                .and_then(|s| s.split_once("```\n"))
                .map(|(_, c)| c.split_once('\n').map(|(_, c)| c).unwrap_or(c))
                .unwrap_or(&content);

            // Create parent directories if needed
            if let Some(parent) = file.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(file, clean_content)?;
            println!("✅ Successfully wrote to {:?}", file);
            println!("📊 Written {} bytes", clean_content.len());
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn run_edit_mode(gateway: &mut GatewayState, file: &PathBuf, prompt: &str) -> Result<()> {
    println!("✏️ Editing: {:?}\n", file);
    println!("🤖 Prompt: {}\n", prompt);

    if !file.exists() {
        eprintln!("❌ File does not exist: {:?}", file);
        return Ok(());
    }

    let current_content = fs::read_to_string(file)?;
    let original_len = current_content.len();

    println!("📄 Current content ({} bytes)\n", original_len);
    println!("⏳ Generating edit...\n");

    let edit_prompt = format!(
        "You are editing a file. Return ONLY the COMPLETE edited file content.\n\n\
         FILE: {}\n\n\
         CURRENT CONTENT ({} bytes):\n\
         ```\n{}\n```\n\n\
         EDIT INSTRUCTION: {}\n\n\
         OUTPUT RULES:\n\
         1. Return ONLY the COMPLETE edited file content\n\
         2. NO markdown code blocks\n\
         3. NO explanations or comments\n\
         4. Preserve ALL existing code not mentioned\n\
         5. Return FULL file, not just changes\n\n\
         Edited content:",
        file.display(),
        original_len,
        current_content,
        prompt
    );

    match gateway.qwen.chat(&edit_prompt).await {
        Ok(new_content) => {
            let clean_content = new_content
                .trim()
                .strip_prefix("```")
                .and_then(|s| s.split_once("```\n"))
                .map(|(_, c)| c.split_once('\n').map(|(_, c)| c).unwrap_or(c))
                .unwrap_or(&new_content);

            fs::write(file, clean_content)?;
            println!("✅ Successfully edited {:?}", file);
            println!("📊 {} bytes → {} bytes", original_len, clean_content.len());
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

fn run_command_mode(command: &str) -> Result<()> {
    println!("🚀 Running command: {}\n", command);

    #[cfg(windows)]
    let output = std::process::Command::new("cmd")
        .arg("/c")
        .arg(command)
        .output();

    #[cfg(unix)]
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                println!("📝 Output:\n{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("⚠️  Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
            }
            println!("\n✅ Command completed with exit code: {:?}", output.status.code());
        }
        Err(e) => {
            eprintln!("❌ Command failed: {}", e);
        }
    }

    Ok(())
}

fn run_status_mode(_gateway: &GatewayState) -> Result<()> {
    println!("\n📊 Ochi Gateway Status");
    println!("   Status: Ready");
    println!("   Backend: ochi-trung-cung");
    println!("   Modules: QwenCode, NLP, AmDuongRouter");
    println!();
    Ok(())
}
