#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, no_main)]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]
#![feature(alloc_error_handler)]
#![feature(asm_const)]
// For unlikely branch compiler hint
#![feature(core_intrinsics)]

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

// --------------
// TEST
// --------------

#[test]
fn test_basics() {
    assert_eq!(1, 1);
}
