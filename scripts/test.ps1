# Testing Pipeline for Ochi Core
# Run tests in parallel with proper isolation

param(
    [switch]$All,
    [switch]$Unit,
    [switch]$Integration,
    [switch]$Hardware,
    [switch]$AI,
    [string]$Filter
)

$ErrorActionPreference = "Stop"
$TestResults = @()
$StartTime = Get-Date

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Ochi Core - Test Pipeline" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Unit Tests (Core)
function Test-Unit {
    Write-Host "[1/4] Running Unit Tests..." -ForegroundColor Yellow
    $result = cargo test --lib 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Unit Tests PASSED" -ForegroundColor Green
        return $true
    } else {
        Write-Host "✗ Unit Tests FAILED" -ForegroundColor Red
        return $false
    }
}

# Test 2: Hardware Detection
function Test-Hardware {
    Write-Host ""
    Write-Host "[2/4] Running Hardware Detection Tests..." -ForegroundColor Yellow
    $result = cargo test hardware::detector::tests::test_detect_hardware -- --nocapture 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Hardware Detection PASSED" -ForegroundColor Green
        return $true
    } else {
        Write-Host "✗ Hardware Detection FAILED" -ForegroundColor Red
        return $false
    }
}

# Test 3: Auto-Tuner
function Test-AutoTuner {
    Write-Host ""
    Write-Host "[3/4] Running Auto-Tuner Tests..." -ForegroundColor Yellow
    $result = cargo test hardware::tuner::tests::test_auto_tuner -- --nocapture 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Auto-Tuner PASSED" -ForegroundColor Green
        return $true
    } else {
        Write-Host "✗ Auto-Tuner FAILED" -ForegroundColor Red
        return $false
    }
}

# Test 4: Integration Tests
function Test-Integration {
    Write-Host ""
    Write-Host "[4/4] Running Integration Tests..." -ForegroundColor Yellow
    
    # Check if model exists
    if (Test-Path "models/qwen3.5-0.8b.gguf") {
        Write-Host "Model found, running AI tests..." -ForegroundColor Cyan
        $result = cargo test -- --test-threads=1 2>&1
    } else {
        Write-Host "Model not found, skipping AI integration tests" -ForegroundColor Yellow
        return $true
    }
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Integration Tests PASSED" -ForegroundColor Green
        return $true
    } else {
        Write-Host "✗ Integration Tests FAILED" -ForegroundColor Red
        return $false
    }
}

# Run Tests
$Passed = 0
$Failed = 0

if ($Unit -or $All -or (-not ($Integration -or $Hardware -or $AI))) {
    if (Test-Unit) { $Passed++ } else { $Failed++ }
}

if ($Hardware -or $All -or (-not ($Unit -or $Integration -or $AI))) {
    if (Test-Hardware) { $Passed++ } else { $Failed++ }
}

if ($AI -or $All -or (-not ($Unit -or $Hardware -or $Integration))) {
    if (Test-AutoTuner) { $Passed++ } else { $Failed++ }
}

if ($Integration -or $All -or (-not ($Unit -or $Hardware -or $AI))) {
    if (Test-Integration) { $Passed++ } else { $Failed++ }
}

# Summary
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Test Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Passed: $Passed" -ForegroundColor Green
Write-Host "Failed: $Failed" -ForegroundColor $(if ($Failed -eq 0) { "Green" } else { "Red" })
Write-Host ""

$EndTime = Get-Date
$Duration = $EndTime - $StartTime
Write-Host "Duration: $($Duration.Minutes)m $($Duration.Seconds)s" -ForegroundColor Cyan
Write-Host ""

if ($Failed -gt 0) {
    exit 1
}
