// expose certain things to Cargo.toml in [deps.arcboot]
// hook onto test or something

// link your kernel crate, call your build function, then call this build function when you run `cargo arcbuild`
// the build function you specify compiles your kernel crate to a staticlib as kernel.a
// this build function then runs after that

// A handy way to build the kernel + bootloader and run it without ever touching cargo or qemu
// Useful for SpectroVM where we make a bootable Bare ELF64 file. The spectroVM can run bare metal ELF. But syscalls and stuff need the kernel. So the kernel itself
// is on bare metal, at least the low-mid level components like fs, services, processes, memory management. WM, DE, Apps not really.
