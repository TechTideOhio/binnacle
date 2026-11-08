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
$TempZip = Join-Path $env:TEMP "binnacle-release.zip"
try {
    # Release binaries are x86_64 (also runs on Windows ARM64 via emulation).
    $Release = Invoke-RestMethod -Uri $ReleaseUri -UseBasicParsing
    $AssetName = "binnacle-windows-x86_64.zip"
    $Asset = $Release.assets | Where-Object { $_.name -eq $AssetName } | Select-Object -First 1
    $HashAsset = $Release.assets | Where-Object { $_.name -eq "$AssetName.sha256" } | Select-Object -First 1
    if (-not $Asset) {
        throw "No $AssetName found in latest release."
    }
    Invoke-WebRequest -Uri $Asset.browser_download_url -OutFile $TempZip -UseBasicParsing
} catch {
    $Asset = $null
    Write-Host "Notice: Release asset download encountered ($($_.Exception.Message)). Building via cargo install..." -ForegroundColor Yellow
}

if ($Asset) {
    # Fail closed: never install an unverified binary.
    if (-not $HashAsset) {
        Remove-Item -Force $TempZip -ErrorAction SilentlyContinue
        throw "Checksum file $AssetName.sha256 is missing for release $($Release.tag_name); refusing to install."
    }
    $Expected = ((Invoke-RestMethod -Uri $HashAsset.browser_download_url -UseBasicParsing).ToString().Trim() -split '\s+')[0].ToLower()
    $Actual = (Get-FileHash -Algorithm SHA256 $TempZip).Hash.ToLower()
    if ($Expected -ne $Actual) {
        Remove-Item -Force $TempZip -ErrorAction SilentlyContinue
        throw "Checksum mismatch for $AssetName (expected $Expected, got $Actual); refusing to install."
    }
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
    Remove-Item -Force $TempZip
} else {
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
