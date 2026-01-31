# RustyPing Web Installer
# Usage: iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "
██████╗ ██╗   ██╗███████╗████████╗██╗   ██╗██████╗ ██╗███╗   ██╗ ██████╗ 
██╔══██╗██║   ██║██╔════╝╚══██╔══╝╚██╗ ██╔╝██╔══██╗██║████╗  ██║██╔════╝ 
██████╔╝██║   ██║███████╗   ██║    ╚████╔╝ ██████╔╝██║██╔██╗ ██║██║  ███╗
██╔══██╗██║   ██║╚════██║   ██║     ╚██╔╝  ██╔═══╝ ██║██║╚██╗██║██║   ██║
██║  ██║╚██████╔╝███████║   ██║      ██║   ██║     ██║██║ ╚████║╚██████╔╝
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝   ╚═╝     ╚═╝╚═╝  ╚═══╝ ╚═════╝
_ _ _ ____ ___     _ _  _ ____ ___ ____ _    _    ____ ____ 
| | | |___ |__]    | |\ | [__   |  |__| |    |    |___ |__/ 
|_|_| |___ |__]    | | \| ___]  |  |  | |___ |___ |___ |  \ 
" -ForegroundColor Cyan

# 1. Check Prerequisites
Write-Host "[*] Checking prerequisites..." -ForegroundColor Yellow

if (-not (Get-Command "git" -ErrorAction SilentlyContinue)) {
    Write-Host "[!] Git is not installed." -ForegroundColor Red
    Write-Host "    Please install Git: winget install Git.Git"
    exit 1
}

if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "[!] Rust is not installed." -ForegroundColor Red
    Write-Host "    Please install Rust: winget install Rustlang.Rustup"
    exit 1
}

# Check for C++ Build Tools (link.exe)
if (-not (Get-Command "link" -ErrorAction SilentlyContinue)) {
    Write-Host "[!] C++ Build Tools (link.exe) not found." -ForegroundColor Red
    Write-Host "    Rust on Windows requires the MSVC Linker." -ForegroundColor Red
    Write-Host ""
    Write-Host "    Please run the following command in an Administrator PowerShell:" -ForegroundColor Yellow
    Write-Host "    winget install Microsoft.VisualStudio.2022.BuildTools --override `"--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --norestart`"" -ForegroundColor Gray
    Write-Host ""
    Write-Host "    After installing, restart your terminal and try again."
    exit 1
}

# 2. Setup Installation Directory
$repoUrl = "https://github.com/pdzjtechnagy/RustyPing.git"
$tempDir = [System.IO.Path]::Combine($env:TEMP, "RustyPing_Install")

if (Test-Path $tempDir) {
    Remove-Item -Path $tempDir -Recurse -Force
}
New-Item -Path $tempDir -ItemType Directory | Out-Null

# 3. Clone Repository
Write-Host "[*] Cloning RustyPing from GitHub..." -ForegroundColor Yellow
Set-Location -Path $tempDir
try {
    git clone $repoUrl .
} catch {
    Write-Host "[!] Failed to clone repository." -ForegroundColor Red
    exit 1
}

# 4. Build and Install
Write-Host "[*] Building and Installing RustyPing..." -ForegroundColor Yellow
Write-Host "    This may take a few minutes..." -ForegroundColor Gray

try {
    cargo install --path . --force
} catch {
    Write-Host "[!] Installation failed." -ForegroundColor Red
    exit 1
}

# 5. Cleanup
Set-Location -Path $env:USERPROFILE
Remove-Item -Path $tempDir -Recurse -Force

Write-Host ""
Write-Host "[+] RustyPing has been successfully installed!" -ForegroundColor Green
Write-Host "    You can now run it by typing: rping" -ForegroundColor Cyan