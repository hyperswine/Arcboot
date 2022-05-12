// All setup code in a single file

// -------------
// IMPORTS
// -------------

use core::arch::global_asm;

use cortex_a::{asm::barrier, registers::*};

use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};

// -------------
// ASM
// -------------

global_asm!(include_str!("setup.S"),
CONST_CURRENTEL_EL2 = const 0x8,
CONST_CORE_ID_MASK = const 0b11);

// -------------
// Start Rust
// -------------

#[no_mangle]
pub unsafe extern "C" fn _start_rust(phys_boot_core_stack_end_exclusive_addr: u64) -> ! {
    prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr);
    cortex_a::asm::eret()
}

unsafe fn kernel_init() -> ! {
    loop {}
}

// ---------------
// EXPORT API
// ---------------

/// Arcboot Kernels to go into EL1
#[inline(always)]
pub unsafe fn prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr: u64) {
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
    SP_EL1.set(phys_boot_core_stack_end_exclusive_addr);
}

// -------------
// Memory
// -------------

// Default = 48 bit VA with 3 level page tables
// No segmentation support

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
