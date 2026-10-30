param([string]$TestBinary)

# cargo already built the test binary, so no cargo build here.
# Fern loads "\kernel" from the ESP, so the test binary gets that name.
Copy-Item $TestBinary "$PSScriptRoot/../fern_bootloader/esp/kernel" -Force

Set-Location "$PSScriptRoot/../fern_bootloader"
./run.ps1 -Test        # see below

# QEMU exits with (value << 1) | 1, so a kernel writing 0x10 gives 33
if ($LASTEXITCODE -eq 33) { exit 0 } else { exit 1 }