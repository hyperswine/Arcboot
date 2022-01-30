use std::{env, fs};
use std::process::{exit, Command};

const default_arch_build: Arch = Riscv64;

// assumes you are running this in the root of your kernel with Cargo.toml visible
// can suppress output by default, then print to stdout if --verbose or -v is specified
fn main() {
    // check what was run, either arcboot build or arcboot test

    // only allow <= 3 args or if arg[1] != test or build, exit
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 || (args[1] != "build" && args[1] != "test")  {
        exit(1);
    }

    // if 3rd arg is --release, --debug, --test, keep in mind

    // if build, take the config file kernel.build and build it
    if args[1].eq("build") {
        let res_map = read_env();

        // immutable references
        let out_dir = res_map["OUT_DIR"];;
        let asm_files = res_map["ASM_FILES"];
        let linker_script = res_map["LINKER_SCRIPT"];

        let arch_build: Arch = default_arch_build;

        // collect the arch, if not specified, assume spectro/riscv64
        if args.contains(&"aarch64") {
            arch_build = Arch::Aarch64;
        }
        
        // build
        let build = Build::new().rust_build()

        // make a test to test out example/kernel.build

        Command::new("cargo")
        .arg("rustc")
        .arg("--debug");
    }

    // when testing, build the with the --test flag instead of the --debug or --release flag
    if args[1].eq("test") {
        let QEMU = "qemu-system-riscv64";

        Command::new("cargo")
        .arg("rustc")
        .arg("--test")
        .arg("-- --nocapture");

        // then run it on qemu like normal. Im not sure if the stdout will be captured, so maybe specify --nocapture above
        Command::new(QEMU)
        .arg("");
    }

    // when "run" is specified without a config, pass it to cargo as `run` by itself, and it should run the previously built cfg or the default one
    // when run with a config, build first with that config then run
    if args[1].eq("run") {
        let QEMU = "qemu-system-riscv64";

        Command::new("cargo")
        .arg("rustc")
        .arg("--test")
        .arg("-- --nocapture");

        // then run it on qemu like normal. Im not sure if the stdout will be captured, so maybe specify --nocapture above
        Command::new(QEMU)
        .arg("");
    }

}
