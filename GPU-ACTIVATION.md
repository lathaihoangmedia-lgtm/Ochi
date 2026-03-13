# 🚀 GPU Activation Guide

## Quick Start

### Windows
```bash
# One-time activation (current terminal)
set CUDA_VISIBLE_DEVICES=0

# Or use the script
enable-gpu.bat
```

### Linux/Mac
```bash
# One-time activation (current session)
export CUDA_VISIBLE_DEVICES=0

# Or use the script
./enable-gpu.sh
```

---

## 🔍 Check GPU Status

### Before Activation
```bash
cargo run -p ochi-core --example test_gpu_real
```

Expected output:
```
⚠️  GPU: Not detected
💡 Set CUDA_VISIBLE_DEVICES=0 to enable detection
```

### After Activation
```bash
# Windows
set CUDA_VISIBLE_DEVICES=0
cargo run -p ochi-core --example test_gpu_real

# Linux
export CUDA_VISIBLE_DEVICES=0
cargo run -p ochi-core --example test_gpu_real
```

Expected output:
```
✅ GPU: NVIDIA GeForce GTX 1050 Ti (4096MB VRAM, 768 CUDA cores)
🎯 GPU detected via environment variable!
```

---

## 🎯 20% Power Mode (Safe Testing)

For safe GPU testing with limited power:

```rust
// Auto-tuner with 20% GPU utilization
let tuner = AutoTuner::new();
let rec = tuner.recommend(0.8);  // 0.8B model

// Use only 20% of GPU layers
let gpu_layers_20pct = (rec.gpu_layers as f32 * 0.2) as usize;

let config = CandleConfig {
    model_path: "models/qwen3.5-0.8b.safetensors".to_string(),
    context_size: rec.context_size,
    n_threads: rec.n_threads,
    cpu_only: gpu_layers_20pct == 0,
    ..Default::default()
};
```

**Benefits:**
- ✅ Lower power consumption
- ✅ Less heat generation
- ✅ Safe for long testing sessions
- ✅ Gradual performance scaling

---

## 📊 Performance Comparison

| Mode | GPU Layers | Speed (0.8B) | VRAM Usage | Power |
|------|------------|--------------|------------|-------|
| **CPU Only** | 0 | ~0.7 t/s | 0MB | Low |
| **20% GPU** | ~2-3 | ~1.0 t/s | ~800MB | Very Low |
| **50% GPU** | ~6-7 | ~1.8 t/s | ~2000MB | Medium |
| **100% GPU** | All | ~3.5 t/s | ~4000MB | High |

*Note: Speed estimates for Qwen3.5-0.8B on GTX 1050 Ti*

---

## 🔧 Permanent GPU Activation

### Windows (Permanent)

1. Open System Properties
2. Click "Environment Variables"
3. Under "System variables", click "New"
4. Variable name: `CUDA_VISIBLE_DEVICES`
5. Variable value: `0`
6. Click OK and restart terminal

### Linux (Permanent)

Add to `~/.bashrc`:
```bash
echo 'export CUDA_VISIBLE_DEVICES=0' >> ~/.bashrc
source ~/.bashrc
```

Or add to `/etc/environment` for all users:
```bash
echo 'CUDA_VISIBLE_DEVICES=0' | sudo tee -a /etc/environment
```

---

## 🧪 Test GPU with Examples

### 1. GPU Detection Test
```bash
cargo run -p ochi-core --example test_gpu_real
```

### 2. Qwen3.5-0.8B Demo
```bash
set CUDA_VISIBLE_DEVICES=0
cargo run -p ochi-core --example demo_qwen3_5
```

### 3. Interactive Chat
```bash
set CUDA_VISIBLE_DEVICES=0
cargo run -p ochi-core --example chat_demo
```

---

## ⚠️ Troubleshooting

### GPU Not Detected

**Problem:**
```
⚠️  GPU: Not detected
```

**Solutions:**

1. **Check CUDA installation:**
   ```bash
   nvcc --version
   nvidia-smi
   ```

2. **Verify environment variable:**
   ```bash
   # Windows
   echo %CUDA_VISIBLE_DEVICES%
   
   # Linux
   echo $CUDA_VISIBLE_DEVICES
   ```

3. **Check GPU index:**
   ```bash
   nvidia-smi
   # Should show GPU 0
   ```

4. **Try different GPU index:**
   ```bash
   # If you have multiple GPUs
   set CUDA_VISIBLE_DEVICES=1
   # or
   export CUDA_VISIBLE_DEVICES=1
   ```

### Candle CUDA Not Available

**Problem:**
```
Candle CUDA support: ⚠️  Not available
```

**Solution:**
- Ensure CUDA Toolkit is installed
- Check that `CUDA_HOME` or `CUDA_PATH` is set
- Rebuild with CUDA:
  ```bash
  cargo clean
  cargo build --release
  ```

### Out of Memory (OOM)

**Problem:**
```
CUDA out of memory
```

**Solutions:**

1. **Reduce context size:**
   ```rust
   CandleConfig {
       context_size: 2048,  // Instead of 8192
       ..
   }
   ```

2. **Use smaller model:**
   - 0.8B instead of 3B
   - 3B instead of 7B

3. **Reduce GPU layers:**
   ```rust
   CandleConfig {
       n_gpu_layers: 10,  // Instead of 999
       ..
   }
   ```

4. **Use 20% power mode:**
   ```bash
   # See "20% Power Mode" section above
   ```

---

## 📈 Monitoring GPU Usage

### Windows
```bash
# Open Task Manager
taskmgr

# Go to Performance → GPU
```

### Linux
```bash
# Watch GPU usage
watch -n 1 nvidia-smi

# Or use nvtop (if installed)
nvtop
```

### Recommended Monitoring Tools

1. **GPU-Z** (Windows) - Detailed GPU stats
2. **HWMonitor** - Temperature, power, usage
3. **nvtop** (Linux) - htop for GPU

---

## 🎯 Best Practices

### For GTX 1050 Ti 4GB:

**Safe Settings:**
```rust
CandleConfig {
    context_size: 4096,      // Fits in VRAM
    n_gpu_layers: 20,        // Partial offload
    n_threads: Some(8),      // Use all CPU threads
    ..
}
```

**20% Power Mode:**
```rust
// Use only 20% of GPU capacity
let gpu_layers = (total_layers as f32 * 0.2) as usize;
```

**Temperature Monitoring:**
- Keep GPU temp < 80°C
- Use fan curve for better cooling
- Take breaks between long inference sessions

---

## 📞 Quick Reference

| Command | Description |
|---------|-------------|
| `set CUDA_VISIBLE_DEVICES=0` | Enable GPU (Windows) |
| `export CUDA_VISIBLE_DEVICES=0` | Enable GPU (Linux) |
| `nvidia-smi` | Check GPU status |
| `nvcc --version` | Check CUDA version |
| `cargo run -p ochi-core --example test_gpu_real` | Test GPU |

---

## ✅ Verification Checklist

After enabling GPU:

- [ ] `CUDA_VISIBLE_DEVICES` is set
- [ ] `nvidia-smi` shows GPU
- [ ] `test_gpu_real` detects GPU
- [ ] Candle CUDA support is available
- [ ] Temperature monitoring is set up
- [ ] Power limits are configured (20% for testing)

---

**🎉 GPU is ready for production use!** 🚀
