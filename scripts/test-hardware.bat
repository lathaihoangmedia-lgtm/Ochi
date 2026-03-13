@echo off
echo ========================================
echo Ochi Core - Test Hardware Detection
echo ========================================
echo.

cargo test --features cuda hardware::detector::tests::test_detect_hardware -- --nocapture

pause
