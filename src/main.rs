#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(arcboot::test_runner)]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]

// --------------
// UEFI WRAPPERS
// --------------

extern crate alloc;

use core::{arch::asm, ptr::NonNull};

use aarch64::regs::{ELR_EL2, ELR_EL3, HCR_EL2, SPSR_EL3};
use cortex_a::{asm, registers};
use log::{info, Level, Metadata, Record};
use tock_registers::interfaces::{Readable, Writeable};
use uefi::{
    prelude::*,
    proto::console::{serial::Serial, text::Output},
    table::{
        boot::{OpenProtocolAttributes, OpenProtocolParams},
        Runtime,
    },
};
use uefi_services;

// I DONT THINK IT WORKS FOR RISCV? I know U-boot works
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // might just have to init them manually
    uefi_services::init(&mut system_table).expect("Failed to initialize utilities");

    // should initialise logger and allocator if using custom logging (or just manually without init)
    // otherwise uefi-rs automatically handles it

    info!("Entry into arcboot!");

    // Reset the console before running all the other tests
    system_table
        .stdout()
        .reset(false)
        .expect("Failed to reset stdout");

    // Ensure the tests are run on a version of UEFI we support
    arcboot::efi::check_revision(system_table.uefi_revision());

    // Test all the boot services.
    info!("Testing boot services...");

    let bt = system_table.boot_services();

    // Try retrieving a handle to the file system the image was booted from
    bt.get_image_file_system(image)
        .expect("Failed to retrieve boot file system");

    // TESTS TO ENSURE A WORKING SYSTEM

    arcboot::efi::boot::test(bt);
    arcboot::efi::proto::test(image, &mut system_table);
    arcboot::efi::runtime::test(system_table.runtime_services());

    // BOOT PROTOCOL

    info!("Initialising boot protocol");

    // IF HYPERVISOR feature is on, trap into EL2 instead
    // since we are at EL1

    // Exit boot services as a proof that it works :)
    let sizes = system_table.boot_services().memory_map_size();
    let max_mmap_size = sizes.map_size + 2 * sizes.entry_size;
    let mut mmap_storage = alloc::vec![0; max_mmap_size].into_boxed_slice();

    info!("max_mmap_size: {}", max_mmap_size);

    let (st, _iter) = system_table
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect("Failed to exit boot services");
    // bootservices no longer exists. Use our own implementations for better control

    // -----------
    // LOAD ARCBOOT DRIVERS
    // -----------

    // logger to UART0

    // linked list allocator

    //

    // setup new virtual table
    // st.set_virtual_address_map(map, new_system_table_virtual_addr);

    // HAND OFF TO KERNEL
    // search for an arcboot compliant kernel.elf img. Or a default kernel partition
    load_arcboot_kernel();

    // Kernel returns, shutdown system
    // NOTE: kernel should return 0. If not, log to /logs/<timestamp.log> [KERNEL EXIT FAILURE]

    // info!("Shutting down arcboot!");

    arcboot::efi::shutdown(st);
}

pub fn load_arcboot_kernel() {}

// ----------------
// BARE BIOS PANIC
// ----------------

#[cfg(not(feature = "uefi_support"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// --------------
// TEST
// --------------

#[test_case]
fn test_basics() {
    assert_eq!(1, 1);
}
