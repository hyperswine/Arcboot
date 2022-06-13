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

use aarch64::regs::{
    CurrentEL, ELR_EL2, ELR_EL3, HCR_EL2, MAIR_EL1, SPSR_EL3,
    TCR_EL1::{self, EPD0::EnableTTBR0Walks},
    TTBR0_EL1, TTBR0_EL2, TTBR1_EL1,
};
use alloc::string::String;
use arcboot::{
    aarch64::drivers::_print_serial, logger::init_runtime_logger, memory::heap::init_heap,
    print_serial_line, write_uart, write_uart_line,
};
use core::{arch::asm, ptr::NonNull};
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

// For RISC-V, use U-Boot
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // -----------
    // UEFI BootServices
    // -----------

    assert!(init_runtime_logger().is_ok());
    // ? Maybe endianess? But UEFI should be little endian I think
    init_heap();

    let s = String::from("Hi");
    info!("s = {s}");
    // 0x478071a8 is the address sometimes
    // ??? id expect it to be 0x80000. Maybe something with virtual memory/not id mapped?
    info!("address of s = {:p}", &s);

    // try to read 8 bytes into a &str at 0x80000
    let ptr: *const u8 = 0x80000 as *const u8;
    unsafe {
        print_serial_line!("{}", *ptr.offset(0) as char);
        print_serial_line!("{}", *ptr.offset(1) as char);
    }

    // can you somehow inspect the memory address directly?
    // prob. We have to know where its allocating as well
    // maybe Box or something. A pointer to Box

    // this uses uefi's internal alloc and logger
    // uefi_services::init(&mut system_table).expect("Failed to initialize utilities");

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

    // -----------
    // BOOT PROTOCOL
    // -----------

    info!("Initialising boot protocol");

    // IF HYPERVISOR feature is on, trap into EL2 instead since we are at EL1
    fn trap_to_el2() {}
    #[cfg(feature = "archypervisor")]
    trap_to_el2();

    // Exit boot services as a proof that it works :)
    let sizes = system_table.boot_services().memory_map_size();
    let max_mmap_size = sizes.map_size + 2 * sizes.entry_size;
    let mut mmap_storage = alloc::vec![0; max_mmap_size].into_boxed_slice();

    info!("max_mmap_size: {}", max_mmap_size);

    let (st, _iter) = system_table
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect("Failed to exit boot services");

    // bootservices no longer exists, cannot access stdout and stuff
    // paging is identity mapped with a singular L1 table I think, for DRAM frames
    // since if you try to access something higher it page faults

    // -----------
    // LOAD ARCBOOT DRIVERS
    // -----------

    print_serial_line!("Hello from runtime!");
    let curr_el = CurrentEL.get();
    assert_eq!(curr_el, 0x4);
    print_serial_line!("Current EL = {}", curr_el);

    let mem = MAIR_EL1.get();
    print_serial_line!("MAIR EL1 = {mem:#b}\n");

    let mem = TTBR0_EL1.get();
    print_serial_line!("TTBR0 EL1 = {mem:#b}\n");

    let mem = TTBR1_EL1.get();
    print_serial_line!("TTBR1 EL1 = {mem:#b}\n");

    // let mem = TCR_EL1::EPD1::Value;
    // print_serial_line!("TCR EL1 = {mem:#b}\n"));

    // apparently doesnt work with macro?
    // let mem = TTBR0_EL2.get();
    // print_serial_line!("TTBR0 EL2 = {mem:#b}\n");

    // i think EnableTTBR1Walks sets TTBCR
    // no it actually sets TCR_EL1, one of the flags
    print_serial_line!("Enabling TTBR0 walks...\n");
    // EnableTTBR0Walks.

    // TLBI ALLE1
    // before kernel loads userspace, do TLBI ALLE0

    // i think the kernel's pages should be be made global
    // so nG=0
    // to save pid space

    let s = String::from("Hello");

    print_serial_line!("s = {s}!");

    print_serial_line!("Heap initialised!");

    // setup new virtual table
    // st.set_virtual_address_map(map, new_system_table_virtual_addr);

    // HAND OFF TO KERNEL
    // search for an arcboot compliant kernel.elf img. Or a default kernel partition
    load_arcboot_kernel();

    // Kernel returns, shutdown system
    // NOTE: kernel should return 0. If not, log to /logs/<timestamp.log> [KERNEL EXIT FAILURE]

    info!("Shutting down arcboot!");

    arcboot::efi::shutdown(st);
}

pub fn load_arcboot_kernel() {}

// ----------------
// PANIC
// ----------------

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print_serial_line!("Panicked at {info}");
    loop {}
}

// --------------
// TEST
// --------------

#[test_case]
fn test_basics() {
    assert_eq!(1, 1);
}
