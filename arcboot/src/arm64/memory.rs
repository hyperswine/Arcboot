//-------------------
// IMPORT
//-------------------

use bitfield::bitfield;
use core::intrinsics::unlikely;
use cortex_a::{asm::barrier, registers::*};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields,
};

// -------------
// Memory
// -------------

// Default = 48 bit VA with 3 level page tables

const PAGE_SIZE: usize = 4096;
const PAGE_MASK: usize = PAGE_SIZE - 1;

type PageTableIndex = [bool; 9];
type PageTableOffset = [bool; 12];

#[repr(C)]
struct VirtualAddress48 {
    offset: PageTableOffset,
    l1_index: PageTableIndex,
    l2_index: PageTableIndex,
    l3_index: PageTableIndex,
    ttbr_number: u16,
}

// In order to bookkeep page tables, require a pointer and a page fault handler
// Or non existent page handler

fn on_page_fault() {
    // if page does not exist in the page table
    // create a mapping using the next free frame
    let next_free_frame: usize;

    // or page is swapped to disk
}

// ------------------
// DESCRIPTORS
// ------------------

// NOTE: set each field to a repr(C) enum

// 4K => starts with 1's
bitfield! {
    pub struct TableDescriptor4K(u64);
    impl Debug;
    u8;
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
    u8;
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
    pub ns, set_ns: 5;
    pub index_into_mair, set_index_into_mair: 4, 2;
}

// L0-L2 Table Descriptor, for Stage 1 EL1
register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        /// Physical address of the next descriptor [47:16]
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [],
        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],
        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

// L3 Page Descriptor
register_bitfields! {u64,
    STAGE1_PAGE_DESCRIPTOR [
        /// Unprivileged execute-never
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],
        /// Privileged execute-never
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],
        /// Physical address of the next table descriptor (lvl2) or the page descriptor (lvl3)
        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [],
        /// Access Flag
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],
        /// Shareable
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],
        /// Access Permissions
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],
        /// Memory attributes index into MAIR_EL1
        AttrIndx OFFSET(2) NUMBITS(3) [],
        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],
        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

//-------------------
// PAGING & MMU IMPL
//-------------------

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
// * make sure to make arcboot use TTBR0 addressing (all 0s)

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
// MMIO and other memory regions. USE DEVICE TREE! Or just arcservices/memory map. Gotta create that asap

// TODO: setup the paging structures by impl'ing arcboot_api's ArcMemory
// how does limine do it?

/// Get a few free frames that you know ought to be free
pub fn boot_free_frames() -> &'static [u64] {
    static res: [u64; 1] = [0; 1];

    &res
}

pub fn set_frame_used(frame: u64) {}

/// Given a virtual address range, find free frames to map them to (4K aligned). Region_size: number of pages
pub fn map_region_ttbr1(region_start: u64, region_size: usize) {
    let free_frames = boot_free_frames();

    for r in 0..region_size {
        // ttbr1.walk_addr_entry
    }
}

use arcboot_api::MemoryMap;

/// Maps the key kernel regions to TTBR1
pub fn setup_kernel_tables(memory_map: MemoryMap) {
    // map 16 pages from high
    map_region_ttbr1(KERNEL_BOOT_STACK_START - 16 * 4096, KERNEL_BOOT_STACK_PAGES);
    map_region_ttbr1(KERNEL_BOOT_HEAP_START, KERNEL_BOOT_HEAP_PAGES);
}
