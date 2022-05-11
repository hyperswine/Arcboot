# Design

Like Oreboot and GRUB, Arcboot is a multiboot UEFI bootloader. For stages 1 and 2 after platform firmware starts and loads the image (BIN or ELF) into RAM.

- should be flashed onto a drive on its own FAT32 partition. The `/` partition should be saved for the kernel. Instead the FAT partition should have a `/boot` dir with extra config files for runtime boot (stages 1 and 2)
- the `/boot` fat32 partition can then be mounted on `/boot` on the main VFS filesystem root
- being very close to bare metal, arches are found in the src/ root and are quite independent of each other. The general interface in drivers/, lib, etc. use those arch code on conditional compilation
