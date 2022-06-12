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

// depends on how much RAM/if paging is on
// virtual mem is identity mapped at first
// maybe a problem with this
// grows up towards higher vals
// at 524KB DRAM, unless theres some hole
// 524K - 590K
pub const HEAP_START: usize = 0x80000;
pub const HEAP_SIZE: usize = 16 * 0x1000;
