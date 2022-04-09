# ArcBoot v0

A uefi bootloader for riscv, arm and x86. Comes in the form of a single executable.

- the source code contains a single executable target and no libraries. By default, builds for aarch64
- build arcutils first, then use that to build, run and test arcboot

## TODO

- get uart working properly on qemu
- get linker scripts to work properly and setup certain sections for bootloader stage 1 [ ]

## Design

Like Oreboot and GRUB, Arcboot is a multiboot UEFI bootloader. For stages 1 and 2 after platform firmware starts and loads the image (BIN or ELF) into RAM.

- should be flashed onto a drive on its own FAT32 partition. The `/` partition should be saved for the kernel. Instead the FAT partition should have a `/boot` dir with extra config files for runtime boot (stages 1 and 2)
- the `/boot` fat32 partition can then be mounted on `/boot` on the main VFS filesystem root

### ELF Runner

Kernel images can be built as an ELF executable or just a binary blob in the case of ARM.

- arcboot sets up a stack, heap, bss, etc. for the kernel
- it then sets up 48 bit paging and goes into S-Mode. Then it can pass control to the loaded kernel code

No support for 39/52/64 bit paging yet.

### RISC-V

RISC-V systems should implement SBI. Arcboot uses `rust-sbi` and loads an SEE before loading a kernel ELF image.

- Neutron-riscv compiles an `ecall` subsystem for simpler hw management
- I think SBI handles a lot of the multicore scheduling though IDK

### ARM

ARM systems have better capabilities than RISCV ones on the market. Usually more library and driver support as well.

- There are things like vector tables and stuff for bare metal cortex M. I assume a generic aarch64 setup with rings 0-3. Usually a multicore Cortex-A system
- And any usual data structures and boot methods

### Graphics

- an ARM based GPU like Mali or PowerVR can be used with ARM/RISCV processors. Neutron loads driver modules for those components based on MMIO. Arcboot tells Neutron via ACPI the graphics units available
- Neutron should load its 3D graphics subsystem when a supported GPU is detected by ACPI

## QEMU

Arcboot should be flashed onto a vhd file. QEMU should be run with OVMF firmware.

To specify the vhd file `--disk=<main_disk.vhd>`. If everything goes well, QEMU should be able to load Arcboot into the host memory and hand off control to it in pseudo M mode.
