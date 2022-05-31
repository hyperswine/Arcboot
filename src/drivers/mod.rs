// FOR GENERIC DRIVERS using UEFI tables/device trees
// for ssds, timers, etc.
// allows filesystem views

// FOR UART of arm, risc, x86, the drivers are stored in there and can be compiled by turning on a specific device's features, e.g. sifive

struct TimerDriver;
struct ModelDriver;

// -------------
// Quick Filesystem Drivers
// -------------
// use qfs;

// -------------
// Neutron Filesystem Drivers
// -------------
// use nefs;

// if detect an nefs partition on GPT
// load the arc driver into memory as a read only part
// at some page range
/*
pub fn load_drivers() {
    // for each drive
    // check gpt tables for filesystems
    for drive in drives {
        if drive.id == NeFS_GUID {
            // load NeFS drivers if not loaded already
            load_nefs_driver();
            // do what it does
        }
    }
}
*/
