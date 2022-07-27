#![no_std]

extern crate alloc;
use alloc::vec;

// ---------------
// ARCBOOT API
// ---------------

use alloc::vec::Vec;

#[repr(C)]
pub enum DeviceType {
    USBController,
    PCIeController,
    MainProcessor,
    DRAM,
    /// ROM, not mass storage (usb/pcie)
    FlashMemory,
    Unknown,
}

// Neutron doesnt have to touch the registers directly,
// instead it can use PTTTBR1.set(), .read(), etc to do it

#[repr(C)]
pub struct PageTableTTBR1 {
    phys_addr_base: u64,
}

impl PageTableTTBR1 {
    pub fn new(phys_addr_base: u64) -> Self {
        Self { phys_addr_base }
    }
}

/// For ArcAPI only. When exposing to userspace (rust std), use neutron memory regions
pub enum MemoryRegionType {
    Standard,
    MMIO,
    ACPI,
}

pub type AddressRange = (u64, u64);

/// Get the address range (4K granule)
pub fn address_range_4k(start_addr: u64, n_pages: u64) -> AddressRange {
    let res = (start_addr, start_addr + n_pages * 4096);

    res
}

#[repr(C)]
pub struct MemoryRegion {
    region_type: MemoryRegionType,
    address_range: AddressRange,
}

impl MemoryRegion {
    pub fn new(region_type: MemoryRegionType, address_range: AddressRange) -> Self {
        Self {
            region_type,
            address_range,
        }
    }
}

#[repr(C)]
pub struct ArcDevice {
    device_type: DeviceType,
    numa_id: usize,
}

impl ArcDevice {
    pub fn new(device_type: DeviceType, numa_id: usize) -> Self {
        Self {
            device_type,
            numa_id,
        }
    }
}

pub struct MemoryMap {
    memory_regions: Vec<MemoryRegion>,
}

impl MemoryMap {
    pub fn new(memory_regions: Vec<MemoryRegion>) -> Self {
        Self { memory_regions }
    }

    pub fn push(&mut self, memory_region: MemoryRegion) {
        self.memory_regions.push(memory_region);
    }
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            memory_regions: Default::default(),
        }
    }
}

// ? maybe instead of a direct memory map, you set it up in a higher abstract way
// cause you already setup paging and interrupt vectors, neutron just has to
// use it somehow, and it can ditch if it wants

#[repr(C)]
pub struct ArcServices {
    paging: PageTableTTBR1,
    devices: Vec<ArcDevice>,
    memory_map: MemoryMap,
    interrupt_handler_addr_start: u64,
}

impl ArcServices {
    pub fn new(
        paging: PageTableTTBR1,
        devices: Vec<ArcDevice>,
        memory_map: MemoryMap,
        interrupt_handler_addr_start: u64,
    ) -> Self {
        Self {
            paging,
            devices,
            memory_map,
            interrupt_handler_addr_start,
        }
    }
}

// DEFAULTS

pub type DefaultServices = ArcServices;

// for testing

/// Should be called by arcboot to make the structures and passed to neutron entry
pub fn make_default() -> DefaultServices {
    let device = ArcDevice::new(DeviceType::DRAM, 0);
    let devices = vec![device];
    let memory_map = MemoryMap::default();
    let service = ArcServices::new(PageTableTTBR1::new(0x4000_0000), devices, memory_map, 0x0);

    service
}

// ---------------
// TESTS
// ---------------

#[test]
fn basics() {
    assert_eq!(1, 1)
}

#[test]
fn make_default_arc_services() {
    make_default();
}
