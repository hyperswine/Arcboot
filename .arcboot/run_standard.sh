# * could prob just have one script that checks $1 for arch, $2 for standard or uefi, $3 .. $10 for extra args to pass into qemu

cargo barmstd && \
cd .arcboot && \
rm -rf diskImage/EFI/BOOT/BOOTAA64.EFI && \
cp ../build/uefi.efi diskImage/EFI/BOOT/BOOTAA64.EFI && \
qemu-system-aarch64 -M virt -m 2G -cpu cortex-a72 -bios aarch64/qemu/OVMF_EFI.fd -drive file=fat:rw:diskImage -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none $1 $2
