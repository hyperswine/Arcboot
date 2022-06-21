#![cfg_attr(not(test), no_std)]
#![no_main]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]

// --------------
// UEFI WRAPPERS
// --------------

#[macro_use]
extern crate log;
extern crate alloc;

use aarch64::regs::{
    CurrentEL, ELR_EL2, ELR_EL3, HCR_EL2, MAIR_EL1, SP, SPSR_EL3, SP_EL1,
    TCR_EL1::{self, EPD0::EnableTTBR0Walks},
    TTBR0_EL1, TTBR0_EL2, TTBR1_EL1,
};
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use alloc::{
    string::String,
    vec::{self, Vec},
};
use arcboot::efi::acpi::get_acpi_tables;
use arcboot::efi::get_mem_map;
use arcboot::{
    efi::{acpi::AcpiHandle, AlignToMemoryDescriptor},
    logger::init_runtime_logger,
    memory::heap::init_heap,
    print_serial_line, write_uart, write_uart_line,
};

use arcboot::*;

use core::{
    arch::asm,
    borrow::Borrow,
    ptr::{null, NonNull},
};
use cortex_a::{asm, registers};
use log::{info, Level, Metadata, Record};
use tock_registers::interfaces::{Readable, Writeable};
use uefi::{
    prelude::*,
    proto::console::{serial::Serial, text::Output},
    table::{
        boot::{OpenProtocolAttributes, OpenProtocolParams},
        cfg::{self, ConfigTableEntry},
        runtime::VariableVendor,
        Runtime,
    },
    Guid,
};
use uefi_macros::entry;

use core::ffi::c_void;

// For RISC-V, use U-Boot

// Mostly for setting up UEFI services. Calls other functions
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // -----------
    // UEFI BootServices
    // -----------

    // Initialise logger
    assert!(init_runtime_logger().is_ok());

    // Reset the console after initialising logger
    system_table
        .stdout()
        .reset(false)
        .expect("Failed to reset stdout");

    info!("Entry into arcboot!");

    let curr_el = CurrentEL.get();
    info!("Current EL = {curr_el}");

    // Read 2 bytes at a the beginning of the Heap area. Should print out null if page is zeroed, or just junk
    info!("[TEST] Printing Junk...");
    let ptr: *const u8 = 0x4000_0000 as *const u8;
    unsafe {
        print_serial_line!("0x4000_0000 = {}", *ptr.offset(0) as char);
        print_serial_line!("0x4000_0001 = {}", *ptr.offset(1) as char);

        // print what is at 0x0
        // print_serial_line!("Address of 0x0 = {:p}", 0x0 as *const u8);
    }
    info!("Hopefully you saw 2 diamond question marks");

    // Ensure the tests are run on a version of UEFI we support
    arcboot::efi::check_revision(system_table.uefi_revision());

    let bt = system_table.boot_services();

    // Initialise heap area at 0x4000_0000
    init_heap();

    let s = String::from("This is an Allocated String");
    info!("Printing Allocated String: {s}");
    // 0xbf80_8110 -> 3,212,869,904 bytes
    info!("Address of allocated string = {:p}", &s);

    // 0xBF80_7D70
    let sp = SP.get();
    info!("Stack pointer EL1 = {sp:#04X}");

    // Retrieve a handle to the file system the image was booted from
    bt.get_image_file_system(image)
        .expect("Failed to retrieve boot file system");

    // Get the memory map
    let _size = bt.memory_map_size().map_size;
    info!("Memory map size (bytes)= {_size}");

    // !! ARCH64-UNKNOWN-UEFI OMEGALUL
    // let memory_map = get_mem_map(bt);
    handle_memory_map(bt);

    // -----------
    // UEFI STARTUP CHECKS
    // -----------

    info!("Testing boot services...");

    arcboot::efi::boot::test(bt);
    arcboot::efi::proto::test(image, &mut system_table);
    arcboot::efi::runtime::test(system_table.runtime_services());

    // -----------
    // BOOT PROTOCOL
    // -----------

    info!("Initialising boot protocol");

    // IF HYPERVISOR feature is on, trap into EL2 instead since we are at EL1 for riscv, trap to H-Mode
    #[cfg(feature = "archypervisor")]
    arcboot::arm64::trap_to_el2();

    // Exit boot services. We can get the MMAP here
    let sizes = system_table.boot_services().memory_map_size();
    let max_mmap_size = sizes.map_size + 2 * sizes.entry_size;
    let mut mmap_storage = alloc::vec![0; max_mmap_size].into_boxed_slice();

    info!("max_mmap_size: {}", &max_mmap_size);

    // ? why does this work?? WHY??
    // but when I scope it, it doesnt. OMG

    let (st, _iter) = system_table
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect("Failed to exit boot services");

    // ? Maybe need an ARC

    // -----------
    // LOAD ARCBOOT DRIVERS
    // -----------

    // MAYBE JUST SETUP PAGING WITH 4-level page tables

    print_serial_line!("Hello from runtime!");

    let rt = unsafe { st.runtime_services() };

    // PRINT OUT MEMORY MAP
    // ? Need to cast this as an array of MemoryKey
    // mmap_storage
    //     .iter()
    //     .for_each(|i| info!("Memory Descriptor (byte) = {i}"));

    let config_table = st.config_table();

    // !! GET ACPI RSDT. AARCH64
    // get_acpi_tables(rt, config_table);

    let curr_el = CurrentEL.get();
    assert_eq!(curr_el, 0x4);
    print_serial_line!("Current EL = {}", curr_el);

    let mem = MAIR_EL1.get();
    print_serial_line!("MAIR EL1 = {mem:#b}\n");

    let mem = TTBR0_EL1.get();
    print_serial_line!("TTBR0 EL1 = {mem:#b}\n");

    let mem = TTBR1_EL1.get();
    print_serial_line!("TTBR1 EL1 = {mem:#b}\n");

    // 0xBF807A90
    let sp = SP.get();
    info!("Stack pointer EL1 = {sp:#04X}");

    // READ A BYTE FROM THE SP

    let mem = TCR_EL1::EPD1.val(0);
    print_serial_line!("TCR EL1 field 0 = {}\n", mem.value);

    // apparently doesnt work with macro?
    // since its not readable directly as a whole
    // let mem = TTBR0_EL2.read();
    // print_serial_line!("TTBR0 EL2 = {mem:#b}\n");

    // I thought EnableTTBR1Walks sets TTBCR. But it actually sets TCR_EL1, one of the flags
    print_serial_line!("Enabling TTBR0 walks...\n");
    // EnableTTBR0Walks.

    // ! Setup new virtual table, IDK if TTBRR0 then prob doesnt make too much sense
    // Maybe do that in the kernel. Also hand off mmap_storage to the kernel to give it an idea of the memory map
    // Also LAI?? Use that in the kernel if possible. ACPI is pretty bad
    // st.set_virtual_address_map(map, new_system_table_virtual_addr);

    print_serial_line!("Attempting to Load Kernel...!");

    // HAND OFF TO KERNEL. Search for an arcboot compliant kernel ELF img in the standard location on the main configured NeFS
    // or EFI boot config where DEFAULT_KERNEL_PARTITION=drive<number>partiton<number>
    // NOTE: before kernel loads userspace, do TLBI ALLE0 to clear TLB
    // PASS: the runtime services table, RSDP pointer, and thats pretty much it
    load_arcboot_kernel();

    // Kernel returns or traps to EL2 with reset exception/shutdown exception
    info!("Shutting down arcboot!");

    arcboot::efi::shutdown(st);
}

// Maybe the scoping rules in #[entry] isnt good enough
// fn get_runtime_table(mmap_storage: &Box<u8>, mut system_table: &SystemTable<Boot>) -> (SystemTable<Runtime>, impl ExactSizeIterator<Item = &MemoryDescriptor> + Clone) {
//     let m = &mut mmap_storage;
//     let res = system_table
//         .exit_boot_services(image, &mut m[..])
//         .expect("Failed to exit boot services");

//     res
// }

/// Move the mem descriptors here
fn handle_memory_map(bootservices: &BootServices) {
    let res = get_mem_map(bootservices);

    // set the global variable or something. Or maybe just have a global var in lib.rs
}

/// Load kernel
fn load_arcboot_kernel() {}

// ----------------
// PANIC
// ----------------

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print_serial_line!("Panicked at {info}");
    loop {}
}

// Disable stack protection
#[no_mangle] extern "C" fn __chkstk () {}
