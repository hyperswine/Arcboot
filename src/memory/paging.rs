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
