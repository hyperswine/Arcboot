# Testing Arcboot

I dunno rn tbh. It is quite close to bare metal. It is bare metal without std. A custom test harness is possible to use with a `arcutils` flasher and QEMU runner.

Maybe `arcutils test --arcboot` to actually build a QEMU-test config/debug build. And flash it onto a vhd to then run QEMU.

## ELF Runner

Kernel images can be built as an ELF executable or just a binary blob in the case of ARM.

- arcboot sets up a stack, heap, bss, etc. for the kernel
- it then sets up 48 bit paging and goes into S-Mode. Then it can pass control to the loaded kernel code

No support for 39/52/64 bit paging yet.

## QEMU

Arcboot should be flashed onto a vhd file. QEMU should be run with OVMF firmware.

To specify the vhd file `--disk=<main_disk.vhd>`. If everything goes well, QEMU should be able to load Arcboot into the host memory and hand off control to it in M-mode/EL2.

Or a VFAT. Just specify `fat:`.

To launch:

```bash
qemu-system-aarch64 -M virt -cpu cortex-a57 -bios ./build/aarch64/OVMF_EFI.fd -kernel build/arcboot.efi -serial mon:stdio -nographic
```

NOTE: you cant use `-cpu host -accel hvf` on M1 mac for arcboot.

For a virtual FAT partition, specify `-drive file=fat:rw:build/aarch64/qemu`. You can specify as many drives of file=fat on UEFI I think. Not sure if that actually works though

## SMP

To use SMP with QEMU, do `-smp cpus=8`. Or how many CPUs you want to simulate. If using `-cpu host`. I think you can just do the amount of SMP cores on your system. On x86, can also specify `threads=16` to use hyperthreading/clustered multithreading.
