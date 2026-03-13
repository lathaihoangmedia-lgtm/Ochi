# 🚀 Ochi Core - One Click Setup

> **SETUP TỰ ĐỘNG HOÀN TOÀN - CHẠY 1 LỆNH DUY NHẤT!**

---

## 🎯 Quick Start (3 Bước)

### Bước 1: Download & Giải Nén

```bash
# Download project (nếu chưa có)
# Hoặc clone từ GitHub
git clone https://github.com/your-repo/ochi.git
cd ochi
```

### Bước 2: Chạy Setup (1 lệnh duy nhất!)

**PowerShell (Administrator):**

```powershell
.\SETUP.ps1
```

**Hoặc click đúp:**
```
SETUP.bat  (Right-click → Run as Administrator)
```

### Bước 3: Chạy Demo

```bash
cargo run --example demo --features cuda
```

---

## 📦 Setup Script Tự Động Làm Gì?

### ✅ Check & Install Prerequisites

1. **Visual Studio Build Tools** - Download & install tự động
2. **Rust** - Install nếu chưa có
3. **Chocolatey** - Install nếu chưa có

### ✅ Install Dependencies

4. **LLVM/Clang 17+** - Build dependency
5. **CUDA Toolkit 12.x** - GPU acceleration
6. **NVIDIA Driver** - Update nếu cần

### ✅ Setup Environment

7. Add PATH variables
8. Download sample model
9. Build project
10. Run tests

---

## 🔧 Scripts Available

| Script | Description | Time |
|--------|-------------|------|
| `SETUP.ps1` | **Full auto setup** | ~15-20 min |
| `SETUP.bat` | Same as above (BAT version) | ~15-20 min |
| `scripts\quick-setup.ps1` | Quick setup (skip checks) | ~10 min |
| `scripts\download-models.bat` | Download models only | ~5 min |

---

## 🎮 Manual Setup (Nếu Auto Failed)

### 1. Install Visual Studio Build Tools

```bash
# Download: https://visualstudio.microsoft.com/downloads/
# Install: "Desktop development with C++"
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3. Install Dependencies

```powershell
# Run as Administrator
choco install llvm cuda -y
```

### 4. Build

```bash
cargo build --release --features cuda
```

---

## 📊 Setup Time Estimates

| Step | Time | Notes |
|------|------|-------|
| Check prerequisites | 30 sec | |
| Install Rust | 2-3 min | If not installed |
| Install LLVM | 3-5 min | |
| Install CUDA | 10-15 min | Large download |
| Build project | 5-10 min | First time |
| **Total** | **~20-30 min** | Varies by internet |

---

## ✅ Verification Checklist

Sau khi setup xong, check:

```bash
# Rust installed?
rustc --version

# LLVM installed?
clang --version

# CUDA installed?
nvcc --version

# NVIDIA detected?
nvidia-smi

# Build successful?
cargo build --features cuda

# Demo runs?
cargo run --example demo --features cuda
```

---

## 🛠️ Troubleshooting

### Setup fails at "Visual Studio Build Tools"

```bash
# Manual install:
# 1. Download: https://visualstudio.microsoft.com/downloads/
# 2. Install "Desktop development with C++"
# 3. Re-run SETUP.ps1
```

### Setup fails at "CUDA"

```bash
# Check NVIDIA GPU:
nvidia-smi

# If no GPU, build CPU-only:
cargo build --features ai
```

### Build fails with "link.exe not found"

```bash
# Install Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/
```

### "Permission denied" errors

```bash
# Run as Administrator!
# Right-click PowerShell → Run as Administrator
```

---

## 📞 Need Help?

1. Check logs: `setup.log`
2. Read: [REQUIREMENTS.md](REQUIREMENTS.md)
3. Read: [START-HERE.md](START-HERE.md)
4. Open GitHub Issue

---

## 🎉 Success Indicators

Setup thành công khi thấy:

```
✅ Rust installed
✅ LLVM installed
✅ CUDA installed
✅ Build successful
✅ Demo runs
✅ All tests pass
```

---

**Next:** Run `cargo run --example demo` to test!

**Enjoy! 🚀**
