# TODO

- set the boot order of the NVRAM. Can technically use VARS.fd but efivars is fine
- set the stack pointer up at 0x6M. Then go into EL2

## C & ASM Stubs

Its possible to precompile C sources. Then link those object together as well as a header API. ASM can be included into rust directly. And global asm is apparently stable now. So I guess Im just doing it wrong.

### Combining UEFI

uefi-rs seems to implement a lot of the specs already.
[Here](https://retrage.github.io/edk2-nightly/) is a convenient place to get the nightly EDK2 images.

I think its basically `-disk if=bios.bin` for OVMF_EFI.fd.

On Mac, you just have to do:

```bash
mkdir -p diskImage/EFI/BOOT
cp bootx64.efi diskImage/EFI/BOOT/BOOTX64.EFI
hdiutil create -fs fat32 -ov -size 48m -volname NEWOS -format UDTO -srcfolder diskImage uefi.cdr
```

## Export API

Arcboot exports an API with 3 modules and a few functions to help setup an arcboot compliant kernel and to then request services from the arcboot environment. The arcboot env includes the SEE on riscv and seems like limine.
