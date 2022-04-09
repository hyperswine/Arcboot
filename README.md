# ArcBoot v0

A uefi bootloader for riscv, arm and x86. Comes in the form of a single executable.

- the source code contains a single executable target and no libraries. By default, builds for aarch64
- build arcutils first, then use that to build, run and test arcboot

## Arcutils Features

- create disk images of type GPT, partitioned with FAT and BTRFS/DOS/EXT4
-[flash](https://qemu.readthedocs.io/en/latest/tools/qemu-img.html) my arcboot bootloader onto the disk, in UEFI mode (/EFI/boot/...)
- flash any arm/riscv kernel (elf) images onto a BTRFS/DOS/EXT4 partition, including any extra stuff like the root partition, home partition, user partition, etc

NOTE:

- only works for `sh` shells. I dont really wanna add support for other shells rn
