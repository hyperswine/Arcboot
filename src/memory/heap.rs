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

// Temporary for Identity Mapping
pub const HEAP_START: usize = 0xa000_0000;
pub const HEAP_SIZE: usize = 4 * 0x1000;

// Can "reinit" the heap after 4 level page tables is setup with these virtual address ranges
// For the kernel only. Userspace apps must also use virtual memory (lower half) instead
// pub const HEAP_START: usize = 0x_4444_4444_0000;
// pub const HEAP_SIZE: usize = 100 * 1024;
