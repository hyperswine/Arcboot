## Arcboot Bootloader

The main bootloader images for riscv, arm and x86. I want to optimise for riscv though.

- all asm and linker scripts for arcboot itself and neutron is contained here. Mostly for building a UEFI app that sets the right registers after BIOS, disables interrupts, sets up a stack pointer, heap pointer, etc. for temp bootloader use on physical memory. Then sets up paging and SBI, sets the registers again and passes higher abstracted structures to the kernel

```bash
cargo bx86 -> builds for x86_64
cargo brv -> builds for riscv64

// actually dont do this
cargo barm2 -> builds for armv8.2a (pi 4b)
cargo barm4 -> builds for armv8.4a (samsung galaxy s20)

// just do this
cargo barm
```

## Arcdriver

Like uboot and oreboot `drivers/`. Device specific, loaded on the fly based on what generic device, e.g. Pi4B, SiFive board, etc. the bootloader detects. Also allows disk drive management and filesystem views. Like GRUB.
