@echo off
REM Download and Run Real Model

echo 🚀 Real Model Setup Guide
echo.
echo ================================================================
echo.
echo OPTION 1: Use Ollama (EASIEST - Recommended)
echo ================================================================
echo.
echo 1. Install Ollama (if not installed):
echo    irm https://ollama.com/install.ps1 ^| iex
echo.
echo 2. Download model:
echo    ollama pull qwen2.5:0.5b
echo.
echo 3. Run interactive chat:
echo    ollama run qwen2.5:0.5b
echo.
echo 4. Or use in Rust app:
echo    cargo run -p ochi-core --example chat_demo
echo.
echo ================================================================
echo.
echo OPTION 2: Download from Hugging Face (For Candle)
echo ================================================================
echo.
echo 1. Install Python + huggingface_hub:
echo    pip install huggingface_hub
echo.
echo 2. Download model:
echo    huggingface-cli download Qwen/Qwen3.5-0.5B ^
echo      --include "*.safetensors" ^
echo      --local-dir models\qwen3.5-0.5b
echo.
echo 3. Run with Candle:
echo    cargo run -p ochi-core --example run_real_model
echo.
echo ================================================================
echo.
echo Current Status:
echo.

REM Check if Ollama is installed
where ollama >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo ✅ Ollama is installed
    echo.
    echo Available models:
    ollama list
) else (
    echo ⚠️  Ollama is NOT installed
    echo.
    echo 💡 Recommended: Install Ollama for easiest model management
)

echo.
echo ================================================================
pause
