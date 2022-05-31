// common stuff

// UART DRIVER

use alloc::string::ToString;

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

#[macro_export]
macro_rules! write_serial {
    ($($arg:tt)*) => (arcboot::aarch64::drivers::_print_serial(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! write_serial_line {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::write_serial!("{}\n", format_args!($($arg)*)));
}

pub fn _print_serial(args: core::fmt::Arguments) {
    // if alloc doesnt work, this wont work
    let a = args.to_string();
    write_uart!(a.as_bytes());
}

/*
pub struct Uart(());

static IN_USE: AtomicBool = AtomicBool:::new(false);

impl Uart {
    pub fn new() -> Option<Uart> {
        impl Drop for Uart {
            fn drop(&mut self) { IN_USE.store(false, Ordering::Release); }
        }

        IN_USE.swap(true, Ordering::Acquire).not().then(|| Uart(()))
    }
}

impl fmt::Write for Uart {
    fn write_str (self: &'_ mut Uart, s: &'_ str)
      -> fmt::Result
    {
        const UART_ADDR: *mut u8 = 0x09000000_usize as _;
        s.bytes().for_each(|b| unsafe {
            UART_ADDR.write_volatile(
                if matches!(b, 0x20..=0x7e | b'\n') { b } else { 0xfe }
            )
        });
        Ok(())
    }
}

pub fn _print_serial(args: core::fmt::Arguments) {
    Uart::new()
        .expect("Concurrent UART access!")
        .write_fmt(args)
        .unwrap() // we just wrote the impl, this is unreachable
}
*/
