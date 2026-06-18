$ErrorActionPreference = "Stop"

$AppName = "gazeOff"
$Subject = "CN=$AppName"
$ExePath = ".\target\release\gazeoff.exe"

Write-Host "Building application in release mode..." -ForegroundColor Cyan
# Run cargo build
cargo build --release

if (-not (Test-Path $ExePath)) {
    Write-Error "Could not find $ExePath. Build might have failed."
    exit 1
}

Write-Host "Checking for existing self-signed certificate for $AppName..." -ForegroundColor Cyan
$Cert = Get-ChildItem Cert:\CurrentUser\My | Where-Object { $_.Subject -eq $Subject } | Select-Object -First 1

if (-not $Cert) {
    Write-Host "No certificate found. Generating a new self-signed certificate..." -ForegroundColor Yellow
    $Cert = New-SelfSignedCertificate -Subject $Subject -Type CodeSigningCert -CertStoreLocation "Cert:\CurrentUser\My" -FriendlyName "$AppName Code Signing"
    Write-Host "Created certificate with Thumbprint: $($Cert.Thumbprint)" -ForegroundColor Green
} else {
    Write-Host "Found existing certificate with Thumbprint: $($Cert.Thumbprint)" -ForegroundColor Green
}

# Find signtool.exe
$SignToolPath = ""
$WindowsKitsDir = "C:\Program Files (x86)\Windows Kits\10\bin"
if (Test-Path $WindowsKitsDir) {
    # Find the latest SDK version
    $LatestKit = Get-ChildItem $WindowsKitsDir | Where-Object { $_.PSIsContainer -and $_.Name -match "^10\." } | Sort-Object Name -Descending | Select-Object -First 1
    if ($LatestKit) {
        $PossiblePath = Join-Path $LatestKit.FullName "x64\signtool.exe"
        if (Test-Path $PossiblePath) {
            $SignToolPath = $PossiblePath
        }
    }
}

if ($SignToolPath -eq "") {
    # Fallback to checking the system path
    $SignToolPath = (Get-Command signtool.exe -ErrorAction SilentlyContinue).Source
}

if (-not $SignToolPath) {
    Write-Error "Could not find signtool.exe. Please ensure the Windows SDK is installed."
    exit 1
}

Write-Host "Using signtool from: $SignToolPath" -ForegroundColor Cyan

# Sign the executable
Write-Host "Signing the executable..." -ForegroundColor Cyan
# We use the SHA256 file digest algorithm (/fd) and apply a timestamp (/tr)
& $SignToolPath sign /sha1 $Cert.Thumbprint /fd SHA256 /tr "http://timestamp.digicert.com" /td SHA256 $ExePath

if ($LASTEXITCODE -eq 0) {
    Write-Host "Successfully signed $ExePath!" -ForegroundColor Green
    
    # Export certificate to a .cer file so the user can easily install it
    $ExportPath = ".\$AppName-Cert.cer"
    Export-Certificate -Cert $Cert -FilePath $ExportPath -Force | Out-Null
    Write-Host "Exported the public certificate to: $ExportPath" -ForegroundColor Cyan
    Write-Host "NOTE: This is a self-signed cert for LOCAL testing only." -ForegroundColor Yellow
    Write-Host "  - Installing it into 'Trusted Root' silences SmartScreen on THIS PC only." -ForegroundColor DarkYellow
    Write-Host "  - It does NOT silence SmartScreen for other users, and it does NOT" -ForegroundColor DarkYellow
    Write-Host "    satisfy Smart App Control anywhere. Those need a real CA / Azure" -ForegroundColor DarkYellow
    Write-Host "    Trusted Signing certificate. See README -> Installing." -ForegroundColor DarkYellow
} else {
    Write-Error "Failed to sign the executable."
    exit 1
}
