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

// Maybe wrap around ACPI.. I think its a good idea

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

pub struct ArcMemory {}

#[repr(C)]
pub struct ArcServices {
    paging: PageTableTTBR1,
    devices: Vec<ArcDevice>,
    memory_map: MemoryMap,
    interrupts: ArcInterrupts,
}

pub type InterruptHandler = fn();

pub struct InterruptArm64 {
    vector_table_start: u64,
}

pub struct ArcInterrupts {
    arm64: InterruptArm64,
}

impl ArcInterrupts {
    pub fn new(arm64: InterruptArm64) -> Self {
        Self { arm64 }
    }
}

impl ArcServices {
    pub fn new(
        paging: PageTableTTBR1,
        devices: Vec<ArcDevice>,
        memory_map: MemoryMap,
        interrupts: ArcInterrupts,
    ) -> Self {
        Self {
            paging,
            devices,
            memory_map,
            interrupts,
        }
    }

    pub fn register_interrupt_handler(&mut self, interrupt_id: u64, handler: InterruptHandler) {
        // cfg aarch64, use the vector table
    }
}

// arcboot can allow you to register interrupt handlers
// but since every arch is kinda different
// we can maybe try to abstract that into sync, async interrupt handlers
// and a specific

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

/// Set the stack pointer at a certain location. Which should have been mapped already
pub fn set_stack(vaddr: u64) {
    // should be 8-byte aligned
    if vaddr % 8 != 0 {
        panic!("set_stack not 8 byte aligned!");
    }

    // aarch64, set sp
    #[cfg(target_arch = "aarch64")]
    {
        use cortex_a::registers::SP;
        SP.set(vaddr)
    }
}

/// Map a 4K aligned segment to somewhere in TTBR1 VA space
/// Usually copies from boot stack/heap -> address
pub fn map_segment(segment: &[u8], vaddr: u64) {}

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
