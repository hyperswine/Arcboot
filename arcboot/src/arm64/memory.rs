/// Setup virtual memory with MMIO/DMA. Uses DEVICE TREE or UEFI
/// Make sure to make arcboot use TTBR0 addressing (all 0s), while the Kernel uses TTBR1 addressing
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

// ------------------
// DESCRIPTORS
// ------------------

// NOTE: set each field to a repr(C) enum

// 4K => starts with 1's
bitfield! {
    pub struct TableDescriptor4K(u64);
    impl Debug;
    u64;
    pub ns_table, set_ns_table: 63;
    pub ap_table, set_ap_table: 62, 61;
    pub xn_table, set_xn_table: 60;
    pub pxn_table, set_pxn_table: 59;
    pub next_lvl_table_addr, set_next_lvl_table_addr: 47, 12;
    pub id_one, set_one: 1;
    pub valid, set_valid :0;
}

// 4K => starts with 0's
bitfield! {
    pub struct BlockDescriptor4K(u64);
    impl Debug;
    u64;
    pub custom, set_custom: 58, 55;
    pub uxn, set_uxn: 54;
    pub pxn, set_pxn: 53;
    pub contig, set_contig: 52;
    pub output_addr, set_output_addr: 47, 12;
    pub zeroes, set_zeroes: 1, 0;
    pub non_gathering, set_non_gathering: 11;
    pub access_flag, set_access_flag: 10;
    pub shared, set_shared: 9, 8;
    pub access_permissions, set_access_permissions: 7, 6;
    pub non_secure, set_non_secure: 5;
    pub index_into_mair, set_index_into_mair: 4, 2;
    pub id_zero, set_zero: 1;
    pub valid, set_valid: 0;
}

pub fn default_unmapped_table_descriptor() -> TableDescriptor4K {
    let res = TableDescriptor4K(0);

    res
}

pub fn default_unmapped_block_descriptor() -> BlockDescriptor4K {
    let res = BlockDescriptor4K(0);

    res
}

// 4K VAddr (should be 1s for TTBR1)
bitfield! {
    pub struct VAddr48_4K(u64);
    impl Debug;
    u64;
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

pub fn setup() {
    unsafe {
        let res = enable_mmu_and_caching(0x4000_1000);
        match res {
            Ok(r) => info!("MMU enabled successfully!"),
            Err(err) => panic!("MMU could not be started! Error = {err}"),
        }
    }
}

/// Get the value of TTBR1
#[inline(always)]
pub fn ttbr1() -> u64 {
    TTBR1_EL1.get()
}

pub const KERNEL_BOOT_STACK_PAGES: usize = 16;
pub const KERNEL_BOOT_STACK_START: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const KERNEL_BOOT_HEAP_START: u64 = 0x0000_FFFF_FFFF_FFFF;
pub const KERNEL_BOOT_HEAP_PAGES: usize = 16;

pub fn get_table_desc(base_addr: u64, index: u64) -> u64 {
    unsafe {
        // maybe encapsulate this logic into walk_table(base_addr, index) -> u64
        let level_descriptor_addr = (base_addr + index * 8) as *const u64;
        let level_descriptor = core::ptr::read_volatile(level_descriptor_addr);

        level_descriptor
    }
}

/// For L0-L2 Page Tables. Also returns the actual frame number of the table
pub fn get_next_lvl_table(base_pt_addr: u64, l_index: u64) -> Option<u64> {
    let level_descriptor = TableDescriptor4K(get_table_desc(base_pt_addr, l_index));

    // if valid, return Some & decode descriptor's addr that it points to
    if level_descriptor.valid() {
        Some(level_descriptor.next_lvl_table_addr())
    } else {
        None
    }
}

/// For L3 Page Table
pub fn get_output_addr_block(base_pt_addr: u64, l_index: u64) -> Option<u64> {
    let level_descriptor = BlockDescriptor4K(get_table_desc(base_pt_addr, l_index));

    if level_descriptor.valid() {
        Some(level_descriptor.output_addr())
    } else {
        None
    }
}

/// If there is already a table there, it will overwrite it. So make sure you dont do something dumb
/// Returns the PAddr of the new table start (NOT NEEDED, but oh well)
pub fn make_page_table(free_frame: u64) -> u64 {
    // create an l1 table and update that descriptor
    let frame_addr = (free_frame * PAGE_SIZE as u64) as *mut u64;
    pub const N_PAGES_IN_TABLE: u64 = 512;

    // start writing to it in unsafe
    unsafe {
        // for each page (0..2^9), set all entries to default table descriptors that are invalid
        for page_number in 0..2_u64.pow(9) {
            let unmapped_desc = default_unmapped_table_descriptor();
            core::ptr::write_volatile(frame_addr, unmapped_desc.0);
        }
    }

    free_frame
}

/// Makes new table and points prev descriptor at it
pub fn setup_table(table_frame: u64, prev_base_pt_addr: u64, prev_desc_index: u64) {
    // create an l1 table and update that descriptor
    // POINT THE ORIGINAL L0 ENTRY TO NEW as well. Pass L0 entry u64 addr to it?
    // Point and set to valid basically, for entry curr-1
    make_page_table(table_frame);
    let mut l0_desc = TableDescriptor4K(get_table_desc(prev_base_pt_addr, prev_desc_index));
    // set valid and next_level_table_flags
    l0_desc.set_valid(true);
    l0_desc.set_next_lvl_table_addr(table_frame);

    // write volatile to point at new table addr
    unsafe {
        core::ptr::write_volatile(table_frame as *mut u64, l0_desc.0);
    }
}

/// Turn an address base to a page numbe
pub fn base_addr_to_page_number(addr: u64) -> u64 {
    addr / PAGE_SIZE as u64
}

/// Given a virtual address range, find free frames to map them to (4K aligned). Region_size: number of pages
pub fn map_region_ttbr1<const N: usize>(
    region_start: u64,
    n_pages: u64,
    free_frames: &mut FreePages<N>,
) {
    // we already calc number of pages (ceiling) required to map the vaddr range

    for page in 0..n_pages {
        // attempt to map a frame (should prob return Option?)
        let frame_number = free_frames.pop();

        // this is the vaddr. NOTE: should have 16 1's from MSB
        let mut vaddr_start = region_start + page * PAGE_SIZE as u64;
        // ensure addr will be a TTBR1 addr
        vaddr_start = vaddr_start | 0xFFFF_0000_0000_0000;

        let actual_vaddr = VAddr48_4K(vaddr_start);

        // walk the table to map each index
        let base_pt_addr = ttbr1();

        // l0 index (47..39)

        // IF ALREADY MAPPED, continue; or maybe panic!? or maybe overwrite?
        // or maybe send an enum Continue, Panic, Overwrite
        // FOR NOW: if one of the levels returns Option::None, make the descriptor using another free frame

        let l0_index = actual_vaddr.l0_index();
        // What if base table wasnt mapped? Or encountered? western
        let l1_table_addr = get_next_lvl_table(base_pt_addr, l0_index);

        // damn, ok maybe get the index as well from get_next_lvl_table
        // frame number = table base_addr / 4K

        let (l2_table_addr, l1_base_addr) = match l1_table_addr {
            Some(l) => (get_next_lvl_table(base_pt_addr, l), base_addr_to_page_number(base_pt_addr)),
            None => {
                // create an l1 table and update that descriptor
                let table_frame = free_frames.pop();
                setup_table(table_frame, base_pt_addr, l0_index);

                // return the next one, which is unmapped
                (None, table_frame)
            }
        };

        let l3_table_addr = match l2_table_addr {
            Some(l) => (get_next_lvl_table(l1_base_addr, l), base_addr_to_page_number(l1_base_addr)),
            None => {
                let table_frame = free_frames.pop();
                setup_table(table_frame, base_pt_addr, l0_index);
                (None, table_frame)
            }
        };

        // NOW, get the output addr (since L3 entries are block descriptors)
        // based on if l3 is even mapped. I think UEFI should zero out..? Or at least ensure the free pages have valid = false?
        // let output_addr = get_output_addr_block(base_pt_addr, l3_table_addr);
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
        KERNEL_BOOT_STACK_START - 16 * PAGE_SIZE as u64,
        KERNEL_BOOT_STACK_PAGES as u64,
        &mut free_frames,
    );
    map_region_ttbr1(
        KERNEL_BOOT_HEAP_START,
        KERNEL_BOOT_HEAP_PAGES as u64,
        &mut free_frames,
    );
}

// ------------------
// INTERRUPT HANDLERS
// ------------------

/// In order to bookkeep page tables, require a pointer and a page fault handler. Or non existent page handler
fn on_page_fault() {
    // if page does not exist in the page table, create a mapping using the next free frame, or page is swapped to disk
    let next_free_frame: usize;
}

// TTBR0 (Arcboot Addressing)
