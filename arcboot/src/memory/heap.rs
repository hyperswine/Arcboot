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

// Temporary for Identity Mapping. 0xA000_0000 works too
// Either: 0x4000_0000 which works
// Doesnt work but kinda expected to work:
// 0x0, 0x1000, 0x4000
// but using them allows me to have MMIO memory descriptor...
// STACK GROWS DOWN FROM a pretty high address (3.2G)
// BUT FOR SOME REASON HEAP START ALWAYS AT 3.2G too?? Maybe it thinks it only needs 0x16_000 worth of memory so it puts it at 3.2G - 90K
// Uh maybe make it 1GB as well. As the stack
pub const HEAP_START: usize = 0x4000_0000;
pub const HEAP_SIZE: usize = 0x4000_0000;

// Can "reinit" the heap after 4 level page tables is setup with these virtual address ranges
// For the kernel only. Userspace apps must also use virtual memory (lower half) instead
// pub const HEAP_START: usize = 0x_4444_4444_0000;
// pub const HEAP_SIZE: usize = 100 * 1024;