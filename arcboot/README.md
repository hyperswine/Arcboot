## Arcboot Bootloader

The main bootloader images for riscv, arm and x86. I want to optimise for riscv though.

- all asm and linker scripts for arcboot itself and neutron is contained here. Mostly for building a UEFI app that sets the right registers after BIOS, disables interrupts, sets up a stack pointer, heap pointer, etc. for temp bootloader use on physical memory. Then sets up paging and SBI, sets the registers again and passes higher abstracted structures to the kernel

## Arcdriver

Like uboot and oreboot `drivers/`. Device specific, loaded on the fly based on what generic device, e.g. Pi4B, SiFive board, etc. the bootloader detects. Also allows disk drive management and filesystem views. Like GRUB.
