#![no_std]

// ---------------
// ARCBOOT API
// ---------------

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

#[repr(C)]
pub struct ArcServices<const D: usize, const M: usize> {
    paging: PageTableTTBR1,
    devices: [ArcDevice; D],
    memory_map: [MemoryRegion; M],
}

impl<const D: usize, const M: usize> ArcServices<D, M> {
    pub fn new(
        paging: PageTableTTBR1,
        devices: [ArcDevice; D],
        memory_map: [MemoryRegion; M],
    ) -> Self {
        Self {
            paging,
            devices,
            memory_map,
        }
    }
}

// DEFAULTS

// for testing

/// Should be called by arcboot to make the structures and passed to neutron entry
pub fn make_default() {
    let device = ArcDevice::new(DeviceType::DRAM, 0);
    let devices = [device];
    let memory_map = [MemoryRegion {}];
    let service = ArcServices::new(PageTableTTBR1 {}, devices, memory_map);
}

// ---------------
// TESTS
// ---------------

#[test]
fn test_basics() {
    assert_eq!(1, 1)
}
