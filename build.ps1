# RustyPing Build Script
# Builds the release binary with optimizations

Write-Host "Building RustyPing 2.0..." -ForegroundColor Cyan
Write-Host ""

# Build release version
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Build successful!" -ForegroundColor Green
    Write-Host "Binary location: target\release\rping.exe" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "To install globally, run: .\install.ps1" -ForegroundColor Yellow
} else {
    Write-Host ""
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}


