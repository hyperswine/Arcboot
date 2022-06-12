// --------------
// USE
// --------------

use crate::{aarch64::drivers::Uart, print_serial_line};
use core::fmt::Write;
use core::{fmt, ptr::NonNull};
use lazy_static::lazy_static;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError, Log};

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

    /// Cannot log if anotherr task is using the same output stream
    /// Could prob spinlock on it
    fn log(&self, record: &Record) {
        // spinlock
        // while !self.enabled(record.metadata()) {}

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
