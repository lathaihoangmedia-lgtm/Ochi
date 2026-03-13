# Ochi Next - AI-Powered Core

> Rust Workspace với GGUF Inference + CUDA Acceleration + Auto-Tuning

---

## 🚀 One-Click Setup!

### Windows (Administrator):

```bash
# Option 1: Click đúp file SETUP.bat
SETUP.bat

# Option 2: PowerShell
.\SETUP.ps1
```

**Setup tự động:**
- ✅ Check & install Rust
- ✅ Install LLVM/Clang  
- ✅ Install CUDA Toolkit
- ✅ Download sample model
- ✅ Build project
- ✅ Run tests

**Time:** ~15-20 minutes (first time)

---

## 📁 Cấu Trúc

```
ochi/
├── crates/ochi-core/    # Rust core library
│   ├── src/
│   │   ├── ai/          # GGUF inference
│   │   ├── hardware/    # Auto-tune module
│   │   └── lib.rs
│   └── Cargo.toml
├── models/              # GGUF models
│   └── qwen3.5-0.8b.gguf
├── docs/
│   ├── rust2go.md       # Go integration
│   └── gguf-models.md   # Model guide
├── SETUP.ps1            # Auto setup script ⭐
├── SETUP.bat            # Auto setup (BAT version)
├── START-HERE.md        # Quick start
└── README.md            # This file
```

---

## 🔧 Quick Start

### Option 1: One-Click Setup (Recommended)

```bash
# Run as Administrator
SETUP.bat
# or
.\SETUP.ps1
```

### Option 2: Manual Commands

```powershell
# Install dependencies
choco install llvm cuda -y

# Build
cargo build --features cuda

# Run demo
cargo run --example demo --features cuda
```

---

## 📚 Documentation

| File | Description |
|------|-------------|
| **[START-HERE.md](START-HERE.md)** | **BẮT ĐẦU TỪ ĐÂY!** 📖 |
| [SETUP-AUTO.md](SETUP-AUTO.md) | One-click setup guide |
| [REQUIREMENTS.md](REQUIREMENTS.md) | System requirements |
| [USAGE-AI.md](USAGE-AI.md) | AI usage examples |
| [docs/gguf-models.md](docs/gguf-models.md) | Model selection |
| [docs/rust2go.md](docs/rust2go.md) | Go integration |

---

## 🎯 Features

- **GGUF Model Inference** - llama.cpp integration
- **CUDA GPU Acceleration** - NVIDIA support (GTX/RTX)
- **Auto-Tuning** - Tự động optimize theo hardware
- **rust2go FFI** - Go agents integration
- **Lightweight** - Chạy model 0.8B-70B với quantization

---

## 📊 Performance (GTX 1050 Ti 4GB)

| Model | Quant | VRAM | Speed |
|-------|-------|------|-------|
| Qwen3.5-0.8B | IQ4_NL | ~1GB | ~80-100 tok/s ⚡ |
| Phi-3-mini | Q4_K_M | ~2.5GB | ~50-70 tok/s |
| Llama-3-8B | Q4_K_M | ~4GB | ~25-35 tok/s |

---

## 🎮 Hardware Recommendations

| Component | Minimum | Recommended | Your System |
|-----------|---------|-------------|-------------|
| CPU | 4 cores | 8 cores | Xeon E3-1231 v3 ✅ |
| RAM | 8GB | 16-32GB | 32GB DDR3 ✅ |
| GPU | None | 4GB+ VRAM | GTX 1050 Ti 4GB ✅ |

---

## 🧪 Testing

### Quick Test
```bash
# Run test pipeline
.\scripts\test.ps1

# Unit tests only
cargo test --lib --features ai

# Hardware tests
cargo test --features ai hardware -- --nocapture

# Integration tests
cargo test --features ai -- --test-threads=1
```

### CI Pipeline
- ✅ Check & Lint
- ✅ Unit Tests (Parallel)
- ✅ Hardware Tests (Multi-OS)
- ✅ Integration Tests (Sequential)
- ✅ Build Release

See [docs/TESTING.md](docs/TESTING.md) for details.

### Auto-Tuner
```rust
let tuner = AutoTuner::new()?;
let config = tuner.tune(0.8, TuningProfile::Balanced);
```

### Manual Config
```rust
let config = GGUFConfig::balanced("model.gguf")
    .with_gpu_layers(999)  // Full GPU offload
    .with_context_size(4096);
```

### FFI (Go Integration)
```c
ffi_context_new()
ffi_model_load(ctx, path, ctx_size, gpu_layers, threads)
ffi_model_generate(ctx, prompt)
```

---

## 🛠️ Troubleshooting

### Setup fails
```bash
# Run as Administrator
Right-click SETUP.bat → Run as administrator
```

### Build fails
```bash
# Install Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/
# Select: "Desktop development with C++"
```

### CUDA not found
```bash
# Reinstall CUDA
choco install cuda -y
```

---

**License:** MIT  
**Authors:** Ochi Team  
**Stack:** Rust + CUDA + GGUF + Go (FFI)

---

**🎉 Start here:** Run `SETUP.bat` or read [START-HERE.md](START-HERE.md)
