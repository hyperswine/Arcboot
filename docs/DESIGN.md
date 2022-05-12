# Design

Like Oreboot and GRUB, Arcboot is a multiboot UEFI bootloader. For stages 1 and 2 after platform firmware starts and loads the image (BIN or ELF) into RAM.

- should be flashed onto a drive on its own FAT32 partition. The `/` partition should be saved for the kernel. Instead the FAT partition should have a `/boot` dir with extra config files for runtime boot (stages 1 and 2)
- the `/boot` fat32 partition can then be mounted on `/boot` on the main VFS filesystem root
- being very close to bare metal, arches are found in the src/ root and are quite independent of each other. The general interface in drivers/, lib, etc. use those arch code on conditional compilation
- all asm and linker scripts for arcboot itself and neutron is contained here. Mostly for building a UEFI app that sets the right registers after BIOS, disables interrupts, sets up a stack pointer, heap pointer, etc. for temp bootloader use on physical memory. Then sets up paging and SBI, sets the registers again and passes higher abstracted structures to the kernel

## Stage 1

We assume a conventional EFI boot flow for an armv8a system.

First the firmware boots up and eventually gets to loading a suitable bootloader. It finds an EFI system partition (FAT32) and goes into /EFI/boot.efi to load the PE image (most code segment) into memory.
