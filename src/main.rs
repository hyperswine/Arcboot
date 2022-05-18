#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(arcboot::test::test_runner)]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]

// ----------------
// RENDEVOUS POINT
// ----------------

extern "C" fn _main() -> ! {
    loop {}
}

#[cfg(not(feature = "uefi_support"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// --------------
//  UEFI WRAPPERS
// --------------

extern crate alloc;

use alloc::string::String;
use uefi::prelude::*;
use uefi::proto::console::serial::Serial;
use uefi::table::boot::{OpenProtocolAttributes, OpenProtocolParams};
use uefi_services;

// I DONT THINK IT WORKS FOR RISCV. Needs a PR
#[entry]
fn efi_main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    Status::SUCCESS
}
