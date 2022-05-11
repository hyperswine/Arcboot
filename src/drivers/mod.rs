// FOR GENERIC DRIVERS using UEFI tables/device trees
// for ssds, timers, etc.
// allows filesystem views

// FOR UART of arm, risc, x86, the drivers are stored in there and can be compiled by turning on a specific device's features, e.g. sifive

struct TimerDriver;
struct ModelDriver;

// -------------
// Quick Filesystem Drivers
// -------------

// -------------
// Neutron Filesystem Drivers
// -------------

type RootFS = u64;

// storage of filesystem metadata in memory
struct NeFSPartition {
    partition: Partition,
    root_fs: RootFS,
}

// for GPT
const NeFS_GUID: u128 = 0x1FF;

// contains the offset + size for the partition on disk
pub struct Partition {
    id: u64,
    offset: u64,
    size: u64,
}

// given a nefs partition, collect data on it
// from the headers and stuff
pub fn nefs(partition: &Partition) {}

// if detect an nefs partition on GPT
// load the driver
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
