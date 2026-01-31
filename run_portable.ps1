# RustyPing Portable Launcher
# This script downloads the latest release binary to the Temp folder and runs it.

$repo = "pdzjtechnagy/RustyPing"
$tempDir = "$env:TEMP\RustyPing"
$exePath = "$tempDir\rping.exe"

# Create temp directory if it doesn't exist
if (!(Test-Path -Path $tempDir)) {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
}

Write-Host "[*] Checking for latest RustyPing version..." -ForegroundColor Cyan

try {
    # Get the latest release data from GitHub API
    $latestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -ErrorAction Stop
    $tag = $latestRelease.tag_name
    
    # Find the Windows executable asset
    $asset = $latestRelease.assets | Where-Object { $_.name -like "*.exe" } | Select-Object -First 1

    if (!$asset) {
        Write-Error "No Windows executable found in the latest release ($tag)."
        exit 1
    }

    # Download if it doesn't exist or is a different size (simple cache check)
    if ((Test-Path $exePath) -and ((Get-Item $exePath).Length -eq $asset.size)) {
        Write-Host "[+] Using cached version ($tag)" -ForegroundColor Green
    }
    else {
        Write-Host "[*] Downloading RustyPing ($tag)..." -ForegroundColor Yellow
        Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $exePath
        Write-Host "[+] Download complete!" -ForegroundColor Green
    }

    # Run RustyPing
    Write-Host "[*] Launching..." -ForegroundColor Cyan
    Start-Process -FilePath $exePath -Wait -NoNewWindow

    # Cleanup (Optional - remove the '#' below if you want it to truly "disappear")
    # Remove-Item -Path $exePath -Force
}
catch {
    if ($_.Exception.Response.StatusCode -eq [System.Net.HttpStatusCode]::NotFound) {
        Write-Host "[-] No releases found on GitHub yet." -ForegroundColor Red
        Write-Host "    Please create a release at https://github.com/$repo/releases/new" -ForegroundColor Yellow
    }
    else {
        Write-Error "Failed to launch RustyPing: $_"
        Write-Host "Make sure you are connected to the internet and GitHub is accessible." -ForegroundColor Red
    }
}
