@echo off
echo Generating SSH key for GitHub...
echo.

set /p GIT_EMAIL="Enter your GitHub email: "
if "%GIT_EMAIL%"=="" set GIT_EMAIL=your-email@domain.com

ssh-keygen -t ed25519 -C "%GIT_EMAIL%" -N "" -f "%USERPROFILE%/.ssh/id_ed25519"

echo.
echo ========================================
echo SSH Key generated successfully!
echo ========================================
echo.
echo Public key location: %USERPROFILE%/.ssh/id_ed25519.pub
echo.
echo NEXT STEP: Copy the contents of the .pub file and add it to:
echo https://github.com/settings/keys
echo.
echo Then run: ssh -T git@github.com
echo ========================================
pause
