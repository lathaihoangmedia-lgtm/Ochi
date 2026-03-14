@echo off
REM Test Ollama Integration

echo === Ochi Ollama Test ===
echo.

REM Check if Ollama is running
echo [1/3] Checking Ollama service...
curl -s http://localhost:11434/api/tags >nul 2>&1
if errorlevel 1 (
    echo ERROR: Ollama is not running!
    echo Please start Ollama first.
    exit /b 1
)
echo ✅ Ollama is running

REM List models
echo.
echo [2/3] Available models:
ollama list

REM Test generation
echo.
echo [3/3] Testing generation...
ollama run qwen2.5:0.5b "Xin chào, hãy giới thiệu ngắn gọn"

echo.
echo ✅ Test complete!
echo.
echo Next: cargo run --example ollama_autotune -p ochi-llm --features ollama
