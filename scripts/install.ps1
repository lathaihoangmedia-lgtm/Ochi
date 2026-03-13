# Ochi Core - Auto Install (PowerShell)
# IMPORTANT: Run as Administrator!

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Ochi Core - Auto Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "ERROR: Please run as Administrator!" -ForegroundColor Red
    Write-Host "Right-click PowerShell → Run as Administrator" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Or run: .\scripts\install-manual.bat" -ForegroundColor Yellow
    exit 1
}

Write-Host "[1/3] Installing LLVM/Clang..." -ForegroundColor Green
choco install llvm -y --no-progress 2>&1 | Write-Host

Write-Host ""
Write-Host "[2/3] Installing CUDA Toolkit..." -ForegroundColor Green
Write-Host "Note: CUDA package is large, this may take a while..." -ForegroundColor Yellow
choco install cuda -y --no-progress 2>&1 | Write-Host

Write-Host ""
Write-Host "[3/3] Verifying installation..." -ForegroundColor Green
Write-Host ""

# Refresh environment variables
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

Write-Host "Checking LLVM:" -ForegroundColor Cyan
try {
    clang --version 2>&1 | Select-Object -First 1
} catch {
    Write-Host "LLVM not found in PATH. May need to restart terminal." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Checking CUDA:" -ForegroundColor Cyan
try {
    nvcc --version 2>&1 | Select-Object -Last 1
} catch {
    Write-Host "CUDA not found in PATH. May need to restart terminal." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Setup Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "IMPORTANT: Close and reopen your terminal!" -ForegroundColor Yellow
Write-Host ""
Write-Host "Then run: cargo build --features cuda" -ForegroundColor White
Write-Host ""
