// use arcboot::memory;

use arcboot::memory::map_segment;

extern crate std;

#[test]
fn test_arcboot() {
    // let x = memory::mmu::AccessPermissions::ReadOnly;
    // println!("x = {x:?}")
    // #[cfg(target_arch = "aarch64")]
    // arcboot::arm64::trap_to_el2();

    let x = map_segment(&[0; 1], 1);
}
