#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(feature = "main_test"), no_main)]

// --------------
// STANDARD ENTRY
// --------------

// A standard entry uses the standard-<arch>.json target

extern "C" fn arcboot_entry() -> ! {
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // print_serial_line!("Panicked at {info}");
    loop {}
}
