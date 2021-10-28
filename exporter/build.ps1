Invoke-Expression -Command "cargo build --release"

if (Test-Path -Path ".\exporter.exe") {
    Remove-Item -Path ".\exporter.exe"
}

if (Test-Path -Path ".\target\release\exporter.exe") {
    Move-Item -Path ".\target\release\exporter.exe" -Destination ".\"
}