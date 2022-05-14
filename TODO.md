# TODO

- something isnt working, idk what. Im guessing its something to do with the structure cause the non setup.S also seems to fail
- maybe ENTRY(_start) is wrong somehow or IDK. Its just not creating the ELF image properly from the physical addresses and assembly Im giving it
- new feature: allow an arcboot kernel to be completely writable in rust using arcboot export functions
- since uefi-rs seems to have done quite a bit of what we want for us, maybe we just use that by default

## C & ASM Stubs

Its possible to precompile C sources. Then link those object together as well as a header API. ASM can be included into rust directly. And global asm is apparently stable now. So I guess Im just doing it wrong.

## Export API

Arcboot exports an API with 3 modules and a few functions to help setup an arcboot compliant kernel and to then request services from the arcboot environment. The arcboot env includes the SEE on riscv and seems like limine.
