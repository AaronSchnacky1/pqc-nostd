# PowerShell script to extract clean kat.rs from test output
param(
    [string]$InputFile = "kat_output.txt",
    [string]$OutputFile = "src\kat.rs"
)

Write-Host "Extracting clean kat.rs from $InputFile..."

# Read all lines
$lines = Get-Content $InputFile -Encoding UTF8

# Find start line (first line starting with //)
$startLine = -1
for ($i = 0; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match '^// -+') {
        $startLine = $i
        Write-Host "Found start at line $($i + 1)"
        break
    }
}

# Find end line (line with "test create_kat_file")
$endLine = -1
for ($i = $startLine; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match '^test create_kat_file') {
        $endLine = $i - 1
        Write-Host "Found end at line $($i)"
        break
    }
}

if ($startLine -eq -1 -or $endLine -eq -1) {
    Write-Error "Could not find start or end markers"
    exit 1
}

# Extract the lines
$extractedLines = $lines[$startLine..$endLine]

# Write to output file
$extractedLines | Out-File -FilePath $OutputFile -Encoding UTF8

Write-Host "Successfully extracted $($extractedLines.Count) lines to $OutputFile"
Write-Host "File size: $((Get-Item $OutputFile).Length) bytes"
