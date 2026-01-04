# PowerShell script to self-sign roblox_booster.exe
# Run as Administrator

param(
    [string]$ExePath = "roblox_booster.exe",
    [string]$CertName = "Roblox Booster Certificate"
)

Write-Host "================================" -ForegroundColor Cyan
Write-Host "Self-Signing Roblox Booster" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as admin
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "ERROR: This script must be run as Administrator!" -ForegroundColor Red
    Write-Host "Right-click PowerShell and select 'Run as Administrator'" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

# Check if exe exists
if (-not (Test-Path $ExePath)) {
    Write-Host "ERROR: $ExePath not found!" -ForegroundColor Red
    exit 1
}

Write-Host "[1/4] Creating self-signed certificate..." -ForegroundColor Yellow

# Create certificate
$cert = New-SelfSignedCertificate `
    -Type CodeSigningCert `
    -Subject "CN=$CertName" `
    -CertStoreLocation Cert:\CurrentUser\My `
    -NotAfter (Get-Date).AddYears(5)

Write-Host "✓ Certificate created: $($cert.Thumbprint)" -ForegroundColor Green
Write-Host ""

Write-Host "[2/4] Moving certificate to Trusted Root..." -ForegroundColor Yellow

# Export and import to Trusted Root (so Windows trusts it)
$certPath = "Cert:\CurrentUser\My\$($cert.Thumbprint)"
$rootStore = New-Object System.Security.Cryptography.X509Certificates.X509Store(
    [System.Security.Cryptography.X509Certificates.StoreName]::Root,
    [System.Security.Cryptography.X509Certificates.StoreLocation]::CurrentUser
)
$rootStore.Open("ReadWrite")
$rootStore.Add($cert)
$rootStore.Close()

Write-Host "✓ Certificate trusted" -ForegroundColor Green
Write-Host ""

Write-Host "[3/4] Signing executable..." -ForegroundColor Yellow

# Sign the exe
try {
    Set-AuthenticodeSignature -FilePath $ExePath -Certificate $cert -TimestampServer "http://timestamp.digicert.com"
    Write-Host "✓ Executable signed successfully!" -ForegroundColor Green
} catch {
    Write-Host "ERROR: Failed to sign executable: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[4/4] Verifying signature..." -ForegroundColor Yellow

# Verify signature
$signature = Get-AuthenticodeSignature -FilePath $ExePath
if ($signature.Status -eq "Valid") {
    Write-Host "✓ Signature verified!" -ForegroundColor Green
} else {
    Write-Host "⚠ Signature status: $($signature.Status)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "================================" -ForegroundColor Green
Write-Host "Signing Complete!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green
Write-Host ""
Write-Host "Certificate Thumbprint: $($cert.Thumbprint)" -ForegroundColor Cyan
Write-Host "Valid Until: $($cert.NotAfter)" -ForegroundColor Cyan
Write-Host ""
Write-Host "NOTE: This is a self-signed certificate." -ForegroundColor Yellow
Write-Host "Users on other computers will still see a warning unless they install this certificate." -ForegroundColor Yellow
Write-Host ""
Write-Host "To export certificate for distribution:" -ForegroundColor Cyan
Write-Host "  Export-Certificate -Cert Cert:\CurrentUser\My\$($cert.Thumbprint) -FilePath certificate.cer" -ForegroundColor White
Write-Host ""

Read-Host "Press Enter to exit"