# CUDA Setup Guide for Windows

> Setup GPU acceleration for Ochi Core with NVIDIA GTX 1050 Ti

## Hardware Requirements

- **GPU:** NVIDIA GTX 1050 Ti (4GB VRAM) ✅
- **CUDA Compute Capability:** 6.1 (Pascal architecture)
- **Driver:** NVIDIA Game Ready Driver (latest)

## Step-by-Step Setup

### 1. Install NVIDIA Driver

**Check current driver:**
```bash
nvidia-smi
```

**Update if needed:**
- Download từ: https://www.nvidia.com/Download/index.aspx
- Search: GeForce GTX 1050 Ti
- Install Game Ready Driver

### 2. Install CUDA Toolkit

**Option A: CUDA 12.3 (Recommended)**

```bash
# Download: https://developer.nvidia.com/cuda-12-3-0-download-archive
# Chọn: Windows → 10/11 → x86_64 → exe (local)

# Direct link (12.3.2):
# https://developer.download.nvidia.com/compute/cuda/12.3.2/local_installers/cuda_12.3.2_546.12_windows.exe
```

**Option B: Chocolatey (Quick)**
```bash
choco install cuda --version=12.3.2
```

**Install Location:**
```
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3
```

### 3. Verify CUDA Installation

```bash
# Check CUDA version
nvcc --version

# Should show:
# Cuda compilation tools, release 12.3, V12.3.103
```

### 4. Set Environment Variables

Add to System Environment Variables:

```bash
CUDA_PATH = C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3
CUDA_PATH_V12_3 = C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3
CUDA_PATH_V12 = C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3

# Add to PATH:
%CUDA_PATH%\bin
%CUDA_PATH%\libnvvp
```

**PowerShell (temporary for current session):**
```powershell
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3"
$env:Path += ";$env:CUDA_PATH\bin"
```

### 5. Install Visual Studio Build Tools

**Required Components:**
- MSVC v143 - VS 2022 C++ x64/x86 build tools
- Windows 10/11 SDK
- C++ CMake tools

Download: https://visualstudio.microsoft.com/downloads/

### 6. Build Ochi Core (GPU Optional)

```bash
# Navigate to project
cd e:\Ochi

# Build release
cargo build --release

# Or for development
cargo build --workspace
```

### 7. Test GPU Detection

```bash
# Run hardware detection test
cargo test -p ochi-core hardware::detector::tests::test_detect_hardware -- --nocapture
```

**Expected Output:**
```
CPU: Intel Xeon E3-1231 v3 (4 cores, 8 threads)
Memory: 32GB total, 28GB available
GPU: GeForce GTX 1050 Ti (768 CUDA cores, 4096MB VRAM)
→ GPU acceleration: ENABLED
```

## Troubleshooting

### Error: "No CUDA toolset found"

**Solution:**
1. Make sure CUDA Toolkit is installed
2. Check environment variables
3. Restart terminal/IDE

### Error: "Unable to find libclang"

**Solution:**
```bash
choco install llvm
```

### Error: "CUDA initialization failed"

**Solution:**
1. Update NVIDIA driver
2. Check GPU is detected: `nvidia-smi`
3. Restart computer

### Error: "Out of memory" (VRAM)

**Solution:**
```rust
// Reduce GPU layers
let mut config = CandleConfig::balanced("models/qwen3.5-0.8b.gguf");
config.context_size = 2048;
config.cpu_only = false;
```

## Performance Expectations

### GTX 1050 Ti (4GB VRAM)

| Model | Quant | GPU Layers | Speed |
|-------|-------|------------|-------|
| Qwen3.5-0.8B | IQ4_NL | All (~30) | ~80-100 tok/s |
| Phi-3-mini | Q4_K_M | Most (~25) | ~50-70 tok/s |
| Llama-3-8B | Q4_K_M | ~20-25 | ~25-35 tok/s |
| Mistral-7B | Q4_K_M | ~25-30 | ~30-40 tok/s |

## Quick Test Script

Create `examples/gpu_test.rs`:

```rust
use ochi_core::AutoTuner;
use ochi_llm::{CandleModel, CandleConfig};

fn main() {
    // Detect hardware
    let tuner = AutoTuner::new().unwrap();
    tuner.print_summary();
    
    // Load model with auto-tuned config
let config = CandleConfig::balanced("models/qwen3.5-0.8b.gguf");
let model = CandleModel::load("models/qwen3.5-0.8b.gguf", config).unwrap();
    
    // Test inference
    let output = model.generate("Hello, how are you?").unwrap();
    println!("Response: {}", output);
}
```

Run:
```bash
cargo run --example gpu_test
```

## Optimization Tips

### 1. Maximize GPU Offload

```rust
// Use GPU if available
let mut config = CandleConfig::speed("models/qwen3.5-0.8b.gguf");
config.cpu_only = false;
```

### 2. Balance VRAM Usage

```rust
// For larger models
let mut config = CandleConfig::balanced("models/llama-3-8b.gguf");
config.context_size = 2048;
config.cpu_only = false;
```

### 3. Multi-Model Setup

```rust
// Run multiple small models
let config1 = CandleConfig::speed("models/qwen3.5-0.8b.gguf");
let config2 = CandleConfig::speed("models/phi-2.gguf");
```

## Next Steps

1. ✅ Install CUDA Toolkit
2. ✅ Build (no feature flags)
3. ✅ Test GPU detection
4. 📚 Read [docs/rust2go.md](docs/rust2go.md) for Go integration
5. 📚 Read [docs/gguf-models.md](docs/gguf-models.md) for model selection

---

**Resources:**
- [CUDA Toolkit Docs](https://docs.nvidia.com/cuda/)
- [Candle Docs](https://docs.rs/candle-core)
- [NVIDIA Developer](https://developer.nvidia.com/)
