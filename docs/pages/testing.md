---
layout: default
title: Testing
---

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

For a virtual FAT partition, specify `-drive format=raw,file=fat:rw:build/aarch64/qemu`. You can specify as many drives of file=fat on UEFI I think. Not sure if that actually works though.

Should also specify `-nodefaults` to make sure QEMU doesnt enable a whole bunch of stuff that slows down boot.

For RNG, use `-device virtio-rng-pci`.

## Network

To use the network interface on QEMU:

```bash
-nic user,model=e1000,net=192.168.17.0/24,tftp=uefi-test-runner/tftp/,bootfile=fake-boot-file
```

We need a fake-boot-file.

## SMP

To use SMP with QEMU, do `-smp cpus=8`. Or how many CPUs you want to simulate. If using `-cpu host`. I think you can just do the amount of SMP cores on your system. On x86, can also specify `threads=16` to use hyperthreading/clustered multithreading.

## Debugging

I think its just `-s -S` to enable debugging. Then connect via `lldb`'s remote gdb function at localhost:1234.

On arcutils. just run `arc debug --arcboot` to connect to a running arcboot with default settings. Note you should run `arc run --arcboot --debug` to run arcboot in a separate shell first, and wait for it to initialise and wait for connection.
