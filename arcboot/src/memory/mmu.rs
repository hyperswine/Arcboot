// --------------
// IMPORT
// --------------

use core::ops::RangeInclusive;

// --------------
// MMU
// --------------

pub trait MMU {
    /// Called during init. Supposed to take the translation tables from the
    /// `BSP`-supplied `virt_mem_layout()` => impl and call it for its MMU
    unsafe fn enable_mmu_and_caching(&self) -> Result<(), MMUEnableError>;

    /// Returns true if the MMU is enabled, false otherwise
    fn is_enabled(&self) -> bool;
}

#[derive(Debug)]
pub enum MMUEnableError {
    AlreadyEnabled,
    Other(&'static str),
}

// --------------
// Address & Translation
// --------------

pub struct TranslationGranule<const GRANULE_SIZE: usize>;

impl<const GRANULE_SIZE: usize> TranslationGranule<GRANULE_SIZE> {
    pub const SIZE: usize = Self::size_checked();

    /// The granule's shift, basically log2(size)
    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(GRANULE_SIZE.is_power_of_two());

        GRANULE_SIZE
    }
}

/// Implemented by a BSP. ACPI could prob detect it somehow
pub struct AddressSpace<const AS_SIZE: usize>;

impl<const AS_SIZE: usize> AddressSpace<AS_SIZE> {
    /// The address space size
    pub const SIZE: usize = Self::size_checked();

    /// The 'shift', basically log2(size)
    pub const SIZE_SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(AS_SIZE.is_power_of_two());

        // Check for architectural restrictions as well
        // this should be a trait
        // Self::arch_address_space_size_sanity_checks();

        AS_SIZE
    }
}

#[derive(Copy, Clone)]
pub enum Translation {
    Identity,
    Offset(usize),
}

// A64 attributes. Technically would be device nGnRnE (non gatherable, reorderable, early writable)
#[derive(Copy, Clone)]
pub enum MemAttributes {
    CacheableDRAM,
    Device,
}

#[derive(Copy, Clone)]
pub enum AccessPermissions {
    ReadOnly,
    ReadWrite,
}

#[derive(Copy, Clone)]
pub struct AttributeFields {
    pub mem_attributes: MemAttributes,
    pub acc_perms: AccessPermissions,
    pub execute_never: bool,
}

impl Default for AttributeFields {
    fn default() -> AttributeFields {
        AttributeFields {
            mem_attributes: MemAttributes::CacheableDRAM,
            acc_perms: AccessPermissions::ReadWrite,
            execute_never: true,
        }
    }
}

/// Architecture agnostic descriptor for a memory range
#[allow(missing_docs)]
pub struct TranslationDescriptor {
    pub name: &'static str,
    pub virtual_range: fn() -> RangeInclusive<usize>,
    pub physical_range_translation: Translation,
    pub attribute_fields: AttributeFields,
}

/// Type for expressing the kernel's virtual memory layout
pub struct KernelVirtualLayout<const NUM_SPECIAL_RANGES: usize> {
    max_virt_addr_inclusive: usize,

    /// Descriptors for non-cacheable memory regions
    inner: [TranslationDescriptor; NUM_SPECIAL_RANGES],
}

// A microarch should implement this. And be detectable at runtime with ACPI MMIO ranges
// Or just hardcoded for testing
impl<const NUM_SPECIAL_RANGES: usize> KernelVirtualLayout<{ NUM_SPECIAL_RANGES }> {
    pub const fn new(max: usize, layout: [TranslationDescriptor; NUM_SPECIAL_RANGES]) -> Self {
        Self {
            max_virt_addr_inclusive: max,
            inner: layout,
        }
    }

    /// For a virtual address, find and return the physical output address and attributes
    pub fn virt_addr_properties(
        &self,
        virt_addr: usize,
    ) -> Result<(usize, AttributeFields), &'static str> {
        if virt_addr > self.max_virt_addr_inclusive {
            return Err("Address out of range");
        }

        for i in self.inner.iter() {
            if (i.virtual_range)().contains(&virt_addr) {
                let output_addr = match i.physical_range_translation {
                    Translation::Identity => virt_addr,
                    Translation::Offset(a) => a + (virt_addr - (i.virtual_range)().start()),
                };

                return Ok((output_addr, i.attribute_fields));
            }
        }

        Ok((virt_addr, AttributeFields::default()))
    }
}
