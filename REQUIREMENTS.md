# System Requirements

## Minimum Requirements

| Component | Requirement | Notes |
|-----------|-------------|-------|
| **OS** | Windows 10/11 (64-bit) | |
| **CPU** | 4 cores | Xeon E3, i5, Ryzen 5 |
| **RAM** | 8GB | 16GB+ recommended |
| **Storage** | 5GB free | For models + dependencies |
| **GPU** | Optional | NVIDIA for GPU acceleration |

## Recommended Requirements

| Component | Requirement | Notes |
|-----------|-------------|-------|
| **OS** | Windows 11 (64-bit) | |
| **CPU** | 8 cores | Xeon E3, i7, Ryzen 7 |
| **RAM** | 32GB | For large models |
| **Storage** | 20GB+ SSD | Faster model loading |
| **GPU** | NVIDIA 4GB+ VRAM | GTX 1050 Ti, RTX 3060+ |

## Your System (Reference)

```
CPU: Xeon E3-1231 v3 (4 cores/8 threads)
RAM: 32GB DDR3
GPU: NVIDIA GTX 1050 Ti (4GB VRAM)
Status: ✅ Fully Compatible
```

---

## Software Dependencies

### Auto-Installed by Setup Script

| Software | Version | Purpose | Auto-Install |
|----------|---------|---------|--------------|
| **Rust** | Latest | Core language | ✅ (if needed) |
| **LLVM/Clang** | 17+ | Build dependency | ✅ |
| **CUDA Toolkit** | 12.x | GPU acceleration (optional) | ✅ |
| **Chocolatey** | Latest | Package manager | ✅ (if needed) |

### Pre-Requirements

| Software | Required | Install Link |
|----------|----------|--------------|
| **Visual Studio Build Tools** | ✅ | [Download](https://visualstudio.microsoft.com/downloads/) |
| **NVIDIA Driver** | ✅ (for GPU) | [Download](https://www.nvidia.com/Download/index.aspx) |

---

## Quick Check Commands

```powershell
# Check Rust
rustc --version

# Check LLVM (after install)
clang --version

# Check CUDA (after install, optional)
nvcc --version

# Check NVIDIA GPU (optional)
nvidia-smi
```

---

## Performance Expectations

### With GTX 1050 Ti (4GB VRAM)

| Model | Quant | VRAM | Speed |
|-------|-------|------|-------|
| Qwen3.5-0.8B | IQ4_NL | ~1GB | ~80-100 tok/s |
| Phi-3-mini | Q4_K_M | ~2.5GB | ~50-70 tok/s |
| Llama-3-8B | Q4_K_M | ~4GB | ~25-35 tok/s |

### CPU-Only (32GB RAM)

| Model | Quant | RAM | Speed |
|-------|-------|-----|-------|
| Qwen3.5-0.8B | IQ4_NL | ~1GB | ~30 tok/s |
| Phi-3-mini | Q4_K_M | ~3GB | ~20 tok/s |
| Llama-3-8B | Q4_K_M | ~6GB | ~10 tok/s |

---

## Troubleshooting

### "Not enough storage"
- Free up disk space (need 5GB+)
- Use smaller quantization (Q3_K_S instead of Q4_K_M)

### "GPU out of memory"
- Reduce context size
- Use smaller model
- Close other GPU applications

### "Build failed"
- Install Visual Studio Build Tools
- Run setup as Administrator
- Restart terminal after install

---

**Next:** See [START-HERE.md](START-HERE.md) for setup instructions
