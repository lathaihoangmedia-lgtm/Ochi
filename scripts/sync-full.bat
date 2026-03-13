@echo off
REM Full bidirectional sync
echo ========================================
echo Full Sync: GitHub <-> Local
echo ========================================

echo Step 1: Fetching from remote...
git fetch origin

echo.
echo Step 2: Pulling latest changes...
git pull --rebase origin main

if %ERRORLEVEL% NEQ 0 (
    echo ========================================
    echo ERROR: Pull failed. Resolve conflicts first.
    echo ========================================
    exit /b 1
)

echo.
echo Step 3: Checking status...
git status

echo.
set /p commitmsg="Enter commit message for any local changes (or press Enter to skip): "
if not "%commitmsg%"=="" (
    git add -A
    git commit -m "%commitmsg%"
)

echo.
echo Step 4: Pushing to remote...
git push origin main

if %ERRORLEVEL% EQU 0 (
    echo ========================================
    echo SUCCESS: Full sync completed!
    echo ========================================
) else (
    echo ========================================
    echo ERROR: Push failed.
    echo ========================================
)
