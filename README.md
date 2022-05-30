# ArcBoot v0

A uefi bootloader for riscv, arm and x86. Comes in the form of an executable and an API for arcboot kernels to use in order to become arcboot compliant and use arcboot services/data structures.

- Build arcutils first, then use that to build, run and test arcboot
- Arcboot kernels written in rust can use the arcboot API for the most part and not get into asm esp in the earlier parts. Arcboot API also exports proc macros (`export/`) that sets things up and helps register exceptions which the kernel can call and hook onto its own handlers

## Running

Be sure to `chmod +x run.sh`!

## Arcdriver

Arc drivers are low level code that can be transferred from the bootloader to the kernel. In the form of a dynamically linked library. The arcdrivers are placed in `/arc/driver/<driver>.so`. If neutron is built with ARC_DRIVER=true, then it will not link with drivers like nefs, graphics and wifi/eth. It will instead assume they exist. And the neutron driver module should be able to find them while iterating through a list of available drivers to load. If the arcdriver code (just a vdso) is linked properly
