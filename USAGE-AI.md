# Ochi Core - AI Usage Guide

> Using GGUF models with CUDA acceleration and auto-tuning

## Quick Start

### 1. Auto-Tune (Recommended)

```rust
use ochi_core::{AutoTuner, GGUFModel, GGUFConfig, TuningProfile};

fn main() {
    // Auto-detect hardware and tune
    let tuner = AutoTuner::new().unwrap();
    tuner.print_summary();
    
    // Get optimal config for your hardware
    let config = tuner.tune(0.8, TuningProfile::Balanced);
    
    // Load model
    let model = GGUFModel::load("models/qwen3.5-0.8b.gguf", config).unwrap();
    
    // Generate
    let output = model.generate("Hello!").unwrap();
    println!("{}", output);
}
```

### 2. Manual Configuration

```rust
use ochi_core::{GGUFModel, GGUFConfig};

fn main() {
    // Quick presets
    let config = GGUFConfig::balanced("models/qwen3.5-0.8b.gguf");
    // or
    let config = GGUFConfig::speed("models/qwen3.5-0.8b.gguf");
    // or
    let config = GGUFConfig::quality("models/qwen3.5-0.8b.gguf");
    
    let model = GGUFModel::load("models/qwen3.5-0.8b.gguf", config).unwrap();
    let output = model.generate("Explain quantum computing").unwrap();
    println!("{}", output);
}
```

### 3. With GPU Offloading

```rust
use ochi_core::{GGUFModel, GGUFConfig};

fn main() {
    let config = GGUFConfig::default()
        .with_gpu_layers(999)      // Offload all to GPU
        .with_context_size(4096)
        .with_temperature(0.7);
    
    let model = GGUFModel::load("models/qwen3.5-0.8b.gguf", config).unwrap();
    let output = model.generate("What is AI?").unwrap();
    println!("{}", output);
}
```

## Configuration Options

### GGUFConfig

```rust
pub struct GGUFConfig {
    pub model_path: String,       // Path to .gguf file
    pub context_size: usize,      // Context window (1024-8192)
    pub n_gpu_layers: usize,      // GPU offload layers (0 = CPU)
    pub temperature: f32,         // Sampling temp (0.0-1.0)
    pub max_tokens: usize,        // Max generation length
    pub n_threads: Option<usize>, // CPU threads (None = auto)
    pub n_batch: usize,           // Batch size (512-2048)
}
```

### Presets

| Preset | Context | Temp | Max Tokens | Use Case |
|--------|---------|------|------------|----------|
| `speed()` | 1024 | 0.5 | 256 | Fast responses |
| `balanced()` | 4096 | 0.7 | 512 | General use ⭐ |
| `quality()` | 8192 | 0.8 | 1024 | High quality |

## Auto-Tuner Profiles

```rust
use ochi_core::TuningProfile;

// Speed-focused
let config = tuner.tune(0.8, TuningProfile::Speed);

// Balanced (default)
let config = tuner.tune(0.8, TuningProfile::Balanced);

// Quality-focused
let config = tuner.tune(0.8, TuningProfile::Quality);

// Custom
let config = tuner.tune(0.8, TuningProfile::Custom {
    context_size: 2048,
    n_gpu_layers: 30,
    temperature: 0.7,
});
```

## Streaming Generation

```rust
use ochi_core::GGUFModel;

let model = GGUFModel::load("models/qwen3.5-0.8b.gguf", config).unwrap();

// Stream tokens as they're generated
model.generate_stream("Tell me a story", |token| {
    print!("{}", token);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    true  // Return false to stop
}).unwrap();
```

## Hardware Detection

```rust
use ochi_core::HardwareInfo;

let hardware = HardwareInfo::detect().unwrap();

println!("CPU: {} ({} cores)", hardware.cpu.name, hardware.cpu.cores);
println!("RAM: {}GB available", hardware.memory.available);

if let Some(gpu) = &hardware.gpu {
    println!("GPU: {} ({}MB VRAM)", gpu.name, gpu.vram_total);
    println!("CUDA Cores: {}", gpu.cuda_cores);
}

// Get recommendations
println!("Recommended model: ~{}B params", hardware.recommended_model_size());
println!("GPU layers: {}", hardware.recommended_gpu_layers(0.8));
```

## FFI Usage (Go Integration)

### Rust Side (already built)

```c
// Functions exported for CGO
ffi_context_new() -> *mut FFIContext
ffi_context_free(ctx: *mut FFIContext)
ffi_model_load(ctx, path, context_size, n_gpu_layers, n_threads) -> int
ffi_model_generate(ctx, prompt) -> *mut char
ffi_string_free(s: *mut char)
ffi_has_gpu() -> int
```

### Go Side

```go
package main

/*
#cgo LDFLAGS: -L../target/release -lochi_core
#include <stdlib.h>

typedef struct FFIContext FFIContext;
FFIContext* ffi_context_new();
void ffi_context_free(FFIContext*);
int ffi_model_load(FFIContext*, const char*, int, int, int);
char* ffi_model_generate(FFIContext*, const char*);
void ffi_string_free(char*);
int ffi_has_gpu();
*/
import "C"
import "unsafe"

type RustCore struct {
    ctx *C.FFIContext
}

func NewRustCore() *RustCore {
    return &RustCore{ctx: C.ffi_context_new()}
}

func (r *RustCore) Close() {
    C.ffi_context_free(r.ctx)
}

func (r *RustCore) LoadModel(path string, ctxSize, gpuLayers, threads int) error {
    cPath := C.CString(path)
    defer C.free(unsafe.Pointer(cPath))
    
    ret := C.ffi_model_load(r.ctx, cPath, C.int(ctxSize), C.int(gpuLayers), C.int(threads))
    if ret != 0 {
        return fmt.Errorf("load failed: %d", ret)
    }
    return nil
}

func (r *RustCore) Generate(prompt string) string {
    cPrompt := C.CString(prompt)
    defer C.free(unsafe.Pointer(cPrompt))
    
    cResult := C.ffi_model_generate(r.ctx, cPrompt)
    defer C.ffi_string_free(cResult)
    
    return C.GoString(cResult)
}

func HasGPU() bool {
    return C.ffi_has_gpu() == 1
}

func main() {
    core := NewRustCore()
    defer core.Close()
    
    // Check GPU
    if HasGPU() {
        fmt.Println("GPU available!")
    }
    
    // Load model with GPU offload
    core.LoadModel("models/qwen3.5-0.8b.gguf", 4096, 999, 0)
    
    // Generate
    result := core.Generate("Hello, world!")
    fmt.Println(result)
}
```

## Build Commands

```bash
# CPU-only build
cargo build --features ai

# CUDA build
cargo build --features cuda

# Release build
cargo build --release --features cuda

# Run tests
cargo test --features cuda

# Check hardware detection
cargo test --features cuda hardware::detector::tests::test_detect_hardware -- --nocapture
```

## Example Configurations

### For GTX 1050 Ti (4GB VRAM)

```rust
// Qwen3.5-0.8B - Full GPU offload
let config = GGUFConfig::balanced("models/qwen3.5-0.8b.gguf")
    .with_gpu_layers(999);

// Llama-3-8B - Partial offload
let config = GGUFConfig::balanced("models/llama-3-8b.gguf")
    .with_gpu_layers(25);

// Mistral-7B - Partial offload
let config = GGUFConfig::balanced("models/mistral-7b.gguf")
    .with_gpu_layers(30);
```

### For CPU-Only (32GB RAM)

```rust
// Any model up to 13B
let config = GGUFConfig::balanced("models/mistral-7b.gguf")
    .with_gpu_layers(0)  // CPU-only
    .with_context_size(4096);
```

## Troubleshooting

### Model fails to load

```rust
// Check file exists
assert!(std::path::Path::new("models/qwen3.5-0.8b.gguf").exists());

// Try with fewer GPU layers
let config = config.with_gpu_layers(20);
```

### Out of memory (VRAM)

```rust
// Reduce GPU layers
let config = config.with_gpu_layers(15);

// Or reduce context
let config = config.with_context_size(2048);
```

### Slow inference

```rust
// Increase GPU layers
let config = config.with_gpu_layers(999);

// Use speed preset
let config = GGUFConfig::speed("models/qwen3.5-0.8b.gguf");
```

---

**See Also:**
- [SETUP-CUDA.md](SETUP-CUDA.md) - CUDA installation
- [docs/rust2go.md](docs/rust2go.md) - Go integration
- [docs/gguf-models.md](docs/gguf-models.md) - Model selection
