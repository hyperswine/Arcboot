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

use log::info;
use tock_registers::interfaces::Readable;
use uefi::prelude::*;
use uefi_services;

// I DONT THINK IT WORKS FOR RISCV? I know U-boot works
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).expect("Failed to initialize utilities");

    // maybe its failing right after this line
    info!("Booted!");

    // Reset the console before running all the other tests.
    system_table
        .stdout()
        .reset(false)
        .expect("Failed to reset stdout");

    // Ensure the tests are run on a version of UEFI we support.
    check_revision(system_table.uefi_revision());

    // Test all the boot services.
    let bt = system_table.boot_services();

    // Try retrieving a handle to the file system the image was booted from.
    bt.get_image_file_system(image)
        .expect("Failed to retrieve boot file system");

    // boot::test(bt);

    // Test all the supported protocols.
    // proto::test(image, &mut system_table);

    // runtime services work before boot services are exited, but we'd
    // probably want to test them after exit_boot_services. However,
    // exit_boot_services is currently called during shutdown.

    // runtime::test(system_table.runtime_services());

    // ? its not being able to load the hardware properly into UEFI/ACPI tables or something
    // #[cfg(target_arch = "aarch64")]
    // {
    //     use cortex_a::registers;
    //     let curr_el = registers::CurrentEL;
    //     let val = curr_el.get();
    //     info!("current EL = {}", val);
    //     // GO INTO EL2 IF NOT ALREADY
    //     unsafe {
    //         const EL2_ID: u64 = 0x8;
    //         const CORE_ID_MASK: u64 = 0b11;
    //         core::arch::asm!(
    //             "
    //             mrs x0, CurrentEL
    //             cmp x0, {EL2_ID}
    //             "
    //         );
    //     }
    // }

    info!("Success!");

    shutdown(image, system_table);
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
    if cfg!(not(feature = "qemu")) {
        info!("Testing complete, shutting down in 3 seconds...");
        st.boot_services().stall(3_000_000);
    } else {
        info!("Testing complete, shutting down...");
    }

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
