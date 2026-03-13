# Ochi AI Agents (Go)

> Go agents powered by Rust AI core

---

## Quick Start

### 1. Build Rust Library

```bash
# From project root
cargo build --release --features ai
```

### 2. Build Go Agent

**Windows:**
```bash
build.bat
```

**Linux/macOS:**
```bash
go build -o ochi-agent
```

### 3. Run Agent

**Windows:**
```bash
ochi-agent.exe
```

**Linux/macOS:**
```bash
./ochi-agent
```

---

## Files

- `main.go` - Main agent program
- `rust.go` - Rust FFI bindings
- `go.mod` - Go module definition
- `build.bat` - Windows build script

---

## Requirements

- **Go 1.21+** - [Download](https://go.dev/dl/)
- **Rust library** - Built from `crates/ochi-core`
- **Model file** - `models/qwen3.5-0.8b.gguf`

---

## Architecture

```
┌─────────────────┐
│   Go Agent      │
│  (main.go)      │
└────────┬────────┘
         │ CGO
         │
┌────────▼────────┐
│  Rust FFI       │
│  (rust.go)      │
└────────┬────────┘
         │ C ABI
         │
┌────────▼────────┐
│  Rust Core      │
│  (ochi_core)    │
└────────┬────────┘
         │
┌────────▼────────┐
│  GGUF Model     │
│  (inference)    │
└─────────────────┘
```

---

## Example Session

```
========================================
Ochi AI Agent (Go + Rust)
========================================

✅ GPU acceleration available

Loading model: ../../models/qwen3.5-0.8b.gguf
✅ Model loaded successfully

Enter your prompts (type 'quit' to exit):

You: Hello!
AI: Hello! How can I help you today?

You: What is AI?
AI: AI stands for Artificial Intelligence. It refers to...

You: quit

Goodbye!
```

---

## Development

### Add New Agent

1. Create new Go file: `agent_name.go`
2. Import `rust.go` bindings
3. Use `RustCore` API

### Example: Multi-Agent

```go
agents := []*RustCore{
    NewRustCore(),
    NewRustCore(),
}

for _, agent := range agents {
    defer agent.Close()
    agent.LoadModel(modelPath, 2048, 999, 0)
    
    response, _ := agent.Generate("Hello!")
    fmt.Println(response)
}
```

---

## Troubleshooting

### "DLL not found"

```bash
# Windows: Add to PATH
set PATH=..\..\target\release;%PATH%
```

### "model not found"

```bash
# Check model exists
dir ..\..\models\qwen3.5-0.8b.gguf
```

### "CGO enabled"

```bash
# Enable CGO
set CGO_ENABLED=1
```

---

## Performance

| Metric | Value |
|--------|-------|
| Speed | ~75 tok/s |
| Memory | ~1.1GB |
| Startup | ~2s |

---

## Next Steps

- [ ] Add streaming support
- [ ] Add agent communication
- [ ] Add load balancing
- [ ] Add agent pool

---

**See Also:**
- [docs/GO-EXAMPLE.md](../../docs/GO-EXAMPLE.md) - Detailed guide
- [docs/rust2go.md](../../docs/rust2go.md) - Architecture
