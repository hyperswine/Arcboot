// Default heap allocator for testing
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

// Reset the heap, e.g. to reinit it in another region
pub fn zero_heap(heap_start: usize, heap_size: usize) {
    for x in heap_start..(heap_start + heap_size) {
        unsafe {
            core::ptr::write_volatile(x as *mut u8, 0x0);
        }
    }
}

// STACK GROWS DOWN FROM AT (3.2G)
// BUT FOR SOME REASON HEAP START ALWAYS AT 3.2G too??
pub const HEAP_START: usize = 0x4000_0000;
pub const HEAP_SIZE: usize = 0x4000;
