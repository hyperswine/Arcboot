// --------------
// USE
// --------------

use core::fmt::Write;
use core::ops::Not;
use core::sync::atomic::{AtomicBool, Ordering};
use core::{fmt, ptr::NonNull};
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

#[cfg(target_arch="aarch64")]
const UART_ADDR: usize = 0x9000000;

#[cfg(target_arch="riscv64")]
const UART_ADDR: usize = 0x8000000;

#[cfg(target_arch="x86_64")]
const UART_ADDR: usize = 0x0080000;

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
            .then(|| Uart(UART_ADDR))
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
    ($($arg:tt)*) => ($crate::logger::_print_serial(format_args!($($arg)*)));
}

/// Use this in most cases, or a logger
#[macro_export]
macro_rules! print_serial_line {
    () => ($crate::print_serial!("\n"));
    ($($arg:tt)*) => ($crate::print_serial!("{}\n", format_args!($($arg)*)));
}


// --------------
// RUNTIME LOGGER
// --------------

// can specify console as well but Uart for now
// maybe put it somewhere else and call use() to do something else
struct RuntimeLogger;

impl Log for RuntimeLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let _ = metadata.level() <= Level::Info;
        Uart::new().is_some()
    }

    /// Cannot log if another task is using the same output stream
    /// Could prob spinlock on it
    fn log(&self, record: &Record) {
        // spinlock
        while !self.enabled(record.metadata()) {}

        if self.enabled(record.metadata()) {
            print_serial_line!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: RuntimeLogger = RuntimeLogger;

pub fn init_runtime_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
