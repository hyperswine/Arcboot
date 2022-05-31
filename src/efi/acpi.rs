use acpi::{AcpiHandler, PhysicalMapping};

// could prob impl acpihandler for paging

#[derive(Debug, Clone, Copy)]
pub struct AcpiHandle {}

// impl AcpiHandler for AcpiHandle {
//     // create a physical mapping object
//     unsafe fn map_physical_region<T>(
//         &self,
//         physical_address: usize,
//         size: usize,
//     ) -> acpi::PhysicalMapping<Self, T> {
//         PhysicalMapping::new(physical_start, virtual_start, region_length, mapped_length, *self)
//     }

//     fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>) {
//         todo!()
//     }
// }
