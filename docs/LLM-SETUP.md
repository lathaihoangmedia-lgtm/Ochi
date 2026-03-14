# 🦙 Ochi LLM Setup Guide

## ✅ Model Đã Có Sẵn

**Local GGUF:** `models/qwen3.5-0.8b.gguf` (0.8B parameters)

---

## 🔧 Cấu Hình

### File: `config/ochi.toml`

```toml
[llm]
# Ollama (primary - khi available)
ollama_url = "http://localhost:11434"
ollama_model = "qwen2.5:0.5b"

# Candle (fallback - local GGUF)
candle_model_path = "models/qwen3.5-0.8b.gguf"
candle_enabled = true

[hardware]
gpu_enabled = false
cpu_threads = 4
```

---

## 📦 Option 1: Ollama (Recommended)

### Bước 1: Cài Ollama

```bash
# Windows
winget install Ollama.Ollama

# Hoặc tải từ: https://ollama.ai
```

### Bước 2: Pull Model

```bash
# Model nhỏ nhất cho testing (260MB)
ollama pull qwen2.5:0.5b

# Hoặc model 3b (1.9GB)
ollama pull qwen2.5:3b

# Kiểm tra models đã cài
ollama list
```

### Bước 3: Test

```bash
# Test CLI
ollama run qwen2.5:0.5b "Xin chào, hãy giới thiệu ngắn gọn"

# Test với Rust example
cd e:\Ochi
cargo run --example ollama_autotune --features ollama
```

---

## 🕯️ Option 2: Candle (Local GGUF)

### Bước 1: Kiểm tra model

```bash
dir models\qwen3.5-0.8b.gguf
# ✅ File exists: 466,560,544 bytes
```

### Bước 2: Chạy demo

```bash
# Build với ollama feature (cho auto-tune)
cargo run --example candle-demo --features ollama
```

### Bước 3: Integration

```rust
use ochi_llm::{CandleModel, CandleConfig};

// Load model GGUF
let config = CandleConfig::balanced("models/qwen3.5-0.8b.gguf");
let model = CandleModel::load("models/qwen3.5-0.8b.gguf", config)?;
```

---

## 🎯 So Sánh

| Feature | Ollama | Candle (GGUF) |
|---------|--------|---------------|
| **Setup** | ✅ Dễ (1 lệnh) | ⚠️ Cần integration |
| **Speed** | ⚡ 50-100 tok/s | ⚡ 50-80 tok/s (CPU) |
| **Models** | ✅ 100+ models | ✅ Any GGUF |
| **Streaming** | ✅ Built-in | ⚠️ Cần implement |
| **VRAM** | ✅ GPU support | ⚠️ CPU only |
| **Size** | ⚠️ 0.5-70GB | ✅ 0.5-7GB |

---

## 🧪 Testing

### Test Ollama Connection

```bash
cargo test -p ochi-llm --features ollama -- --nocapture
```

### Test Candle Config

```rust
use ochi_llm::AutoConfigurator;

let auto = AutoConfigurator::new();
let result = auto.auto_configure("models/qwen3.5-0.8b.gguf");

println!("Context: {}", result.config.context_size);
println!("Threads: {:?}", result.config.n_threads);
```

---

## 📝 Model Recommendations

| Model | Size | Speed | Quality | Use |
|-------|------|-------|---------|-----|
| `qwen2.5:0.5b` | 260MB | ⚡⚡⚡ | ⭐⭐ | Dev/Test |
| `qwen2.5:1.5b` | 890MB | ⚡⚡ | ⭐⭐⭐ | Chat/Code |
| `qwen2.5:3b` | 1.9GB | ⚡ | ⭐⭐⭐⭐ | Production |
| `llama3.2:1b` | 1.1GB | ⚡⚡ | ⭐⭐⭐ | English |
| `llama3.2:3b` | 2GB | ⚡ | ⭐⭐⭐⭐ | General |

---

## 🚀 Quick Start

### Dev/Test (Nhanh nhất)

```bash
ollama pull qwen2.5:0.5b
cargo run --example ollama_autotune qwen2.5:0.5b --features ollama
```

### Production (Chất lượng)

```bash
ollama pull qwen2.5:3b
# Update config/ochi.toml
# ollama_model = "qwen2.5:3b"
```

### Offline (GGUF only)

```bash
# Use local GGUF with Candle
# Already configured in config/ochi.toml
```

---

## 🛠️ Troubleshooting

### Ollama không chạy

```bash
# Check service
ollama list

# Restart
ollama serve
```

### Model not found

```bash
# Pull again
ollama pull qwen2.5:0.5b

# Or use local GGUF
# Candle will fallback automatically
```

### Build errors

```bash
# Clean build
cargo clean
cargo build --workspace
```

---

## 📞 Next Steps

1. ✅ **Đã có model GGUF:** `models/qwen3.5-0.8b.gguf`
2. ⏸️ **Ollama:** Chưa có model → `ollama pull qwen2.5:0.5b`
3. 🧪 **Test:** `cargo run --example ollama_autotune --features ollama`

**Ready to go! 🚀**
