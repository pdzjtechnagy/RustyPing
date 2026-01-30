# Fix Rust Build Environment on Windows
# This script installs the necessary C++ Build Tools and configures Rust to use them.

$ErrorActionPreference = "Stop"

Write-Host "=== RustyPing Environment Fixer ===" -ForegroundColor Cyan

# 1. Check if we already have the linker
if (Get-Command "link.exe" -ErrorAction SilentlyContinue) {
    Write-Host "[OK] MSVC Linker (link.exe) found." -ForegroundColor Green
} else {
    Write-Host "[INFO] MSVC Linker not found." -ForegroundColor Yellow
    Write-Host "Installing Visual Studio Build Tools 2022 (C++ Workload)..." -ForegroundColor Cyan
    Write-Host "NOTE: This will download ~2-3GB. A UAC prompt may appear on your taskbar." -ForegroundColor Magenta
    
    # Install VS Build Tools via Winget
    # Interactive mode (no --passive) so user can see what's happening and handle UAC/Errors
    $installCmd = "winget install --id Microsoft.VisualStudio.2022.BuildTools --override `"--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended`""
    
    Write-Host "Launching Visual Studio Installer..." -ForegroundColor Cyan
    Write-Host "Please follow the on-screen instructions to complete the installation." -ForegroundColor Yellow
    
    Invoke-Expression $installCmd
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[ERROR] Installation failed. You may need to run this script as Administrator." -ForegroundColor Red
        exit 1
    }
}

# 2. Configure Rust
Write-Host "`nConfiguring Rust Toolchain..." -ForegroundColor Cyan
rustup default stable-x86_64-pc-windows-msvc
rustup update stable-x86_64-pc-windows-msvc

Write-Host "`n[SUCCESS] Environment configured!" -ForegroundColor Green
Write-Host "Please RESTART your terminal/IDE to ensure the new PATH is loaded." -ForegroundColor Yellow
Write-Host "Then run: cargo run --release" -ForegroundColor White
