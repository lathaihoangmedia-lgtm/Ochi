//! Ochi CLI - Qwen Code Integration
//! 
//! Quick start:
//!   ochi chat              - Interactive chat mode
//!   ochi code "prompt"     - Generate code
//!   ochi read <file>       - Read and analyze file
//!   ochi scan              - Scan project structure

use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(name = "ochi")]
#[command(author = "Ochi Team")]
#[command(version = "0.1.0")]
#[command(about = "Ochi CLI - AI-powered coding assistant (Local Ollama + Cloud)", long_about = None)]
struct Cli {
    /// Model to use
    /// Local: qwen2.5:0.5b, qwen2.5:3b
    /// Cloud: llama-3.3-70b-versatile (requires --api-key)
    #[arg(short, long, default_value = "qwen2.5:3b")]
    model: String,

    /// Ollama base URL (for local models)
    #[arg(long, default_value = "http://localhost:11434", env = "OLLAMA_HOST")]
    ollama_url: String,

    /// Groq API key (for cloud models, optional)
    #[arg(short, long, env = "GROQ_API_KEY", default_value = "")]
    api_key: String,

    /// Use local Ollama instead of cloud
    #[arg(short, long)]
    local: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive chat mode
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

    /// Scan project structure
    Scan {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Quick question
    Ask {
        #[arg(required = true)]
        question: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

struct OchiClient {
    client: Client,
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    messages: Vec<ChatMessage>,
}

impl OchiClient {
    fn new(api_key: String, model: String, ollama_url: String, use_local: bool) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
            ollama_url,
            use_local,
            messages: vec![],
        }
    }

    async fn chat(&mut self, user_input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Add user message
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        let response = if self.use_local || self.api_key.is_empty() {
            // Use local Ollama
            self.call_ollama().await?
        } else {
            // Use Groq cloud
            self.call_groq().await?
        };

        // Add AI response to history
        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response)
    }

    async fn call_ollama(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Ollama API: POST /api/chat
        let payload = serde_json::json!({
            "model": self.model,
            "messages": self.messages,
            "stream": false,
        });

        let response = self
            .client
            .post(format!("{}/api/chat", self.ollama_url))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(format!("Ollama Error {}: {}", status, text).into());
        }

        // Ollama response format: { "message": { "content": "..." }, ... }
        let ollama_response: serde_json::Value = response.json().await?;
        
        if let Some(content) = ollama_response["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            Err("No response from Ollama".into())
        }
    }

    async fn call_groq(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Groq API: POST /openai/v1/chat/completions
        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": self.model,
                "messages": self.messages,
                "max_tokens": 4096,
                "temperature": 0.7,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(format!("Groq Error {}: {}", status, text).into());
        }

        let chat_response: ChatResponse = response.json().await?;

        if let Some(choice) = &chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from Groq".into())
        }
    }

    async fn code(&mut self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = r#"You are an expert code assistant. 
When asked to write code:
1. Understand the requirements clearly
2. Write clean, production-ready code
3. Include comments explaining key parts
4. Follow best practices for the language
5. Consider edge cases and error handling
6. Use Rust for system-level tasks, Python for scripting
7. Always format code in markdown code blocks

Be concise but thorough. Prefer working code over perfect code."#;

        // Add system prompt if first message
        if self.messages.is_empty() {
            self.messages.push(ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            });
        }

        self.chat(prompt).await
    }

    fn clear_history(&mut self) {
        self.messages.clear();
    }
}

fn print_banner() {
    println!(r#"
╔═══════════════════════════════════════════╗
║  🦀 Ochi CLI - AI-Powered Coding Assistant ║
║  Powered by Groq (Llama 3.3 70B)           ║
╚═══════════════════════════════════════════╝

Commands:
  /help     - Show this help
  /clear    - Clear conversation history
  /code     - Switch to code mode
  /exit     - Exit

Type your message or code request:
"#);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Auto-detect: if no API key, use local Ollama
    let use_local = cli.local || cli.api_key.is_empty();
    
    if use_local {
        println!("🦀 Ochi CLI - Local Mode (Ollama)");
        println!("   Model: {}", cli.model);
        println!("   URL: {}", cli.ollama_url);
        println!();
    } else {
        println!("🦀 Ochi CLI - Cloud Mode (Groq)");
        println!("   Model: {}", cli.model);
        println!();
    }

    match &cli.command {
        Some(Commands::Chat) => {
            run_interactive_mode(cli.api_key, cli.model, cli.ollama_url, use_local).await?;
        }
        Some(Commands::Code { prompt }) => {
            run_code_mode(cli.api_key, cli.model, cli.ollama_url, use_local, prompt).await?;
        }
        Some(Commands::Read { file }) => {
            run_read_mode(cli.api_key, cli.model, cli.ollama_url, use_local, file).await?;
        }
        Some(Commands::Scan { path }) => {
            run_scan_mode(cli.api_key, cli.model, cli.ollama_url, use_local, path).await?;
        }
        Some(Commands::Ask { question }) => {
            run_quick_ask(cli.api_key, cli.model, cli.ollama_url, use_local, question).await?;
        }
        None => {
            // Default to interactive chat
            run_interactive_mode(cli.api_key, cli.model, cli.ollama_url, use_local).await?;
        }
    }

    Ok(())
}

async fn run_interactive_mode(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    print_banner();

    let mut client = OchiClient::new(api_key, model, ollama_url, use_local);
    let stdin = io::stdin();

    loop {
        print!("👉 You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("👋 Goodbye!");
            break;
        }

        if input.eq_ignore_ascii_case("help") || input.eq_ignore_ascii_case("/help") {
            print_banner();
            continue;
        }

        if input.eq_ignore_ascii_case("clear") || input.eq_ignore_ascii_case("/clear") {
            client.messages.clear();
            println!("✨ Conversation cleared\n");
            continue;
        }

        if input.is_empty() {
            continue;
        }

        println!("🤔 AI is thinking...\n");

        match client.chat(input).await {
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

async fn run_code_mode(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    prompt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Ochi Code Generator\n");
    println!("📝 Prompt: {}\n", prompt);
    println!("⏳ Generating code...\n");

    let mut client = OchiClient::new(api_key, model, ollama_url, use_local);

    match client.code(prompt).await {
        Ok(response) => {
            println!("\n{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn run_read_mode(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📖 Reading file: {:?}\n", file);

    let content = fs::read_to_string(file)
        .map_err(|e| format!("Cannot read file: {}", e))?;

    println!("📄 File content ({} bytes)\n", content.len());
    println!("⏳ Analyzing...\n");

    let mut client = OchiClient::new(api_key, model, ollama_url, use_local);
    
    let prompt = format!(
        "Analyze this code file and provide:\n\
         1. Brief summary of what it does\n\
         2. Key functions/structures\n\
         3. Any issues or improvements\n\
         4. Dependencies\n\n\
         File:\n```rust\n{}\n```",
        content
    );

    match client.chat(&prompt).await {
        Ok(response) => {
            println!("\n{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn run_scan_mode(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning project: {:?}\n", path);

    let mut files = Vec::new();
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" || ext == "toml" || ext == "md" {
                        files.push(path);
                    }
                }
            }
        }
    }

    println!("📁 Found {} relevant files\n", files.len());
    
    for file in &files {
        println!("  - {:?}", file);
    }

    println!("\n⏳ Analyzing project structure...\n");

    let mut client = OchiClient::new(api_key, model, ollama_url, use_local);
    
    let prompt = format!(
        "Analyze this Rust project structure:\n\n\
         Files:\n{}\n\n\
         Describe:\n\
         1. Project purpose\n\
         2. Architecture overview\n\
         3. Key components\n\
         4. Suggestions for improvement",
        files.iter()
            .filter_map(|f| f.file_name())
            .map(|n| n.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n")
    );

    match client.chat(&prompt).await {
        Ok(response) => {
            println!("\n{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn run_quick_ask(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    question: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("❓ Question: {}\n", question);
    println!("⏳ Getting answer...\n");

    let mut client = OchiClient::new(api_key, model, ollama_url, use_local);

    match client.chat(question).await {
        Ok(response) => {
            println!("\n{}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}
