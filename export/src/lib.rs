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

#[repr(C)]
pub struct PageTableTTBR1;

#[repr(C)]
pub struct MemoryRegion;

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
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            memory_regions: Default::default(),
        }
    }
}

#[repr(C)]
pub struct ArcServices {
    paging: PageTableTTBR1,
    devices: Vec<ArcDevice>,
    memory_map: MemoryMap,
}

impl ArcServices {
    pub fn new(paging: PageTableTTBR1, devices: Vec<ArcDevice>, memory_map: MemoryMap) -> Self {
        Self {
            paging,
            devices,
            memory_map,
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
    let service = ArcServices::new(PageTableTTBR1 {}, devices, memory_map);

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
