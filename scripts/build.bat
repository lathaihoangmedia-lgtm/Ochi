@echo off
echo ========================================
echo Ochi Core - Quick Build
echo ========================================
echo.

echo Building with CUDA support...
echo.

cargo build --release --features cuda

if %errorLevel% equ 0 (
    echo.
    echo ========================================
    echo Build Successful!
    echo ========================================
    echo.
    echo Output: target\release\ochi_core.dll
    echo.
) else (
    echo.
    echo ========================================
    echo Build Failed!
    echo ========================================
    echo.
    echo Try running install-all.bat first
    echo.
)

pause
