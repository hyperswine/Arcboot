#![cfg_attr(not(test), no_std)]
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
// RE-EXPORT
// ---------------

pub use aarch64;
pub use cortex_a;
pub use acpi;
pub use tock_registers;

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
pub mod arm64;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

// --------------
// TEST
// --------------

#[test]
fn test_basics() {
    assert_eq!(1, 1);
}
