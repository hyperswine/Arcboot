pub mod memory_map {
    pub const END_INCLUSIVE: usize = 0xFFFF_FFFF;
    pub const GPIO_OFFSET: usize = 0x0020_0000;
    pub const UART_OFFSET: usize = 0x0020_1000;

    pub const MMIO_START: usize = 0xFE00_0000;
    pub const MMIO_GPIO_START: usize = START + GPIO_OFFSET;
    pub const MMIO_PL011_UART_START: usize = START + UART_OFFSET;
    pub const MMIO_END_INCLUSIVE: usize = 0xFF84_FFFF;
}

const NUM_MEM_RANGES: usize = 3;

pub type KernelAddrSpace = AddressSpace<{ memory_map::END_INCLUSIVE + 1 }>;

// COULD BE BETTER DONE WITH A TRAIT. ALSO UEFI ALREADY HAS A MEMORY MAP

/// The virtual memory layout
pub static LAYOUT: KernelVirtualLayout<NUM_MEM_RANGES> = KernelVirtualLayout::new(
    memory_map::END_INCLUSIVE,
    [
        TranslationDescriptor {
            name: "Kernel code and RO data",
            virtual_range: code_range_inclusive,
            physical_range_translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::CacheableDRAM,
                acc_perms: AccessPermissions::ReadOnly,
                execute_never: false,
            },
        },
        TranslationDescriptor {
            name: "Remapped Device MMIO",
            virtual_range: remapped_mmio_range_inclusive,
            physical_range_translation: Translation::Offset(memory_map::mmio::START + 0x20_0000),
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            },
        },
        TranslationDescriptor {
            name: "Device MMIO",
            virtual_range: mmio_range_inclusive,
            physical_range_translation: Translation::Identity,
            attribute_fields: AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            },
        },
    ],
);

use core::ops::RangeInclusive;

fn code_range_inclusive() -> RangeInclusive<usize> {
    // Notice the subtraction to turn the exclusive end into an inclusive end
    #[allow(clippy::range_minus_one)]
    RangeInclusive::new(super::code_start(), super::code_end_exclusive() - 1)
}

fn remapped_mmio_range_inclusive() -> RangeInclusive<usize> {
    // The last 64 KiB slot in the first 512 MiB
    RangeInclusive::new(0x1FFF_0000, 0x1FFF_FFFF)
}

fn mmio_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(memory_map::mmio::START, memory_map::mmio::END_INCLUSIVE)
}

/// Return a reference to the virtual memory layout
pub fn virt_mem_layout() -> &'static KernelVirtualLayout<NUM_MEM_RANGES> {
    &LAYOUT
}
