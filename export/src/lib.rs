#![no_std]

pub enum DeviceType {
    USBController,
    PCIeController,
    MainProcessor,
    DRAM,
    /// ROM, not mass storage (usb/pcie)
    FlashMemory
}
pub struct PageTableTTBR1;

pub struct ArcServices {
    paging: PageTableTTBR1,
    devices: &'static [ArcDevice],
}

pub struct ArcDevice {
    device_type: DeviceType,
    numa_id: usize,
}

#[test]
fn test_basics() {
    assert_eq!(1, 1)
}
