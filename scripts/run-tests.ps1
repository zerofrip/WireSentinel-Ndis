# WireSentinel-Ndis Windows test/build placeholder (WDK required)
param(
    [string]$Configuration = "Release",
    [string]$Platform = "x64"
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $Root "..")
$Project = Join-Path $RepoRoot "ndis-filter\guardian_lwf.vcxproj"

Write-Host "WireSentinel-Ndis Windows build placeholder"
Write-Host "  Repo:        $RepoRoot"
Write-Host "  Project:     $Project"
Write-Host "  Config:      $Configuration"
Write-Host "  Platform:    $Platform"

if (-not (Test-Path $Project)) {
    Write-Error "Driver project not found: $Project"
}

# Rust workspace tests (stub IOCTL on non-kernel paths)
Push-Location $RepoRoot
try {
    cargo test --workspace
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
finally {
    Pop-Location
}

# WDK MSBuild — uncomment when running on a WDK-equipped agent:
# msbuild $Project /p:Configuration=$Configuration /p:Platform=$Platform

Write-Host "WDK driver build skipped (placeholder). Enable msbuild on Windows CI agents."
