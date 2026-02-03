# RustyPing Installation Script
# Installs rping.exe to a directory in PATH

Write-Host "RustyPing v2.4.4 - Installation Script" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Check if release binary exists
$binaryPath = "target\release\rping.exe"
if (-not (Test-Path $binaryPath)) {
    Write-Host "Error: Binary not found at $binaryPath" -ForegroundColor Red
    Write-Host "Please run 'cargo build --release' first." -ForegroundColor Yellow
    exit 1
}

# Determine install directory
$installDir = "$env:USERPROFILE\.local\bin"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Write-Host "Created directory: $installDir" -ForegroundColor Green
}

# Copy binary
$installPath = Join-Path $installDir "rping.exe"
Copy-Item $binaryPath $installPath -Force
Write-Host "Installed rping.exe to: $installPath" -ForegroundColor Green

# Check if directory is in PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    Write-Host ""
    Write-Host "Adding $installDir to PATH..." -ForegroundColor Yellow
    
    $newPath = $currentPath + ";$installDir"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    
    Write-Host "PATH updated! Please restart your terminal for changes to take effect." -ForegroundColor Yellow
} else {
    Write-Host "Directory already in PATH." -ForegroundColor Green
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host "You can now use 'rping' command from anywhere." -ForegroundColor Cyan
Write-Host ""
Write-Host "Usage examples:" -ForegroundColor Cyan
Write-Host "  rping 8.8.8.8" -ForegroundColor White
Write-Host "  rping google.com" -ForegroundColor White
Write-Host "  rping --list" -ForegroundColor White


