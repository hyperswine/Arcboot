// Handy way to compile neutron kernel code and arcboot bl code together and link into a bootable image

// SECTION A: Take a small BIOS MBR stub (entry.S) and assemble it. Then compile the kernel and link with it into a bootable .ELF. This can be run directly with qemu -bios bootimg.elf

// SECTION B: Basically does what `arcboot flash --neutron <disk>` does. Perform a full .ISO image creation and flash onto a virtual qhd file. Doesnt support extras yet like quantii
