use acpi::{AcpiHandler, PhysicalMapping, AcpiTables};
use core::{intrinsics::size_of, ptr::NonNull, ffi::c_void};
use spin::Mutex;
use uefi::{
    prelude::*,
    proto::console::{serial::Serial, text::Output},
    table::{
        boot::{OpenProtocolAttributes, OpenProtocolParams},
        cfg::{self, ConfigTableEntry},
        runtime::VariableVendor,
        Runtime,
    },
    Guid,
};

#[derive(Debug, Clone, Copy)]
pub struct AcpiHandle;

use lazy_static::lazy_static;

// maybe make it 0x4000_0000 basically 1GB, or 3GB
lazy_static! {
    static ref CURRENT_VIRTUAL_ADDRESS: Mutex<usize> = Mutex::new(0x4000_0000);
}

impl AcpiHandler for AcpiHandle {
    // Create a physical mapping object
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        // Add size to virtual address each time. Problem is when you unmap it, you cant reuse it
        let mut curr = CURRENT_VIRTUAL_ADDRESS.lock();
        *curr += size;
        let va = NonNull::new(*curr as *mut T).unwrap();

        info!("Mapping a region! Physical addr = {physical_address:#04X}. Size = {size:#04X}");
        info!("Virtual addr = {:#04X}", *curr);

        // exception here...
        PhysicalMapping::new(physical_address, va, size_of::<T>(), size, Self)
    }

    // Does nothing. In reality you prob should dealloc the pages in the page tables (TTBR0 for UEFI)
    fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>) {
        info!("Unmapping a region!");
        info!(
            "Region: phys start = {:#04X}, virt start = {:?}, phys length = {:#04X}, mapped length = {:#04X}",
            region.physical_start(),
            region.virtual_start(),
            region.region_length(),
            region.mapped_length()
        );

        let handler = region.handler();
    }
}

pub fn get_acpi_tables(rt: &RuntimeServices, config_table: &[ConfigTableEntry]) {
    let res = rt.variable_keys().unwrap();

    let rsdp = cfg::ACPI2_GUID;
    let rsdp_v1 = cfg::ACPI_GUID;

    // Find value of RSDP v1
    let res = config_table.iter().find(|c| c.guid == rsdp_v1);

    let rsdp_v1 = match res {
        Some(r) => {
            info!("Found RSDP v1 entry, at addr = {:p}", r.address);
            r.address
        }
        None => {
            info!("Couldn't find RSDP in the ACPI v1 table... trying v2");
            0 as *const c_void
        }
    };

    // Find the value of RSDP v2. Always prioritise it if RSDP v1 is found
    let res = config_table.iter().find(|c| c.guid == rsdp);

    let rsdp = match res {
        Some(r) => {
            info!("Found RSDP v2 entry, at addr = {:p}", r.address);
            r.address
        }
        None => {
            panic!("Couldn't find RSDP in the ACPI v2 table! Uh oh! Are you sure you set it up properly?")
        }
    };

    // set the virtual address map
    // you already have the prev one. Just reclaim the bootservices space as CONVENTIONAL
    // maybe thats the problem

    // but doesnt explain the weird virt_start and phys_start values
    // maybe virtual memory isnt "turned on" or something

    fn get_table(rsdp: *const c_void) {
        let handler = AcpiHandle {};

        let res = unsafe { AcpiTables::from_rsdp(handler, rsdp as usize) };
        match res {
            Ok(r) => {
                let p = r.platform_info().unwrap();
                info!("Power profile = {:?}", p.power_profile);
                info!("Interrupt model = {:?}", p.interrupt_model);
                info!(
                    "Boot Processor Info = {:?}",
                    &p.processor_info.as_ref().unwrap().boot_processor
                );
                info!(
                    "Application Processor Info = {:?}",
                    &p.processor_info.unwrap().application_processors
                );
                info!("PM Timer = {:?}", p.pm_timer.unwrap().base);
            }
            Err(err) => {
                panic!("Couldn't get ACPI tables. Maybe the handler wasnt set up properly?")
            }
        }
    }

    get_table(rsdp);
}
