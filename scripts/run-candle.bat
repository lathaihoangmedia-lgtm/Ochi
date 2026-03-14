@echo off
REM Run Candle inference with local GGUF model

echo === Ochi Candle Inference ===
echo Model: models\qwen3.5-0.8b.gguf
echo.

cd /d "%~dp0.."

REM Check if model exists
if not exist "models\qwen3.5-0.8b.gguf" (
    echo ERROR: Model file not found!
    echo Please download model to models/ folder
    exit /b 1
)

echo Model found. Building...
cargo build --release -p ochi-llm --features ollama

if errorlevel 1 (
    echo Build failed!
    exit /b 1
)

echo.
echo Build complete. Ready to run inference.
echo.
echo Next steps:
echo   1. Create a Rust example to load the model
echo   2. Or use ochi-runtime to integrate with agents
echo.
