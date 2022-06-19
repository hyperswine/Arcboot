use acpi::{AcpiHandler, PhysicalMapping};
use core::{intrinsics::size_of, ptr::NonNull};

#[derive(Debug, Clone, Copy)]
pub struct AcpiHandle;

// Identity mapped pages => 1GB offset. So begins at 0x4000_0000 - 0xB847C000
// This chunk is 2GB (0x7847C000) in size

// maybe keep a Vec of regions here
// and always make virt start = 0

// Identity mapped virtual address space
// When UEFI/ACPI doesnt need it anymore, it expects the virtual address to be usable
// I think we can simulate an MMU for now
// pub struct IdentityMapping {
//     // originally, 48bit, so like
//     free_virtual_pages: Vec<u64>,
// }

impl AcpiHandler for AcpiHandle {
    // create a physical mapping object
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        // region_length can just be size_of<T>
        info!("Mapping a region! Physical addr = {physical_address}. Size = {size}");
        info!(
            "Virtual addr = {}. Region length = {}",
            physical_address - 0x4000_0000,
            size_of::<T>()
        );
        // phys_addr - 0x4000_0000 doesnt crash sometimes, but doesnt find the tables either
        // i have no idea, I think + 0x4000_0000 isnt correct because thats phys addr for some reason
        let va = NonNull::new((physical_address - 0x4000_0000) as *mut T).unwrap();

        // ! for now
        let va = NonNull::new(0x0 as *mut T).unwrap();
        info!(
            "Actual virtual addr = {}. Region length = {}",
            0,
            size_of::<T>()
        );

        PhysicalMapping::new(physical_address, va, size_of::<T>(), size, Self)
    }

    fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>) {
        info!("Unmapping a region!");
        info!(
            "Region: phys start = {}, virt start = {:?}, phys length = {}, mapped length = {}",
            region.physical_start(),
            region.virtual_start(),
            region.region_length(),
            region.mapped_length()
        );

        let handler = region.handler();
    }
}
