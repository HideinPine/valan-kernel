cargo build 
sleep 1
cp target/x86_64-unknown-uefi/debug/primer.efi esp/efi/boot/bootx64.efi
sleep 1
qemu-system-x86_64 -enable-kvm \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.4m.fd \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.4m.fd \
  -drive format=raw,file=fat:rw:esp
