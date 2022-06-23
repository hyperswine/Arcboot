// ---------------------
// IMPORT
// ---------------------

use core::{
    marker::PhantomData,
    ops::{Add, RangeInclusive, Sub},
};

// ---------------------
// MMU
// ---------------------

pub trait MMU {
    /// Turns on the MMU for the first time and enables data and instruction caching
    unsafe fn enable_mmu_and_caching(
        &self,
        phys_tables_base_addr: Address<Physical>,
    ) -> Result<(), MMUEnableError>;

    /// Returns true if the MMU is enabled
    fn is_enabled(&self) -> bool;
}

#[derive(Debug)]
pub enum MMUEnableError {
    AlreadyEnabled,
    Other(&'static str),
}

// ---------------------
// Address & Translation
// ---------------------

/// Metadata trait for marking the type of an address
pub trait AddressType: Copy + Clone + PartialOrd + PartialEq + Ord + Eq {}

/// Zero-sized type to mark a physical address
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum Physical {}

/// Zero-sized type to mark a virtual address
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum Virtual {}

/// Generic address type
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Address<ATYPE: AddressType> {
    value: usize,
    _address_type: PhantomData<fn() -> ATYPE>,
}

// ---------------------
// ADDRESS TYPE
// ---------------------

impl AddressType for Physical {}
impl AddressType for Virtual {}

// * For KernelGranule, theres prob a better idea. Maybe #[cfg(aarch64)] use arm64::KernelGranule

impl<ATYPE: AddressType> Address<ATYPE> {
    /// Create an instance
    pub const fn new(value: usize) -> Self {
        Self {
            value,
            _address_type: PhantomData,
        }
    }

    /// Convert to usize
    pub const fn as_usize(self) -> usize {
        self.value
    }

    /// Align down to page size
    #[must_use]
    pub const fn align_down_page(self) -> Self {
        // * KernelGranule should actually be set by UEFI. E.g. 64 * 1024 for 64K to index into L3
        // 4 * 1024 for 4K
        let aligned = align_down(self.value, KernelGranule::SIZE);

        Self::new(aligned)
    }

    /// Align up to page size
    #[must_use]
    pub const fn align_up_page(self) -> Self {
        let aligned = align_up(self.value, KernelGranule::SIZE);

        Self::new(aligned)
    }

    /// Checks if the address is page aligned
    pub const fn is_page_aligned(&self) -> bool {
        is_aligned(self.value, KernelGranule::SIZE)
    }

    /// Return the address' offset into the corresponding page
    pub const fn offset_into_page(&self) -> usize {
        self.value & KernelGranule::MASK
    }
}

impl<ATYPE: AddressType> Add<usize> for Address<ATYPE> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: usize) -> Self::Output {
        match self.value.checked_add(rhs) {
            None => panic!("Overflow on Address::add"),
            Some(x) => Self::new(x),
        }
    }
}

impl<ATYPE: AddressType> Sub<Address<ATYPE>> for Address<ATYPE> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Address<ATYPE>) -> Self::Output {
        match self.value.checked_sub(rhs.value) {
            None => panic!("Overflow on Address::sub"),
            Some(x) => Self::new(x),
        }
    }
}

pub struct TranslationGranule<const GRANULE_SIZE: usize>;

// DEFAULT GRANULE SIZE
pub const DEFAULT_KERNEL_GRANULE_SIZE: usize = 4 * 1024;

pub type KernelGranule = TranslationGranule<DEFAULT_KERNEL_GRANULE_SIZE>;

impl<const GRANULE_SIZE: usize> TranslationGranule<GRANULE_SIZE> {
    pub const SIZE: usize = Self::size_checked();

    pub const MASK: usize = Self::SIZE - 1;

    /// The granule's left shift, basically log2(size)
    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(GRANULE_SIZE.is_power_of_two());

        GRANULE_SIZE
    }
}

/// Implemented by a BSP. ACPI could prob detect it somehow. All you really need is either the phys or virt space, normally 48
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

/// For top half kernel
pub trait AssociatedTranslationTable {
    /// A translation table with range [u64::MAX, (u64::MAX - AS_SIZE) + 1]
    type TableStartFromTop;

    /// A translation table with range [AS_SIZE - 1, 0]
    type TableStartFromBottom;
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

#[derive(Debug, Copy, Clone)]
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

// A microarch should implement this. And be detectable at runtime with ACPI MMIO ranges. Or just hardcoded for testing
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

// ---------------
// ALIGN FUNCTIONS
// ---------------

#[inline(always)]
pub const fn is_aligned(value: usize, alignment: usize) -> bool {
    assert!(alignment.is_power_of_two());

    (value & (alignment - 1)) == 0
}

/// Align down
#[inline(always)]
pub const fn align_down(value: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two());

    value & !(alignment - 1)
}

/// Align up
#[inline(always)]
pub const fn align_up(value: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two());

    (value + alignment - 1) & !(alignment - 1)
}

/// Convert a size into human readable format
pub const fn size_human_readable_ceil(size: usize) -> (usize, &'static str) {
    const KIB: usize = 1024;
    const MIB: usize = 1024 * 1024;
    const GIB: usize = 1024 * 1024 * 1024;

    if (size / GIB) > 0 {
        (size.div_ceil(GIB), "GiB")
    } else if (size / MIB) > 0 {
        (size.div_ceil(MIB), "MiB")
    } else if (size / KIB) > 0 {
        (size.div_ceil(KIB), "KiB")
    } else {
        (size, "Byte")
    }
}
