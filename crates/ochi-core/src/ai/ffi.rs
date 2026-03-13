//! FFI Bindings for Go Integration (rust2go)
//!
//! C-compatible functions for GPU-accelerated inference

use crate::ai::model::{GGUFModel, GGUFConfig};
use std::os::raw::{c_char, c_float, c_int};
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Mutex;

/// Opaque handle for FFI context
pub struct FFIContext {
    model: Mutex<Option<GGUFModel>>,
}

/// Create new FFI context
#[no_mangle]
pub extern "C" fn ffi_context_new() -> *mut FFIContext {
    let ctx = Box::new(FFIContext {
        model: Mutex::new(None),
    });
    Box::into_raw(ctx)
}

/// Free FFI context
#[no_mangle]
pub extern "C" fn ffi_context_free(ctx: *mut FFIContext) {
    if !ctx.is_null() {
        unsafe {
            let _ = Box::from_raw(ctx);
        }
    }
}

/// Load GGUF model with GPU offloading
/// 
/// # Arguments
/// * `ctx` - FFI context
/// * `model_path` - Path to GGUF file
/// * `context_size` - Context window size
/// * `n_gpu_layers` - Number of layers to offload to GPU (0 = CPU-only)
/// * `n_threads` - CPU threads (0 = auto)
/// 
/// # Returns
/// * `0` - Success
/// * `-1` - Invalid argument
/// * `-2` - Already loaded
/// * `-3` - Load failed
#[no_mangle]
pub extern "C" fn ffi_model_load(
    ctx: *mut FFIContext,
    model_path: *const c_char,
    context_size: c_int,
    n_gpu_layers: c_int,
    n_threads: c_int,
) -> c_int {
    if ctx.is_null() || model_path.is_null() {
        return -1;
    }
    
    let path = unsafe {
        match CStr::from_ptr(model_path).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };
    
    let config = GGUFConfig {
        model_path: path,
        context_size: context_size as usize,
        n_gpu_layers: n_gpu_layers as usize,
        temperature: 0.7,
        max_tokens: 512,
        n_threads: if n_threads > 0 { Some(n_threads as usize) } else { None },
        n_batch: 512,
    };
    
    let ctx_ref = unsafe { &mut *ctx };
    let mut model_guard = ctx_ref.model.lock().unwrap();
    
    match model_guard.as_ref() {
        Some(_) => -2,  // Already loaded
        None => {
            match GGUFModel::load(&config.model_path, config) {
                Ok(model) => {
                    *model_guard = Some(model);
                    0
                }
                Err(_) => -3,
            }
        }
    }
}

/// Generate text (returns allocated string, caller must free)
#[no_mangle]
pub extern "C" fn ffi_model_generate(
    ctx: *mut FFIContext,
    prompt: *const c_char,
) -> *mut c_char {
    if ctx.is_null() || prompt.is_null() {
        return ptr::null_mut();
    }
    
    let prompt_str = unsafe {
        match CStr::from_ptr(prompt).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        }
    };
    
    let ctx_ref = unsafe { &*ctx };
    let model_guard = ctx_ref.model.lock().unwrap();
    
    match model_guard.as_ref() {
        None => ptr::null_mut(),
        Some(model) => {
            match model.generate(prompt_str) {
                Ok(output) => {
                    match CString::new(output) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => ptr::null_mut(),
                    }
                }
                Err(_) => ptr::null_mut(),
            }
        }
    }
}

/// Free allocated string
#[no_mangle]
pub extern "C" fn ffi_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Get model info: n_vocab
#[no_mangle]
pub extern "C" fn ffi_model_get_vocab(ctx: *mut FFIContext) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    
    let ctx_ref = unsafe { &*ctx };
    let model_guard = ctx_ref.model.lock().unwrap();
    
    match model_guard.as_ref() {
        None => -1,
        Some(model) => model.info().n_vocab as c_int,
    }
}

/// Get model info: n_ctx
#[no_mangle]
pub extern "C" fn ffi_model_get_ctx(ctx: *mut FFIContext) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    
    let ctx_ref = unsafe { &*ctx };
    let model_guard = ctx_ref.model.lock().unwrap();
    
    match model_guard.as_ref() {
        None => -1,
        Some(model) => model.info().n_ctx as c_int,
    }
}

/// Get model info: n_params
#[no_mangle]
pub extern "C" fn ffi_model_get_params(ctx: *mut FFIContext) -> c_int {
    if ctx.is_null() {
        return -1;
    }
    
    let ctx_ref = unsafe { &*ctx };
    let model_guard = ctx_ref.model.lock().unwrap();
    
    match model_guard.as_ref() {
        None => -1,
        Some(model) => model.info().n_params as c_int,
    }
}

/// Check if model is loaded
#[no_mangle]
pub extern "C" fn ffi_model_is_loaded(ctx: *mut FFIContext) -> c_int {
    if ctx.is_null() {
        return 0;
    }
    
    let ctx_ref = unsafe { &*ctx };
    let model_guard = ctx_ref.model.lock().unwrap();
    
    if model_guard.is_some() { 1 } else { 0 }
}

/// Quick hardware detect (returns 1 if GPU available)
#[no_mangle]
pub extern "C" fn ffi_has_gpu() -> c_int {
    #[cfg(feature = "cuda")]
    {
        use nvml_wrapper::Nvml;
        if Nvml::init().is_ok() {
            return 1;
        }
    }
    0
}
