// FOR GENERIC DRIVERS using UEFI tables/device trees
// for ssds, timers, etc.
// allows filesystem views

// FOR UART of arm, risc, x86, the drivers are stored in there and can be compiled by turning on a specific device's features, e.g. sifive

struct TimerDriver;
struct ModelDriver;

// * use uefi logger

use uefi::logger::Logger;
use uefi::proto::console::text::{Output, OutputMode};

trait New {
    fn new() -> Self;
}

impl New for Output<'_> {
    fn new() -> Self {
        Self {
            
        }
    }
}

// enable the logger when entering boot
pub fn write() {
    //     let output = Output{
    //         reset: false,
    //         output_string: "",
    //         test_string: "",
    // query_mode: (0,0,0),
    // set_mode: (),
    // set_attribute: (),
    // clear_screen: false,
    // set_cursor_position
    // enable_cursor
    // data
    //     };

    unsafe {
        let mut output = Output::new();
        let logger = Logger::new(&mut output);
    }
}

// disable the logger as we are handing off to kernel
