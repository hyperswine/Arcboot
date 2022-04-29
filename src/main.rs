#![no_main]
#![no_std]
#![feature(abi_efiapi)]

// -------------
//  ENTRY
// -------------

use core::ptr::null_mut;
use uefi::prelude::*;
use uefi::logger::Logger;
use uefi::alloc::Allocator;

struct Hi {}

// I DONT THINK IT WORKS FOR RISCV
// CREATE PR FOR IT
#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // init allocator

    // trace_macros!(true);
    // could use or something
    // trace!("Hello, World!");
    let allocator = Allocator{};
    let s = "Hello World";

    log::info!("Testing!");

    // let logger = Logger {
    //     writer: null_mut()
    // };    

    let _hi = Hi {};

    Status::SUCCESS
}
