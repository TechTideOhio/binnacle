# Binnacle Installer for Windows
# Usage:
#   Invoke-WebRequest -Uri https://raw.githubusercontent.com/TechTideOhio/binnacle/main/install.ps1 -OutFile install.ps1
#   powershell -ExecutionPolicy Bypass -File install.ps1

$ErrorActionPreference = "Stop"
$Repo = "TechTideOhio/binnacle"
$BinName = "binnacle.exe"
$InstallDir = "$env:LOCALAPPDATA\Programs\binnacle\bin"

Write-Host "Installing Binnacle (TideOS Evaluator Agent)..." -ForegroundColor Cyan

$ReleaseUri = "https://api.github.com/repos/$Repo/releases/latest"
try {
    $Release = Invoke-RestMethod -Uri $ReleaseUri -UseBasicParsing
    $Asset = $Release.assets | Where-Object { $_.name -like "*windows*.zip" } | Select-Object -First 1
    if (-not $Asset) {
        throw "No Windows archive found in latest release."
    }
    $DownloadUrl = $Asset.browser_download_url
    
    $TempZip = Join-Path $env:TEMP "binnacle-release.zip"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
    Remove-Item -Force $TempZip
} catch {
    Write-Host "Notice: Release asset download encountered ($($_.Exception.Message)). Building via cargo install..." -ForegroundColor Yellow
    cargo install --git "https://github.com/$Repo"
    $InstallDir = Join-Path $env:USERPROFILE ".cargo\bin"
}

$UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", [EnvironmentVariableTarget]::User)
    Write-Host "Added $InstallDir to User PATH." -ForegroundColor Green
}
$env:Path = "$InstallDir;$env:Path"

Write-Host "Binnacle successfully installed to $InstallDir!" -ForegroundColor Green
Write-Host "Verifying installation..." -ForegroundColor Cyan

$BinnacleBin = Join-Path $InstallDir $BinName
if (Test-Path $BinnacleBin) {
    & $BinnacleBin table | Out-Null
    Write-Host "Verification successful! Run 'binnacle doctor' to inspect tool health." -ForegroundColor Green
} else {
    Write-Host "Installed, but $BinnacleBin not found directly. Please ensure $InstallDir is in your PATH." -ForegroundColor Yellow
}
