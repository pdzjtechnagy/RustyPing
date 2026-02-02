# Release Helper Script for RustyPing v2.2.0
# Run this script in PowerShell to build and prepare for release.

$ErrorActionPreference = "Stop"

Write-Host "🚀 Starting Release Process for v2.2.0..." -ForegroundColor Cyan

# 1. Verify Cargo is installed
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Error "Cargo is not found in PATH. Please install Rust."
    exit 1
}

# 2. Run Tests
Write-Host "`n🧪 Running Tests..." -ForegroundColor Yellow
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Error "Tests failed! Aborting release."
    exit 1
}
Write-Host "✅ Tests Passed." -ForegroundColor Green

# 3. Build Release Binary
Write-Host "`n🔨 Building Release Binary..." -ForegroundColor Yellow

# Ensure no running instances lock the file
Get-Process rping -ErrorAction SilentlyContinue | Stop-Process -Force

cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
    exit 1
}
Write-Host "✅ Build Complete: target/release/rping.exe" -ForegroundColor Green

# 4. Instructions for Git Tagging & Publishing
Write-Host "`n📦 READY TO PUBLISH!" -ForegroundColor Cyan
Write-Host "Perform the following steps manually to push to GitHub:" -ForegroundColor White
Write-Host "---------------------------------------------------" -ForegroundColor Gray
Write-Host "1. git add ." -ForegroundColor Yellow
Write-Host "2. git commit -m 'chore: release v2.3.0'" -ForegroundColor Yellow
Write-Host "3. git tag -a v2.3.0 -m 'Release v2.3.0'" -ForegroundColor Yellow
Write-Host "4. git push origin main --follow-tags" -ForegroundColor Yellow
Write-Host "---------------------------------------------------" -ForegroundColor Gray

# 5. Instructions for GitHub Release
Write-Host "`n🌐 GitHub Release Instructions:" -ForegroundColor Cyan
Write-Host "1. Go to: https://github.com/pdzjtechnagy/RustyPing/releases/new" -ForegroundColor White
Write-Host "2. Select Tag: v2.3.0" -ForegroundColor White
Write-Host "3. Title: v2.3.0 - CSV Logging & Jitter Metrics" -ForegroundColor White
Write-Host "4. Copy contents from CHANGELOG.md into the description." -ForegroundColor White
Write-Host "5. Upload the following binaries:" -ForegroundColor White
Write-Host "   - target/release/rping.exe (Windows)" -ForegroundColor Gray
Write-Host "   - (If built) target/x86_64-unknown-linux-musl/release/rping (Alpine/Linux)" -ForegroundColor Gray

Write-Host "`n✅ Script Complete." -ForegroundColor Green
