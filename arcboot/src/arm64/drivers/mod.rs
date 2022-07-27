// --------------
// DRIVERS
// --------------

// If not using UEFI, might as well use this
// #[cfg(not(feature = "uefi_support"))]
// pub mod pi4b;
pub mod uart;
pub mod pcie;

// should prob use acpi as much as possible
// but rn pi4b and uart is mostly hardcoded
// uart even more so for qemu only
