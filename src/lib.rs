#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]
// Bootloader should have no heap though they can. Mainly for tests and UEFI
#![feature(alloc_error_handler)]
// REQUIRED TO DEFINE CONSTANTS IN ASM
#![feature(asm_const)]

// ---------------
// CRATE WIDE USE
// ---------------

#[macro_use]
extern crate log;
extern crate alloc;

pub mod efi;
pub mod qemu;
pub mod logger;

// ---------------
// API EXPORT
// ---------------

pub mod memory;

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

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        self();
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    for test in tests {
        test.run();
    }
    loop {}
}

// PANIC HANDLER FOR TEST, prob not needed
#[cfg_attr(test, panic_handler)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
