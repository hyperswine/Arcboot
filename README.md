# ArcBoot v0

A set of utilities including a bootloader for riscv and arm. Comes in the form of a single executable.

- the source code contains multiple executable targets. One for the main `arcutils` and another for the `arcboot` bootloader itself
- build arcutils first, then use that to build, run and test arcboot

## Features

- create disk images of type GPT, partitioned with FAT and BTRFS/DOS/EXT4
-[flash](https://qemu.readthedocs.io/en/latest/tools/qemu-img.html) my arcboot bootloader onto the disk, in UEFI mode (/EFI/boot/...)
- flash any arm/riscv kernel (elf) images onto a BTRFS/DOS/EXT4 partition, including any extra stuff like the root partition, home partition, user partition, etc

NOTE:

- only works for `sh` shells. I dont really wanna add support for other shells rn
