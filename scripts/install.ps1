# Ochi installer for Windows
 iex| iex
 iex"| iex"
#
# Flags (via environment variables):
#   $env:OCHI_INSTALL_DIR      = custom install directory
#   $env:OPENFANG_INSTALL_DIR  = legacy custom install directory
#   $env:OCHI_VERSION          = specific version tag (e.g. "v0.1.0")
#   $env:OPENFANG_VERSION      = legacy version override

$ErrorActionPreference = 'Stop'

$Repo = "lathaihoangmedia-lgtm/Ochi"
$DefaultInstallDir = Join-Path $env:USERPROFILE ".ochi\bin"
$InstallDir = if ($env:OCHI_INSTALL_DIR) {
    $env:OCHI_INSTALL_DIR
} elseif ($env:OPENFANG_INSTALL_DIR) {
    $env:OPENFANG_INSTALL_DIR
} else {
    $DefaultInstallDir
}

function Write-Banner {
    Write-Host ""
    Write-Host "  Ochi Installer" -ForegroundColor Cyan
    Write-Host "  ==============" -ForegroundColor Cyan
    Write-Host ""
}

function Write-InstallHelp {
    Write-Host "    cargo install --git https://github.com/$Repo ochi-cli --bin ochi"
}

function Get-Architecture {
    $arch = ""

    try { $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString() } catch {}
    if (-not $arch -or $arch -eq "") { try { $arch = $env:PROCESSOR_ARCHITECTURE } catch {} }
    if (-not $arch -or $arch -eq "") {
        try {
            $wmiArch = (Get-CimInstance Win32_Processor).Architecture
            if ($wmiArch -eq 9) { $arch = "AMD64" }
            elseif ($wmiArch -eq 12) { $arch = "ARM64" }
        } catch {}
    }
    if (-not $arch -or $arch -eq "") {
        if ([IntPtr]::Size -eq 8) { $arch = "X64" }
    }

    $archUpper = "$arch".ToUpper().Trim()
    switch ($archUpper) {
        { $_ -in "X64", "AMD64", "X86_64" } { return "x86_64" }
        { $_ -in "ARM64", "AARCH64", "ARM" } { return "aarch64" }
        default {
            Write-Host "  Unsupported architecture: $arch (detection may have failed)" -ForegroundColor Red
            Write-Host "  Try:" -ForegroundColor Yellow
            Write-InstallHelp
            exit 1
        }
    }
}

function Get-LatestVersion {
    if ($env:OCHI_VERSION) { return $env:OCHI_VERSION }
    if ($env:OPENFANG_VERSION) { return $env:OPENFANG_VERSION }

    Write-Host "  Fetching latest release..."
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        return $release.tag_name
    }
    catch {
        Write-Host "  Could not determine latest version." -ForegroundColor Red
        Write-Host "  Install from source instead:" -ForegroundColor Yellow
        Write-InstallHelp
        exit 1
    }
}

function Install-Ochi {
    Write-Banner

    $arch = Get-Architecture
    $version = Get-LatestVersion
    $target = "${arch}-pc-windows-msvc"
    $archive = "ochi-${target}.zip"
    $url = "https://github.com/$Repo/releases/download/$version/$archive"
    $checksumUrl = "$url.sha256"

    Write-Host "  Installing Ochi $version for $target..."

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $tempDir = Join-Path ([System.IO.Path]::GetTempPath()) "ochi-install"
    if (Test-Path $tempDir) { Remove-Item -Recurse -Force $tempDir }
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

    $archivePath = Join-Path $tempDir $archive
    $checksumPath = Join-Path $tempDir "$archive.sha256"

    try {
        Invoke-WebRequest -Uri $url -OutFile $archivePath -UseBasicParsing
    }
    catch {
        Write-Host "  Download failed. The release may not exist for your platform." -ForegroundColor Red
        Write-Host "  Install from source instead:" -ForegroundColor Yellow
        Write-InstallHelp
        Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
        exit 1
    }

    $checksumDownloaded = $false
    try {
        Invoke-WebRequest -Uri $checksumUrl -OutFile $checksumPath -UseBasicParsing
        $checksumDownloaded = $true
    }
    catch {
        Write-Host "  Checksum file not available, skipping verification." -ForegroundColor Yellow
    }

    if ($checksumDownloaded) {
        $expectedHash = (Get-Content $checksumPath -Raw).Split(" ")[0].Trim().ToLower()
        $actualHash = (Get-FileHash $archivePath -Algorithm SHA256).Hash.ToLower()
        if ($expectedHash -ne $actualHash) {
            Write-Host "  Checksum verification FAILED!" -ForegroundColor Red
            Write-Host "    Expected: $expectedHash" -ForegroundColor Red
            Write-Host "    Got:      $actualHash" -ForegroundColor Red
            Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
            exit 1
        }
        Write-Host "  Checksum verified." -ForegroundColor Green
    }

    Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force

    $ochiExe = Join-Path $tempDir "ochi.exe"
    $openfangExe = Join-Path $tempDir "openfang.exe"

    if (-not (Test-Path $ochiExe)) {
        $foundOchi = Get-ChildItem -Path $tempDir -Filter "ochi.exe" -Recurse | Select-Object -First 1
        if ($foundOchi) { $ochiExe = $foundOchi.FullName }
    }
    if (-not (Test-Path $openfangExe)) {
        $foundOpenfang = Get-ChildItem -Path $tempDir -Filter "openfang.exe" -Recurse | Select-Object -First 1
        if ($foundOpenfang) { $openfangExe = $foundOpenfang.FullName }
    }

    if (-not (Test-Path $ochiExe) -and -not (Test-Path $openfangExe)) {
        Write-Host "  Could not find ochi.exe/openfang.exe in archive." -ForegroundColor Red
        Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
        exit 1
    }

    if (Test-Path $ochiExe) {
        Copy-Item -Path $ochiExe -Destination (Join-Path $InstallDir "ochi.exe") -Force
    }
    if (Test-Path $openfangExe) {
        Copy-Item -Path $openfangExe -Destination (Join-Path $InstallDir "openfang.exe") -Force
    }

    # Compatibility shim: if only one binary exists, duplicate to the other name
    $installedOchi = Join-Path $InstallDir "ochi.exe"
    $installedOpenfang = Join-Path $InstallDir "openfang.exe"
    if (-not (Test-Path $installedOchi) -and (Test-Path $installedOpenfang)) {
        Copy-Item -Path $installedOpenfang -Destination $installedOchi -Force
    }
    if (-not (Test-Path $installedOpenfang) -and (Test-Path $installedOchi)) {
        Copy-Item -Path $installedOchi -Destination $installedOpenfang -Force
    }

    Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue

    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$InstallDir;$currentPath", "User")
        Write-Host "  Added $InstallDir to user PATH." -ForegroundColor Green
        Write-Host "  Restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
    }

    if (Test-Path $installedOchi) {
        try {
            $versionOutput = & $installedOchi --version 2>&1
            Write-Host ""
            Write-Host "  Ochi installed successfully! ($versionOutput)" -ForegroundColor Green
        }
        catch {
            Write-Host ""
            Write-Host "  Ochi binary installed to $installedOchi" -ForegroundColor Green
        }
    }

    Write-Host ""
    Write-Host "  Get started:" -ForegroundColor Cyan
    Write-Host "    ochi init"
    Write-Host ""
    Write-Host "  Compatibility alias remains available:"
    Write-Host "    openfang init"
    Write-Host ""
}

Install-Ochi
