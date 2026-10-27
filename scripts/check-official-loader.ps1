# Fail-closed reuse of the EXACT user-tested winmm.dll from the v1.1.0
# release, originally verified against ThirteenAG's official x64 asset.
# This avoids trusting the mutable upstream "x64-latest" tag, which changed
# its ZIP hash after v1.1.0. Do not silently approve a different DLL.
#
# Source-of-truth chain:
#  v1.1.0 public ZIP digest (release's locked artifact) -> extracted
#  original winmm.dll digest -> 2026-10-25 original ThirteenAG provenance.
param(
  [string]$Upstream = 'upstream',
  [string]$ExpectedReleaseZipSha256 = 'a3abfbe40a0e55002876fa988e67896ad1eccc8fc52adc4e69df546dcc2d26b0',
  [string]$ExpectedDllSha256 = '900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937',
  [string]$OriginalOfficialAssetZipSha256 = '6be4584294c4428e85bd69209dca7c25140461baf3535df547c16478d92fe251'
)
$ErrorActionPreference = 'Stop'
$up = (Resolve-Path -LiteralPath $Upstream).Path
$dest = Join-Path $up 'loader_audit'
New-Item -ItemType Directory -Path $dest -Force | Out-Null
$uri = 'https://github.com/DDejua/FFRS-Ultrawide-Enhanced/releases/download/v1.1.0/FFRS.Ultrawide.DOF.Fix.V1.1.0_RimuModding.zip'
$tmp = Join-Path $env:RUNNER_TEMP 'ffrs-verified-v1.1.0.zip'
$unpacked = Join-Path $env:RUNNER_TEMP 'ffrs-verified-v1.1.0-extract'
Invoke-WebRequest -Uri $uri -OutFile $tmp -MaximumRedirection 10
$assetHash = (Get-FileHash -LiteralPath $tmp -Algorithm SHA256).Hash.ToLowerInvariant()
if ($assetHash -ne $ExpectedReleaseZipSha256.ToLowerInvariant()) {
  throw "Historical verified v1.1.0 ZIP mismatch: $assetHash. Fail closed."
}
New-Item -ItemType Directory -Path $unpacked -Force | Out-Null
Expand-Archive -LiteralPath $tmp -DestinationPath $unpacked -Force
$dll = @(Get-ChildItem -LiteralPath $unpacked -Filter 'winmm.dll' -File -Recurse)
if ($dll.Count -ne 1) { throw "Expected one v1.1.0 winmm.dll, found $($dll.Count)" }
$dllHash = (Get-FileHash -LiteralPath $dll[0].FullName -Algorithm SHA256).Hash.ToLowerInvariant()
if ($dllHash -ne $ExpectedDllSha256.ToLowerInvariant()) {
  throw "Published v1.1.0 loader differs from author-approved ThirteenAG DLL: $dllHash"
}
Copy-Item -LiteralPath $dll[0].FullName -Destination (Join-Path $dest 'winmm.dll') -Force
# Include the hash-verified previous manual package in the internal CI artifact so
# a new manual-only release may retain all original files, legal notices and
# HUD presets byte-for-byte. Never distribute this nested old ZIP as the update.
Copy-Item -LiteralPath $tmp -Destination (Join-Path $dest 'approved-v1.1.0-source.zip') -Force
$originalSource = 'https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/download/x64-latest/winmm-x64.zip'
$report = @(
  'Ultimate ASI Loader: frozen verified original v1.1.0 binary'
  "Current artifact verified from: $uri"
  "Immutable release ZIP SHA256: $assetHash"
  "Embedded winmm.dll SHA256: $dllHash"
  "Originally verified against ThirteenAG x64-latest on 2026-10-25: $originalSource"
  "Original upstream ZIP digest (historical, NOT a current x64-latest check): $OriginalOfficialAssetZipSha256"
  'Reason for freezing: mutable upstream asset ZIP changed since the approved v1.1.0 release.'
  'Result: EXACT winmm.dll byte-for-byte preserved from validated v1.1.0 distribution.'
  'Copyright: ThirteenAG Ultimate ASI Loader (MIT)'
  'License: https://github.com/ThirteenAG/Ultimate-ASI-Loader/blob/master/license'
) -join [Environment]::NewLine
[IO.File]::WriteAllText((Join-Path $dest 'LOADER_PROVENANCE.txt'),$report,(New-Object Text.UTF8Encoding($false)))
Write-Host $report
