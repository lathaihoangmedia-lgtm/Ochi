@echo off
cd /d "%USERPROFILE%\.ssh"
ssh-keygen -q -t ed25519 -N "" -f id_ed25519
echo.
echo Da tao SSH key!
echo.
echo Public key:
type id_ed25519.pub
echo.
echo Copy noi dung tren va add vao: https://github.com/settings/keys
pause
