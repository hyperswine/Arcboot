# idk about VARS.fd maybe we can store it into the VFAT instead of VARS.fd. It comes with 44 variables

cargo barm && \
cd .arcboot && \
rm -rf diskImage/EFI/BOOT/BOOTAA64.EFI && \
cp ../build/arcboot_main.efi diskImage/EFI/BOOT/BOOTAA64.EFI && \
qemu-system-aarch64 -M virt -m 2G -cpu cortex-a72 -bios aarch64/qemu/OVMF_EFI.fd -drive file=fat:rw:diskImage -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none

# NOTE: to use another storage for variables, do -drive file=aarch64/qemu/VARS.fd

# NOTE: pipes stdout and stderr to .arcboot/out

# REMOVED: hdiutil create -fs fat32 -ov -size 48m -volname NEWOS -format UDTO -srcfolder diskImage uefi.cdr && \
# qemu-system-aarch64 -M virt -cpu cortex-a72 -bios aarch64/qemu/OVMF_EFI.fd -drive file=fat:rw:diskImage -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none
# Idk why rsw doesnt work. rsw is for read write? rw for readonly?
