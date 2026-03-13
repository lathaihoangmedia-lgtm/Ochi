# 🚀 Ochi Core - Bắt Đầu Nhanh

> Đọc file này trước tiên!

---

## 📦 Bạn Có Gì?

✅ **Rust Core** với GGUF inference  
✅ **CUDA support** cho NVIDIA GPU  
✅ **Auto-tuning** theo hardware  
✅ **FFI bindings** cho Go integration  
✅ **Qwen3.5-0.8B model** (đã download sẵn)  
✅ **One-click setup** - Tự động hóa 100%!  

---

## 🎯 One-Click Setup (1 LỆNH DUY NHẤT!)

### Windows (Administrator):

**Option 1: Click đúp file**
```
1. Mở File Explorer
2. Navigate đến: e:\Ochi
3. Right-click SETUP.bat
4. Chọn "Run as administrator"
5. Đợi setup hoàn thành (~15-20 phút)
```

**Option 2: PowerShell**
```powershell
# 1. Mở PowerShell as Administrator
# 2. Chạy:
.\SETUP.ps1
```

### Setup Script Tự Động:
- ✅ Check & install Rust
- ✅ Install LLVM/Clang
- ✅ Install CUDA Toolkit
- ✅ Download sample model
- ✅ Build project
- ✅ Run tests

### Sau Khi Setup Xong:

```bash
# Close terminal và mở lại
# Run demo
cargo run --example demo --features cuda
```

---

## 📁 Scripts Available

| Script | Description |
|--------|-------------|
| `SETUP.bat` | **One-click setup** (Run as Admin) ⭐ |
| `SETUP.ps1` | Same as above (PowerShell) |
| `scripts\build.bat` | Build project |
| `scripts\test-hardware.bat` | Test hardware detection |
| `scripts\download-model.bat` | Download models |

---

## 🎮 Hardware Của Bạn

```
CPU: Xeon E3-1231 v3 (4 cores/8 threads)
RAM: 32GB DDR3
GPU: NVIDIA GTX 1050 Ti (4GB VRAM)
```

### Performance Kỳ Vọng:

| Model | Speed |
|-------|-------|
| Qwen3.5-0.8B | ~80-100 tokens/sec ⚡ |
| Phi-3-mini | ~50-70 tokens/sec |
| Llama-3-8B | ~25-35 tokens/sec |

---

## 📚 Tài Liệu

### Cơ Bản:
1. **START-HERE.md** - Hướng dẫn này ✅
2. **README.md** - Overview dự án
3. **USAGE-AI.md** - Cách dùng AI API

### Nâng Cao:
4. **SETUP-AUTO.md** - One-click setup guide
5. **REQUIREMENTS.md** - System requirements
6. **docs/gguf-models.md** - Chọn model
7. **docs/rust2go.md** - Go integration

---

## 🛠️ Troubleshooting Nhanh

### Lỗi: "Not running as Administrator"
```bash
# Right-click SETUP.bat → Run as administrator
```

### Lỗi: "Unable to find libclang"
```bash
choco install llvm
```

### Lỗi: "CUDA not found"
```bash
choco install cuda -y
```

### Lỗi: "Model not found"
```bash
scripts\download-model.bat
```

---

## ✅ Checklist

- [ ] Chạy `SETUP.bat` (as Admin)
- [ ] Đợi setup hoàn thành (~15-20 phút)
- [ ] Close và mở terminal mới
- [ ] Chạy `cargo run --example demo`
- [ ] Đọc `USAGE-AI.md` để học API
- [ ] Tích hợp với Go agents (docs/rust2go.md)

---

## 🎉 Success!

Nếu thấy output như sau là thành công:

```
=== Hardware Detection ===
CPU: Xeon E3-1231 v3 (4 cores, 8 threads)
Memory: 32GB total, 28GB available
GPU: GeForce GTX 1050 Ti (768 CUDA cores, 4096MB VRAM)
→ GPU acceleration: ENABLED

Setup Complete!
Next: Run: cargo run --example demo --features cuda
```

---

**Cần giúp đỡ?** Check [SETUP-AUTO.md](SETUP-AUTO.md) hoặc mở issue.

**Happy Coding! 🚀**
