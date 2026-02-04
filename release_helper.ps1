# Release Helper Script for RustyPing v2.5.7
# Run this script in PowerShell to build and prepare for release.

$ErrorActionPreference = "Stop"

Write-Host "🚀 Starting Release Process for v2.5.7..." -ForegroundColor Cyan

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
Write-Host "2. git commit -m 'chore: release v2.5.7'" -ForegroundColor Yellow
Write-Host "3. git tag -a v2.5.7 -m 'Release v2.5.7'" -ForegroundColor Yellow
Write-Host "4. git push origin main" -ForegroundColor Yellow
Write-Host "5. git push origin v2.5.7" -ForegroundColor Yellow
Write-Host "---------------------------------------------------" -ForegroundColor Gray

Write-Host "🌐 GITHUB RELEASE STEPS:" -ForegroundColor Cyan
Write-Host "1. Go to: https://github.com/pdzjtechnagy/RustyPing/releases/new" -ForegroundColor White
Write-Host "2. Select Tag: v2.5.7" -ForegroundColor White
Write-Host "3. Title: v2.5.7 - Stability & Diagnostics" -ForegroundColor White
Write-Host "4. Paste RELEASE_NOTES.md contents into description." -ForegroundColor White
Write-Host "5. CI/CD will automatically handle asset uploads upon pushing the tag." -ForegroundColor White
Write-Host "6. Click 'Publish Release' once CI/CD finishes." -ForegroundColor White

Write-Host "`n✅ Script Complete." -ForegroundColor Green
