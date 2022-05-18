// ! I dunno how this interacts with UEFI alloc
// I think we dont have to do this
#[cfg(not(feature = "uefi_support"))]
pub mod heap;
