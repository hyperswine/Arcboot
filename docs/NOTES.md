# Technicalities

Technically, you dont need a bootloader if your kernel has an EFI stub associated with it. But arcboot is a complete UEFI bootloader so it can be configured a number of ways.

- usually, on a neutron/quantii install, you bundle the arcboot bl as the same .iso image. Since they the bl and kernel image themselves dont need to be changed often, they can live on a single FAT32 partition. Usually around 2-5gb
- the kernel is pretty much useless by itself. Neutron is packaged with a builtin grub-like shell like in FreeBSD. Everything can be rendered by the CPU using the cpu-framebuffer drriver. If a supported GPU is detected, its associated driver can be loaded on `init_drivers()`. Then we can draw the console using vulkan -> compile the console app for vulkan
- for windowing, its prob best to use a GPU. The cpu can technically do anything we want but its not great for data/graphics, rather branch prediction and instruction processing
- neutron itself does contain a window system based on wayland. It is prob not great for arcboot bl to hook onto this in a clean way. So arcboot bl only has access to pure vulkan and its own simple graphics framework for drawing simple things

**NOTE**: PCI devices like GPU should prob be plugged in before boot. I think hotplug technically supports it. And maybe a graphics driver could also support on the fly loading, but rn I just do it the normal way.
**NOTE**: Neutron-Arcwin is basically X11/Wayland by itself. Its better over a simple console, but you prob want to have an actual DE. Thats where Quantii comes in, with a prepackaged DE, set of apps and settings like GNU/Linux. Arcwin is pretty useful if you just wanna run stuff like `firefox` and a bunch of pseudo terminals open. But a DE provides full fledged graphics manipulation of system settings and no need for terminals at all, but they can still be useful for general programming and scripting

## UEFI

- Boots on UEFI. Needs to be flashed onto a GPT disk using `arcutils flash <disk>`. If disk isnt GPT or is already formatted in another scheme, its headers will be cleared to a default GPT with the first partition being FAT32 storing the bootloader img.
- Kernel images that you want to boot from can be flashed onto any GPT disk. Needs to have multiboot compliant headers and entry to be discoverable by arcboot.
- Create an EFI system partition of 50MiB and copy arcboot.img and neutron.elf into it. This is the default way to boot
- I will also support checking other partitions for bootable images for kernel booting or chain booting

## Arcboot v1+ (LETS GO)

### Bootloader Features v1+

- Simple GUI using arcgraphics drivers shared with neutron when installed with neutron as a primary OS in `PRIMARY` mode. Can also load any drivers configured to be loaded at boot. Including arcgraphics, arcmouse, arckey
- Graphical troubleshoot options to reset drives and apps. Can reset a neutron/quantii install if there is a problem whe installed in `PRIMARY` mode. Also a full recovery mode for broken quantii installs -> grub-like shell for recovery commands on NFS root and boot partitions
- Allows turning on hypervisor mode Arcvisor if you want to startup a vm server process or run any bootloader time vm scripts

## Interaction with Spectro Bios v1+

- A UEFI firmware for spectros r1. Has graphical elements to configure security settings, overclocking cpu/ram/gpu, undervolting cpu, disabling certain cards and components or features like optimus. Also things like enabling/disabling virtualisation and other riscv extensions for speed, safety, etc.
- Preliminary startup of devices, esp spectro hardware.

Arcboot is optimised for spectro bios. It also works for pi4 bios though some extra features may not be supported, and may be buggy.
