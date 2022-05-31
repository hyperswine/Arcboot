# if succeed build test, then take the last 3 lines
# and bind that to VARS
# OUT1 - lib target
# OUT2 - bin target
# for test_case, always run test_case or something

cargo tarm | LIB_TGT=grep "Finished" && \
cd .arcboot && \
rm -rf diskImage/EFI/BOOT/BOOTAA64.EFI && \
cp ../build/arcboot.efi diskImage/EFI/BOOT/BOOTAA64.EFI && \
qemu-system-aarch64 -M virt -cpu cortex-a72 -bios aarch64/qemu/OVMF_EFI.fd -drive file=fat:rw:diskImage -drive file=aarch64/qemu/VARS.fd -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none
