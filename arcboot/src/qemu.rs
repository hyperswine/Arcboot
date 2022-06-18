// ----------
// QEMU
// ----------

use qemu_exit::QEMUExit;

#[cfg(target_arch = "aarch64")]
const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();

#[cfg(target_arch = "riscv64")]
const QEMU_EXIT_HANDLE: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);

pub fn qemu_exit_failure() -> ! {
    QEMU_EXIT_HANDLE.exit_failure()
}

pub fn qemu_exit_success() -> ! {
    QEMU_EXIT_HANDLE.exit_success()
}
