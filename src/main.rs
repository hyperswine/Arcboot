#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(feature = "main_test"), no_main)]
// maybe just make abi_efiapi there
#![feature(abi_efiapi)]

// ? somehow use the builtin_allocator feature

// --------------
// UEFI WRAPPERS
// --------------

#[macro_use]
extern crate log;
extern crate alloc;

#[cfg(feature = "builtin_allocator")]
use arcboot::memory::heap::init_heap;

use aarch64::regs::{
    CurrentEL, ELR_EL2, ELR_EL3, HCR_EL2, MAIR_EL1, SCTLR_EL1, SP, SPSR_EL3, SP_EL1,
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
    print_serial_line,
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
    }
    info!("Hopefully you saw 2 diamond question marks");

    // Ensure the tests are run on a version of UEFI we support
    arcboot::efi::check_revision(system_table.uefi_revision());

    let bt = system_table.boot_services();

    // Initialise heap area at 0x4000_0000
    #[cfg(feature = "builtin_allocator")]
    init_heap();

    let s = String::from("This is an Allocated String");
    info!("Printing Allocated String: {s}");
    // 0xbf80_8110 -> 3,212,869,904 bytes
    info!("Address of allocated string = {:p}", &s);

    info!("[TEST] Printing from 0x4000_0000 again...");
    let ptr: *const u8 = 0x4000_0000 as *const u8;
    unsafe {
        print_serial_line!("0x4000_0000 = {}", *ptr.offset(0) as char);
        print_serial_line!("0x4000_0001 = {}", *ptr.offset(1) as char);
    }
    info!("Hopefully you saw 'Th'");

    // 0xBF80_7D70
    let sp = SP.get();
    info!("Stack pointer EL1 = {sp:#04X}");

    // Retrieve a handle to the file system the image was booted from
    bt.get_image_file_system(image)
        .expect("Failed to retrieve boot file system");

    // Get the memory map
    let _size = bt.memory_map_size().map_size;
    info!("Memory map size (bytes)= {_size}");

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

    let (st, _iter) = system_table
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect("Failed to exit boot services");

    // -----------
    // LOAD ARCBOOT DRIVERS
    // -----------

    print_serial_line!("Hello from runtime!");

    let rt = unsafe { st.runtime_services() };

    let config_table = st.config_table();

    let curr_el = CurrentEL.get();
    assert_eq!(curr_el, 0x4);
    info!("Current EL = {}", curr_el);

    // NOTE: append leading zeroes to get 64 bits

    // SCTLR for endianess
    let sctlr = SCTLR_EL1.get();
    info!("SCTLR EL1 = {sctlr:#066b}\n");
    // Endianess is in EE
    // SCTLR_EL1::EE.read(0);

    // 63 - 0 (little endian, [0] is LSB)

    // 0b0000000000000000000000000000000011111111101110110100010000000000
    // nGnRE maybe
    let mem = MAIR_EL1.get();
    info!("MAIR EL1 = {mem:#066b}\n");

    // 0b0000000000000000000000000000000010111111111111111111000000000000
    // BASE ADDR = 0xBFFF_F000 (3.2G)
    let mem = TTBR0_EL1.get();
    info!("TTBR0 EL1 = {mem:#066b}\n");

    // no need to reset TTBR0 since we're not using it?

    // 0b0000000000000000000000000000000000000000000000000000000000000000
    let mem = TTBR1_EL1.get();
    info!("TTBR1 EL1 = {mem:#066b}\n");

    // 0b0000000000000000000000000000010010000000100000000011010100010100
    // SO ITS ACTUALLY 6
    // T0SZ = 20
    // EPD0 = 0 => use TTBR0 for translation table walks (should be mutually exclusive with TTBR1?)
    // IRGN = 10 => write through, cacheable inner
    // ORGN0 = 10 => write through, cacheable outer
    // SHO = 10 => outer shareable
    // TG0 = 01 => 64KB granule for TTBR0 (what??)
    // T1SZ = 0 (not set pretty much)
    // A1 = 0 (TTBR0 defines the ASID)
    // EPD1 = 0 (ON)
    let tcr = aarch64::regs::TCR_EL1.get();
    info!("TCR EL1 = {tcr:#066b}\n");

    // 0xBF807A90
    let sp = SP.get();
    info!("Stack pointer EL1 = {sp:#04X}");

    // EXCEPTION DUE TO READING HIGHER EXCEPTION LEVEL CONTROL REG. I.e. security for hypervisor
    // let mem = TTBR0_EL2.read();
    // info!("TTBR0 EL2 = {mem:#b}\n");

    info!("Setting up Arc Memory Protocol...");

    // Maybe setup memory in the kernel. Could then hand off mmap_storage to the kernel to give it an idea of the memory map
    // st.set_virtual_address_map(map, new_system_table_virtual_addr); Or use a custom format
    #[cfg(target_arch = "aarch64")]
    arcboot::arm64::memory::setup();

    info!("Attempting to Load Kernel...");

    // HAND OFF TO KERNEL. Search for an arcboot compliant kernel ELF img in the standard location on the main configured NeFS or EFI boot config where DEFAULT_KERNEL_PARTITION=drive<number>partiton<number>
    // NOTE: before kernel loads userspace, do TLBI ALLE0 to clear TLB
    // PASS: the runtime services table, RSDP pointer, and thats pretty much it
    load_arcboot_kernel();

    // GET ACPI RSDT. AARCH64, in the kernel
    // get_acpi_tables(rt, config_table);

    // Kernel returns or traps to EL2 with reset exception/shutdown exception
    info!("Shutting down arcboot!");

    arcboot::efi::shutdown(st);
}

/// Move the mem descriptors here
fn handle_memory_map(bootservices: &BootServices) {
    let res = get_mem_map(bootservices);
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
#[no_mangle]
extern "C" fn __chkstk() {}

// RUN cargo test --features main_test

#[cfg(feature = "main_test")]
fn main() {}

#[cfg(feature = "main_test")]
mod test {
    #[test]
    fn test_main() {
        assert_eq!(0, 0);
    }
}
