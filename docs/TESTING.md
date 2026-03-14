# CI Testing Pipeline

## Testing Strategy

Ochi Core uses a **multi-layered testing approach** with isolated, parallelizable tests.

---

## Test Layers

### 1. Unit Tests (Fast, Isolated)

**Location:** `crates/*/src/*`, inline `#[cfg(test)]` modules

**Run:**
```bash
cargo test --lib
```

**Tests include:**
- Error type conversions
- Utility functions
- Config validation

---

### 2. Hardware Detection Tests (System-Dependent)

**Location:** `crates/ochi-core/src/hardware/detector.rs`

**Run:**
```bash
cargo test -p ochi-core hardware::detector::tests::test_detect_hardware -- --nocapture
```

**Tests:**
- CPU detection
- Memory detection
- GPU detection (if available)

---

### 3. Auto-Tuner Tests (Configuration)

**Location:** `crates/ochi-core/src/hardware/mod.rs`

**Run:**
```bash
cargo test -p ochi-core hardware::tests::test_auto_tuner -- --nocapture
```

**Tests:**
- Hardware profiling
- Config generation
- Model recommendations

---

### 4. Integration Tests (End-to-End)

**Location:** `tests/`

**Run:**
```bash
cargo test -- --test-threads=1
```

**Tests:**
- Model loading
- Inference
- FFI bindings
- Streaming generation

---

## CI Pipeline (GitHub Actions)

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
      
      - name: Run Unit Tests
        run: cargo test --lib
      
      - name: Run Hardware Tests
        run: cargo test -p ochi-core hardware -- --nocapture
      
      - name: Run Integration Tests
        run: cargo test -- --test-threads=1
```

---

## Local Testing

### Quick Test (Unit Only)
```bash
./scripts/test.ps1 -Unit
```

### Full Test Suite
```bash
./scripts/test.ps1 -All
```

### Test Specific Module
```bash
./scripts/test.ps1 -Hardware
./scripts/test.ps1 -AI
```

### Filter Tests
```bash
cargo test test_generate_id
cargo test ai::model::tests
```

---

## Test Isolation

### Memory Isolation
- Each test gets fresh memory allocation
- Models unloaded between tests
- GPU memory freed explicitly

### Thread Isolation
- Tests run in parallel by default
- Use `--test-threads=1` for sequential
- FFI tests require sequential execution

### Environment Isolation
- Tests don't share environment variables
- Model paths are test-specific
- Temporary files cleaned up

---

## Mocking & Fixtures

### Hardware Mocking
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_with_mock_hardware() {
        let mock_hw = HardwareInfo::mock()
            .with_cpu_cores(8)
            .with_gpu(true)
            .with_ram_gb(32);
        
        let tuner = AutoTuner::with_hardware(mock_hw);
        // Test...
    }
}
```

### Model Fixtures
```rust
#[cfg(test)]
mod tests {
    fn load_test_model() -> CandleModel {
        CandleModel::load(
            "test_fixtures/tiny-model.gguf",
            CandleConfig::default()
        ).unwrap()
    }
    
    #[test]
    fn test_inference() {
        let model = load_test_model();
        let output = model.generate("test").unwrap();
        assert!(!output.is_empty());
    }
}
```

---

## Performance Benchmarks

```bash
# Run benchmarks (nightly only)
cargo bench

# Benchmark specific function
cargo bench ai::model::generate
```

---

## Coverage Report

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out html

# Open report
start ./tarpaulin-report.html
```

---

## Troubleshooting

### Test Hangs
```bash
# Run with timeout
timeout 60 cargo test

# Run single-threaded
cargo test -- --test-threads=1
```

### Out of Memory
```bash
# Limit test threads
cargo test -- --test-threads=1

# Skip heavy integration tests
cargo test --lib
```

### GPU Tests Fail
```bash
# Skip GPU tests (if not available)
cargo test -- --skip gpu
```

---

## Test Organization

```
crates/ochi-core/
├── src/
│   ├── hardware/
│   │   ├── detector.rs    # + #[cfg(test)] mod tests
│   │   └── mod.rs         # + #[cfg(test)] mod tests
│   └── lib.rs
├── tests/
│   ├── integration.rs     # End-to-end tests
│   └── ffi_tests.rs       # FFI binding tests
└── test_fixtures/
    └── tiny-model.gguf    # Small test model
```

---

**Key Principles:**
1. ✅ Fast tests run first
2. ✅ Slow tests run last
3. ✅ Tests are independent
4. ✅ Tests are repeatable
5. ✅ Tests are isolated
