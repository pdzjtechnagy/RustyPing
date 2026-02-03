@echo off
echo ============================================
echo   RustyPing v2.4.2 Build Script
echo ============================================
echo.

REM Check Rust
rustc --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Rust not installed
    echo Install: winget install Rustlang.Rustup
    pause
    exit /b 1
)

echo [OK] Rust found: 
rustc --version
echo.

REM Check if in correct directory
if not exist "Cargo.toml" (
    echo [ERROR] Cargo.toml not found
    echo Run this script from C:\PINGTOOL\rustyping2\
    pause
    exit /b 1
)

echo [OK] Found Cargo.toml
echo.

echo NOTE: This is a COMPLETE REWRITE with:
echo   - Braille graphs (btop-style)
echo   - Blacksite theme
echo   - Smart history with fuzzy find
echo   - Speed test integration (on-demand)
echo   - Port scanning (on-demand)
echo   - Settings menu (ESC)
echo.
echo Building... (3-5 minutes first time)
echo.

cargo build --release

if errorlevel 1 (
    echo.
    echo [ERROR] Build failed
    echo.
    echo Common fixes:
    echo   1. Missing modules - Check all src/*.rs files exist
    echo   2. Linker error - Install Visual Studio Build Tools
    echo   3. Dependency error - Run: cargo update
    echo.
    pause
    exit /b 1
)

echo.
echo ============================================
echo   Build Complete!
echo ============================================
echo.
echo Executable: .\target\release\rping.exe
echo.
echo Create launcher:
echo @.\target\release\rping.exe %%* > rping.bat
echo.
echo Run with:
echo   .\target\release\rping.exe
echo   .\target\release\rping.exe 8.8.8.8
echo   .\target\release\rping.exe --list
echo.
echo Next steps:
echo   1. Create rping.bat launcher (copy command above)
echo   2. Add C:\PINGTOOL\rustyping2 to PATH
echo   3. Run: rping
echo.
pause
