use core::ptr::NonNull;
use acpi::{AcpiHandler, PhysicalMapping};

#[derive(Debug, Clone, Copy)]
pub struct AcpiHandle;

impl AcpiHandler for AcpiHandle {
    // create a physical mapping object
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        info!("Mapping a region!");
        let va = NonNull::new(physical_address as *mut T).unwrap();
        PhysicalMapping::new(physical_address, va, size, size, Self)
    }

    fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>) {
        info!("Unmapping a region!");
        let handler = region.handler();
    }
}
