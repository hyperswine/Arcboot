#![no_main]
#![no_std]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]

#[cfg(feature = "uefi_support")]
pub mod uefi;

// ---------------
//  ARCHITECTURES
// ---------------

// OTHERWISE use uefi::alloc instead
#[cfg(not(feature = "uefi_support"))]
extern crate alloc;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
