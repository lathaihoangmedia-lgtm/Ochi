# Quick Start Guide

## 3 Bước Đơn Giản

### Bước 1: Cài Đặt Dependencies

**Chạy 1 lệnh duy nhất (Administrator):**

```bash
# Right-click scripts\install-all.bat → Run as Administrator
scripts\install-all.bat
```

Script sẽ tự động:
- ✅ Cài LLVM/Clang
- ✅ Cài CUDA Toolkit 12.3
- ✅ Setup environment variables
- ✅ Verify installation

### Bước 2: Build

```bash
scripts\build.bat
```

### Bước 3: Test

```bash
# Test hardware detection
scripts\test-hardware.bat

# Chạy demo
cargo run --example demo --features cuda
```

---

## Nếu Không Muốn Install Tự Động

### Install Thủ Công:

```bash
# 1. LLVM
choco install llvm

# 2. CUDA
choco install cuda --version=12.3.2

# 3. Build
cargo build --features cuda
```

---

## Troubleshooting

### Lỗi: `GH001` / `gh.io/lfs` khi `git push`

Nguyên nhân thường do thư mục `models/` chứa file GGUF >100MB bị track bởi Git.

```bash
git rm --cached -r models
git add .gitignore
git commit -m "chore: stop tracking models"
git push origin main
```

`models/` vẫn ở máy local, chỉ không đẩy lên remote.

### Lỗi: "Unable to find libclang"

```bash
# Install LLVM
choco install llvm

# Restart terminal
# Build lại
cargo build --features cuda
```

### Lỗi: "CUDA not found"

```bash
# Install CUDA
choco install cuda --version=12.3.2

# Restart terminal
# Build lại
cargo build --features cuda
```

### Lỗi: Chocolatey not found

```bash
# Install Chocolatey first
powershell -ExecutionPolicy Bypass -Command "Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"

# Then install dependencies
choco install llvm cuda --version=12.3.2
```

---

## Verify Installation

```bash
# Check LLVM
clang --version

# Check CUDA
nvcc --version
nvidia-smi

# Build
cargo build --features cuda

# Test
cargo test --features cuda
```

---

## Next Steps

1. ✅ Install dependencies
2. ✅ Build project
3. ✅ Run demo: `cargo run --example demo`
4. 📚 Read [USAGE-AI.md](USAGE-AI.md) for detailed usage
5. 🤖 Integrate with Go agents (see docs/rust2go.md)
