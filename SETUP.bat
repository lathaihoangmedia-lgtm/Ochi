@echo off
REM ============================================================================
REM Ochi Core - One Click Setup (Batch Version)
REM ============================================================================
REM Run as Administrator!
REM ============================================================================

echo ============================================================================
echo Ochi Core - One Click Setup
echo ============================================================================
echo.

REM Check if running as admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ERROR: Please run as Administrator!
    echo Right-click this file and select 'Run as administrator'
    echo.
    pause
    exit /b 1
)

echo Running as Administrator...
echo.

REM Run PowerShell script
powershell -ExecutionPolicy Bypass -File "%~dp0SETUP.ps1" %*

pause
