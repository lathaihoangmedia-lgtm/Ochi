@echo off
REM Setup auto-commit hooks for Ochi repository
cd /d "%~dp0.."

echo Setting up auto-commit hooks...

REM Make hooks executable (Unix-style, but works on Windows with Git Bash)
copy /Y ".git\hooks\commit-msg.sample" ".git\hooks\commit-msg.bak" > nul
copy /Y ".git\hooks\pre-commit.sample" ".git\hooks\pre-commit.bak" > nul

REM Create Windows-friendly hook runner
echo @echo off > ".git\hooks\commit-msg.cmd"
echo call "%~dp0..\scripts\auto-commit.bat" %%1 >> ".git\hooks\commit-msg.cmd"

echo.
echo Auto-commit setup complete!
echo.
echo Usage:
echo   1. Manual: scripts\auto-commit.bat [message]
echo   2. Auto: Git will use hooks automatically
echo.
echo To commit now:
echo   git add -A ^&^& git commit -m "your message"
echo.
echo Or run: scripts\auto-commit.bat "your message"
