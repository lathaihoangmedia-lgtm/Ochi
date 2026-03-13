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
