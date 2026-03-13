# ============================================================================
# Ochi Core - One Click Setup
# ============================================================================
# This script fully automates the setup process for Ochi Core
# Run as Administrator!
# ============================================================================

param(
    [switch]$SkipChecks,
    [switch]$NoCUDA,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$SetupLog = "setup.log"
$StartTime = Get-Date

# ============================================================================
# Helper Functions
# ============================================================================

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logEntry = "[$timestamp] [$Level] $Message"
    Add-Content -Path $SetupLog -Value $logEntry
    Write-Host $Message
}

function Write-Step {
    param([string]$Message)
    Write-Host ""
    Write-Host "════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host " $Message" -ForegroundColor Cyan
    Write-Host "════════════════════════════════════════" -ForegroundColor Cyan
}

function Test-Admin {
    $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    return $isAdmin
}

function Test-Command {
    param([string]$Command)
    return $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

function Test-Rust {
    return Test-Command "cargo"
}

function Test-Choco {
    return Test-Command "choco"
}

function Test-LLVM {
    return Test-Command "clang"
}

function Test-CUDA {
    return Test-Command "nvcc"
}

function Install-Rust {
    Write-Log "Installing Rust..."
    Invoke-WebRequest https://sh.rustup.rs -OutFile rustup-init.exe
    Start-Process -FilePath ".\rustup-init.exe" -ArgumentList "-y", "--default-toolchain", "stable" -Wait
    Remove-Item rustup-init.exe
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    Write-Log "Rust installed successfully"
}

function Install-Choco {
    Write-Log "Installing Chocolatey..."
    Set-ExecutionPolicy Bypass -Scope Process -Force
    iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    Write-Log "Chocolatey installed successfully"
}

function Install-LLVM {
    Write-Log "Installing LLVM/Clang..."
    choco install llvm -y --no-progress
    Write-Log "LLVM installed successfully"
}

function Install-CUDA {
    Write-Log "Installing CUDA Toolkit..."
    choco install cuda -y --no-progress
    Write-Log "CUDA installed successfully"
}

function Install-VSBuildTools {
    Write-Log "Checking Visual Studio Build Tools..."
    if (Test-Path "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vs_buildtools.exe") {
        Write-Log "Visual Studio Build Tools already installed"
        return
    }
    
    Write-Log "Visual Studio Build Tools not found. Please install manually:"
    Write-Log "Download: https://visualstudio.microsoft.com/downloads/"
    Write-Log "Install: 'Desktop development with C++' workload"
    Write-Host ""
    Write-Host "Press any key to continue after installing VS Build Tools..." -ForegroundColor Yellow
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}

function Download-Model {
    Write-Log "Downloading sample model..."
    $modelDir = "models"
    $modelFile = "$modelDir\qwen3.5-0.8b.gguf"
    
    if (Test-Path $modelFile) {
        Write-Log "Model already exists: $modelFile"
        return
    }
    
    if (-not (Test-Path $modelDir)) {
        New-Item -ItemType Directory -Path $modelDir | Out-Null
    }
    
    Write-Log "Downloading Qwen3.5-0.8B..."
    try {
        $url = "https://huggingface.co/Qwen/Qwen3.5-0.8B-GGUF/resolve/main/qwen3.5-0.8b-iq4_nl.gguf"
        Invoke-WebRequest -Uri $url -OutFile $modelFile -UseBasicParsing
        Write-Log "Model downloaded successfully"
    } catch {
        Write-Log "Failed to download model. Please download manually from HuggingFace." "WARN"
    }
}

function Build-Project {
    Write-Log "Building project..."
    $features = if ($NoCUDA) { "ai" } else { "cuda" }
    cargo build --release --features $features
    if ($LASTEXITCODE -eq 0) {
        Write-Log "Build successful!"
    } else {
        Write-Log "Build failed!" "ERROR"
        throw "Build failed"
    }
}

function Test-Hardware {
    Write-Log "Running hardware detection test..."
    cargo test --features cuda hardware::detector::tests::test_detect_hardware -- --nocapture
}

function Print-Summary {
    $endTime = Get-Date
    $duration = $endTime - $StartTime
    
    Write-Step "Setup Summary"
    Write-Host ""
    Write-Host "✅ Setup Complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Duration: $($duration.Minutes)m $($duration.Seconds)s" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Next Steps:" -ForegroundColor Yellow
    Write-Host "  1. Close and reopen your terminal" -ForegroundColor White
    Write-Host "  2. Run: cargo run --example demo --features cuda" -ForegroundColor White
    Write-Host ""
    Write-Host "Documentation:" -ForegroundColor Yellow
    Write-Host "  - START-HERE.md    : Quick start guide" -ForegroundColor White
    Write-Host "  - USAGE-AI.md      : AI usage examples" -ForegroundColor White
    Write-Host "  - REQUIREMENTS.md  : System requirements" -ForegroundColor White
    Write-Host ""
}

# ============================================================================
# Main Setup Process
# ============================================================================

try {
    Clear-Host
    
    Write-Step "🚀 Ochi Core - Auto Setup"
    Write-Host ""
    Write-Host "Starting at: $($StartTime.ToString('HH:mm:ss'))" -ForegroundColor Gray
    Write-Host ""
    
    # Check admin rights
    if (-not (Test-Admin)) {
        Write-Log "ERROR: Not running as Administrator!" "ERROR"
        Write-Host ""
        Write-Host "❌ Please run as Administrator!" -ForegroundColor Red
        Write-Host "Right-click this file → 'Run as Administrator'" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Or run: .\scripts\install-manual.bat" -ForegroundColor Yellow
        Start-Sleep -Seconds 3
        exit 1
    }
    
    Write-Log "Running as Administrator ✓"
    
    # Install Rust if needed
    Write-Step "Checking Rust"
    if (Test-Rust) {
        Write-Log "Rust already installed: $(rustc --version)"
    } else {
        Install-Rust
    }
    
    # Install Chocolatey if needed
    Write-Step "Checking Chocolatey"
    if (Test-Choco) {
        Write-Log "Chocolatey already installed"
    } else {
        Install-Choco
    }
    
    # Install VS Build Tools
    Write-Step "Checking Visual Studio Build Tools"
    Install-VSBuildTools
    
    # Install LLVM
    Write-Step "Checking LLVM/Clang"
    if (Test-LLVM) {
        Write-Log "LLVM already installed: $(clang --version | Select-Object -First 1)"
    } else {
        Install-LLVM
    }
    
    # Install CUDA (unless skipped)
    if (-not $NoCUDA) {
        Write-Step "Checking CUDA"
        if (Test-CUDA) {
            Write-Log "CUDA already installed: $(nvcc --version | Select-Object -Last 1)"
        } else {
            Install-CUDA
        }
    } else {
        Write-Log "CUDA installation skipped (NoCUDA flag)"
    }
    
    # Refresh environment variables
    Write-Step "Refreshing Environment"
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    
    # Download model
    Write-Step "Downloading Sample Model"
    Download-Model
    
    # Build project
    Write-Step "Building Project"
    Build-Project
    
    # Run tests
    Write-Step "Running Hardware Detection Test"
    try {
        Test-Hardware
    } catch {
        Write-Log "Hardware test failed, but continuing..." "WARN"
    }
    
    # Print summary
    Print-Summary
    
} catch {
    Write-Log "ERROR: $($_.Exception.Message)" "ERROR"
    Write-Host ""
    Write-Host "❌ Setup failed!" -ForegroundColor Red
    Write-Host "Check $SetupLog for details" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
