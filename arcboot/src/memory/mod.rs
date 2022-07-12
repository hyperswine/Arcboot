use tock_registers::interfaces::Writeable;

#[cfg(feature = "builtin_allocator")]
pub mod heap;
pub mod mmu;

// ARC MEMORY PROTOCOL

/// Map a 4K aligned segment to somewhere in TTBR1 VA space
/// Usually copies from boot stack/heap -> address
pub fn map_segment(segment: &[u8], vaddr: u64) {}

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
