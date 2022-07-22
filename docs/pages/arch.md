---
layout: default
title: Architecture
---

## RISC-V

RISC-V systems should implement SBI. Arcboot uses `rust-sbi` and loads an SEE before loading a kernel ELF image.

- Neutron-riscv compiles an `ecall` subsystem for simpler hw management
- I think SBI handles a lot of the multicore scheduling though IDK

## ARM

ARM systems have better capabilities than RISCV ones on the market. Usually more library and driver support as well.

- There are things like vector tables and stuff for bare metal cortex M. I assume a generic aarch64 setup with rings 0-3. Usually a multicore Cortex-A system
- And any usual data structures and boot methods

### Memory Layout

The ARMv8.2A memory layout with 52-bit virtual addresses:

```rust
0000000000000000      000fffffffffffff           4PB          user
fff0000000000000      fff7ffffffffffff           2PB          kernel logical memory map
fff8000000000000      fffd9fffffffffff        1440TB          [gap]
fffda00000000000      ffff9fffffffffff         512TB          kasan shadow region
ffffa00000000000      ffffa00007ffffff         128MB          bpf jit region
ffffa00008000000      ffffa0000fffffff         128MB          modules
ffffa00010000000      fffff81ffffeffff         ~88TB          vmalloc
fffff81fffff0000      fffffc1ffe58ffff          ~3TB          [guard region]
fffffc1ffe590000      fffffc1ffe9fffff        4544KB          fixed mappings
fffffc1ffea00000      fffffc1ffebfffff           2MB          [guard region]
fffffc1ffec00000      fffffc1fffbfffff          16MB          PCI I/O space
fffffc1fffc00000      fffffc1fffdfffff           2MB          [guard region]
fffffc1fffe00000      ffffffffffdfffff        3968GB          vmemmap
ffffffffffe00000      ffffffffffffffff           2MB          [guard region]
```

48-bit virtual addressing:

```rust
+--------+--------+--------+--------+--------+--------+--------+--------+
  |63    56|55    48|47    40|39    32|31    24|23    16|15     8|7      0|
  +--------+--------+--------+--------+--------+--------+--------+--------+
   |                 |         |         |         |         |
   |                 |         |         |         |         v
   |                 |         |         |         |   [11:0]  in-page offset
   |                 |         |         |         +-> [20:12] L3 index
   |                 |         |         +-----------> [29:21] L2 index
   |                 |         +---------------------> [38:30] L1 index
   |                 +-------------------------------> [47:39] L0 index
   +-------------------------------------------------> [63] TTBR0/1
```

- Around 70MB when full

You can label with a '.' beginning to prevent the assembler from optimising them out.
