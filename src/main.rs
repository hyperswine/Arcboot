#![cfg_attr(not(feature = "uefi_support"), no_main)]
#![no_std]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(arcboot::test::test_runner)]
#![cfg_attr(feature = "uefi_support", feature(abi_efiapi))]

extern "C" fn _start() -> ! {
    loop {}
}

#[cfg(not(feature = "uefi_support"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
