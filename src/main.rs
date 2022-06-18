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
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use alloc::{
    string::String,
    vec::{self, Vec},
};
use arcboot::{
    logger::init_runtime_logger, memory::heap::init_heap, print_serial_line, write_uart,
    write_uart_line,
};
use core::{arch::asm, borrow::Borrow, ptr::NonNull};
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
    init_heap();

    let s = String::from("This is an Allocated String");
    info!("Printing Allocated String: {s}");
    // 0xbf808110
    info!("address of allocated string = {:p}", &s);

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

    fn get_mem_map(bt: &BootServices) {
        let mut memory_map = AlignToMemoryDescriptor {
            0: [0 as u8; MAX_MEMORY_MAP_SIZE],
        };
        let res = bt.memory_map(&mut memory_map.0);

        match res {
            Ok(r) => {
                info!("Retrieved Memory Map!");
                // identifies this memory map
                info!("Memory Map Key = {:?}", &r.0);
                // second...
                let iterator_item = r.1;
                iterator_item.for_each(|i| info!("Memory Descriptor = {i:?}"));
            }
            Err(err) => panic!("Could not get memory map. Error: {err:?}"),
        }

        // for aarch64, generally
        // 0x0 - 0x400_0000 is MMIO
        // BFFF_C000 - 0xBFFFD000 is MMIO
        // 0x4000_0000 + 492632 * 4K is conventional
        // no MMIO anymore?? maybe cause of different heap

        // 1GB-3GB ('phys') is DRAM
        // maybe it means vaddr with a 1GB offset
    }

    get_mem_map(bt);

    // TESTS TO ENSURE A WORKING SYSTEM

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
    arcboot::aarch64::trap_to_el2();

    // Exit boot services as a proof that it works :)
    let sizes = system_table.boot_services().memory_map_size();
    let max_mmap_size = sizes.map_size + 2 * sizes.entry_size;
    let mut mmap_storage = alloc::vec![0; max_mmap_size].into_boxed_slice();

    info!("max_mmap_size: {}", max_mmap_size);

    let (st, _iter) = system_table
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect("Failed to exit boot services");

    // -----------
    // LOAD ARCBOOT DRIVERS
    // -----------

    print_serial_line!("Hello from runtime!");

    let rt = unsafe { st.runtime_services() };

    let config_table = st.config_table();

    fn quick(rt: &RuntimeServices, config_table: &[ConfigTableEntry]) {
        let res = rt.variable_keys().unwrap();
        // try EB9D2D30-2D88-11D3-9A16-0090273FC14D first
        // try 8868E871-E4F1-11D3-BC22-0080C73C8881 next

        let rsdp = cfg::ACPI2_GUID;

        // Find the value of RSDP
        let res = config_table.iter().find(|c| c.guid == rsdp);

        let rsdp = match res {
            Some(r) => {
                info!("Found RSDP entry, at addr = {:p}", r.address);
                r.address
            }
            None => {
                panic!("Couldn't find RSDP in the ACPI v2 table")
            }
        };

        // Identity mapped
        #[derive(Debug, Clone)]
        struct Handler {
            // base addr, sizek
            regions: (usize, usize),
        }

        struct Holder {
            regions: Vec<(usize, usize)>
        }

        let holder = Holder {regions: Vec::new()};

        impl AcpiHandler for Handler {
            // gotta keep a list of mapped regions
            unsafe fn map_physical_region<T>(
                &self,
                physical_address: usize,
                size: usize,
            ) -> PhysicalMapping<Self, T> {
                info!("Mapping a region!");
                let va = NonNull::new(physical_address as *mut T).unwrap();
                PhysicalMapping::new(
                    physical_address,
                    va,
                    size,
                    size,
                    Self {
                        regions: (physical_address, size),
                    },
                )
            }

            // have to call a global variable or something
            fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {
                info!("Unmapping a region!");
                let handler = region.handler();
                // prob not how they want you to do it then, since passes const ref
                // handler.regions = (0, 0);
            }
        }

        let handler = Handler {
            regions: (0xc0000_0000, 0x100000),
        };

        // acpi
        // let res = unsafe { AcpiTables::from_rsdp(handler, rsdp as usize) };
        // match res {
        //     Ok(r) => {
        //         let p = r.platform_info().unwrap();
        //         info!("Power profile = {:?}", p.power_profile);
        //         info!("Interrupt model = {:?}", p.interrupt_model);
        //         info!(
        //             "Boot Processor Info = {:?}",
        //             &p.processor_info.as_ref().unwrap().boot_processor
        //         );
        //         info!(
        //             "Application Processor Info = {:?}",
        //             &p.processor_info.unwrap().application_processors
        //         );
        //         info!("PM Timer = {:?}", p.pm_timer.unwrap().base);
        //     }
        //     Err(err) => panic!("couldn't get ACPI tables"),
        // }
    }

    quick(rt, config_table);

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
