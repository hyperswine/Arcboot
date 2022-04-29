#![no_main]
#![no_std]
#![feature(abi_efiapi)]

// -------------
//  FEATURES
// -------------

// USE uefi::alloc instead
// extern crate alloc;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod drivers;

use uefi::prelude::*;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    Status::SUCCESS
}
