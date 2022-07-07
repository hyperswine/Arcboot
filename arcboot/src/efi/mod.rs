use spin::Mutex;
use uefi::{
    prelude::BootServices,
    table::{runtime::ResetType, Runtime, SystemTable},
    Status,
};

// Contains the startup boot code (and tests)
pub mod acpi;
pub mod boot;
pub mod proto;
pub mod runtime;

pub fn check_revision(rev: uefi::table::Revision) {
    let (major, minor) = (rev.major(), rev.minor());

    info!("UEFI {}.{}", major, minor / 10);

    assert!(major >= 2, "Running on an old, unsupported version of UEFI");
    assert!(
        minor >= 30,
        "Old version of UEFI 2, some features might not be available."
    );
}

pub fn shutdown(mut st: SystemTable<Runtime>) -> ! {
    // Shut down the system
    let rt = unsafe { st.runtime_services() };
    rt.reset(ResetType::Shutdown, Status::SUCCESS, None);
}

// maximum bytes (1248 or 1348 or something)
pub const MAX_MEMORY_MAP_SIZE: usize = 1400;

/// Array of bytes aligned to 8 bytes
#[repr(C, align(8))]
pub struct AlignToMemoryDescriptor([u8; MAX_MEMORY_MAP_SIZE]);

use lazy_static::lazy_static;

pub fn get_mem_map(bt: &BootServices) -> AlignToMemoryDescriptor {
    let mut memory_map = AlignToMemoryDescriptor {
        0: [0 as u8; MAX_MEMORY_MAP_SIZE],
    };
    {
        let res = bt.memory_map(&mut memory_map.0);

        match res {
            Ok(r) => {
                info!("Retrieved Memory Map!");
                info!("Memory Map Key = {:?}", &r.0);
                let iterator_item = r.1;
                iterator_item.for_each(|i| info!("Memory Descriptor = {i:?}"));
            }
            Err(err) => panic!("Could not get memory map. Error: {err:?}"),
        }
    }

    // for aarch64, generally
    // 0x0 - 0x400_0000 is MMIO
    // BFFF_C000 - 0xBFFF_D000 is MMIO
    // 0x4000_0000 + 492632 * 4K is conventional
    // no MMIO anymore?? maybe cause of different heap

    // 1GB-3GB ('phys') is DRAM
    // maybe it means vaddr with a 1GB offset

    memory_map
}
