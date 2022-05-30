// I dunno how this interacts with UEFI alloc
// If we want our custom allocator. Maybe just dont enable alloc in uefi features
#[cfg(not(feature = "uefi_support"))]
pub mod heap;

// PAGING SETUP
pub mod paging;
