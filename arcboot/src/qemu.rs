// ----------
// QEMU
// ----------

use qemu_exit::QEMUExit;

// // iobase is like 0x04
// const IO_BASE: usize = 0x04;
// const EXIT_CODE: usize = 0x01;

// #[cfg(target_arch = "x86_64")]
// const QEMU_EXIT_HANDLE: qemu_exit::X86 = qemu_exit::X86::new(IO_BASE, EXIT_CODE);

// #[cfg(target_arch = "aarch64")]
// const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();

// #[cfg(target_arch = "riscv64")]
// const QEMU_EXIT_HANDLE: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);

pub fn qemu_exit_failure() -> ! {
    let io_base = 0x4;
    let custom_exit_success = 1;

    #[cfg(target_arch = "aarch64")]
    let qemu_exit_handle = qemu_exit::AArch64::new();

    // addr: The address of sifive_test.
    #[cfg(target_arch = "riscv64")]
    let qemu_exit_handle = qemu_exit::RISCV64::new(addr);

    // io_base:             I/O-base of isa-debug-exit.
    // custom_exit_success: A custom success code; Must be an odd number.
    #[cfg(target_arch = "x86_64")]
    let qemu_exit_handle = qemu_exit::X86::new(io_base, custom_exit_success);

    qemu_exit_handle.exit_failure()
}

pub fn qemu_exit_success() -> ! {
    let io_base = 0x4;
    let custom_exit_success = 1;

    #[cfg(target_arch = "aarch64")]
    let qemu_exit_handle = qemu_exit::AArch64::new();

    // addr: The address of sifive_test.
    #[cfg(target_arch = "riscv64")]
    let qemu_exit_handle = qemu_exit::RISCV64::new(addr);

    // io_base:             I/O-base of isa-debug-exit.
    // custom_exit_success: A custom success code; Must be an odd number.
    #[cfg(target_arch = "x86_64")]
    let qemu_exit_handle = qemu_exit::X86::new(io_base, custom_exit_success);

    qemu_exit_handle.exit_success()
}
