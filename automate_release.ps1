# RustyPing Release Automation Script
# This script automates the entire release process for RustyPing.

$ErrorActionPreference = "Stop"

function Write-Step ($message) {
    Write-Host "`n🚀 $message" -ForegroundColor Cyan
}

function Write-Success ($message) {
    Write-Host "✅ $message" -ForegroundColor Green
}

function Write-Warning ($message) {
    Write-Host "⚠️ $message" -ForegroundColor Yellow
}

function Write-Error-Exit ($message) {
    Write-Host "❌ $message" -ForegroundColor Red
    exit 1
}

# 1. Check Prerequisites
Write-Step "Checking Prerequisites..."

if (!(Get-Command "git" -ErrorAction SilentlyContinue)) {
    Write-Error-Exit "Git is not installed or not in PATH."
}

if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Error-Exit "Cargo is not installed or not in PATH."
}

# Ensure we are on main
$currentBranch = git rev-parse --abbrev-ref HEAD
if ($currentBranch -ne "main") {
    Write-Warning "You are not on the 'main' branch (current: $currentBranch)."
    $confirmBranch = Read-Host "Proceed anyway? (y/n)"
    if ($confirmBranch -ne "y") { exit 1 }
}

# Check for uncommitted changes
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Warning "Working directory is not clean. Please commit or stash changes first."
    git status
    exit 1
}

# Ensure remote 'origin' exists
if (!(git remote | Select-String "origin")) {
    Write-Error-Exit "Remote 'origin' not found. Please add a remote first."
}

# 2. Get Version Info
Write-Step "Getting Version Info..."
$currentVersion = (Get-Content "Cargo.toml" | Select-String "^version = `"(.*)`"").Matches.Groups[1].Value
Write-Host "Current Version: $currentVersion" -ForegroundColor Gray

$newVersion = Read-Host "Enter New Version (e.g., 2.5.9)"
if ([string]::IsNullOrWhiteSpace($newVersion)) {
    Write-Error-Exit "Version cannot be empty."
}

# Ensure the version format is correct (e.g., X.Y.Z)
if ($newVersion -notmatch "^\d+\.\d+\.\d+$") {
    Write-Warning "Version format should be X.Y.Z (e.g., 2.5.9). Proceeding anyway..."
}

$fullVersion = "v$newVersion"
$releaseBranch = "release/$fullVersion"

# 3. Create Release Branch
Write-Step "Creating Release Branch: $releaseBranch"
git checkout -b $releaseBranch

# 4. Update Version Numbers in Files
Write-Step "Updating Version Numbers in Files..."

$filesToUpdate = @(
    @{ Path = "Cargo.toml"; Pattern = '^version = ".*"'; Replace = "version = `"$newVersion`"" },
    @{ Path = "src/main.rs"; Pattern = 'RustyPing v\d+\.\d+\.\d+'; Replace = "RustyPing $fullVersion" },
    @{ Path = "src/menu.rs"; Pattern = 'Span::raw\(" v\d+\.\d+\.\d+"\)'; Replace = "Span::raw(`" $fullVersion`")" },
    @{ Path = "linux_install.sh"; Pattern = 'RustyPing Linux Utility v\d+\.\d+\.\d+'; Replace = "RustyPing Linux Utility $fullVersion" },
    @{ Path = "run_portable.sh"; Pattern = 'RustyPing Linux Portable Launcher v\d+\.\d+\.\d+'; Replace = "RustyPing Linux Portable Launcher $fullVersion" },
    @{ Path = "run_portable.ps1"; Pattern = 'RustyPing Portable Launcher v\d+\.\d+\.\d+'; Replace = "RustyPing Portable Launcher $fullVersion" }
)

foreach ($file in $filesToUpdate) {
    if (Test-Path $file.Path) {
        Write-Host "Updating $($file.Path)..." -ForegroundColor Gray
        $content = Get-Content $file.Path
        $newContent = $content -replace $file.Pattern, $file.Replace
        Set-Content $file.Path $newContent
    } else {
        Write-Warning "File not found: $($file.Path)"
    }
}

# 5. Update Changelog & Release Notes
Write-Step "Updating Changelog and Release Notes..."
$today = Get-Date -Format "yyyy-MM-dd"
$changelogPath = "CHANGELOG.md"
$releaseNotesPath = "RELEASE_NOTES.md"

if (Test-Path $changelogPath) {
    $changelog = Get-Content $changelogPath
    if (!($changelog -match "## \[$fullVersion\]")) {
        Write-Warning "No entry for $fullVersion found in CHANGELOG.md."
        Write-Host "Please add the entry now. I will wait for you to save the file." -ForegroundColor Yellow
        pause
    }
    
    # Extract latest version notes for RELEASE_NOTES.md
    $extracting = $false
    $notes = @()
    foreach ($line in $changelog) {
        if ($line -match "## \[$fullVersion\]") {
            $extracting = $true
            $notes += "# Release Notes - RustyPing $fullVersion"
            $notes += ""
            continue
        }
        if ($extracting) {
            if ($line -match "## \[v\d+\.\d+\.\d+\]") {
                break
            }
            $notes += $line
        }
    }
    
    if ($notes.Count -gt 0) {
        Write-Host "Generating RELEASE_NOTES.md from CHANGELOG.md..." -ForegroundColor Gray
        $notes | Set-Content $releaseNotesPath
    }
}

# 6. Build and Test
Write-Step "Running Tests..."
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Error-Exit "Tests failed. Release aborted."
}
Write-Success "Tests passed."

# 7. Commit and Tag
Write-Step "Committing Changes and Tagging..."
git add .
git commit -m "chore: release $fullVersion"
git tag -a $fullVersion -m "Release $fullVersion"
Write-Success "Changes committed and tagged as $fullVersion."

# 8. Push to Remote
Write-Step "Ready to Push to Remote?"
$confirm = Read-Host "Push branch '$releaseBranch' and tag '$fullVersion' to origin? (y/n)"
if ($confirm -eq "y") {
    git push origin $releaseBranch --tags
    Write-Success "Release pushed successfully!"
    
    Write-Host "`n🚀 WHAT'S NEXT?" -ForegroundColor Cyan
    Write-Host "1. Open a Pull Request for '$releaseBranch' on GitHub." -ForegroundColor White
    Write-Host "2. Once merged, the GitHub Actions will automatically build and publish assets." -ForegroundColor White
    Write-Host "3. Check: https://github.com/pdzjtechnagy/RustyPing/actions" -ForegroundColor White
} else {
    Write-Warning "Push skipped. You can push manually using:"
    Write-Host "git push origin $releaseBranch --tags" -ForegroundColor Gray
}

Write-Step "Cleaning up release_helper.ps1 (deprecated)..."
if (Test-Path "release_helper.ps1") {
    Remove-Item "release_helper.ps1"
    Write-Success "Old helper removed."
}

Write-Host "`n✨ Release process complete!" -ForegroundColor Green
