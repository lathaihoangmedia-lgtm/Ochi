@echo off
echo ========================================
echo Download Qwen3.5-0.8B Model
echo ========================================
echo.

set MODEL_DIR=models
set MODEL_FILE=%MODEL_DIR%\qwen3.5-0.8b.gguf

if exist %MODEL_FILE% (
    echo Model already exists: %MODEL_FILE%
    echo Skipping download...
) else (
    echo Downloading Qwen3.5-0.8B GGUF...
    echo.
    
    if not exist %MODEL_DIR% mkdir %MODEL_DIR%
    
    :: Download from HuggingFace (TheBloke's quantization)
    echo Downloading from HuggingFace...
    curl -L "https://huggingface.co/Qwen/Qwen3.5-0.8B-GGUF/resolve/main/qwen3.5-0.8b-iq4_nl.gguf" -o %MODEL_FILE%
    
    if %errorLevel% equ 0 (
        echo.
        echo Download complete!
        echo Model: %MODEL_FILE%
    ) else (
        echo.
        echo Download failed!
        echo Please download manually from:
        echo https://huggingface.co/Qwen/Qwen3.5-0.8B-GGUF
    )
)

echo.
pause
