# RustyPing Diagnostic Toolset Installer
# This script installs the necessary Rust tools for auditing and stability checks.
# Note: Uses --locked and specific versions for MSRV (Rust 1.88) compatibility.

Write-Host "--- Installing RustyPing Diagnostic Tools ---" -ForegroundColor Cyan

# 1. cargo-deny (Security & License Audit)
Write-Host "[1/5] Installing cargo-deny..." -ForegroundColor Yellow
cargo install --locked cargo-deny

# 2. cargo-bloat (Binary Size Analysis)
Write-Host "[2/5] Installing cargo-bloat..." -ForegroundColor Yellow
cargo install cargo-bloat

# 3. cargo-outdated (Dependency Version Check)
Write-Host "[3/5] Installing cargo-outdated..." -ForegroundColor Yellow
cargo install cargo-outdated

# 4. cargo-nextest (Enhanced Testing UI)
# Pinning to v0.9.114 for Rust 1.88 compatibility
Write-Host "[4/5] Installing cargo-nextest (v0.9.114)..." -ForegroundColor Yellow
cargo install cargo-nextest --version 0.9.114 --locked

# 5. cargo-expand (Macro Debugging)
Write-Host "[5/5] Installing cargo-expand..." -ForegroundColor Yellow
cargo install cargo-expand

Write-Host "`n--- Installation Complete ---" -ForegroundColor Green
Write-Host "You can now run '.\check_stability.ps1' to perform a local audit."
