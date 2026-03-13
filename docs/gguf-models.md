# GGUF Models Guide

> Hướng dẫn chọn và sử dụng GGUF models cho Ochi Core

## GGUF Là Gì?

**GGUF** (GGML Unified Format) là định dạng model được tối ưu cho CPU inference, hỗ trợ quantization để giảm kích thước và RAM usage.

### Ưu Điểm

| Feature | Lợi Ích |
|---------|---------|
| **Quantization** | Giảm 4-8x kích thước model |
| **CPU-friendly** | Chạy không cần GPU |
| **Memory efficient** | Ít RAM hơn raw model |
| **Fast loading** | Load nhanh hơn safetensors |

## Quantization Levels

| Quant | Size Ratio | Quality | RAM Usage |
|-------|------------|---------|-----------|
| Q2_K | ~2.5 bits/w | Thấp ⬇️ | ~2GB |
| Q3_K_S | ~3 bits/w | Medium | ~3GB |
| Q4_K_M | ~4 bits/w | Good ✅ | ~4GB |
| Q5_K_M | ~5 bits/w | Better | ~5GB |
| Q6_K | ~6 bits/w | Very Good | ~6GB |
| Q8_0 | ~8 bits/w | Near Lossless | ~8GB |
| F16 | 16 bits/w | Lossless | ~14GB |
| F32 | 32 bits/w | Full | ~28GB |

**Khuyến nghị:** `Q4_K_M` - cân bằng giữa quality và performance

## Models Khuyến Nghị

### 🏆 Top Picks (2024)

#### 1. Tiny/Small (< 3B) - Fast & Light

| Model | Params | Q4 Size | RAM | Use Case |
|-------|--------|---------|-----|----------|
| **Qwen3.5-0.8B** | 0.8B | ~0.6GB | ~2GB | Ultra-fast, multilingual ⭐ |
| **TinyLlama-1.1B** | 1.1B | ~0.7GB | ~2GB | Ultra-fast inference |
| **Phi-2** | 2.7B | ~1.7GB | ~3GB | Quick tasks, embedding |
| **Qwen1.5-1.8B** | 1.8B | ~1.2GB | ~2.5GB | Multilingual |
| **Gemma-2b** | 2B | ~1.4GB | ~3GB | Google quality |

```bash
# Download Phi-2 Q4_K_M
huggingface-cli download TheBloke/phi-2-GGUF phi-2.Q4_K_M.gguf
```

#### 2. Medium (3B-8B) - Sweet Spot ⭐

| Model | Params | Q4 Size | RAM | Use Case |
|-------|--------|---------|-----|----------|
| **Phi-3-mini** | 3.8B | ~2.3GB | ~4GB | Best small model 🏆 |
| **Mistral-7B** | 7B | ~4.1GB | ~6GB | General purpose |
| **Llama-3-8B** | 8B | ~4.9GB | ~7GB | Meta quality |
| **Gemma-7B** | 7B | ~4.4GB | ~6GB | Google alternative |
| **Qwen1.5-7B** | 7B | ~4.2GB | ~6GB | Multilingual 🌏 |

```bash
# Download Phi-3-mini Q4_K_M
huggingface-cli download TheBloke/Phi-3-mini-4k-instruct-GGUF phi-3-mini-4k-instruct.Q4_K_M.gguf

# Download Llama-3-8B Q4_K_M
huggingface-cli download QuantFactory/Meta-Llama-3-8B-Instruct-GGUF Meta-Llama-3-8B-Instruct.Q4_K_M.gguf
```

#### 3. Large (10B-20B) - High Quality

| Model | Params | Q4 Size | RAM | Use Case |
|-------|--------|---------|-----|----------|
| **Mistral-Nemo-12B** | 12B | ~7GB | ~10GB | Extended context |
| **Yi-34B** | 34B | ~20GB | ~24GB | High quality |

### 📊 Comparison Table

```
Model          | Q4 Size | RAM    | Speed (tok/s) | Quality
---------------|---------|--------|---------------|----------
Phi-2          | 1.7 GB  | 3 GB   | ~50           | ⭐⭐⭐
Phi-3-mini     | 2.3 GB  | 4 GB   | ~40           | ⭐⭐⭐⭐
Mistral-7B     | 4.1 GB  | 6 GB   | ~25           | ⭐⭐⭐⭐
Llama-3-8B     | 4.9 GB  | 7 GB   | ~20           | ⭐⭐⭐⭐⭐
```

## Download Sources

### HuggingFace Repos

| Publisher | Quality | Notes |
|-----------|---------|-------|
| **TheBloke** | ⭐⭐⭐⭐⭐ | Most quantized models |
| **QuantFactory** | ⭐⭐⭐⭐⭐ | High quality quants |
| **bartowski** | ⭐⭐⭐⭐ | New models fast |
| **MaziyarPanahi** | ⭐⭐⭐⭐ | Good selection |

### Search Commands

```bash
# Search for GGUF models
huggingface.co/models?search=gguf

# Specific model
huggingface.co/models?search=phi-3-gguf
huggingface.co/models?search=llama-3-gguf
```

## Usage với Ochi Core

### 1. Setup Model Path

```rust
use ochi_core::ai::{GGUFModel, GGUFConfig};

let config = GGUFConfig {
    model_path: "models/phi-3-mini.Q4_K_M.gguf".to_string(),
    context_size: 2048,
    n_gpu_layers: 0,  // CPU-only
    temperature: 0.7,
    max_tokens: 512,
};

let model = GGUFModel::load("models/phi-3-mini.Q4_K_M.gguf", config)?;
let output = model.generate("Hello, how are you?")?;
```

### 2. FFI Usage (Go Side)

```go
package main

import "C"

// Load model
C.ffi_model_load(
    ctx,
    C.CString("models/phi-3-mini.Q4_K_M.gguf"),
    C.int(2048),  // context_size
    C.int(0),     // n_gpu_layers
)

// Generate
result := C.ffi_model_generate(ctx, C.CString("Hello!"))
```

### 3. Model Directory Structure

```
ochi/
├── models/
│   ├── phi-3-mini.Q4_K_M.gguf    # Active model
│   ├── llama-3-8B.Q4_K_M.gguf    # Backup
│   └── downloads/                 # Temp downloads
└── workers/
    └── agents/
```

## Performance Benchmarks

### M1/M2 Mac (Unified Memory)

```
Model          | Q4 | Tokens/sec | RAM
---------------|----|------------|-----
Phi-3-mini     | 4  | ~40 tok/s  | 4GB
Llama-3-8B     | 4  | ~25 tok/s  | 7GB
```

### Intel/AMD CPU

```
Model          | Q4 | Tokens/sec | RAM
---------------|----|------------|-----
Phi-2          | 4  | ~15 tok/s  | 3GB
Phi-3-mini     | 4  | ~12 tok/s  | 4GB
Mistral-7B     | 4  | ~8 tok/s   | 6GB
```

### With GPU (CUDA/Metal)

```
Model          | Q4 | Tokens/sec | VRAM
---------------|----|------------|-----
Phi-3-mini     | 4  | ~100 tok/s | 4GB
Llama-3-8B     | 4  | ~60 tok/s  | 7GB
```

## Recommendations by Hardware

### 🖥️ Low-End (4-8GB RAM)

```
✅ Phi-2 Q3_K_S (1.5GB)
✅ TinyLlama Q4_K_M (0.7GB)
✅ Phi-3-mini Q3_K_S (2GB)
```

### 💻 Mid-Range (8-16GB RAM)

```
✅ Phi-3-mini Q4_K_M (2.3GB) ⭐
✅ Mistral-7B Q4_K_M (4.1GB)
✅ Llama-3-8B Q4_K_M (4.9GB)
```

### 🚀 High-End (16-32GB RAM)

```
✅ Llama-3-8B Q6_K (6GB)
✅ Mistral-Nemo-12B Q4_K_M (7GB)
✅ Yi-34B Q3_K_S (14GB)
```

### 🎮 With GPU (8GB+ VRAM)

```
✅ Offload all layers to GPU
✅ Use higher quantization (Q6_K, Q8_0)
✅ Run larger models (20B+)
```

## Quick Start Script

```bash
#!/bin/bash
# download-model.sh

MODEL=${1:-"phi-3-mini"}
QUANT=${2:-"Q4_K_M"}

echo "Downloading $MODEL ($QUANT)..."

huggingface-cli download TheBloke/${MODEL}-GGUF ${MODEL}.${QUANT}.gguf \
  --local-dir ./models \
  --local-dir-use-symlinks false

echo "✅ Downloaded to models/${MODEL}.${QUANT}.gguf"
```

```bash
# Usage
./download-model.sh phi-3-mini Q4_K_M
./download-model.sh llama-3-8B Q4_K_M
```

## Tips & Tricks

### 1. Model Selection

- **Code tasks:** Phi-3, StarCoder
- **Chat:** Llama-3-Instruct, Phi-3-Instruct
- **Multilingual:** Qwen1.5, Gemma
- **Embedding:** all-MiniLM-L6-v2 (GGUF)

### 2. Memory Optimization

```rust
// Use smaller context
config.context_size = 1024;  // Instead of 2048

// Use lower quantization
config.quantization = "Q3_K_S";  // Instead of Q4_K_M

// CPU threads
config.n_threads = 4;  // Match your CPU cores
```

### 3. Warm-up Model

```rust
// Run a dummy inference to load model
model.generate("warmup")?;
```

## Troubleshooting

### Model Too Large

```
Error: Not enough memory
Solution: Use Q3_K_S or smaller model
```

### Slow Inference

```
Solution 1: Reduce context_size
Solution 2: Use smaller model
Solution 3: Enable GPU offloading
```

### Bad Quality

```
Solution 1: Use higher quantization (Q5_K_M+)
Solution 2: Try different model
Solution 3: Increase temperature (0.7 → 0.8)
```

---

**Resources:**
- [HuggingFace GGUF Search](https://huggingface.co/models?search=gguf)
- [TheBloke's Models](https://huggingface.co/TheBloke)
- [llama.cpp Docs](https://github.com/ggerganov/llama.cpp)
- [GGUF Format Spec](https://github.com/ggerganov/ggml/blob/master/docs/gguf.md)
