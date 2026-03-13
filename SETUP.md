# Setup Guide for Ochi Core

## Quick Setup (Windows)

### 1. Install LLVM/Clang (Required)

**Option A: Chocolatey (Recommended - 1 command)**
```bash
choco install llvm
```

**Option B: Download Installer**
1. Download: https://releases.llvm.org/download.html
2. Get: **LLVM-17.0.6-win64.exe** (or latest)
3. Install to: `C:\Program Files\LLVM`
4. Add to PATH: `C:\Program Files\LLVM\bin`

**Option C: Scoop**
```bash
scoop install llvm
```

**Verify:**
```bash
clang --version
# Should show: clang version 17.x.x
```

### 2. Install CUDA Toolkit (For GPU Acceleration)

**Required for:** NVIDIA GPU (GTX 1050 Ti, RTX series, etc.)

**Option A: Chocolatey**
```bash
choco install cuda --version=12.3.2
```

**Option B: Download Installer**
1. Download: https://developer.nvidia.com/cuda-12-3-0-download-archive
2. Select: Windows → 10/11 → x86_64 → exe (local)
3. Install to default location

**Verify:**
```bash
nvcc --version
nvidia-smi
```

### 3. Build Ochi Core

```bash
# CPU-only build (no CUDA)
cargo build --features ai

# Full build with CUDA
cargo build --features cuda

# Release build
cargo build --release --features cuda
```

---

## Hardware Requirements

### Minimum
- **CPU:** 4 cores
- **RAM:** 8GB
- **Storage:** 1GB for models

### Recommended
- **CPU:** 8 cores (Xeon E3, i7, Ryzen 7)
- **RAM:** 16-32GB
- **GPU:** NVIDIA 4GB+ VRAM (GTX 1050 Ti, RTX 3060, etc.)

### Your System (E3-1231 v3 + GTX 1050 Ti + 32GB RAM) ✅
- **CPU:** Xeon E3-1231 v3 (4c/8t) - Good
- **RAM:** 32GB DDR3 - Excellent
- **GPU:** GTX 1050 Ti 4GB - Good for models ≤ 8B

---

## Troubleshooting

### Error: Unable to find libclang

```
error: Unable to find libclang: "couldn't find any valid shared libraries"
```

**Solution:**
```bash
# Install LLVM
choco install llvm

# Or set LIBCLANG_PATH manually
set LIBCLANG_PATH=C:\Program Files\LLVM\bin
```

### Error: CUDA not found

```
error: CUDA toolkit not found
```

**Solution:**
1. Install CUDA Toolkit 12.3
2. Restart terminal
3. Verify: `nvcc --version`

### Error: Out of memory (VRAM)

```
Error: CUDA out of memory
```

**Solution:**
```rust
// Reduce GPU layers
let config = GGUFConfig::balanced("model.gguf")
    .with_gpu_layers(20);  // Instead of 999
```

---

## Next Steps

1. ✅ Install LLVM (choco install llvm)
2. ✅ Install CUDA (choco install cuda)
3. ✅ Build: `cargo build --features cuda`
4. 📚 Read [USAGE-AI.md](USAGE-AI.md) for usage guide
5. 📚 Read [SETUP-CUDA.md](SETUP-CUDA.md) for detailed CUDA setup

---

**Quick Test:**
```bash
# After setup, test build
cargo build --features cuda

# Test hardware detection
cargo test --features cuda hardware::detector::tests::test_detect_hardware -- --nocapture
```
