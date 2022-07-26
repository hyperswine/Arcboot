//-------------------
// IMPORT
//-------------------

use arcboot_api::MemoryMap;
use bitfield::bitfield;
use core::intrinsics::unlikely;
use cortex_a::{asm::barrier, registers::*};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

// -------------
// DEFINITIONS
// -------------

/// Default = 48 bit VA with 4 level page tables
const PAGE_SIZE: usize = 4096;
const PAGE_MASK: usize = PAGE_SIZE - 1;

type PageTableIndex = [bool; 9];
type PageTableOffset = [bool; 12];

// -------------
// HANDLERS
// -------------

/// In order to bookkeep page tables, require a pointer and a page fault handler. Or non existent page handler
fn on_page_fault() {
    // if page does not exist in the page table, create a mapping using the next free frame, or page is swapped to disk
    let next_free_frame: usize;
}

// ------------------
// DESCRIPTORS
// ------------------

// NOTE: set each field to a repr(C) enum

// 4K => starts with 1's
bitfield! {
    pub struct TableDescriptor4K(u64);
    impl Debug;
    u32;
    pub ns_table, set_ns_table: 63;
    pub ap_table, set_ap_table: 62, 61;
    pub xn_table, set_xn_table: 60;
    pub pxn_table, set_pxn_table: 59;
    pub next_lvl_table_addr, set_next_lvl_table_addr: 47, 12;
    pub ones, set_ones: 1, 0;
}

// 4K => starts with 0's
bitfield! {
    pub struct BlockDescriptor4K(u64);
    impl Debug;
    u32;
    pub custom, set_custom: 58, 55;
    pub uxn, set_uxn: 54;
    pub pxn, set_pxn: 53;
    pub contig, set_config: 52;
    pub output_addr, set_output_addr: 47, 12;
    pub zeroes, set_zeroes: 1, 0;
    pub non_gathering, set_non_gathering: 11;
    pub access_flag, set_access_flag: 10;
    pub shared, set_shared: 9, 8;
    pub access_permissions, set_access_permissions: 7, 6;
    pub non_secure, set_non_secure: 5;
    pub index_into_mair, set_index_into_mair: 4, 2;
}

// 4K VAddr (should be 1s for TTBR1)
bitfield! {
    pub struct VAddr48_4K(u64);
    impl Debug;
    u16;
    pub sign, set_sign: 63, 48;
    pub l0_index, set_l0_index: 47, 39;
    pub l1_index, set_l1_index: 38, 30;
    pub l2_index, set_l2_index: 29, 21;
    pub l3_index, set_l3_index: 21, 12;
    pub offset, set_offset: 11, 0;
}

//-------------------
// PAGING & MMU IMPL
//-------------------

pub type PageNumber = u64;

/// Statically sized Stack of Free Pages
pub struct FreePages<const N: usize> {
    curr_head: usize,
    pages: [PageNumber; N],
}

impl<const N: usize> FreePages<N> {
    pub fn new(curr_head: usize, pages: [PageNumber; N]) -> Self {
        Self { curr_head, pages }
    }

    pub fn push(&mut self, page_number: PageNumber) {
        self.curr_head += 1;
        self.pages[self.curr_head] = page_number;
    }

    pub fn pop(&mut self) -> PageNumber {
        let res = self.pages[self.curr_head];
        self.curr_head -= 1;

        res
    }

    pub fn size(&self) -> usize {
        self.pages.len()
    }
}

/// Setup MAIR_EL1 for memory attributes like writeback/writethrough and nGnRE
fn set_up_mair() {
    MAIR_EL1.write(
        MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
            + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
            + MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck,
    );
}

/// Configure stage 1 of EL1 translation (TTBR1) for KERNEL
fn configure_translation_control() {
    let t1sz = 16;

    info!("Attempting to write to TCR_EL1...");

    TCR_EL1.write(
        // TCR_EL1::TBI1::Used
        TCR_EL1::IPS::Bits_48
            + TCR_EL1::TG0::KiB_4
            + TCR_EL1::TG1::KiB_4
            + TCR_EL1::SH1::Inner
            + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::EPD1::EnableTTBR1Walks
            // enable instead of disable, addresses depends on 1s or 0s
            + TCR_EL1::EPD0::EnableTTBR0Walks
            + TCR_EL1::A1::TTBR1
            + TCR_EL1::T1SZ.val(t1sz),
    );

    info!("Written to TCR_EL1");
}

#[inline(always)]
pub fn is_enabled_mmu() -> bool {
    SCTLR_EL1.matches_all(SCTLR_EL1::M::Enable)
}

/// Call this to enable MMU using a page table base addr
pub unsafe fn enable_mmu_and_caching(phys_tables_base_addr: u64) -> Result<(), &'static str> {
    // FOR UEFI, would already be enabled and ID mapped, but we gotta reset some regs like TCR_EL1

    info!("Attempting to enable MMU");

    // Fail early if translation granule is not supported
    if unlikely(!ID_AA64MMFR0_EL1.matches_all(ID_AA64MMFR0_EL1::TGran4::Supported)) {
        return Err("4KiB Translation granule not supported in HW");
    }

    info!("Setting MAIR...");

    // Prepare the memory attribute indirection register
    set_up_mair();

    info!("Setting TTBR1...");

    // Set TTBR1
    TTBR1_EL1.set_baddr(phys_tables_base_addr);

    info!("Setting TCR_EL1...");

    // Configure the EL1 translation
    configure_translation_control();

    info!("Setting SCTLR_EL1...");

    // Switch MMU on. Ensure the MMU init instruction is executed after everything else
    barrier::isb(barrier::SY);

    // Enable the MMU + data + instruction caching
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

    barrier::isb(barrier::SY);

    info!("Done! MMU setup complete");

    Ok(())
}

// Where to setup page tables? Maybe at 0x4000_1000 DRAM? Really anywhere that isnt used rn, esp by the bootloader. If load bootloader at 0x80_000 DRAM and heap 0x4000_0000-0x4100_0000, start page tables TTBR1 at 0x4100_0000
// Kernel should setup TTBR0 tables if possible. Though prob already setup by uboot. If ID mapped TTBR0, just leave it (since arcboot uses TTBR0 for its own code)
// * Make sure to make arcboot use TTBR0 addressing (all 0s)

pub fn setup() {
    unsafe {
        let res = enable_mmu_and_caching(0x4000_1000);
        match res {
            Ok(r) => info!("MMU enabled successfully!"),
            Err(err) => panic!("MMU could not be started! Error = {err}"),
        }
    }
}

pub fn goto_table_descriptor(addr: u64) -> u64 {
    // dont get MMU to walk the table, manually do it
    let x: u64 = 1;
    // cast x as STAGE 1 descriptor

    0
}

pub const KERNEL_BOOT_STACK_PAGES: usize = 16;
pub const KERNEL_BOOT_STACK_START: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const KERNEL_BOOT_HEAP_START: u64 = 0x0000_FFFF_FFFF_FFFF;
pub const KERNEL_BOOT_HEAP_PAGES: usize = 16;

// MMIO and other memory regions. USE DEVICE TREE or UEFI

// ? Get a few free frames that you know ought to be free
// NOT POSSIBLE AS A STATIC METHOD, only for neutron when you link
// pub fn boot_free_frames() -> &'static [u64] {
//     static res: [u64; 1] = [0; 1];

//     &res
// }

pub fn set_frame_used(frame: u64) {}

/// Given a virtual address range, find free frames to map them to (4K aligned). Region_size: number of pages
pub fn map_region_ttbr1<const N: usize>(
    region_start: u64,
    region_size: usize,
    free_frames: &mut FreePages<N>,
) {
    for r in 0..region_size {
        // ttbr1.walk_addr_entry
    }
}

/// Maps the key kernel regions to TTBR1
pub fn setup_kernel_tables(memory_map: MemoryMap) {
    // get all the Standard memory regions and map them to the same kernel virt addr range. after 2GB vaddr? Nope, 2GB down

    // 0x0 and 0xFFFF... should always be reserved vaddrs i think. For specific blocks, not general areas or stack or something

    let n_pages = 100;
    let mut free_frames = FreePages::new(n_pages - 1, [100]);

    // map 16 pages from high
    map_region_ttbr1(
        KERNEL_BOOT_STACK_START - 16 * 4096,
        KERNEL_BOOT_STACK_PAGES,
        &mut free_frames,
    );
    map_region_ttbr1(
        KERNEL_BOOT_HEAP_START,
        KERNEL_BOOT_HEAP_PAGES,
        &mut free_frames,
    );
}
