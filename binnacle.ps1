#!/usr/bin/env pwsh
# binnacle: TideOS Evaluator Agent runner wrapper

[CmdletBinding()]
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Arguments
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

$BinRelease = Join-Path $ScriptDir "target\release\binnacle.exe"
$BinDebug = Join-Path $ScriptDir "target\debug\binnacle.exe"

if (-not (Test-Path $BinRelease) -and -not (Test-Path $BinDebug)) {
    Write-Host "[binnacle] Compiling release binary..." -ForegroundColor Cyan
    Push-Location $ScriptDir
    try {
        cargo build --release
    } finally {
        Pop-Location
    }
}

$Bin = if (Test-Path $BinRelease) { $BinRelease } else { $DebugBin }
& $Bin @Arguments
exit $LASTEXITCODE
