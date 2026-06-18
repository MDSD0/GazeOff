#!/usr/bin/env pwsh
# dev.ps1 — kill, rebuild, relaunch gazeOff
# Usage: .\dev.ps1

Write-Host "Stopping old instance..." -ForegroundColor DarkGray
taskkill /F /IM gazeoff.exe 2>$null | Out-Null
Start-Sleep -Milliseconds 400

# Touch main.rs so tauri-codegen re-embeds HTML files
(Get-Item src\main.rs).LastWriteTime = Get-Date

Write-Host "Building..." -ForegroundColor DarkGray
$result = cargo build 2>&1
$ok = $result | Select-String "Finished"
$err = $result | Select-String "^error"

if ($err) {
    Write-Host "`nBuild failed:" -ForegroundColor Red
    $err | ForEach-Object { Write-Host $_ -ForegroundColor Red }
    exit 1
}

Write-Host "Launching..." -ForegroundColor DarkGray
Start-Process .\target\debug\gazeoff.exe
Write-Host "Done. gazeOff is running." -ForegroundColor Green
