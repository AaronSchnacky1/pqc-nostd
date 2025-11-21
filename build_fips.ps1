# Build script for FIPS 140-3 Compliance
# Builds the binary and injects the HMAC for the integrity test.

$ErrorActionPreference = "Stop"

Write-Host "Building FIPS Application..." -ForegroundColor Cyan
cargo build --bin fips_app --features "ml-kem,ml-dsa,fips_140_3"

$binPath = "target\debug\fips_app.exe"

if (-not (Test-Path $binPath)) {
    Write-Error "Binary not found at $binPath"
}

Write-Host "Injecting HMAC for Integrity Test..." -ForegroundColor Cyan
cargo run --bin inject_hmac --features "ml-kem,ml-dsa,fips_140_3" -- $binPath

Write-Host "Build Complete. Running FIPS Application..." -ForegroundColor Green
& $binPath
