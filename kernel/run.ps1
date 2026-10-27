cargo build

cp target/x86_64-valan/debug/kernel ../fern_bootloader/esp/
cd ../fern_bootloader
./../fern_bootloader/run.ps1