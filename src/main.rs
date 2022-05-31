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
// UEFI WRAPPERS
// --------------

extern crate alloc;

use core::arch::asm;

use aarch64::regs::{ELR_EL2, ELR_EL3, HCR_EL2, SPSR_EL3};
use cortex_a::{asm, registers};
use log::info;
use tock_registers::interfaces::{Readable, Writeable};
use uefi::{
    prelude::*,
    proto::console::serial::Serial,
    table::boot::{OpenProtocolAttributes, OpenProtocolParams},
};
use uefi_services;

// I DONT THINK IT WORKS FOR RISCV? I know U-boot works
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).expect("Failed to initialize utilities");

    info!("Entry into arcboot!");

    // Reset the console before running all the other tests
    system_table
        .stdout()
        .reset(false)
        .expect("Failed to reset stdout");

    // Ensure the tests are run on a version of UEFI we support
    check_revision(system_table.uefi_revision());

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

    #[cfg(target_arch = "aarch64")]
    transition_to_el2();

    // HAND OFF TO KERNEL
    // search for an arcboot compliant kernel.elf img. Or a default kernel partition
    // rn just exits
    load_arcboot_kernel();

    // Kernel returns, shutdown system
    // NOTE: kernel should return 0. If not, log to /logs/<timestamp.log> [KERNEL EXIT FAILURE]

    // IDK why test requires mut ref so this doesnt work
    // bt.stall(3_000_000);

    info!("Success!");

    shutdown(image, system_table);
}

// rn doesnt return
// can then register a shutdown() fn from kernel which calls shutdown() here
// or just returns to main
pub fn load_arcboot_kernel() {
    info!("current EL = {}", registers::CurrentEL.get());
}

fn transition_to_el2() {
    use cortex_a::registers;
    let curr_el = registers::CurrentEL.get();

    info!("current EL = {}", curr_el);

    // NOTE: EL2 is actually 0x4 (0100), which is EL1 I think
    // 0x8 is EL2. The EL value is in bit 2 and 3. b1100 = EL3, b1000 = EL2, b0100 = EL1. b0000 = EL0

    // GO INTO EL2 IF NOT ALREADY
    // NOTE: EFI already puts us into EL1??
    if curr_el < 0x4 {
        // transition_to_el2();

        // maybe i didnt initialise the registers
        info!("Setting EL3 -> El2 return masks. Disable interrupts");
        SPSR_EL3.write(
            SPSR_EL3::D::Masked
                + SPSR_EL3::A::Masked
                + SPSR_EL3::I::Masked
                + SPSR_EL3::F::Masked
                + SPSR_EL3::M::EL2h,
        );

        info!("Setting EL3 return function...");
        // getting an exception here...
        ELR_EL3.set(load_arcboot_kernel as *const () as u64);

        // maybe didnt set stack properly
        // SP_EL2.set(0x1000000);
        // unsafe {
        //     asm!(
        //         "
        //         MSR 0x1000000 SP_EL2
        //         "
        //     );
        // }
        // im actually getting an exception at 0x00000000404801F0
        // when there shouldnt be one
        info!("About to return from exception...");
        asm::eret();
    }
}

// Taken from the uefi-test-runner
fn check_revision(rev: uefi::table::Revision) {
    let (major, minor) = (rev.major(), rev.minor());

    info!("UEFI {}.{}", major, minor / 10);

    assert!(major >= 2, "Running on an old, unsupported version of UEFI");
    assert!(
        minor >= 30,
        "Old version of UEFI 2, some features might not be available."
    );
}

// Taken from the uefi-test-runner
fn shutdown(image: uefi::Handle, mut st: SystemTable<Boot>) -> ! {
    use uefi::table::runtime::ResetType;

    // Get our text output back.
    st.stdout().reset(false).unwrap();

    // Inform the user, and give him time to read on real hardware
    info!("Testing complete, shutting down in 3 seconds...");
    st.boot_services().stall(3_000_000);

    // Exit boot services as a proof that it works :)
    let sizes = st.boot_services().memory_map_size();
    let max_mmap_size = sizes.map_size + 2 * sizes.entry_size;
    let mut mmap_storage = alloc::vec![0; max_mmap_size].into_boxed_slice();
    let (st, _iter) = st
        .exit_boot_services(image, &mut mmap_storage[..])
        .expect("Failed to exit boot services");

    // Shut down the system
    let rt = unsafe { st.runtime_services() };
    rt.reset(ResetType::Shutdown, Status::SUCCESS, None);
}

// --------------
// TEST
// --------------
