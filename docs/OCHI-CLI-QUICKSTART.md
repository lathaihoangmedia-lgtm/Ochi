# Ochi CLI - Quick Start Guide

> **Version:** 0.1.0  
> **Date:** 2026-03-15  
> **Status:** ✅ **READY TO USE**

---

## 🚀 Cài Đặt Nhanh

### **1. Set API Key**

```bash
# Windows PowerShell
$env:GROQ_API_KEY="your_groq_api_key_here"

# Windows CMD
set GROQ_API_KEY=your_groq_api_key_here

# Linux/macOS
export GROQ_API_KEY=your_groq_api_key_here
```

**Get your free API key:** https://console.groq.com/keys

---

### **2. Chạy CLI**

```bash
# Interactive chat mode (default)
ochi

# Or with API key argument
ochi -a your_groq_api_key

# Show help
ochi --help
```

---

## 📋 Commands

### **1. `chat` - Interactive Chat Mode**

```bash
ochi chat
# or just: ochi
```

**Example session:**
```
👉 You: Write a Python script to scrape websites
🤔 AI is thinking...

🤖 AI: Here's a Python script using requests and BeautifulSoup:

```python
import requests
from bs BeautifulSoup import BeautifulSoup

def scrape_website(url):
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')
    
    # Extract all links
    links = [a['href'] for a in soup.find_all('a', href=True)]
    return links

# Usage
links = scrape_website('https://example.com')
print(links)
```

👉 You: Thanks! Now add error handling
🤔 AI is thinking...
...
```

**Commands in chat:**
- `/help` - Show help
- `/clear` - Clear conversation history
- `exit` or `quit` - Exit

---

### **2. `code` - Generate Code**

```bash
ochi code "Write a Rust function to calculate fibonacci"
```

**Output:**
```
🦀 Ochi Code Generator

📝 Prompt: Write a Rust function to calculate fibonacci

⏳ Generating code...

### Fibonacci Sequence Calculation in Rust

```rust
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```
...
```

---

### **3. `read` - Read & Analyze File**

```bash
ochi read src/lib.rs
```

**Output:**
```
📖 Reading file: "src/lib.rs"

📄 File content (1234 bytes)

⏳ Analyzing...

🤖 AI: This is a Rust library crate that provides:
1. Main entry point with lib.rs
2. Public API exports...
...
```

---

### **4. `scan` - Scan Project Structure**

```bash
ochi scan
# or specify path: ochi scan e:\Ochi
```

**Output:**
```
🔍 Scanning project: "."

📁 Found 15 relevant files

  - "Cargo.toml"
  - "src/lib.rs"
  - "src/main.rs"
  ...

⏳ Analyzing project structure...

🤖 AI: This is a Rust workspace with 9 crates:
1. ochi-core - Core utilities
2. ochi-gateway - API gateway
...
```

---

### **5. `ask` - Quick Question**

```bash
ochi ask "What is the difference between Rust and Go?"
```

**Output:**
```
❓ Question: What is the difference between Rust and Go?

⏳ Getting answer...

🤖 AI: Rust and Go are both modern systems programming languages...
```

---

## 🔧 Options

| Option | Description | Default |
|--------|-------------|---------|
| `-m, --model` | AI model to use | `llama-3.3-70b-versatile` |
| `-a, --api-key` | Groq API key | (env `GROQ_API_KEY`) |
| `-h, --help` | Show help | - |
| `-V, --version` | Show version | - |

**Available Models:**
- `llama-3.3-70b-versatile` (default) - Best quality
- `llama-3.1-8b-instant` - Faster, cheaper
- `gemma2-9b-it` - Google's model

---

## 💡 Tips & Tricks

### **1. Code Review**

```bash
ochi read src/main.rs | ochi code "Review this code for bugs"
```

### **2. Generate Tests**

```bash
ochi code "Write unit tests for src/lib.rs"
```

### **3. Explain Code**

```bash
ochi ask "Explain how tokio async works in Rust"
```

### **4. Refactor**

```bash
ochi code "Refactor this code to use Result instead of unwrap"
```

### **5. Documentation**

```bash
ochi code "Add rustdoc comments to this file"
```

---

## ⚠️ Rate Limits

**Groq Free Tier:**
- 30 requests per minute
- 14,400 requests per day

**Monitor usage:**
```bash
# Check your usage at: https://console.groq.com/usage
```

---

## 🐛 Troubleshooting

### **"GROQ_API_KEY not set"**

```bash
# Set environment variable
export GROQ_API_KEY=your_key_here

# Or use argument
ochi -a your_key_here
```

### **"API Error 429"**

Rate limit exceeded. Wait 1 minute or upgrade Groq plan.

### **"API Error 401"**

Invalid API key. Check your key at https://console.groq.com/keys

### **Build fails**

```bash
# Update Rust
rustup update

# Rebuild
cargo build --release -p ochi-gateway
```

---

## 📊 Performance

| Metric | Value |
|--------|-------|
| **Binary size** | ~15 MB |
| **RAM usage** | <50 MB |
| **Startup time** | <1s |
| **Response time** | 1-3s (depends on model) |

---

## 🔜 Next Features

- [ ] File write mode: `ochi write src/new.rs --prompt "Create module"`
- [ ] Edit mode: `ochi edit src/lib.rs --prompt "Add error handling"`
- [ ] Multi-file scan: `ochi scan --deep`
- [ ] Chat history save/load
- [ ] Custom system prompts
- [ ] Local model support (Ollama)

---

## 📚 Related Docs

- [QUICKSTART-QWEN-CODE.md](QUICKSTART-QWEN-CODE.md) - Setup guide
- [TRUNG-CUNG-ARCHITECTURE.md](TRUNG-CUNG-ARCHITECTURE.md) - Architecture
- [UI-ARCHITECTURE-TAURI.md](UI-ARCHITECTURE-TAURI.md) - Future UI

---

**Enjoy coding with Ochi CLI!** 🦀✨
