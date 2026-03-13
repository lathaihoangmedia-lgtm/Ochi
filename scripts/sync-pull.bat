@echo off
REM Sync from GitHub to local
echo ========================================
echo Pulling latest changes from GitHub...
echo ========================================

git fetch origin
git pull --rebase origin main

if %ERRORLEVEL% EQU 0 (
    echo ========================================
    echo SUCCESS: Synced from GitHub!
    echo ========================================
) else (
    echo ========================================
    echo ERROR: Pull failed. Please resolve conflicts.
    echo ========================================
)
