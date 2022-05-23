# ArcBoot v0

A uefi bootloader for riscv, arm and x86. Comes in the form of an executable and an API for arcboot kernels to use in order to become arcboot compliant and use arcboot services/data structures .

- Build arcutils first, then use that to build, run and test arcboot
- Arcboot kernels written in rust can use the arcboot API for the most part and not get into asm esp in the earlier parts. Arcboot API also exports functions that sets up and register exceptions which the kernel can call and hook onto its own handlers

## Original

The original command:

```bash
qemu-system-aarch64 -M virt -cpu cortex-a72 -drive if=pflash,format=raw,unit=0,file=./build/aarch64/OVMF_EFI.fd,readonly=on -drive if=pflash,format=raw,unit=1,file=./build/aarch64/VARS.fd -kernel format=raw,file=build/arcboot.efi -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none
```

Be sure to `chmod +x run.sh`!
