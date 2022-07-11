#[cfg(feature = "builtin_allocator")]
pub mod heap;
pub mod mmu;

// ARC MEMORY PROTOCOL

/// Map a 4K aligned segment to somewhere in TTBR1 VA space
/// Usually copies from boot stack/heap -> address
pub fn map_segment(segment: &[u8], vaddr: u64) {}
