# Collect real license files and SPDX declarations from Cargo's locked packages.
param([string]$Upstream = 'upstream')
$ErrorActionPreference = 'Stop'
$up = (Resolve-Path -LiteralPath $Upstream).Path
$metadata = Join-Path $up 'dependency-metadata.json'
if (!(Test-Path -LiteralPath $metadata)) { throw 'Cargo metadata missing.' }
$all = Get-Content -LiteralPath $metadata -Raw -Encoding UTF8 | ConvertFrom-Json
$pkgs = @($all.packages | Where-Object { $_.source -like 'registry+*' } | Sort-Object name, version)
$dest = Join-Path $up 'license_audit'
New-Item -ItemType Directory -Path $dest -Force | Out-Null
$notice = New-Object System.Text.StringBuilder
$report = New-Object System.Collections.Generic.List[string]
$report.Add('# Pinned Rust dependencies and license source files')
$report.Add('')
$report.Add('Generated from crate source files cached on the Windows CI builder.')
$report.Add('Declared SPDX does not replace manual notice review.')
$report.Add('')
$report.Add('| Crate | Version | Declared SPDX | License files located |')
$report.Add('|---|---|---|---|')
$missing = @()
foreach ($p in $pkgs) {
    $path = Split-Path -Parent $p.manifest_path
    if (!(Test-Path -LiteralPath $path -PathType Container)) { throw "Missing crate source $($p.name) $($p.version)" }
    $licenseFiles = @(Get-ChildItem -LiteralPath $path -File |
        Where-Object { $_.Name -match '^(LICENSE|LICENCE|COPYING|NOTICE|UNLICENSE|PATENTS)([._-].*)?$' } |
        Sort-Object Name)
    # iced-x86 1.21.0's registry tarball omits the repo-root LICENSE.txt.
    # Use the corresponding v1.21.0 notice committed from icedland/iced.
    if ($licenseFiles.Count -eq 0 -and $p.name -eq 'iced-x86' -and $p.version -eq '1.21.0') {
        $repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
        $fallback = Join-Path $repoRoot 'LICENSES\iced-x86-1.21.0_MIT.txt'
        if (!(Test-Path -LiteralPath $fallback -PathType Leaf)) { throw 'Pinned iced-x86 license notice missing.' }
        $licenseFiles = @(Get-Item -LiteralPath $fallback)
    }
    $names = @($licenseFiles | ForEach-Object { $_.Name })
    if ($licenseFiles.Count -eq 0) { $missing += "$($p.name) $($p.version)" }
    $report.Add("| $($p.name) | $($p.version) | $($p.license) | $($names -join ', ') |")
    [void]$notice.AppendLine(('=' * 72))
    [void]$notice.AppendLine("$($p.name) $($p.version)")
    [void]$notice.AppendLine("Declared SPDX: $($p.license)")
    [void]$notice.AppendLine('Registry source: crates.io locked version')
    [void]$notice.AppendLine(('=' * 72))
    [void]$notice.AppendLine()
    foreach ($f in $licenseFiles) {
        [void]$notice.AppendLine("----- $($f.Name) -----")
        [void]$notice.AppendLine([IO.File]::ReadAllText($f.FullName))
        [void]$notice.AppendLine()
    }
    if ($licenseFiles.Count -eq 0) {
        [void]$notice.AppendLine('NO ROOT LICENSE FILE; MANUAL REVIEW REQUIRED.')
        [void]$notice.AppendLine()
    }
}
$report.Add('')
$report.Add("Packages audited: $($pkgs.Count).")
$report.Add("Missing root license files: $($missing.Count).")
if ($missing.Count) { $report.Add('Manual review needed: ' + ($missing -join ', ')) }
[IO.File]::WriteAllText((Join-Path $dest 'CRATE_NOTICE_REPORT.md'), ($report -join [Environment]::NewLine), (New-Object Text.UTF8Encoding($false)))
[IO.File]::WriteAllText((Join-Path $dest 'ThirdPartyRustNotices.txt'), $notice.ToString(), (New-Object Text.UTF8Encoding($false)))
Write-Host "Audited $($pkgs.Count) crates; missing root license files: $($missing.Count)."
