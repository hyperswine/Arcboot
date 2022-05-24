// All setup code in a single file

// -------------
// IMPORTS
// -------------

use core::arch::{asm, global_asm};
use cortex_a::registers::*;
use tock_registers::interfaces::Writeable;

// -------------
// COMMON
// -------------

// For PI 4, the boot core is core 3 (4th core)
// It might not the be same for other boards
const CONST_CURRENTEL_EL2L: u64 = 0x8;
const CONST_CORE_ID_MASK: u64 = 0b11;

// TODO
#[inline(always)]
pub unsafe fn transition_el3_to_el2() {
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    CNTVOFF_EL2.set(0);
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // ? stack starts at 0x06M for uefi bootloader
    const STACK_START: u64 = 0x6000000;

    SP_EL1.set(STACK_START);
    cortex_a::asm::eret();
}

// -------------
// Start Rust (NO UEFI)
// -------------

#[cfg(not(feature = "uefi_support"))]
#[no_mangle]
pub unsafe extern "C" fn _start_rust() -> ! {
    asm!("b _main");
    loop {}
}

// ---------------
// EXPORT API
// ---------------

/// Arcboot Kernels to go into EL1
/// Takes a non parametrised function to eret to
/// Must setup sp in your linker, prob to 0xfff000... vaddr
#[inline(always)]
pub unsafe fn prepare_el2_to_el1_transition(kernel_init: fn(), sp: u64) {
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    CNTVOFF_EL2.set(0);
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    ELR_EL2.set(kernel_init as *const () as u64);
    SP_EL1.set(sp);
    cortex_a::asm::eret();
}

// -------------
// Memory
// -------------

// Default = 48 bit VA with 3 level page tables
// No segmentation support
// No heap export, assumes kernel implements its own heap allocation

const PAGE_SIZE: usize = 4096;
const PAGE_MASK: usize = PAGE_SIZE - 1;

type PageTableIndex = [bool; 10];
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
