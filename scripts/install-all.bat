@echo off
echo ========================================
echo Ochi Core - Auto Setup
echo ========================================
echo.

:: Check if running as admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo Please run as Administrator!
    echo Right-click and 'Run as administrator'
    pause
    exit /b 1
)

echo [1/4] Installing LLVM/Clang...
choco install llvm -y --no-progress

echo.
echo [2/4] Installing CUDA Toolkit 12.3...
choco install cuda --version=12.3.2 -y --no-progress

echo.
echo [3/4] Setting environment variables...
setx CUDA_PATH "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3" /M
setx PATH "%PATH%;C:\Program Files\LLVM\bin;C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3\bin" /M

echo.
echo [4/4] Verifying installation...
echo.
echo Checking LLVM:
clang --version | findstr /C:"version"

echo.
echo Checking CUDA:
nvcc --version | findstr /C:"release"

echo.
echo ========================================
echo Setup Complete!
echo ========================================
echo.
echo Next: Run 'cargo build --features cuda'
echo.
pause
