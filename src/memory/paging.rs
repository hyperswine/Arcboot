pub struct Page;

// setup paging at a good place away from other frames
// assume 8GB at least

// framebuffer => 66.3MiB for 1080p. 4 8-bit values
// UEFI stuff => 256MB prob 0x1000-0x271000
// exception table => 4K, 0x0

// always grows up to 0xFF.. Each process has a 64K space from this addr
// page tables also should be frame aligned
const PAGE_TABLES_START: u64 = 0x3B9ACA00;
