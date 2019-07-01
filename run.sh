#!/bin/sh

cargo xbuild --target x86_64-unknown-uefi
cp ./NvVars ./target/x86_64-unknown-uefi/debug/

qemu-system-x86_64 \
    -nodefaults \
    -nographic \
    -serial stdio \
    -bios /usr/share/ovmf/x64/OVMF_CODE.fd \
    -drive format=raw,file=fat:rw:./target/x86_64-unknown-uefi/debug
