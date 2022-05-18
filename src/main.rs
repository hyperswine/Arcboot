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

use log::info;
use tock_registers::interfaces::Readable;
use uefi::prelude::*;
use uefi_services;

// I DONT THINK IT WORKS FOR RISCV. Needs a PR
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).expect("Failed to initialize utilities");
    info!("Booted!");

    // GO INTO EL2 IF NOT ALREADY
    // unsafe {
    //     const EL2_ID: u64 = 0x8;
    //     const CORE_ID_MASK: u64 = 0b11;
    //     core::arch::asm!(
    //         "
    //         mrs x0, CurrentEL
    //         cmp x0, {EL2_ID}
    //         "
    //     );
    // }

    #[cfg(target_arch = "aarch64")]
    {
        use cortex_a::registers;
        let curr_el = registers::CurrentEL;
        // curr_el.read(field);
        let val = curr_el.get();
        info!("current EL = {}", val);
        
    }

    Status::SUCCESS
}
