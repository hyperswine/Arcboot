use std::process::Command;

pub enum Arch {
    Riscv64,
    Aarch64,
}

const cargo_command: &str = "cargo";
const rustc_command: &str = "rustc";
const rustc_target_command: &str = "--target=";
const rustc_flag_command: &str = "--";
const staticlib_crate_type: &str = "--crate-type=staticlib";

pub struct Build {
    assembler: String,
    linker: String,
}

impl Build {
    pub fn new(arch: Arch) -> Build {
        let res = match arch {
            Arch::Riscv64 => Build {
                assembler: RISCV64_AS.to_string(),
                linker: RISCV64_LD.to_string(),
            },
            Arch::Aarch64 => Build {
                assembler: AARCH64_AS.to_string(),
                linker: AARCH64_LD.to_string(),
            },
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
            .expect("failed to make a build dir. Does it already exist?");

        self
    }

    // Build the kernel
    // args: 'build_config' needs to be in the form --release, --debug, or --target
    // target_arch needs to be in the form riscv64gc-unknown-none-elf, aarch64-none-elf or a JSON file (not supported yet)
    pub fn rust_build(&self, target_arch: &str, build_config: &str, output_dir: &str) -> &Self {
        // debug
        println!("target_arch = {}", target_arch);
        println!("build_config = {}", build_config);
        println!("output_dir = {}", output_dir);

        // assemble the file to an output file
        let output = Command::new(&cargo_command)
            .arg(rustc_command)
            .arg(String::from(rustc_target_command) + target_arch)
            .arg(build_config)
            .arg(rustc_flag_command)
            .arg(staticlib_crate_type)
            .arg("-o")
            .arg(output_dir.to_string() + "/.a")
            .output()
            .expect(
                "failed to execute Cargo. Please check if your dependencies and paths are right",
            );

        // if cargo failed to build, thats on them. Maybe config.toml or Cargo.toml is wrong
        println!("status: {}", output.status);
        assert!(output.status.success());

        self
    }

    pub fn assemble(&self, asm_file: &str, output_file: &str) -> &Self {
        // debug
        println!("asm_file = {}", asm_file);
        println!("output_file = {}", output_file);

        // assemble the file to an output file
        let output = Command::new(&self.assembler)
            .arg("-c")
            .arg(asm_file)
            .arg("-o")
            .arg(output_file)
            .output()
            .expect("failed to execute the assembler, is it in path? Otherwise specify its full path in Cargo.toml under [deps.arcboot]");

        println!("status: {}", output.status);
        assert!(output.status.success());

        self
    }

    // ? DOESNT WORK, MAYBE JUST CALL THE SCRIPT FROM scripts/link.sh
    pub fn link(&self, obj_files: &[String], linker_script: &str, output_file: &str) -> &Self {
        // debug
        println!("linker_script = {}", linker_script);
        println!("obj_files = {:?}", obj_files);
        println!("output_file = {}", output_file);

        let joined_to_link = obj_files.join(" ");

        let output = Command::new(&self.linker)
            .arg("-T")
            .arg(linker_script)
            .arg("-nostdlib")
            .arg(joined_to_link)
            .arg("-o")
            .arg(output_file)
            .output()
            .expect("failed to execute the linker, is it in path? Otherwise specify its full path in Cargo.toml under [deps.arcboot]");

        // literally not event returning 1, instead some other error code. Def not 0
        println!("status: {}", output.status);

        self
    }

    // Clean up the temporary build files
    // NOTE: best to output the final binary to the root or some other folder
    pub fn clean(&self) -> &Self {
        let output = Command::new("rm")
            .args(["-rf", "build"])
            .output()
            .expect("failed to run rm, does it exist or is it linked?");

        println!("status: {}", output.status);
        assert!(output.status.success());

        self
    }
}

const RISCV64_AS: &str = "riscv64-unknown-elf-as";
const RISCV64_LD: &str = "riscv64-unknown-elf-ld";

const AARCH64_AS: &str = "aarch64-none-elf-as";
const AARCH64_LD: &str = "aarch64-none-elf-ld";

// experimental, build everything at once
pub fn full_build(staticlib_dir: &str, arch: Arch) {
    // build riscv
    // match arch {
    //     Arch::Riscv64 => {
    //         let build = Build::new(Arch::Riscv64);
    //         build.assemble("asm/riscv64/boot.S", "build/boot.o").link(
    //             &["build/boot.o", "build/rust/*.a"],
    //             "link/riscv64/linker.ld",
    //             "build/kernel.elf",
    //         );
    //     }
    //     Arch::Aarch64 => {}
    // }
}
