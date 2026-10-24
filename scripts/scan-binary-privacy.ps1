param([Parameter(Mandatory=$true)][string]$Binary)
$ErrorActionPreference = 'Stop'
$path = (Resolve-Path -LiteralPath $Binary).Path
$bytes = [IO.File]::ReadAllBytes($path)
$ascii = [Text.Encoding]::Latin1.GetString($bytes)
$utf16 = [Text.Encoding]::Unicode.GetString($bytes)
$pattern = '(?i)[a-z]:\\users\\[^\\\x00\r\n]{1,80}'
$hits = @()
if ([regex]::IsMatch($ascii,$pattern)) { $hits += 'Single-byte Windows profile path' }
if ([regex]::IsMatch($utf16,$pattern)) { $hits += 'UTF-16 Windows profile path' }
if ($hits.Count -gt 0) {
    throw ('PRIVACY CHECK FAILED: ' + ($hits -join '; ') + '. Do not share the build.')
}
Write-Host 'Basic Windows profile path check: passed.' -ForegroundColor Green
Write-Host 'This is not a comprehensive PII or executable security audit.'
