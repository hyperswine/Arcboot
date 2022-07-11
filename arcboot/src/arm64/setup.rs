// All setup code in a single file

// -------------
// IMPORTS
// -------------

use core::arch::{asm, global_asm};
use cortex_a::registers::*;
use tock_registers::interfaces::{Readable, Writeable};

// -------------
// COMMON
// -------------

#[inline(always)]
pub unsafe fn transition_el3_to_el2(dram_end: u64) {
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

    // stack starts at the end of DRAM
    let STACK_START: u64 = dram_end;

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

/// Arcboot Kernels to go into EL1. Takes a non parametrised function to eret to
/// Must setup sp in your linker, prob to 0xffff_ffff... vaddr (end of TTBR1)
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
