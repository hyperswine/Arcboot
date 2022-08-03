use tock_registers::interfaces::Writeable;

#[cfg(feature = "builtin_allocator")]
pub mod heap;
pub mod mmu;

// ARC MEMORY PROTOCOL
