// --------------
//  UEFI WRAPPERS
// --------------

// RE-EXPORTS
pub use uefi::alloc;
pub use uefi_macros::prelude::*;

// I DONT THINK IT WORKS FOR RISCV. Needs a PR
#[entry]
pub fn main(_handle: Handle, mut system_table: uefi::prelude::SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    Status::SUCCESS
}
