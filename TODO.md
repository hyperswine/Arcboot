# TODO

- set the stack up at 0x6M. Then go into EL2

## C & ASM Stubs

Its possible to precompile C sources. Then link those object together as well as a header API. ASM can be included into rust directly. And global asm is apparently stable now. So I guess Im just doing it wrong.

### Combining UEFI

uefi-rs seems to implement a lot of the specs already.
[Here](https://retrage.github.io/edk2-nightly/) is a convenient place to get the nightly EDK2 images.

## Export API

Arcboot exports an API with 3 modules and a few functions to help setup an arcboot compliant kernel and to then request services from the arcboot environment. The arcboot env includes the SEE on riscv and seems like limine.
