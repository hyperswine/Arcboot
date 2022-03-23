// UTILITIES
pub mod bootimg;
pub mod builder;
pub mod drivers;
pub mod readenv;

// ARCH-SPECIFIC
pub mod armv8;
pub mod riscv64;
pub mod x86_64;

/*
    LIBRARY TESTS
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[test]
fn test_build_basic() {
    let build = builder::Build::new(builder::Arch::Riscv64);
    // compile boot.S. (! should auto convert {...}.s to {...}.o using prefixing)
    build.assemble("asm/riscv64/boot.S", "build/boot.o");

    // should be specifying the staticlib as well, can get it from Cargo.toml or the API
    build.link(
        &[
            "build/boot.o".to_string(),
            "deps/libneutronkern.a".to_string(),
        ],
        "link/riscv64/linker.ld",
        "build/kernel.elf",
    );

    // cleanup
    build.clean();
}

#[test]
fn test_build_basic_chain() {
    let build = builder::Build::new(builder::Arch::Riscv64);
    build
        .assemble("asm/riscv64/boot.S", "build/boot.o")
        .link(
            &[
                "build/boot.o".to_string(),
                "deps/libneutronkern.a".to_string(),
            ],
            "link/riscv64/linker.ld",
            "build/kernel.elf",
        )
        .clean();
}
