use core::{
    fmt::{self, Write},
    ops::Not,
    sync::atomic::{AtomicBool, Ordering},
};

// aarch64 = 0x09000000 for most things
// includes some extra setup code if not using uefi

#[macro_export]
macro_rules! write_uart {
    ($exact:expr) => {
        let p = 0x09000000 as *mut u8;
        for byte in $exact {
            unsafe {
                match byte {
                    0x20..=0x7e | b'\n' => core::ptr::write_volatile(p, *byte),
                    _ => core::ptr::write_volatile(p, 0xfe),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! write_uart_line {
    ($exact:expr) => {
        let p = 0x09000000 as *mut u8;
        for byte in $exact {
            unsafe {
                match byte {
                    0x20..=0x7e | b'\n' => core::ptr::write_volatile(p, *byte),
                    _ => core::ptr::write_volatile(p, 0xfe),
                }
            }
        }
        unsafe {
            core::ptr::write_volatile(p, b'\n');
        }
    };
}
