//-------------------
// IMPORT
//-------------------

use crate::memory::mmu::{AddressSpace, MMUEnableError, TranslationGranule, MMU, AssociatedTranslationTable, Address, Physical};
use core::intrinsics::unlikely;
use cortex_a::{asm::barrier, registers::*};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields,
};

// ------------------
// DESCRIPTORS
// ------------------

// ? Uhh, so if using 4K instead of 64K, what happens? Just change a few vars and stuff and use 4 level paging?

// Table descriptor for ARMv8-A (L1 & L2 64K) or (L0-L2 4K)
register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        /// Physical address of the next descriptor [47:16]
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [],
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

// ? Why change number of L2 tables?? Thats the one after L3
const NUM_LVL2_TABLES: usize = KernelVirtAddrSpace::SIZE >> Granule512MiB::SHIFT;

// 1 GiB worth of pages. L0 just refers to the actual frame and you add the offset to it
pub const DEFAULT_KERNEL_VIRT_ADDR_SPACE_SIZE: usize = 1024 * 1024 * 1024;

pub type KernelVirtAddrSpace = AddressSpace<DEFAULT_KERNEL_VIRT_ADDR_SPACE_SIZE>;

//-------------------
// PAGING & MMU IMPL
//-------------------

/// Memory Management Unit type. Could prob be created with a kernel shift
struct MemoryManagementUnit;

const PAGE_TABLES_START: u64 = 0x3B9ACA00;

pub type Granule512MiB = TranslationGranule<{ 512 * 1024 * 1024 }>;
pub type Granule64KiB = TranslationGranule<{ 64 * 1024 }>;

/// Constants for indexing the MAIR_EL1
#[allow(dead_code)]
pub mod mair {
    pub const DEVICE: u64 = 0;
    pub const NORMAL: u64 = 1;
}

/// TTBR0 EL1 at first. But should be made into TTBR1 somehow...
static mut KERNEL_TABLES: KernelTranslationTable = KernelTranslationTable::new();

/// Always only one MMU. Which represents the entire system
static MMU: MemoryManagementUnit = MemoryManagementUnit;

impl<const AS_SIZE: usize> AddressSpace<AS_SIZE> {
    /// Checks for architectural restrictions
    pub const fn arch_address_space_size_sanity_checks() {
        // Size must be at least one full 512 MiB table
        assert!((AS_SIZE % Granule512MiB::SIZE) == 0);

        // Check for 48 bit virtual address size as maximum, which is supported by any ARMv8
        assert!(AS_SIZE <= (1 << 48));
    }
}

impl MemoryManagementUnit {
    /// Setup MAIR_EL1 for memory types that exist
    fn set_up_mair(&self) {
        // Define the memory types being mapped. Attribute 1 - Cacheable normal DRAM. Attribute 0 - Device
        MAIR_EL1.write(
            MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck,
        );
    }

    /// Configure stage 1 of EL1 translation (TTBR1) for KERNEL
    fn configure_translation_control(&self, kernel_addr_space_log2: i32) {
        let t1sz = (64 - KernelVirtAddrSpace::SIZE_SHIFT) as u64;

        TCR_EL1.write(
            TCR_EL1::TBI1::Used
                + TCR_EL1::IPS::Bits_40
                + TCR_EL1::TG1::KiB_64
                + TCR_EL1::SH1::Inner
                + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::EPD1::EnableTTBR1Walks
                + TCR_EL1::A1::TTBR1
                + TCR_EL1::T1SZ.val(t1sz)
                + TCR_EL1::EPD0::DisableTTBR0Walks,
        );
    }
}

/// Static MMU instance
pub fn mmu() -> &'static impl MMU {
    &MMU
}

impl MMU for MemoryManagementUnit {
    unsafe fn enable_mmu_and_caching(
        &self,
        phys_tables_base_addr: Address<Physical>,
    ) -> Result<(), MMUEnableError> {
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

        // Set the "Translation Table Base Register".
        TTBR1_EL1.set_baddr(phys_tables_base_addr.as_usize() as u64);

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

/// Big monolithic struct for storing the translation tables. Individual levels must be 64 KiB aligned, so the lvl3 is put first
#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_TABLES: usize> {
    /// Page descriptors, covering 64 KiB windows per entry
    lvl3: [[PageDescriptor; 8192]; NUM_TABLES],

    /// Table descriptors, covering 512 MiB windows
    lvl2: [TableDescriptor; NUM_TABLES],
}

/// A translation table type for the kernel space
pub type KernelTranslationTable =
    <KernelVirtAddrSpace as AssociatedTranslationTable>::TableStartFromTop;

// -----------------
// UEFI INTERGRATION
// -----------------

// so i guess we somehow use UEFI
