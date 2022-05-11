# Arcboot Bootloader

The main bootloader images for riscv, arm and x86. I want to optimise for riscv though.

- all asm and linker scripts for arcboot itself and neutron is contained here. Mostly for building a UEFI app that sets the right registers after BIOS, disables interrupts, sets up a stack pointer, heap pointer, etc. for temp bootloader use on physical memory. Then sets up paging and SBI, sets the registers again and passes higher abstracted structures to the kernel

```bash
cargo bx86 -> builds for x86_64
cargo brv -> builds for riscv64
cargo barm -> builds for aarch64
```

## Requires UEFI Firmware

On VMs OVMF works for most arches including x86, arm & riscv.
On actual hardware, it may be possible to flash Das U Boot onto the board firmware.

## Arcdriver

Like uboot and oreboot `drivers/`. Device specific, loaded on the fly based on what generic device, e.g. Pi4B, SiFive board, etc. the bootloader detects. Also allows disk drive management and filesystem views. Like GRUB.

## Testing

For aarch64 and riscv we need to run on qemu using custom runners and xtask.
