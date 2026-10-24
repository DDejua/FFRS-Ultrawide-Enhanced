# Compare the exact bundled winmm.dll to ThirteenAG's official x64-latest ZIP.
# The installed loader SHA256 was measured from the user-tested candidate ZIP.
param(
  [string]$Upstream = 'upstream',
  [string]$ExpectedDllSha256 = '900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937',
  [string]$ExpectedAssetZipSha256 = '6be4584294c4428e85bd69209dca7c25140461baf3535df547c16478d92fe251'
)
$ErrorActionPreference = 'Stop'
$up = (Resolve-Path -LiteralPath $Upstream).Path
$dest = Join-Path $up 'loader_audit'
New-Item -ItemType Directory -Path $dest -Force | Out-Null
$uri = 'https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/download/x64-latest/winmm-x64.zip'
$tmp = Join-Path $env:RUNNER_TEMP 'ffrs-official-winmm-x64.zip'
$unpacked = Join-Path $env:RUNNER_TEMP 'ffrs-official-winmm-extract'
Invoke-WebRequest -Uri $uri -OutFile $tmp -MaximumRedirection 10
$assetHash = (Get-FileHash -LiteralPath $tmp -Algorithm SHA256).Hash.ToLowerInvariant()
if ($assetHash -ne $ExpectedAssetZipSha256.ToLowerInvariant()) {
  throw "Official asset ZIP hash differs from snapshot; investigate release changes. Actual: $assetHash"
}
New-Item -ItemType Directory -Path $unpacked -Force | Out-Null
Expand-Archive -LiteralPath $tmp -DestinationPath $unpacked -Force
$dll = @(Get-ChildItem -LiteralPath $unpacked -Filter 'winmm.dll' -File -Recurse)
if ($dll.Count -ne 1) { throw "Expected one official winmm.dll, found $($dll.Count)" }
$dllHash = (Get-FileHash -LiteralPath $dll[0].FullName -Algorithm SHA256).Hash.ToLowerInvariant()
if ($dllHash -ne $ExpectedDllSha256.ToLowerInvariant()) {
  throw "User-tested bundled winmm.dll does not match this official asset. Actual: $dllHash"
}
$report = @(
  'Ultimate ASI Loader official binary provenance'
  "Official URL: $uri"
  "Official archive SHA256: $assetHash"
  "Unpacked official winmm.dll SHA256: $dllHash"
  'Result: Official DLL matches winmm.dll from user-tested GitHub-built candidate exactly.'
  'Verified against ThirteenAG asset digest noted on 2026-10-24.'
  'License: https://github.com/ThirteenAG/Ultimate-ASI-Loader/blob/master/license (MIT)'
) -join [Environment]::NewLine
[IO.File]::WriteAllText((Join-Path $dest 'LOADER_PROVENANCE.txt'),$report,(New-Object Text.UTF8Encoding($false)))
Write-Host $report
