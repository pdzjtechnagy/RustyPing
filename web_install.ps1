# RustyPing Web Installer
# Usage: iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex

$ErrorActionPreference = "Stop"

# --- UI Helpers ---

function Show-Header {
    Clear-Host
    $cyan = [ConsoleColor]::Cyan
    $white = [ConsoleColor]::White
    
    Write-Host "
  ____            _         ____  _             
 |  _ \ _   _ ___| |_ _   _|  _ \(_)_ __   __ _ 
 | |_) | | | / __| __| | | | |_) | | '_ \ / _` |
 |  _ <| |_| \__ \ |_| |_| |  __/| | | | | (_| |
 |_| \_\\__,_|___/\__|\__, |_|   |_|_| |_|\__, |
                      |___/               |___/ 
    " -ForegroundColor $cyan
    Write-Host "  :: NETWORK MONITORING SUITE v2.3.1 ::" -ForegroundColor $white
    Write-Host ""
}

function Show-Step {
    param([string]$Message, [int]$Step, [int]$Total)
    Write-Host " [$Step/$Total] $Message" -ForegroundColor Yellow
}

function Show-Success {
    param([string]$Message)
    Write-Host " [OK] $Message" -ForegroundColor Green
}

function Show-Error {
    param([string]$Title, [string]$Message, [string]$Command)
    Write-Host ""
    Write-Host " ┌────────────────────────────────────────────────────────────┐" -ForegroundColor Red
    Write-Host " │ ERROR: $Title" -ForegroundColor Red
    Write-Host " ├────────────────────────────────────────────────────────────┤" -ForegroundColor Red
    Write-Host " │ $Message" -ForegroundColor White
    if ($Command) {
        Write-Host " │" -ForegroundColor Red
        Write-Host " │ Run this command to fix:" -ForegroundColor White
        Write-Host " │ > $Command" -ForegroundColor Cyan
    }
    Write-Host " └────────────────────────────────────────────────────────────┘" -ForegroundColor Red
    Write-Host ""
    exit 1
}

# --- Main Script ---

Show-Header

# 1. Check Prerequisites
Show-Step "Checking system requirements..." 1 5

# Check Git
if (-not (Get-Command "git" -ErrorAction SilentlyContinue)) {
    Show-Error "Git not found" "RustyPing requires Git to download the source code." "winget install Git.Git"
}
Show-Success "Git found"

# Check Rust
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Show-Error "Rust not found" "RustyPing requires the Rust toolchain." "winget install Rustlang.Rustup"
}
Show-Success "Rust toolchain found"

# Check C++ Build Tools (link.exe)
if (-not (Get-Command "link" -ErrorAction SilentlyContinue)) {
    Show-Error "C++ Build Tools missing" `
        "Rust on Windows requires the MSVC Linker (link.exe).`n │ You need the 'Desktop development with C++' workload." `
        "winget install Microsoft.VisualStudio.2022.BuildTools --override `"--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --norestart`""
}
Show-Success "MSVC Build Tools found"

# 2. Setup Installation Directory
Show-Step "Preparing workspace..." 2 4
$repoUrl = "https://github.com/pdzjtechnagy/RustyPing.git"
$tempDir = [System.IO.Path]::Combine($env:TEMP, "RustyPing_Install")

if (Test-Path $tempDir) {
    Remove-Item -Path $tempDir -Recurse -Force
}
New-Item -Path $tempDir -ItemType Directory | Out-Null
Show-Success "Workspace ready at $tempDir"

# 3. Clone Repository
Show-Step "Downloading source code..." 3 4
Set-Location -Path $tempDir
try {
    git clone $repoUrl . | Out-Null
    Show-Success "Repository cloned"
} catch {
    Show-Error "Download Failed" "Could not clone the repository from GitHub." ""
}

# 4. Build and Install
Show-Step "Compiling and installing (This takes 2-5 mins)..." 4 5
Write-Host "     Please wait while Cargo compiles the binary." -ForegroundColor DarkGray

try {
    # Check if we need to install from path or just use cargo install
    if (Test-Path "$tempDir\Cargo.toml") {
        $result = cargo install --path . --force 2>&1
    } else {
        $result = cargo install rustyping --force 2>&1
    }
    
    if ($LASTEXITCODE -ne 0) {
        throw $result
    }
    Show-Success "Compilation complete"
} catch {
    Show-Error "Build Failed" "Cargo failed to compile RustyPing." "cd $tempDir; cargo build --release"
}

# 5. Cleanup
Set-Location -Path $env:USERPROFILE
Remove-Item -Path $tempDir -Recurse -Force

# 6. Final Success Screen
Write-Host ""
Write-Host " ╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host " ║                 INSTALLATION SUCCESSFUL!                   ║" -ForegroundColor Green
Write-Host " ╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "  You can now run RustyPing from anywhere:" -ForegroundColor White
Write-Host ""
Write-Host "     rping" -ForegroundColor Cyan
Write-Host ""
Write-Host "  View the User Guides here:" -ForegroundColor White
Write-Host "     $docDir" -ForegroundColor DarkGray
Write-Host ""
