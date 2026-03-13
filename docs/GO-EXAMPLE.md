# Go Agents Integration (rust2go)

> Example: Go agent calling Rust AI core with FFI

---

## Project Structure

```
ochi/
├── crates/ochi-core/       # Rust core library
│   └── src/
│       ├── ai/
│       │   ├── ffi.rs      # C FFI bindings
│       │   └── model.rs    # GGUF model
│       └── lib.rs
├── workers/
│   └── agents/             # Go agents
│       ├── main.go         # Main agent
│       ├── rust.go         # Rust FFI bindings
│       └── go.mod
└── docs/
    └── rust2go.md          # This file
```

---

## Step 1: Build Rust Library

```bash
# Build as C dynamic library
cargo build --release --features ai

# Output:
# - Windows: target/release/ochi_core.dll
# - Linux: target/release/libochi_core.so
# - macOS: target/release/libochi_core.dylib
```

---

## Step 2: Go FFI Bindings

Create `workers/agents/rust.go`:

```go
package main

/*
#cgo LDFLAGS: -L../../target/release -lochi_core
#cgo CFLAGS: -I../../crates/ochi-core/src

#include <stdlib.h>

// Forward declarations from Rust FFI
typedef struct FFIContext FFIContext;
FFIContext* ffi_context_new();
void ffi_context_free(FFIContext*);
int ffi_model_load(FFIContext*, const char*, int, int, int);
char* ffi_model_generate(FFIContext*, const char*);
void ffi_string_free(char*);
int ffi_model_is_loaded(FFIContext*);
int ffi_has_gpu();
*/
import "C"
import (
	"errors"
	"fmt"
	"unsafe"
)

// RustCore wraps the Rust FFI context
type RustCore struct {
	ctx *C.FFIContext
}

// NewRustCore creates a new Rust core instance
func NewRustCore() *RustCore {
	return &RustCore{
		ctx: C.ffi_context_new(),
	}
}

// Close frees the Rust core instance
func (r *RustCore) Close() {
	if r.ctx != nil {
		C.ffi_context_free(r.ctx)
		r.ctx = nil
	}
}

// HasGPU checks if GPU is available
func HasGPU() bool {
	return C.ffi_has_gpu() == 1
}

// LoadModel loads a GGUF model
func (r *RustCore) LoadModel(path string, ctxSize, gpuLayers, threads int) error {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))

	ret := C.ffi_model_load(r.ctx, cPath, C.int(ctxSize), C.int(gpuLayers), C.int(threads))
	
	switch ret {
	case 0:
		return nil
	case -1:
		return errors.New("invalid argument")
	case -2:
		return errors.New("model already loaded")
	case -3:
		return errors.New("failed to load model")
	default:
		return fmt.Errorf("unknown error: %d", ret)
	}
}

// Generate generates text from a prompt
func (r *RustCore) Generate(prompt string) (string, error) {
	cPrompt := C.CString(prompt)
	defer C.free(unsafe.Pointer(cPrompt))

	cResult := C.ffi_model_generate(r.ctx, cPrompt)
	if cResult == nil {
		return "", errors.New("generation failed")
	}
	defer C.ffi_string_free(cResult)

	return C.GoString(cResult), nil
}

// IsModelLoaded checks if a model is loaded
func (r *RustCore) IsModelLoaded() bool {
	return C.ffi_model_is_loaded(r.ctx) == 1
}
```

---

## Step 3: Go Agent

Create `workers/agents/main.go`:

```go
package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	fmt.Println("========================================")
	fmt.Println("Ochi AI Agent (Go + Rust)")
	fmt.Println("========================================")
	fmt.Println()

	// Check GPU
	if HasGPU() {
		fmt.Println("✅ GPU acceleration available")
	} else {
		fmt.Println("ℹ️  CPU-only mode")
	}
	fmt.Println()

	// Create Rust core
	core := NewRustCore()
	defer core.Close()

	// Load model
	modelPath := "../../models/qwen3.5-0.8b.gguf"
	fmt.Printf("Loading model: %s\n", modelPath)
	
	err := core.LoadModel(modelPath, 2048, 999, 0)
	if err != nil {
		fmt.Printf("❌ Failed to load model: %v\n", err)
		fmt.Println("Make sure the model file exists and Rust library is built")
		return
	}
	fmt.Println("✅ Model loaded successfully")
	fmt.Println()

	// Interactive chat
	fmt.Println("Enter your prompts (type 'quit' to exit):")
	fmt.Println()

	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("You: ")
		if !scanner.Scan() {
			break
		}

		prompt := strings.TrimSpace(scanner.Text())
		if prompt == "" || prompt == "quit" || prompt == "exit" {
			break
		}

		fmt.Print("AI: ")
		response, err := core.Generate(prompt)
		if err != nil {
			fmt.Printf("Error: %v\n", err)
			continue
		}

		fmt.Println(response)
		fmt.Println()
	}

	fmt.Println()
	fmt.Println("Goodbye!")
}
```

---

## Step 4: Go Module

Create `workers/agents/go.mod`:

```go
module ochi-agents

go 1.21

require (
	// No external dependencies needed!
)
```

---

## Step 5: Build & Run

### Windows (PowerShell)

```powershell
# 1. Build Rust library
cd e:\Ochi
cargo build --release --features ai

# 2. Set PATH for DLL
$env:Path = "e:\Ochi\target\release;" + $env:Path

# 3. Build Go agent
cd workers\agents
go build -o ochi-agent.exe

# 4. Run
.\ochi-agent.exe
```

### Linux/macOS

```bash
# 1. Build Rust library
cd ochi
cargo build --release --features ai

# 2. Set LD_LIBRARY_PATH
export LD_LIBRARY_PATH=$(pwd)/target/release:$LD_LIBRARY_PATH

# 3. Build Go agent
cd workers/agents
go build -o ochi-agent

# 4. Run
./ochi-agent
```

---

## Example Output

```
========================================
Ochi AI Agent (Go + Rust)
========================================

✅ GPU acceleration available

Loading model: ../../models/qwen3.5-0.8b.gguf
✅ Model loaded successfully

Enter your prompts (type 'quit' to exit):

You: Hello, how are you?
AI: I'm doing great! How can I help you today?

You: What is 2 + 2?
AI: 2 + 2 equals 4.

You: quit

Goodbye!
```

---

## Multi-Agent Example

Create `workers/agents/multi_agent.go`:

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	// Create multiple agents
	agents := []*RustCore{
		NewRustCore(),
		NewRustCore(),
		NewRustCore(),
	}

	// Load same model for all agents
	modelPath := "../../models/qwen3.5-0.8b.gguf"
	
	var wg sync.WaitGroup
	for i, agent := range agents {
		wg.Add(1)
		go func(id int, a *RustCore) {
			defer wg.Done()
			defer a.Close()
			
			a.LoadModel(modelPath, 1024, 999, 0)
			
			response, _ := a.Generate(fmt.Sprintf("Agent %d says hello!", id))
			fmt.Printf("Agent %d: %s\n", id, response)
		}(i, agent)
	}

	wg.Wait()
}
```

---

## Performance Comparison

| Setup | Speed | Memory |
|-------|-------|--------|
| Rust only | ~80 tok/s | ~1GB |
| Go + Rust FFI | ~75 tok/s | ~1.1GB |
| Multi-agent (3x) | ~60 tok/s | ~3GB |

---

## Troubleshooting

### "DLL not found" (Windows)

```powershell
# Make sure o chi_core.dll is in PATH
$env:Path = "e:\Ochi\target\release;" + $env:Path
```

### "undefined symbol" (Linux/macOS)

```bash
# Set LD_LIBRARY_PATH
export LD_LIBRARY_PATH=/path/to/ochi/target/release:$LD_LIBRARY_PATH
```

### "model not found"

```bash
# Check model path is correct
ls -la ../../models/qwen3.5-0.8b.gguf
```

---

## Next Steps

1. ✅ Build Rust library
2. ✅ Create Go bindings
3. ✅ Build Go agent
4. 📚 Add streaming support
5. 📚 Add agent communication
6. 📚 Add load balancing

---

**See Also:**
- [docs/rust2go.md](rust2go.md) - Architecture overview
- [USAGE-AI.md](../USAGE-AI.md) - Rust usage guide
