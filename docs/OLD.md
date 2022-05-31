# What I removed

I had:

```bash
qemu-system-aarch64 -M virt -cpu cortex-a72 -drive if=pflash,format=raw,unit=0,file=./build/aarch64/OVMF_EFI.fd,readonly=on -drive if=pflash,format=raw,unit=1,file=./build/aarch64/VARS.fd -kernel format=raw,file=build/arcboot.efi -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none
```

which didnt work because I didnt specify the options properly.
