@echo off
echo Copy SSH config to your home folder...
copy /Y "%~dp0ssh_config" "%USERPROFILE%\.ssh\config"
echo.
echo Da cau hinh SSH de bo qua xac thuc host
echo.
Thu ket noi:
ssh -T git@github.com
pause
