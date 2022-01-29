#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

enum Arch {
    Riscv64, Aarch64
}

struct Build;

impl Build {
    fn new(arch: Arch) -> Build {
        Build{}
    }
}

#[test]
fn test_build() {
    let build = Build::new(Arch::Riscv64);
    // compile boot.S. (! should auto convert {...}.s to {...}.o using prefixing)
    build.assemble("asm/boot.S", "build/boot.o");
    
    
}
