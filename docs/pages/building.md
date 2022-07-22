---
layout: default
title: Building
---

The main bootloader images for riscv, arm and x86. I want to optimise for riscv but gonna do arm for now.

- NOTE: rust does not yet have tier 3 support for riscv-uefi, but you can just add an PE header to a binary blob anyway. I.e. no symtab/etc. lookups and build the entire thing into a single text section

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
