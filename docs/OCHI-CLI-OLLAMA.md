# Ochi CLI - Local Ollama Setup (Qwen2.5)

> **Version:** 0.2.0  
> **Date:** 2026-03-15  
> **Status:** ✅ **READY** - Local Ollama + Qwen2.5 supported

---

## 🚀 Quick Start với Ollama Local

### **1. Cài Ollama (nếu chưa có)**

```bash
# Windows: Tải từ https://ollama.com/download
# Hoặc winget:
winget install Ollama.Ollama

# macOS:
brew install ollama

# Linux:
curl -fsSL https://ollama.com/install.sh | sh
```

---

### **2. Pull Qwen2.5 Models**

```bash
# Qwen2.5 0.5B (siêu nhẹ, nhanh)
ollama pull qwen2.5:0.5b

# Qwen2.5 3B (cân bằng giữa tốc độ và chất lượng)
ollama pull qwen2.5:3b

# Kiểm tra models đã tải
ollama list
```

**Expected output:**
```
NAME              ID           SIZE
qwen2.5:0.5b      xxx          380 MB
qwen2.5:3b        yyy          2.0 GB
```

---

### **3. Chạy Ochi CLI với Local Models**

```bash
# Mode 1: Tự động detect (không cần API key → dùng Ollama)
ochi

# Mode 2: Chỉ định model cụ thể
ochi -m qwen2.5:3b

# Mode 3: Force local mode
ochi --local

# Mode 4: Custom Ollama URL
ochi --ollama-url http://192.168.1.100:11434
```

---

## 📋 Commands với Local Models

### **1. Interactive Chat**

```bash
# Dùng Qwen2.5:3b (default)
ochi chat

# Hoặc chỉ định model
ochi -m qwen2.5:0.5b chat
```

**Example:**
```
🦀 Ochi CLI - Local Mode (Ollama)
   Model: qwen2.5:3b
   URL: http://localhost:11434

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

👉 You: Write a Python function to reverse a string
🤔 AI is thinking...

🤖 AI: Here's a Python function to reverse a string:

```python
def reverse_string(s):
    return s[::-1]

# Test
print(reverse_string("hello"))  # Output: "olleh"
```
```

---

### **2. Generate Code**

```bash
# Quick code generation
ochi -m qwen2.5:3b code "Write hello world in Rust"

# Or with 0.5B (faster but less accurate)
ochi -m qwen2.5:0.5b code "Fibonacci function"
```

**Output:**
```
🦀 Ochi CLI - Local Mode (Ollama)
   Model: qwen2.5:3b
   URL: http://localhost:11434

🦀 Ochi Code Generator

📝 Prompt: Write hello world in Rust

⏳ Generating code...

```rust
fn main() {
    println!("Hello, World!");
}
```
```

---

### **3. Read & Analyze Files**

```bash
ochi -m qwen2.5:3b read src/lib.rs
```

---

### **4. Scan Project**

```bash
ochi -m qwen2.5:3b scan
```

---

### **5. Quick Question**

```bash
ochi -m qwen2.5:3b ask "What is Rust?"
```

---

## ⚙️ Options

| Option | Default | Description |
|--------|---------|-------------|
| `-m, --model` | `qwen2.5:3b` | Model name (local: `qwen2.5:0.5b`, `qwen2.5:3b`) |
| `--ollama-url` | `http://localhost:11434` | Ollama server URL |
| `-a, --api-key` | (empty) | Groq API key (optional, for cloud models) |
| `-l, --local` | false | Force local mode (ignore API key) |

---

## 🔥 Model Comparison

| Model | Size | Speed | Quality | Use Case |
|-------|------|-------|---------|----------|
| **qwen2.5:0.5b** | 380 MB | ⚡⚡⚡ Fastest | ⭐⭐ Good | Quick tasks, simple code |
| **qwen2.5:3b** | 2.0 GB | ⚡⚡ Fast | ⭐⭐⭐⭐ Very Good | Balanced, recommended |
| **llama-3.3-70b** (Cloud) | N/A | ⚡ Slow | ⭐⭐⭐⭐⭐ Best | Complex tasks, when local fails |

---

## 💡 Tips

### **1. Speed vs Quality**

```bash
# Fast iteration (0.5B)
ochi -m qwen2.5:0.5b code "simple function"

# Quality code (3B)
ochi -m qwen2.5:3b code "complex algorithm"
```

### **2. Run Ollama on Remote Server**

```bash
# Server: Start Ollama with remote access
OLLAMA_HOST=0.0.0.0:11434 ollama serve

# Client: Connect to remote Ollama
ochi --ollama-url http://192.168.1.100:11434
```

### **3. Custom Models**

```bash
# Pull any Ollama model
ollama pull codellama:7b
ollama pull mistral:7b

# Use with ochi
ochi -m codellama:7b code "..."
```

---

## 🐛 Troubleshooting

### **"Connection refused"**

```bash
# Check if Ollama is running
ollama list

# Start Ollama server
ollama serve
```

### **"Model not found"**

```bash
# Pull the model
ollama pull qwen2.5:3b

# Or use correct model name
ochi -m qwen2.5:3b
```

### **Slow response**

- Use smaller model: `qwen2.5:0.5b`
- Close other applications
- Check RAM usage (Ollama needs 2-4GB free)

---

## 📊 Performance (Local Testing)

| Task | qwen2.5:0.5b | qwen2.5:3b |
|------|--------------|------------|
| **Hello World** | <1s | 1-2s |
| **Fibonacci** | 1-2s | 2-3s |
| **Code Review** | 2-3s | 3-5s |
| **RAM Usage** | ~400 MB | ~2 GB |

---

## 🔜 Next Features

- [ ] Streaming response (show tokens as they generate)
- [ ] Multi-model fallback (try 0.5b → 3b → cloud)
- [ ] Local RAG (retrieve from codebase)
- [ ] Fine-tuned Qwen for Rust code

---

## 📚 Related Docs

- [OCHI-CLI-QUICKSTART.md](OCHI-CLI-QUICKSTART.md) - General usage
- [QUICKSTART-QWEN-CODE.md](QUICKSTART-QWEN-CODE.md) - Original setup guide

---

**Happy coding with local AI!** 🦀✨
