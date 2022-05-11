// QUICK ALLOC FOR TESTS AND OTHER STUFF LIKE UEFI WHICH MAY BE GOOD WITH IT
// IDK WHERE TO PUT HEAP, MAYBE AT 0x50000

use linked_list_allocator::{self, LockedHeap};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

pub const HEAP_START: usize = 0xff00_0000_0000_0000;
pub const HEAP_SIZE: usize = 4 * 0x1000;
