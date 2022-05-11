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
