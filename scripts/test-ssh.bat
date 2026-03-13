@echo off
echo Dang test SSH ket noi GitHub...
echo.
echo Neu thay thong bao 'Are you sure you want to continue connecting', hay:
echo   1. Go 'yes' va Enter
echo   2. Sau do se bao 'Hi username! You've successfully authenticated'
echo.
pause
ssh -T git@github.com
echo.
echo Neu thanh cong, ban da co the dung cac lenh sync!
pause
