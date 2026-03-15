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
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// Rate limit tracking
#[derive(Debug, Clone)]
struct RateLimitInfo {
    provider: String,
    requests_remaining: u32,
    reset_time: u64,  // Unix timestamp
    is_available: bool,
}

// Context preservation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionContext {
    session_id: String,
    messages: Vec<ChatMessage>,
    last_task: Option<String>,
    files_touched: Vec<PathBuf>,
    commands_run: Vec<String>,
    created_at: u64,
    updated_at: u64,
}

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
    
    /// Show session recap (quick context summary)
    Recap {
        /// Number of recent messages to summarize
        #[arg(short, long, default_value = "5")]
        last: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    session_context: Option<SessionContext>,
    rate_limits: Vec<RateLimitInfo>,
    current_provider: String,
}

impl OchiClient {
    fn new(api_key: String, model: String, ollama_url: String, use_local: bool) -> Self {
        let session_id = format!(
            "session_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
        
        // Initialize rate limit tracking for providers
        let mut rate_limits = vec![
            RateLimitInfo {
                provider: "ollama".to_string(),
                requests_remaining: 9999, // Local, no limit
                reset_time: 0,
                is_available: true,
            },
        ];
        
        if !api_key.is_empty() {
            rate_limits.push(RateLimitInfo {
                provider: "groq".to_string(),
                requests_remaining: 30, // 30 req/min free tier
                reset_time: 0,
                is_available: true,
            });
        }
        
        Self {
            client: Client::new(),
            api_key,
            model,
            ollama_url,
            use_local,
            messages: vec![],
            session_context: Some(SessionContext {
                session_id,
                messages: vec![],
                last_task: None,
                files_touched: vec![],
                commands_run: vec![],
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            }),
            rate_limits,
            current_provider: if use_local { "ollama".to_string() } else { "groq".to_string() },
        }
    }

    async fn chat(&mut self, user_input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Add user message
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        });
        
        // Update session context
        if let Some(ctx) = &mut self.session_context {
            ctx.messages.push(ChatMessage {
                role: "user".to_string(),
                content: user_input.to_string(),
            });
            ctx.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }

        // Try current provider first, failover if needed
        let response = self.chat_with_failover().await?;

        // Add AI response to history
        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: response.clone(),
        });
        
        // Update session context
        if let Some(ctx) = &mut self.session_context {
            ctx.messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: response.clone(),
            });
            ctx.updated_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }

        Ok(response)
    }
    
    async fn chat_with_failover(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut last_error: Option<Box<dyn std::error::Error>> = None;
        
        // Try each provider in priority order
        let providers_to_try: Vec<&str> = if self.use_local {
            vec!["ollama", "groq"]
        } else {
            vec!["groq", "ollama"]
        };
        
        for provider in providers_to_try {
            // Check if provider is available
            if let Some(rate_limit) = self.rate_limits.iter_mut().find(|r| r.provider == provider) {
                // Check rate limit
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if now < rate_limit.reset_time && rate_limit.requests_remaining == 0 {
                    eprintln!("⚠️  {} rate limited, retry at {}", provider, rate_limit.reset_time);
                    continue;
                }
                
                // Decrement counter
                if rate_limit.requests_remaining > 0 {
                    rate_limit.requests_remaining -= 1;
                }
            }
            
            // Try this provider
            let result = if provider == "ollama" {
                self.call_ollama().await
            } else if provider == "groq" {
                self.call_groq().await
            } else {
                continue;
            };
            
            match result {
                Ok(response) => {
                    // Success! Update current provider
                    self.current_provider = provider.to_string();
                    return Ok(response);
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    eprintln!("⚠️  {} failed: {}", provider, error_msg);
                    last_error = Some(e);
                    
                    // Update rate limit info
                    if let Some(rate_limit) = self.rate_limits.iter_mut().find(|r| r.provider == provider) {
                        if error_msg.contains("429") || error_msg.contains("rate limit") {
                            rate_limit.is_available = false;
                            rate_limit.reset_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 60;
                        }
                    }
                }
            }
        }
        
        // All providers failed
        Err(last_error.unwrap_or_else(|| "All providers unavailable".into()))
    }

    async fn call_ollama(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Auto-detect available local models
        let available_models = self.detect_local_models().await.unwrap_or_else(|_| vec![self.model.clone()]);
        
        // Use requested model if available, otherwise fallback to first available
        let model_to_use = if available_models.contains(&self.model) {
            &self.model
        } else if !available_models.is_empty() {
            eprintln!("⚠️  Model '{}' not found, using '{}'", self.model, available_models[0]);
            &available_models[0]
        } else {
            &self.model
        };
        
        // Ollama API: POST /api/chat
        let payload = serde_json::json!({
            "model": model_to_use,
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
    
    async fn detect_local_models(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Call Ollama /api/tags to get available models
        let response = self
            .client
            .get(format!("{}/api/tags", self.ollama_url))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Ok(vec![]);
        }
        
        let tags_response: serde_json::Value = response.json().await?;
        
        let mut models = vec![];
        if let Some(models_array) = tags_response["models"].as_array() {
            for model in models_array {
                if let Some(name) = model["name"].as_str() {
                    models.push(name.to_string());
                }
            }
        }
        
        Ok(models)
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
        println!("   Failover: Groq (if API key provided)");
        println!();
    } else {
        println!("🦀 Ochi CLI - Cloud Mode (Groq)");
        println!("   Model: {}", cli.model);
        println!("   Failover: Ollama (local)");
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
        Some(Commands::Write { file, prompt }) => {
            run_write_mode(cli.api_key, cli.model, cli.ollama_url, use_local, file, prompt).await?;
        }
        Some(Commands::Edit { file, prompt }) => {
            run_edit_mode(cli.api_key, cli.model, cli.ollama_url, use_local, file, prompt).await?;
        }
        Some(Commands::Run { command }) => {
            run_command_mode(command.to_string()).await?;
        }
        Some(Commands::Scan { path }) => {
            run_scan_mode(cli.api_key, cli.model, cli.ollama_url, use_local, path).await?;
        }
        Some(Commands::Ask { question }) => {
            run_quick_ask(cli.api_key, cli.model, cli.ollama_url, use_local, question).await?;
        }
        Some(Commands::Recap { last }) => {
            run_recap_mode(cli.api_key, cli.model, cli.ollama_url, use_local, *last).await?;
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

async fn run_write_mode(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    file: &PathBuf,
    prompt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Writing to: {:?}\n", file);
    println!("🤖 Prompt: {}\n", prompt);
    println!("⏳ Generating content...\n");

    let mut client = OchiClient::new(api_key, model, ollama_url, use_local);
    
    let write_prompt = format!(
        "Write content to the file '{}'. {}\n\n\
         IMPORTANT: Return ONLY the exact content that should be written to the file.\n\
         Do NOT include markdown code blocks, explanations, or any other text.\n\
         Just return the raw file content.\n\n\
         Request: {}",
        file.display(),
        if file.exists() { "Replace existing content." } else { "Create new file." },
        prompt
    );

    match client.chat(&write_prompt).await {
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

            // Write to file
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

async fn run_edit_mode(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    file: &PathBuf,
    prompt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("✏️ Editing: {:?}\n", file);
    println!("🤖 Prompt: {}\n", prompt);

    if !file.exists() {
        eprintln!("❌ File does not exist: {:?}", file);
        return Ok(());
    }

    let current_content = fs::read_to_string(file)?;
    println!("📄 Current content ({} bytes)\n", current_content.len());
    println!("⏳ Generating edit...\n");

    let mut client = OchiClient::new(api_key, model, ollama_url, use_local);
    
    let edit_prompt = format!(
        "Edit the following file content based on the instruction.\n\n\
         File: {}\n\n\
         Current content:\n```\n{}\n```\n\n\
         Edit instruction: {}\n\n\
         IMPORTANT: Return ONLY the complete edited content.\n\
         Do NOT include explanations or markdown code blocks.\n\
         Return the FULL file content after edits.",
        file.display(),
        current_content,
        prompt
    );

    match client.chat(&edit_prompt).await {
        Ok(new_content) => {
            // Clean up markdown code blocks
            let clean_content = new_content
                .trim()
                .strip_prefix("```")
                .and_then(|s| s.split_once("```\n"))
                .map(|(_, c)| c.split_once('\n').map(|(_, c)| c).unwrap_or(c))
                .unwrap_or(&new_content);

            // Write edited content
            fs::write(file, clean_content)?;
            println!("✅ Successfully edited {:?}", file);
            println!("📊 New size: {} bytes", clean_content.len());
            
            // Show diff summary
            let diff_lines = clean_content.lines().count() as i32 - current_content.lines().count() as i32;
            if diff_lines > 0 {
                println!("📈 Added {} lines", diff_lines);
            } else if diff_lines < 0 {
                println!("📉 Removed {} lines", -diff_lines);
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}

async fn run_command_mode(command: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Running: {}\n", command);
    println!("⏳ Executing...\n");

    let cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
    } else {
        Command::new("sh")
            .args(["-c", &command])
            .output()
    }?;

    if !cmd.stdout.is_empty() {
        println!("📤 Output:\n{}", String::from_utf8_lossy(&cmd.stdout));
    }
    
    if !cmd.stderr.is_empty() {
        eprintln!("⚠️ Errors:\n{}", String::from_utf8_lossy(&cmd.stderr));
    }

    println!("\n✅ Command exited with code: {:?}", cmd.status.code());

    Ok(())
}

async fn run_recap_mode(
    api_key: String,
    model: String,
    ollama_url: String,
    use_local: bool,
    last: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Session Recap\n");
    
    let client = OchiClient::new(api_key, model, ollama_url, use_local);
    
    // Load session context if exists
    if let Some(ctx) = &client.session_context {
        println!("Session ID: {}", ctx.session_id);
        println!("Started: {}", ctx.created_at);
        println!("Last active: {}", ctx.updated_at);
        println!();
        
        if !ctx.files_touched.is_empty() {
            println!("📁 Files touched:");
            for file in &ctx.files_touched {
                println!("  - {:?}", file);
            }
            println!();
        }
        
        if !ctx.commands_run.is_empty() {
            println!("🚀 Commands run:");
            for cmd in &ctx.commands_run {
                println!("  - {}", cmd);
            }
            println!();
        }
        
        // Show recent messages
        let recent_messages: Vec<_> = ctx.messages.iter().rev().take(last).rev().collect();
        
        if !recent_messages.is_empty() {
            println!("💬 Recent conversation (last {} messages):\n", recent_messages.len());
            
            for (i, msg) in recent_messages.iter().enumerate() {
                let role = if msg.role == "user" { "👉 You" } else { "🤖 AI" };
                let preview = if msg.content.len() > 200 {
                    format!("{}...", &msg.content[..200])
                } else {
                    msg.content.clone()
                };
                
                println!("{}. {}: {}", i + 1, role, preview);
            }
        } else {
            println!("✨ No conversation history yet. Start chatting!");
        }
        
        // Quick task reminder
        if let Some(ref task) = ctx.last_task {
            println!("\n📋 Last task: {}", task);
        }
    } else {
        println!("✨ No active session. Start a new chat!");
    }

    Ok(())
}
