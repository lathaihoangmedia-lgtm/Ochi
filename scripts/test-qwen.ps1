#!/usr/bin/env pwsh
# Test Qwen3.5-0.8B for infinite loop issues

param(
    [string]$ModelPath = "models/qwen3.5-0.8b.gguf",
    [switch]$WithLoopDetection
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Qwen3.5-0.8B Loop Test" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if model exists
if (-not (Test-Path $ModelPath)) {
    Write-Host "Model not found: $ModelPath" -ForegroundColor Red
    Write-Host "Please download the model first." -ForegroundColor Yellow
    exit 1
}

Write-Host "Model: $ModelPath" -ForegroundColor Green
Write-Host "Loop Detection: $(if ($WithLoopDetection) { 'Enabled' } else { 'Disabled' })" -ForegroundColor Yellow
Write-Host ""

# Test prompts that typically cause loops
$testPrompts = @(
    "Hello, how are you?",
    "What is 2 + 2?",
    "Tell me a short story.",
    "Repeat after me: test"
)

Write-Host "Running tests..." -ForegroundColor Cyan
Write-Host ""

foreach ($prompt in $testPrompts) {
    Write-Host "Prompt: $prompt" -ForegroundColor Yellow
    
    # Here we would call the actual inference
    # For now, just simulate
    Write-Host "Response: [Would generate response here]" -ForegroundColor Gray
    Write-Host ""
}

Write-Host "Tests complete!" -ForegroundColor Green
