# Kiến Trúc rust2go

> Rust Core + Go Agents Integration

## Tổng Quan

Kiến trúc **rust2go** cho phép tích hợp Rust core library với Go agents thông qua C FFI (Foreign Function Interface).

```
┌─────────────────────────────────────────────────────────────┐
│                     Go Agents Layer                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Agent 1   │  │   Agent 2   │  │   Agent 3   │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │                 │
│         └────────────────┴────────────────┘                 │
│                           │                                 │
│                    CGO Bridge                               │
└───────────────────────────┼─────────────────────────────────┘
                            │ FFI Calls
┌───────────────────────────┼─────────────────────────────────┐
│                    Rust Core Layer                          │
│                           │                                 │
│  ┌────────────────────────┴────────────────────────┐       │
│  │              ochi-core (FFI)                    │       │
│  │  ┌─────────────┐  ┌─────────────┐              │       │
│  │  │  GGUF Model │  │   Utils     │              │       │
│  │  └─────────────┘  └─────────────┘              │       │
│  └─────────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

## Tại Sao rust2go?

| Rust | Go |
|------|-----|
| ✅ Memory safety | ✅ Fast compilation |
| ✅ Zero-cost abstractions | ✅ Simple concurrency |
| ✅ GGUF inference (llama-cpp-rs) | ✅ Easy deployment |
| ✅ FFI-safe | ✅ Rich ecosystem |

## Cấu Trúc Project

```
ochi/
├── crates/
│   └── ochi-core/
│       ├── src/
│       │   ├── ai/
│       │   │   ├── mod.rs      # AI module
│       │   │   ├── model.rs    # GGUF model wrapper
│       │   │   └── ffi.rs      # C FFI bindings ⭐
│       │   ├── lib.rs
│       │   └── error.rs
│       └── Cargo.toml
├── workers/
│   └── agents/
│       ├── main.go             # Go agents entry
│       ├── rust.go             # CGO bindings
│       └── go.mod
└── docs/
    └── rust2go.md              # This file
```

## Setup Rust FFI

### 1. Cargo.toml

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = []
ai = []
```

### 2. FFI Module (ffi.rs)

```rust
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};

/// Opaque handle
pub struct FFIContext {
    // Internal state
}

#[no_mangle]
pub extern "C" fn ffi_context_new() -> *mut FFIContext {
    // Create context
}

#[no_mangle]
pub extern "C" fn ffi_context_free(ctx: *mut FFIContext) {
    // Free context
}

#[no_mangle]
pub extern "C" fn ffi_model_generate(
    ctx: *mut FFIContext,
    prompt: *const c_char,
) -> *mut c_char {
    // Generate text
}
```

### 3. Build Rust Library

```bash
# Build as C dynamic library
cargo build --release --features ai

# Output: target/release/libochi_core.so (Linux)
#         target/release/ochi_core.dll (Windows)
#         target/release/libochi_core.dylib (macOS)
```

## Setup Go CGO

### 1. rust.go

```go
package main

/*
#cgo LDFLAGS: -L../target/release -lochi_core
#cgo CFLAGS: -I../crates/ochi-core/src

#include <stdlib.h>

// Forward declarations from Rust FFI
typedef struct FFIContext FFIContext;
FFIContext* ffi_context_new();
void ffi_context_free(FFIContext*);
char* ffi_model_generate(FFIContext*, const char*);
void ffi_string_free(char*);
*/
import "C"
import "unsafe"

type RustCore struct {
    ctx *C.FFIContext
}

func NewRustCore() *RustCore {
    return &RustCore{
        ctx: C.ffi_context_new(),
    }
}

func (r *RustCore) Close() {
    C.ffi_context_free(r.ctx)
}

func (r *RustCore) Generate(prompt string) string {
    cPrompt := C.CString(prompt)
    defer C.free(unsafe.Pointer(cPrompt))
    
    cResult := C.ffi_model_generate(r.ctx, cPrompt)
    defer C.ffi_string_free(cResult)
    
    return C.GoString(cResult)
}
```

### 2. main.go

```go
package main

import (
    "fmt"
    "log"
)

func main() {
    core := NewRustCore()
    defer core.Close()
    
    // Load model
    // core.LoadModel("models/phi-2.Q4_K_M.gguf")
    
    // Generate
    result := core.Generate("Hello, world!")
    fmt.Println(result)
}
```

## FFI Functions Reference

| Function | Description |
|----------|-------------|
| `ffi_context_new()` | Create FFI context |
| `ffi_context_free()` | Free FFI context |
| `ffi_model_load()` | Load GGUF model |
| `ffi_model_generate()` | Generate text |
| `ffi_model_get_vocab()` | Get vocab size |
| `ffi_model_get_ctx()` | Get context size |
| `ffi_string_free()` | Free allocated string |

## Return Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `-1` | Invalid argument |
| `-2` | Already loaded |
| `-3` | Load failed |

## Best Practices

### Rust Side

1. **Use `#[no_mangle]`** for all FFI functions
2. **Use `extern "C"`** for C ABI compatibility
3. **Opaque pointers** - hide internal structure from Go
4. **Memory ownership** - document who frees what
5. **Error handling** - use return codes, not panics

### Go Side

1. **Always defer free** for allocated resources
2. **Use `unsafe.Pointer`** carefully
3. **Wrap C types** in Go structs
4. **Handle NULL returns** from FFI calls

## Workflow

```bash
# 1. Build Rust library
cd crates/ochi-core
cargo build --release --features ai

# 2. Set library path
export LD_LIBRARY_PATH=$(pwd)/../../target/release:$LD_LIBRARY_PATH

# 3. Build Go agents
cd workers/agents
go build

# 4. Run
./agents
```

## Debugging

### Rust Panics in FFI

```rust
use std::panic;

#[no_mangle]
pub extern "C" fn ffi_safe_function(...) -> c_int {
    panic::catch_unwind(|| {
        // Your code here
    }).unwrap_or(-1)
}
```

### Go CGO Debug

```bash
# Enable CGO debug
export CGO_CFLAGS="-g -O0"
export CGO_LDFLAGS="-g"

# Build with debug symbols
go build -x -v
```

## Performance Tips

1. **Batch calls** - minimize FFI crossings
2. **Reuse context** - don't create/destroy frequently
3. **Zero-copy strings** - when possible
4. **Async in Go** - let Go handle concurrency

## Next Steps

- [ ] Add authentication FFI
- [ ] Add streaming response FFI
- [ ] Add model hot-reload FFI
- [ ] Benchmark rust2go overhead

---

**See Also:**
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Go CGO Guide](https://golang.org/cmd/cgo/)
- [llama-cpp-rs Docs](https://docs.rs/llama-cpp-rs)
