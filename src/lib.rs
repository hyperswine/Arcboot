#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::process::Command;
use std::{vec, vec::Vec};

// API

pub mod bootimg;

pub enum Arch {
    Riscv64, Aarch64
}

pub struct Build {
    assembler: String,
    linker: String
}

impl Build {
    pub fn new(arch: Arch) -> Build {
        // TODO: specify a compiler, either from cfg or Cargo.toml or config.toml or the first compiler found or something
        // if AS and LD arent set, use the default instead
        // let asm = "";
        // let ld = "";

        let res = match arch {
            Arch::Riscv64 => {
                Build{
                    assembler: RISCV64_AS.to_string(),
                    linker: RISCV64_LD.to_string()
                }
            },
            Arch::Aarch64 => {
                Build{
                    assembler: AARCH64_AS.to_string(),
                    linker: AARCH64_LD.to_string()
                }
            }
        };

        // remove previous build, assumed build/
        res.clean();
        // create a new build dir
        res.create_build();

        res
    }

    fn create_build(&self) -> &Self {
        Command::new("mkdir")
            .arg("build")
            .output()
            .expect("failed to execute process");
        
        self
    }

    pub fn rust_build(&self, output_dir: &str) -> &Self {

    }

    pub fn assemble(&self, asm_file: &str, output_file: &str) -> &Self {
        // assemble the file to an output file
        let output = Command::new(&self.assembler)
            .arg("-c")
            .arg(asm_file)
            .arg("-o")
            .arg(output_file)
            .output()
            .expect("failed to execute process");
        
        println!("status: {}", output.status);

        assert!(output.status.success());
        
        self
    }

    pub fn link(&self, obj_files: &[&str], linker_script: &str, output_file: &str) -> &Self {
        let output = Command::new(&self.linker)
            .arg("-T")
            .arg(linker_script)
            .arg("-nostdlib")
            .args(obj_files)
            .arg("-o")
            .arg(output_file)
            .output()
            .expect("failed to execute process");
        
        println!("status: {}", output.status);

        assert!(output.status.success());
        
        self
    }

    // clean up the temporary build files
    // NOTE: best to output the final binary to the root or some other folder
    pub fn clean(&self) -> &Self {
        Command::new("rm")
            .args(["-rf", "build"])
            .output()
            .expect("failed to execute process");
        
        self
    }
}

const RISCV64_AS: &str = "riscv64-unknown-elf-as";
const RISCV64_LD: &str = "riscv64-unknown-elf-ld";

const AARCH64_AS: &str = "aarch64-none-elf-as";
const AARCH64_LD: &str = "aarch64-none-elf-ld";
 
#[test]
fn test_build_basic() {
    let build = Build::new(Arch::Riscv64);
    // compile boot.S. (! should auto convert {...}.s to {...}.o using prefixing)
    build.assemble("asm/riscv64/boot.S", "build/boot.o");

    // should be specifying the staticlib as well, can get it from Cargo.toml or the API
    build.link(&["build/boot.o", "deps/libneutronkern.a"], "link/riscv64/linker.ld", "build/kernel.elf");

    // cleanup
    build.clean();
}

#[test]
fn test_build_basic_chain() {
    let build = Build::new(Arch::Riscv64);
    build.assemble("asm/riscv64/boot.S", "build/boot.o")
        .link(&["build/boot.o", "deps/libneutronkern.a"], "link/riscv64/linker.ld", "build/kernel.elf")
        .clean();
}

// API

// MAYBE make this an executable and specify the output dir in [deps] or config.toml
// two commands, `cargo arcbuild` to make the bootable image from the kernel crate and the other boot/linker scripts.
// `cargo arctest spectro` to run qemu with settings for riscv64 and cfg(test). Maybe cargo run --test would work or cargo build --test then qemu on the image

// args: pass a expect() object in from the build command
// TODO: better idea, specify the output dir of the full kernel staticlib
pub fn full_build(staticlib_dir: &str, arch: Arch) {
    println!("status: {}", build_cmd.status);
    assert!(build_cmd.status.success());

    // build riscv
    match arch {
        Arch::Riscv64 => {
            let build = Build::new(Arch::Riscv64);
            build.assemble("asm/riscv64/boot.S", "build/boot.o")
                .link(&["build/boot.o", "build/rust/*.a"], "link/riscv64/linker.ld", "build/kernel.elf");
        }
    }
    
}
