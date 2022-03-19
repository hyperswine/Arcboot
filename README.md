## ArcBoot
Tools to build a crate and link with a bootloader (asm). And to create a bootable image for `cargo test`.
Supports RISC-V and AARCH64.

Best to be used as a build dependency and configured in `Cargo.toml` under `deps.arcboot`.

Specifically optimised to building Spectro and Pi4B. Lack of options and stuff to keep things self contained, simple and easy to modify.

- only works for shells with `sh` linked

So if you run `cargo install arcboot` where arcboot is actually the HTTPS link to the project's github repo. You can then run `arboot` with `cargo` like `cargo arcboot`.

## Arcdriver

Like uboot and oreboot `drivers/`. Device specific, loaded on the fly based on what generic device, e.g. Pi4B, SiFive board, etc. the bootloader detects. Also allows disk drive management and filesystem views. Like GRUB.

## UEFI
- Boots on UEFI. Needs to be flashed onto a GPT disk using `arcboot flash <disk>`. If disk isnt GPT or is already formatted in another scheme, its headers will be cleared to a default GPT with the first partition being FAT32 storing the bootloader img.
- Kernel images that you want to boot from can be flashed onto any GPT disk. Needs to have multiboot compliant headers and entry to be discoverable by arcboot.

## Features
- Simple GUI using arcgraphics drivers that quantos shares. Optimised for QuantOS/RiscV.
- Recovery mode for broken drives. Simple shell for recovery commands.
- Setup paging for kernel (4K) and loads any drivers configured to be loaded at boot. Including arcgraphics, arcmouse, arckey. Allows turning on hypervisor mode Arcvisor if you want to run any bootloader time vm scripts, e.g. startup a vm server process.
- Graphical troubleshoot options to reset drives and apps. Can reset a Quantos install if there is a problem.

## Interaction with Spectro Bios
- A UEFI firmware for spectros r1. Has graphical elements to configure security settings, overclocking cpu/ram/gpu, undervolting cpu, disabling certain cards and components or features like optimus. Also things like enabling/disabling virtualisation and other riscv extensions for speed, safety, etc.
- Preliminary startup of devices, esp spectro hardware.

Arcboot is optimised for spectro bios. It also works for pi4 bios though some extra features may not be supported, and may be buggy.
