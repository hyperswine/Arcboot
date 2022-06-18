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
    CurrentEL, ELR_EL2, ELR_EL3, HCR_EL2, MAIR_EL1, SP, SPSR_EL3, SP_EL1,
    TCR_EL1::{self, EPD0::EnableTTBR0Walks},
    TTBR0_EL1, TTBR0_EL2, TTBR1_EL1,
};
use alloc::{
    string::String,
    vec::{self, Vec},
};
use arcboot::{
    logger::init_runtime_logger, memory::heap::init_heap, print_serial_line, write_uart,
    write_uart_line,
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
        runtime::VariableVendor,
        Runtime,
    }, Guid,
};

// maximum bytes (1248)
pub const MAX_MEMORY_MAP_SIZE: usize = 1248;

// aligned to 64 bytes. Prob 8 bytes is better
#[repr(C, align(8))]
struct AlignToMemoryDescriptor([u8; MAX_MEMORY_MAP_SIZE]);

// For RISC-V, use U-Boot
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

    let curr_el = CurrentEL.get();
    info!("current el = {curr_el}");

    // try to read 2 bytes at 0xa0000000
    // should print out null if page is zeroed, or just junk
    info!("Printing Junk...");
    let ptr: *const u8 = 0xa0000000 as *const u8;
    unsafe {
        print_serial_line!("0xa0000000 = {}", *ptr.offset(0) as char);
        print_serial_line!("0xa0000000 = {}", *ptr.offset(1) as char);
    }

    info!("Entry into arcboot!");

    // Ensure the tests are run on a version of UEFI we support
    arcboot::efi::check_revision(system_table.uefi_revision());

    let bt = system_table.boot_services();

    // Initialise heap area at 0xa000_0000
    init_heap();

    let s = String::from("Hi");
    info!("s = {s}");
    // 0xbf808110
    info!("address of s = {:p}", &s);

    // 0xBF807D70
    let sp = SP.get();
    info!("stack pointer EL1 = {sp:#04X}");

    // Retrieve a handle to the file system the image was booted from
    bt.get_image_file_system(image)
        .expect("Failed to retrieve boot file system");

    // Get the memory map
    let _size = bt.memory_map_size().map_size;
    info!("Memory map size (bytes)= {_size}");
    // buffer too big it seems.. I have no idea why
    // let mut memory_map = AlignToMemoryDescriptor{0: [0 as u8; MAX_MEMORY_MAP_SIZE]};
    // let res = bt.memory_map(&mut memory_map.0);
    // false, maybe buffer too small?
    // info!("Retrieve Memory Map = {}", res.is_ok());

    // this line causes problems. undefined symbol "__chkstk"
    // info!("Memory Map Keys = {:?}", res.unwrap().0);

    // TESTS TO ENSURE A WORKING SYSTEM

    info!("Testing boot services...");

    arcboot::efi::boot::test(bt);
    arcboot::efi::proto::test(image, &mut system_table);
    arcboot::efi::runtime::test(system_table.runtime_services());

    // -----------
    // BOOT PROTOCOL
    // -----------

    info!("Initialising boot protocol");

    // IF HYPERVISOR feature is on, trap into EL2 instead since we are at EL1
    // for riscv, trap to H-Mode
    #[cfg(feature = "archypervisor")]
    arcboot::aarch64::trap_to_el2();

    // Exit boot services as a proof that it works :)
    let sizes = system_table.boot_services().memory_map_size();
    let max_mmap_size = sizes.map_size + 2 * sizes.entry_size;
    let mut mmap_storage = alloc::vec![0; max_mmap_size].into_boxed_slice();

    info!("max_mmap_size: {}", max_mmap_size);
    // should be all zeros
    // info!("mmap_storage (before): {:?}", &mmap_storage);

    let (st, _iter) = system_table
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect("Failed to exit boot services");

    // -----------
    // LOAD ARCBOOT DRIVERS
    // -----------

    print_serial_line!("Hello from runtime!");

    // info!("mmap_storage (after): {:?}", &mmap_storage);
    // we need to get RDSP from the vector table at 0x0 with some offset I think
    // no its in the EFI_SYSTEM_TABLE, prob at runtime is fine. Its 16B
    // then we can use ACPI::from_rdsp
    let rt = unsafe { st.runtime_services() };

    fn quick(rt: &RuntimeServices) {
        let name = cstr16!("RSD PTR ");
        // maybe heap too small? nah it cant tell
        // __chkstk is a w32 function
        // maybe its not fully supported or something... I have no idea
        let mut vec_rdsp = alloc::vec![0; 16].into_boxed_slice();
        // something wrong with this... I have no idea
        // let mut vec_rdsp: Vec<u8> = Vec::new();
        let res = rt.get_variable(name, &VariableVendor::GLOBAL_VARIABLE, &mut vec_rdsp);
        info!("res = {}", res.is_ok());

        match res {
            Ok(r) => todo!(),
            // not found for RDSP, err
            Err(err) => info!("err = {err:?}"),
        }

        let v1 = cstr16!("EB9D2D30-2D88-11D3-9A16-0090273FC14D");
        let res = rt.get_variable(v1, &VariableVendor::GLOBAL_VARIABLE, &mut vec_rdsp);
        info!("v1 = {}", res.is_ok());

        match res {
            Ok(r) => todo!(),
            Err(err) => info!("err = {err:?}"),
        }

        let v2 = cstr16!("8868E871-E4F1-11D3-BC22-0080C73C8881");
        let res = rt.get_variable(v2, &VariableVendor::GLOBAL_VARIABLE, &mut vec_rdsp);
        info!("v2 = {}", res.is_ok());

        match res {
            Ok(r) => todo!(),
            Err(err) => info!("err = {err:?}"),
        }

        // maybe try vendor

        let v2 = cstr16!("8868E871-E4F1-11D3-BC22-0080C73C8881");
        let v2 = Guid::from_values(0x8868E871, 0xE4F1, 0x11D3, 0xBC22, 0x0080C73C8881);
        let var_vendor = VariableVendor(v2);
        let res = rt.get_variable(name, &var_vendor, &mut vec_rdsp);
        info!("v2 = {}", res.is_ok());

        match res {
            Ok(r) => todo!(),
            Err(err) => info!("err = {err:?}"),
        }

        // try EB9D2D30-2D88-11D3-9A16-0090273FC14D first
        // try 8868E871-E4F1-11D3-BC22-0080C73C8881 next
    }

    quick(rt);

    let curr_el = CurrentEL.get();
    assert_eq!(curr_el, 0x4);
    print_serial_line!("Current EL = {}", curr_el);

    let mem = MAIR_EL1.get();
    print_serial_line!("MAIR EL1 = {mem:#b}\n");

    let mem = TTBR0_EL1.get();
    print_serial_line!("TTBR0 EL1 = {mem:#b}\n");

    let mem = TTBR1_EL1.get();
    print_serial_line!("TTBR1 EL1 = {mem:#b}\n");

    let sp = SP.get();
    info!("stack pointer EL1 = {sp:#04X}");

    let mem = TCR_EL1::EPD1.val(0);
    print_serial_line!("TCR EL1 field 0 = {}\n", mem.value);

    // apparently doesnt work with macro?
    // since its not readable directly as a whole
    // let mem = TTBR0_EL2.read();
    // print_serial_line!("TTBR0 EL2 = {mem:#b}\n");

    // I thought EnableTTBR1Walks sets TTBCR. But it actually sets TCR_EL1, one of the flags
    print_serial_line!("Enabling TTBR0 walks...\n");
    // EnableTTBR0Walks.

    // setup new virtual table
    // st.set_virtual_address_map(map, new_system_table_virtual_addr);

    print_serial_line!("Attempting to Load Kernel...!");

    // HAND OFF TO KERNEL. Search for an arcboot compliant kernel ELF img in the standard location on the main configured NeFS
    // or EFI boot config where DEFAULT_KERNEL_PARTITION=drive<number>partiton<number>
    // NOTE: before kernel loads userspace, do TLBI ALLE0 to clear TLB
    load_arcboot_kernel();

    // Kernel returns or traps to EL2 with reset exception/shutdown exception
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
