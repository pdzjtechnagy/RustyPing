# Pre-Push Stability Check
Write-Host "Starting Stability Check..." -ForegroundColor Cyan

Write-Host "`n[1/3] Checking Code Formatting..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { Write-Error "Formatting check failed. Run 'cargo fmt' to fix."; exit 1 }

Write-Host "[2/3] Running Clippy Lints..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) { Write-Error "Clippy found issues. Fix them before pushing."; exit 1 }

Write-Host "[3/3] Running Tests..." -ForegroundColor Yellow
cargo test --workspace
if ($LASTEXITCODE -ne 0) { Write-Error "Tests failed."; exit 1 }

Write-Host "`nAll checks passed! Your code is stable and ready to push." -ForegroundColor Green
