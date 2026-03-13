@echo off
echo ========================================
echo Ochi Core - Manual Install Commands
echo ========================================
echo.
echo Run these commands in PowerShell:
echo.
echo 1. Install LLVM:
echo    choco install llvm -y
echo.
echo 2. Install CUDA:
echo    choco install cuda --version=12.3.2 -y
echo.
echo 3. Build:
echo    cargo build --features cuda
echo.
echo ========================================
echo Or run: .\scripts\install-all.bat (as Admin)
echo ========================================
pause
