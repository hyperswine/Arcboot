
// --------------
//  UEFI ENTRY
// --------------

use core::ptr::null_mut;
use uefi::alloc::Allocator;
use uefi::logger::Logger;
use uefi::prelude::*;
use uefi::alloc;

// I DONT THINK IT WORKS FOR RISCV. Needs a PR
#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // init allocator

    // trace_macros!(true);
    // could use or something
    // trace!("Hello, World!");
    let allocator = Allocator {};
    let s = "Hello World";

    log::info!("Testing!");

    // let logger = Logger {
    //     writer: null_mut()
    // };

    Status::SUCCESS
}
