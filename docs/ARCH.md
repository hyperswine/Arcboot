# Architecture

## RISC-V

RISC-V systems should implement SBI. Arcboot uses `rust-sbi` and loads an SEE before loading a kernel ELF image.

- Neutron-riscv compiles an `ecall` subsystem for simpler hw management
- I think SBI handles a lot of the multicore scheduling though IDK

## ARM

ARM systems have better capabilities than RISCV ones on the market. Usually more library and driver support as well.

- There are things like vector tables and stuff for bare metal cortex M. I assume a generic aarch64 setup with rings 0-3. Usually a multicore Cortex-A system
- And any usual data structures and boot methods
