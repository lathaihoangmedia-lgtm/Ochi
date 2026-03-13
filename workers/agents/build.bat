@echo off
REM Build Go Agent for Windows

echo ========================================
echo Ochi AI Agent - Build (Windows)
echo ========================================
echo.

REM 1. Check if Rust library exists
if not exist "..\..\target\release\ochi_core.dll" (
    echo Rust library not found!
    echo Building Rust library first...
    echo.
    cd ..\..
    cargo build --release --features ai
    if errorlevel 1 (
        echo.
        echo Rust build failed!
        pause
        exit /b 1
    )
    cd workers\agents
)

echo ✅ Rust library found
echo.

REM 2. Set PATH for DLL
set PATH=..\..\target\release;%PATH%

REM 3. Build Go agent
echo Building Go agent...
go build -o ochi-agent.exe -ldflags="-s -w"

if errorlevel 1 (
    echo.
    echo Go build failed!
    echo Make sure Go is installed: https://go.dev/dl/
    pause
    exit /b 1
)

echo.
echo ========================================
echo Build Successful!
echo ========================================
echo.
echo Run: .\ochi-agent.exe
echo.
pause
