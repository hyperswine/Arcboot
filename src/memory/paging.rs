pub struct Page;

// setup paging at a good place away from other frames
// dont assume DRAM size, if no available frames, then do an out of memory fault

// framebuffer => 66.3MiB for 1080p. 4 8-bit values
// UEFI stuff => 256MB prob 0x1000-0x271000
// exception table => 4K, 0x0

// PAGE TABLES START IN PHYSICAL MEMORY DEPENDS ON THE MEMORY MAP
// and on the system board

// Arcboot's page table should be shared with the kernel
// or pretty much given ownership with normal boot
// with hypervisor boot, then we have stage 2 translation

// always grows up to 0xFF.. Each process has a 64K space from this addr
// page tables also should be frame aligned

// DECIDE PAGE_TABLES_START from the memory view
// if one of the 10 candidates work, for the entire range of DRAM (usually 8GB), from that growing up (easier to index. But still little endian), then select that as TTBR0

const PAGE_TABLES_START: u64 = 0x3B9ACA00;
