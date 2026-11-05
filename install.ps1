# Binnacle Installer for Windows
# Usage: irm https://raw.githubusercontent.com/TechTideOhio/binnacle/main/install.ps1 | iex

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
} catch {
    Write-Host "Warning: Could not fetch release asset ($_.Exception.Message). Falling back to cargo install..." -ForegroundColor Yellow
    cargo install --git "https://github.com/$Repo"
    return
}

$TempZip = Join-Path $env:TEMP "binnacle-release.zip"
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item -Force $TempZip

$UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", [EnvironmentVariableTarget]::User)
    $env:Path += ";$InstallDir"
    Write-Host "Added $InstallDir to User PATH." -ForegroundColor Green
}

Write-Host "Binnacle successfully installed! Run 'binnacle doctor' to get started." -ForegroundColor Green
