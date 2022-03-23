pub mod arcdrivers;

/// Initialise registers and stack
#[link_section = ".startup"]
#[naked]
pub fn init() {
    asm!(
        "   csrr	t0, mhartid
            bnez	t0, 4f
            csrw	satp, zero
        .option push
        .option norelax
            la		gp, _global_pointer
        .option pop
            li		sp, 0x90000000
            li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
            csrw	mstatus, t0
            la		t1, kernel_init
            csrw	mepc, t1
            la		t2, asm_trap_vector
            csrw	mtvec, t2
            li		t3, (1 << 3) | (1 << 7) | (1 << 11)
            csrw	mie, t3
            la		ra, 4f
            mret
        4:
        asm_trap_vector:
            wfi
            j	    4b",
        options(noreturn)
    );
}

// ----------
// TESTING
// ----------

use qemu_exit::QEMUExit;

const QEMU_EXIT_HANDLE: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x10_0000);

pub fn qemu_exit_failure() -> ! {
    QEMU_EXIT_HANDLE.exit_failure()
}

pub fn qemu_exit_success() -> ! {
    QEMU_EXIT_HANDLE.exit_success()
}
