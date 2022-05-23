# NOTE: run.sh is actually run in the root proj dir
# so the paths should be relative to that

cd .arcboot && \
rm -rf diskImage/EFI/BOOT/BOOTARM.EFI && \
cp ../build/arcboot.efi diskImage/EFI/BOOT/BOOTARM.EFI && \
hdiutil create -fs fat32 -ov -size 48m -volname NEWOS -format UDTO -srcfolder diskImage uefi.cdr && \
qemu-system-aarch64 -M virt -cpu cortex-a72 -bios aarch64/qemu/OVMF_EFI.fd -drive file=uefi.cdr -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none
