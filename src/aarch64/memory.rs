//-------------------
// IMPORT
//-------------------

use crate::memory::mmu::{AddressSpace, TranslationGranule, MMUEnableError, MMU};
use core::intrinsics::unlikely;
use cortex_a::{asm::barrier, registers::*};
use tock_registers::{interfaces::{ReadWriteable, Readable, Writeable}, register_bitfields};

// ------------------
// DESCRIPTORS
// ------------------

// so if 4 levels (armv8-a v0), then there will be 3 of them that use the table descriptor
// for 64K though, we only need 3 levels. I think this is 64K by default or something

// Table descriptor for ARMv8-A (L1 & L2 64K) or (L0-L2 4K)
register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        /// Physical address of the next descriptor
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

// Last level descriptor for ARMv8-A
register_bitfields! {u64,
    STAGE1_PAGE_DESCRIPTOR [
        /// Unprivileged execute-never
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Physical address of the next table descriptor (lvl2) or the page descriptor (lvl3)
        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [],

        /// Access Flag
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareable
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        /// Memory attributes index into MAIR_EL1
        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

/// A table descriptor for 64 KiB output
#[derive(Copy, Clone)]
#[repr(C)]
struct TableDescriptor {
    value: u64,
}

/// A page descriptor with 64 KiB output
#[derive(Copy, Clone)]
#[repr(C)]
struct PageDescriptor {
    value: u64,
}

trait StartAddr {
    fn phys_start_addr_u64(&self) -> u64;
    fn phys_start_addr_usize(&self) -> usize;
}

const NUM_LVL2_TABLES: usize = KernelAddrSpace::SIZE >> Granule512MiB::SHIFT;

//-------------------
// PAGING & MMU IMPL
//-------------------

/// Memory Management Unit type.
struct MemoryManagementUnit;

const PAGE_TABLES_START: u64 = 0x3B9ACA00;

pub type Granule512MiB = TranslationGranule<{ 512 * 1024 * 1024 }>;
pub type Granule64KiB = TranslationGranule<{ 64 * 1024 }>;

/// Constants for indexing the MAIR_EL1.
#[allow(dead_code)]
pub mod mair {
    pub const DEVICE: u64 = 0;
    pub const NORMAL: u64 = 1;
}

static mut KERNEL_TABLES: KernelTranslationTable = KernelTranslationTable::new();

static MMU: MemoryManagementUnit = MemoryManagementUnit;

impl<const AS_SIZE: usize> AddressSpace<AS_SIZE> {
    /// Checks for architectural restrictions.
    pub const fn arch_address_space_size_sanity_checks() {
        // Size must be at least one full 512 MiB table.
        assert!((AS_SIZE % Granule512MiB::SIZE) == 0);

        // Check for 48 bit virtual address size as maximum, which is supported by any ARMv8
        // version.
        assert!(AS_SIZE <= (1 << 48));
    }
}

impl MemoryManagementUnit {
    /// Setup function for the MAIR_EL1 register.
    fn set_up_mair(&self) {
        // Define the memory types being mapped.
        MAIR_EL1.write(
            // Attribute 1 - Cacheable normal DRAM.
            MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc +
        MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc +

        // Attribute 0 - Device.
        MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck,
        );
    }

    /// Configure various settings of stage 1 of the EL1 translation regime.
    fn configure_translation_control(&self) {
        let t0sz = (64 - KernelAddrSpace::SIZE_SHIFT) as u64;

        TCR_EL1.write(
            TCR_EL1::TBI0::Used
                + TCR_EL1::IPS::Bits_40
                + TCR_EL1::TG0::KiB_64
                + TCR_EL1::SH0::Inner
                + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::EPD0::EnableTTBR0Walks
                + TCR_EL1::A1::TTBR0
                + TCR_EL1::T0SZ.val(t0sz)
                + TCR_EL1::EPD1::DisableTTBR1Walks,
        );
    }
}

/// Return a reference to the MMU instance.
pub fn mmu() -> &'static impl MMU {
    &MMU
}

impl MMU for MemoryManagementUnit {
    unsafe fn enable_mmu_and_caching(&self) -> Result<(), MMUEnableError> {
        if unlikely(self.is_enabled()) {
            return Err(MMUEnableError::AlreadyEnabled);
        }

        // Fail early if translation granule is not supported.
        if unlikely(!ID_AA64MMFR0_EL1.matches_all(ID_AA64MMFR0_EL1::TGran64::Supported)) {
            return Err(MMUEnableError::Other(
                "Translation granule not supported in HW",
            ));
        }

        // Prepare the memory attribute indirection register.
        self.set_up_mair();

        // Populate translation tables.
        KERNEL_TABLES
            .populate_tt_entries()
            .map_err(MMUEnableError::Other)?;

        // Set the "Translation Table Base Register".
        TTBR0_EL1.set_baddr(KERNEL_TABLES.phys_base_address());

        self.configure_translation_control();

        // Switch the MMU on.
        //
        // First, force all previous changes to be seen before the MMU is enabled.
        barrier::isb(barrier::SY);

        // Enable the MMU and turn on data and instruction caching.
        SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

        // Force MMU init to complete before next instruction.
        barrier::isb(barrier::SY);

        Ok(())
    }

    #[inline(always)]
    fn is_enabled(&self) -> bool {
        SCTLR_EL1.matches_all(SCTLR_EL1::M::Enable)
    }
}