# ArcBoot v0

A set of utilities including a bootloader for riscv and arm. Comes in the form of a single executable.

## Features

- can create disk images of type GPT, partitioned with FAT and BTRFS/DOS/EXT4
- can then [flash](https://qemu.readthedocs.io/en/latest/tools/qemu-img.html) my arcboot bootloader onto the disk, in UEFI mode (/EFI/boot/...)
- can also flash any arm/riscv kernel (elf) images onto a BTRFS/DOS/EXT4 partition, including any extra stuff like the root partition, home partition, user partition, etc

- only works for `sh` shells. I dont really wanna add support for other shells rn

So if you run `cargo install arcboot` where arcboot is actually the HTTPS link to the project's github repo. You can then run `arcboot` directly as any other executable.

## Arcdriver

Like uboot and oreboot `drivers/`. Device specific, loaded on the fly based on what generic device, e.g. Pi4B, SiFive board, etc. the bootloader detects. Also allows disk drive management and filesystem views. Like GRUB.

### Technicalities

Technically, you dont need a bootloader if your kernel has an EFI stub associated with it. But arcboot is a complete UEFI bootloader so it can be configured a number of ways.

- usually, on a neutron/quantii install, you bundle the arcboot bl as the same .iso image. Since they the bl and kernel image themselves dont need to be changed often, they can live on a single FAT32 partition. Usually around 2-5gb
- the kernel is pretty much useless by itself. Neutron is packaged with a builtin grub-like shell like in FreeBSD. Everything can be rendered by the CPU using the cpu-framebuffer drriver. If a supported GPU is detected, its associated driver can be loaded on `init_drivers()`. Then we can draw the console using vulkan -> compile the console app for vulkan
- for windowing, its prob best to use a GPU. The cpu can technically do anything we want but its not great for data/graphics, rather branch prediction and instruction processing
- neutron itself does contain a window system based on wayland. It is prob not great for arcboot bl to hook onto this in a clean way. So arcboot bl only has access to pure vulkan and its own simple graphics framework for drawing simple things

**NOTE**: PCI devices like GPU should prob be plugged in before boot. I think hotplug technically supports it. And maybe a graphics driver could also support on the fly loading, but rn I just do it the normal way.
**NOTE**: Neutron-Arcwin is basically X11/Wayland by itself. Its better over a simple console, but you prob want to have an actual DE. Thats where Quantii comes in, with a prepackaged DE, set of apps and settings like GNU/Linux. Arcwin is pretty useful if you just wanna run stuff like `firefox` and a bunch of pseudo terminals open. But a DE provides full fledged graphics manipulation of system settings and no need for terminals at all, but they can still be useful for general programming and scripting

## UEFI

- Boots on UEFI. Needs to be flashed onto a GPT disk using `arcboot flash <disk>`. If disk isnt GPT or is already formatted in another scheme, its headers will be cleared to a default GPT with the first partition being FAT32 storing the bootloader img.
- Kernel images that you want to boot from can be flashed onto any GPT disk. Needs to have multiboot compliant headers and entry to be discoverable by arcboot.

# Arcboot v1+ (LETS GO)

## Bootloader Features v1+

- Simple GUI using arcgraphics drivers shared with neutron when installed with neutron as a primary OS in `PRIMARY` mode. Can also load any drivers configured to be loaded at boot. Including arcgraphics, arcmouse, arckey
- Graphical troubleshoot options to reset drives and apps. Can reset a neutron/quantii install if there is a problem whe installed in `PRIMARY` mode. Also a full recovery mode for broken quantii installs -> grub-like shell for recovery commands on NFS root and boot partitions
- Allows turning on hypervisor mode Arcvisor if you want to startup a vm server process or run any bootloader time vm scripts

## Interaction with Spectro Bios v1+

- A UEFI firmware for spectros r1. Has graphical elements to configure security settings, overclocking cpu/ram/gpu, undervolting cpu, disabling certain cards and components or features like optimus. Also things like enabling/disabling virtualisation and other riscv extensions for speed, safety, etc.
- Preliminary startup of devices, esp spectro hardware.

Arcboot is optimised for spectro bios. It also works for pi4 bios though some extra features may not be supported, and may be buggy.
