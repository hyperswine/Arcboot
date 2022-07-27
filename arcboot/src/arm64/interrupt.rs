/// Interrupt handling and setup for ARM64
// Would usually be 0x0 as I've heard, or 0x4000_0000... since we can just set the interrupt vector reg
// pub const INTERRUPT_VECTOR_ADDR_BASE: u64 = 0x0;
use aarch64::regs::VBAR_EL1;
use tock_registers::interfaces::Writeable;

/// A full table has defined methods for sync, async, nmi (timers and important signals) and maskable interrupts
/// Set this up (before paging?) and set the interrupt reg
pub struct InterruptVectorTable {
    interrupt_vector_addr_base: u64,
}

pub type HandlerFn = fn();

impl InterruptVectorTable {
    pub fn new(interrupt_vector_addr_base: u64) -> Self {
        Self {
            interrupt_vector_addr_base,
        }
    }

    /// Depending on the types of interrupts. Save your stack in your handler
    /// Simply maps the handler function somewhere in kernel core memory and points VBAR.handler_id to it
    pub fn register_handler(&self, addr: u64, handler: HandlerFn) {}

    pub fn set_interrupt_register_base_addr(addr: u64) {
        VBAR_EL1.set(addr);
    }
}
