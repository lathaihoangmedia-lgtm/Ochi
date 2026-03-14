@echo off
REM Auto-commit script for Ochi repository
REM Usage: auto-commit.bat [commit message]

cd /d "%~dp0.."

REM Check if there are changes
git status --porcelain > nul
if %errorlevel% neq 0 (
    echo No changes to commit.
    exit /b 0
)

REM Stage all changes if no specific message provided
if "%~1"=="" (
    echo Auto-staging all changes...
    git add -A
    
    REM Generate commit message based on changes
    for /f "delims=" %%i in ('git status --porcelain ^| findstr /C:"M" /C:"A" /C:"D" /C:"R" /C:"C"') do (
        set "has_changes=1"
    )
    
    if defined has_changes (
        REM Count change types
        set "added=0" & set "modified=0" & set "deleted=0"
        for /f "tokens=1" %%a in ('git status --porcelain') do (
            if "%%a"=="A" set /a added+=1
            if "%%a"=="M" set /a modified+=1
            if "%%a"=="D" set /a deleted+=1
            if "%%a"=="R" set /a modified+=1
            if "%%a"=="C" set /a modified+=1
            if "%%a"=="AM" set /a added+=1
            if "%%a"=="AD" set /a deleted+=1
        )
        
        set "msg=chore: auto-commit "
        if %added% gtr 0 set "msg=%msg% (+%added%)"
        if %modified% gtr 0 set "msg=%msg% (~%modified%)"
        if %deleted% gtr 0 set "msg=%msg% (-%deleted%)"
        
        echo Committing: %msg%
        git commit -m "%msg%"
    )
) else (
    git add -A
    echo Committing: %*
    git commit -m "%*"
)

if %errorlevel% equ 0 (
    echo Commit successful!
) else (
    echo Commit failed or nothing to commit.
)
