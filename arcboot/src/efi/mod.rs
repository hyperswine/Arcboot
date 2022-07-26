use alloc::vec;
use alloc::vec::Vec;
use arcboot_api::{address_range_4k, MemoryMap, MemoryRegion, MemoryRegionType};
use uefi::table::boot::{MemoryDescriptor, MemoryType};
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

/// Physical RAM Regions that can be directly mapped as Standard Pages
pub const EFI_FREE_MEMORY_REGIONS: &[MemoryType] = &[
    MemoryType::LOADER_CODE,
    MemoryType::LOADER_DATA,
    MemoryType::BOOT_SERVICES_CODE,
    MemoryType::BOOT_SERVICES_DATA,
    MemoryType::CONVENTIONAL,
    MemoryType::PERSISTENT_MEMORY,
];

/// Usable as Standard Pages after retrieving ACPI Tables and all of its data
pub const EFI_RECLAIMABLE_MEMORY_REGIONS: &[MemoryType] =
    &[MemoryType::ACPI_RECLAIM, MemoryType::ACPI_NON_VOLATILE];

/// Memory Regions that shouldn't be used directly, but mapped by AllocateMemory() I think
pub const EFI_ACPI_S_STATE_REGIONS: &[MemoryType] = &[
    MemoryType::RUNTIME_SERVICES_CODE,
    MemoryType::RUNTIME_SERVICES_DATA,
];

/// RAM Ranges that are hooked onto by the MMIO Controller, and need to be marked as pass-through, non-cacheable, with strong ordering, no buffering
pub const EFI_MMIO_MEMORY_REGIONS: &[MemoryType] = &[MemoryType::MMIO_PORT_SPACE, MemoryType::MMIO];

/// Export ArcMemory. NOTE: you should enable ACPI and retrieve the tables, and esp the MMIO APIs before using the regions
pub fn create_arc_memory_from_uefi(mem_map: &MemoryMapEFI) -> MemoryMap {
    let mut arc_memory_map = MemoryMap::new(vec![]);

    for mem_region in mem_map {
        // based on the type, make the MemoryRegion and add it to the map
        let res = if EFI_FREE_MEMORY_REGIONS.contains(&mem_region.ty)
            || EFI_FREE_MEMORY_REGIONS.contains(&mem_region.ty)
        {
            MemoryRegion::new(
                MemoryRegionType::Standard,
                address_range_4k(mem_region.phys_start, mem_region.page_count),
            )
        } else if EFI_ACPI_S_STATE_REGIONS.contains(&mem_region.ty) {
            MemoryRegion::new(
                MemoryRegionType::ACPI,
                address_range_4k(mem_region.phys_start, mem_region.page_count),
            )
        } else {
            MemoryRegion::new(
                MemoryRegionType::Standard,
                address_range_4k(mem_region.phys_start, mem_region.page_count),
            )
        };

        arc_memory_map.push(res);
    }

    arc_memory_map
}
