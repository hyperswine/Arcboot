# * could prob just have one script that checks $1 for arch, $2 for standard or uefi, $3 .. $10 for extra args to pass into qemu

cargo barmstd && \
cd .arcboot && \
rm -rf diskImage/EFI/BOOT/BOOTAA64.EFI && \
qemu-system-aarch64 -M virt -m 2G -cpu cortex-a72 -kernel build/standard.elf -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=2 -net none $1 $2
