# Prepares a pinned upstream clone for a candidate source build.
$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$destination = Join-Path $repoRoot 'upstream'
$expected = '0f5bc2c4bab2b1680c59d8d1c2135253ae14c0df'
if (Test-Path -LiteralPath $destination) {
    throw "Directory already exists: $destination. Nothing changed."
}
& git clone --depth 1 --branch 0.3.0 'https://github.com/PolarWizard/final-fantasy-resonance-fix.git' $destination
if ($LASTEXITCODE -ne 0) { throw 'Upstream clone failed.' }
$actual = (& git -C $destination rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0 -or $actual -ne $expected) {
    throw "Upstream tag did not match pinned commit. Expected $expected; got $actual. Stop!"
}
$overrides = Join-Path $repoRoot 'overrides'
$all = Get-ChildItem -LiteralPath $overrides -File -Recurse
foreach ($source in $all) {
    $relative = $source.FullName.Substring($overrides.Length).TrimStart('\','/')
    $target = Join-Path $destination $relative
    if (!(Test-Path -LiteralPath $target -PathType Leaf) -and
        $relative -ne 'src\fixes\minimap.rs' -and
        $relative -ne 'src/fixes/minimap.rs' -and
        $relative -ne 'src\fixes\dof_aspect.rs' -and
        $relative -ne 'src/fixes/dof_aspect.rs') {
        throw "Expected upstream file missing: $relative"
    }
    $targetFolder = Split-Path -Parent $target
    New-Item -Path $targetFolder -ItemType Directory -Force | Out-Null
    Copy-Item -LiteralPath $source.FullName -Destination $target -Force
}
Write-Host "Source overlay prepared at $destination" -ForegroundColor Green
Write-Host 'Next: cd upstream; cargo build --locked --release'
