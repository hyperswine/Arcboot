#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(feature = "main_test"), no_main)]

// --------------
// STANDARD ENTRY
// --------------

// A standard entry uses the standard-<arch>.json target
// And builds for a uboot-like target. With U-boot, have to also include a UBoot header

#[no_mangle]
extern "C" fn arcboot_entry() -> ! {
    // make an ArcServices, then setup kernel in higher half, load its segments to tis proper positions (ELF)
    // then `j arc_entry` or jump to that address. Its the virt addr isnt it?
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // print_serial_line!("Panicked at {info}");
    loop {}
}
