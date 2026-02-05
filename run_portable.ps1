# RustyPing Portable Launcher v2.5.9
# This script downloads the latest release binary to the Temp folder and runs it.

$repo = "pdzjtechnagy/RustyPing"
$tempDir = "$env:TEMP\RustyPing"
$exePath = "$tempDir\rping.exe"

# 1. Setup Environment
if (!(Test-Path -Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
}

Write-Host "`n  ╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "  ║                RustyPing Portable Launcher                 ║" -ForegroundColor Cyan
Write-Host "  ╚════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

# 2. Check for Updates
Write-Host "[*] Connecting to GitHub..." -ForegroundColor Gray

try {
    # Get the latest release data from GitHub API
    $latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -ErrorAction Stop
    $tag = $latestRelease.tag_name
    
    # Find the Windows executable asset
    $asset = $latestRelease.assets | Where-Object { $_.name -like "*.exe" } | Select-Object -First 1

    if (!$asset) {
        Write-Host "[!] Error: No Windows executable found in the latest release ($tag)." -ForegroundColor Red
        exit 1
    }

    # Download if it doesn't exist or is a different size (simple cache check)
    if ((Test-Path $exePath) -and ((Get-Item $exePath).Length -eq $asset.size)) {
        Write-Host "[+] Using cached version: $tag" -ForegroundColor Green
    }
    else {
        # Ensure no running instance is locking the file
        Get-Process rping -ErrorAction SilentlyContinue | Stop-Process -Force
        
        Write-Host "[*] Downloading RustyPing $tag..." -ForegroundColor Yellow
        Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $exePath -UseBasicParsing
        Write-Host "[+] Download complete!" -ForegroundColor Green
    }

    # 3. Launch
    Write-Host "[*] Launching RustyPing..." -ForegroundColor Cyan
    Write-Host "------------------------------------------------------------" -ForegroundColor Gray
    
    # Run with pass-through arguments if any were provided to the script
    if ($args.Count -gt 0) {
        & $exePath $args
    } else {
        & $exePath
    }

    Write-Host "`n------------------------------------------------------------" -ForegroundColor Gray
    Write-Host "[+] Session ended." -ForegroundColor Cyan
}
catch {
    if ($_.Exception.Response.StatusCode -eq 404) {
        Write-Host "[-] Error: No releases found on GitHub." -ForegroundColor Red
        Write-Host "    Create a release at https://github.com/$repo/releases" -ForegroundColor Yellow
    }
    else {
        Write-Host "[-] Failed to launch: $($_.Exception.Message)" -ForegroundColor Red
        Write-Host "    Check your internet connection." -ForegroundColor Yellow
    }
}
