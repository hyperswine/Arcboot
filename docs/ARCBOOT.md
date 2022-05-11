# Arcboot Protocol

The arcboot protocol is similar to limine and stivale. Except it is optimised for loading arcboot kernels like neutron. It also implements quantii boot protocol and multiboot.

## Overview

Arcboot allows arcboot kernels (just a bare ELF64 image) to be loaded and executed quickly and simply. Its strengths arent in the customisation available, rather the wrapping of layers so that the kernel does not have to do certain things and have access to pretty high level functions off the bat. Functions like serial console output, framebuffer terminal emulation, drivers for NeFS, UART, SPI, I2C.

For complete systems like Pi4, Arcboot does not have a "vision" of what it is. It can load DTBs and setup MMIO with paging, but it will not drive a complete system as well as Neutron. That being said, for arm/riscv, DTBs are found on the `/boot/dtb` dir within the primary BOOT/EFI partition.

NOTE: if EFI isnt needed, the arcboot partition will be simply flashed as a FAT32/QFS partition rather than an EFS partition.

## Graphics

- an ARM based GPU like Mali or PowerVR can be used with ARM/RISCV processors. Neutron loads driver modules for those components based on MMIO. Arcboot tells Neutron via ACPI the graphics units available
- Neutron should load its 3D graphics subsystem when a supported GPU is detected by ACPI
