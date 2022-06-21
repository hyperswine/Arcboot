// use arcboot::memory;

extern crate std;

#[test]
fn test_arcboot() {
    // let x = memory::mmu::AccessPermissions::ReadOnly;
    // println!("x = {x:?}")
    arcboot::arm64::trap_to_el2();
    // theory: somesomething uefi
    // maybe its not compiling with uefi
}
