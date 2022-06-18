use core::{
    fmt::{self, Write},
    ops::Not,
    sync::atomic::{AtomicBool, Ordering},
};

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

/// Could prob put this in drivers and have an AARCH64 ArcDriverManager that impls UART and reexports the write_str
pub struct Uart(usize);

static IN_USE: AtomicBool = AtomicBool::new(false);

impl Uart {
    /// Lazily create an UART out, or if already exists, returns it directly
    /// For QEMU mostly
    pub fn new() -> Option<Uart> {
        impl Drop for Uart {
            fn drop(&mut self) {
                IN_USE.store(false, Ordering::Release);
            }
        }

        IN_USE
            .swap(true, Ordering::Acquire)
            .not()
            .then(|| Uart(0x09000000_usize))
    }
}

impl fmt::Write for Uart {
    fn write_str(self: &'_ mut Uart, s: &'_ str) -> fmt::Result {
        let uart_addr: *mut u8 = self.0 as _;
        s.bytes().for_each(|b| unsafe {
            uart_addr.write_volatile(if matches!(b, 0x20..=0x7e | b'\n') {
                b
            } else {
                0xfe
            })
        });
        Ok(())
    }
}

pub fn _print_serial(args: fmt::Arguments) {
    Uart::new()
        .expect("Concurrent UART access!")
        .write_fmt(args)
        .unwrap()
}

#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => ($crate::arm64::drivers::uart::_print_serial(format_args!($($arg)*)));
}

/// Use this in most cases, or a logger
#[macro_export]
macro_rules! print_serial_line {
    () => ($crate::print_serial!("\n"));
    ($($arg:tt)*) => ($crate::print_serial!("{}\n", format_args!($($arg)*)));
}
