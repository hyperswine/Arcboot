use alloc::vec::Vec;
use uefi::table::boot::MemoryDescriptor;
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

// -----------------
// USEFUL FUNCTIONS
// -----------------

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

// -----------------
// MEMORY MAP
// -----------------

/// Maximum bytes to store UEFI memory map (1248..1348 usually)
pub const MAX_MEMORY_MAP_SIZE: usize = 1400;

/// Array of bytes aligned to 8 bytes
#[repr(C, align(8))]
pub struct AlignToMemoryDescriptor([u8; MAX_MEMORY_MAP_SIZE]);

pub type MemoryMapEFI = Vec<MemoryDescriptor>;

pub fn get_mem_map(bt: &BootServices) -> MemoryMapEFI {
    let mut memory_map = AlignToMemoryDescriptor {
        0: [0 as u8; MAX_MEMORY_MAP_SIZE],
    };

    let res = bt.memory_map(&mut memory_map.0);

    let r_map = match res {
        Ok(r) => {
            info!("Retrieved Memory Map!");
            info!("Memory Map Key = {:?}", &r.0);

            let iterator_item = r.1.clone();
            iterator_item.for_each(|i| info!("Memory Descriptor = {i:?}"));

            r.1
        }
        Err(err) => panic!("Could not get memory map. Error: {err:?}"),
    };

    let m: Vec<MemoryDescriptor> = r_map.copied().collect();

    m
}

/// Export ArcMemory
pub fn create_arc_memory_from_uefi(mem_map: &MemoryMapEFI) {
    for mem_region in mem_map {
        
    }
}
