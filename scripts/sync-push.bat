@echo off
REM Sync local changes to GitHub
echo ========================================
echo Pushing local changes to GitHub...
echo ========================================

git fetch origin
git pull --rebase origin main

if %ERRORLEVEL% NEQ 0 (
    echo ========================================
    echo ERROR: Pull failed. Please resolve conflicts.
    echo ========================================
    exit /b 1
)

git add -A
git status

set /p message="Enter commit message (or press Enter to skip): "
if not "%message%"=="" (
    git commit -m "%message%"
)

git push origin main

if %ERRORLEVEL% EQU 0 (
    echo ========================================
    echo SUCCESS: Synced to GitHub!
    echo ========================================
) else (
    echo ========================================
    echo ERROR: Push failed.
    echo ========================================
)
