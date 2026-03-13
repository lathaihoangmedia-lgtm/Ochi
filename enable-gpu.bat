@echo off
REM Enable GPU for Ochi Core
REM This sets CUDA_VISIBLE_DEVICES to enable GPU detection

echo 🚀 Enabling GPU for Ochi Core...
echo.

REM Check if CUDA is installed
where nvcc >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo ✅ CUDA detected
) else (
    echo ⚠️  CUDA not found in PATH
    echo 💡 Make sure CUDA Toolkit is installed
    echo.
)

REM Set environment variable for this session
set CUDA_VISIBLE_DEVICES=0
echo ✅ Set CUDA_VISIBLE_DEVICES=0
echo.

echo 🎯 GPU is now enabled for this terminal session
echo.
echo To test GPU:
echo   cargo run -p ochi-core --example test_gpu_real
echo.

REM Run GPU test
echo Running GPU test...
cargo run -p ochi-core --example test_gpu_real

echo.
echo 💡 Note: Environment variable is only set for this session
echo    To make it permanent, add to System Environment Variables
pause
