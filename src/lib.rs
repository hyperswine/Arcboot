#![no_std]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Bootloader should have no heap though they can. Mainly for tests and UEFI
#![feature(alloc_error_handler)]
// REQUIRED TO DEFINE CONSTANTS IN ASM
#![feature(asm_const)]

// ---------------
// API EXPORT
// ---------------

#[cfg(feature = "uefi_support")]
pub mod uefi;
pub mod memory;

// OTHERWISE use uefi::alloc instead
#[cfg(not(feature = "uefi_support"))]
extern crate alloc;

// ---------------
// ARCHITECTURES
// ---------------

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

// ----------------
// TESTING
// ----------------

pub mod test;

// NOTE: use the same panic handler as the standard build for testing
// Also, an entry for unit/integration testing is needed in lib

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
