---
layout: default
title: Removed
---

I had:

```bash
qemu-system-aarch64 -M virt -cpu cortex-a72 -drive if=pflash,format=raw,unit=0,file=./build/aarch64/OVMF_EFI.fd,readonly=on -drive if=pflash,format=raw,unit=1,file=./build/aarch64/VARS.fd -kernel format=raw,file=build/arcboot.efi -serial mon:stdio -nographic -nodefaults -device virtio-rng-pci -smp cpus=8 -net none
```

which didnt work because I didnt specify the options properly.

```rust
pub struct Page;

// setup paging at a good place away from other frames
// assume 8GB at least

// framebuffer => 66.3MiB for 1080p. 4 8-bit values
// UEFI stuff => 256MB prob 0x1000-0x271000
// exception table => 4K, 0x0

// always grows up to 0xFF.. Each process has a 64K space from this addr
// page tables also should be frame aligned
const PAGE_TABLES_START: u64 = 0x3B9ACA00;

// all it is is somehow remapping the pages
// since pages are id mapped and prob prevents out of bounds access by trapping into EL1 on page fault below 0x0 and RAM
// since nothing defined in EL1, will prob just hang or abort. Then QEMU might signal something

// im guessing the csr register for page tables (TTBR0) isnt set. So even though paging is on it doesnt actually do anything or use the TLB. It just takes that vaddr and casts that as a paddr

// we need to initialise csr TTBR0

// idk how to set ovmf variables. I think we can maybe try the command line or auto generate one ourselves (AML or something). You have to ensure that drive is set as the first option
// https://github.com/rhuefi/qemu-ovmf-secureboot/blob/master/README.md
```

Mostly notes to self or something.

```rust
    // region_length can just be size_of<T>
    // info!(
    //     "Virtual addr = {}. Region length = {}",
    //     physical_address - 0x4000_0000,
    //     size_of::<T>()
    // );
    // phys_addr - 0x4000_0000 doesnt crash sometimes, but doesnt find the tables either
    // i have no idea, I think + 0x4000_0000 isnt correct because thats phys addr for some reason
    // let va = NonNull::new((physical_address - 0x4000_0000) as *mut T).unwrap();
    // Cant use 0x0 cause thats NULL
    // let va = NonNull::new(0x0 as *mut T).unwrap();
    // info!(
    //     "Actual virtual addr = {}. Region length = {}",
    //     0,
    //     size_of::<T>()
    // );

    // Identity mapped pages => 1GB offset. So begins at 0x4000_0000 - 0xB847C000
    // This chunk is 2GB (0x7847C000) in size

    // maybe keep a Vec of regions here
    // and always make virt start = 0

    // Identity mapped virtual address space
    // When UEFI/ACPI doesnt need it anymore, it expects the virtual address to be usable
    // I think we can simulate an MMU for now
    // pub struct IdentityMapping {
    //     // originally, 48bit, so like
    //     free_virtual_pages: Vec<u64>,
    // }
```

I dunno. Damn it damn it.
