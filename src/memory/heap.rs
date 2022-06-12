// QUICK ALLOC FOR TESTS AND OTHER STUFF LIKE UEFI WHICH MAY BE GOOD WITH IT
// IDK WHERE TO PUT HEAP, MAYBE AT 0x50000

// should use https://docs.rs/context-allocator/latest/context_allocator/
// to init_heap() and register this as the default allocator

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

// 524K - 590K
// no exception so should work. Just that the values are off
pub const HEAP_START: usize = 0x80000;
pub const HEAP_SIZE: usize = 16 * 0x1000;

// getting an exception here. So the above should work?
// pub const HEAP_START: usize = 0x_4444_4444_0000;
// pub const HEAP_SIZE: usize = 100 * 1024;
