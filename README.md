# ArcBoot v0

A uefi bootloader for riscv, arm and x86. Comes in the form of an executable and an API for arcboot kernels to use in order to become arcboot compliant and use arcboot services/data structures .

- Build arcutils first, then use that to build, run and test arcboot
- Arcboot kernels written in rust can use the arcboot API for the most part and not get into asm esp in the earlier parts. Arcboot API also exports functions that sets up and register exceptions which the kernel can call and hook onto its own handlers
