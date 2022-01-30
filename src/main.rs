use std::process::{exit, Command};
use std::{env, fs};

const default_arch_build: Arch = Arch::Riscv64;

use arcboot::builder::*;
use arcboot::readenv::*;
use arcboot::str;

const BUILD_CFG: [&str; 3] = ["--release", "--debug", "--test"];

// assumes you are running this in the root of your kernel with Cargo.toml visible
// can suppress output by default, then print to stdout if --verbose or -v is specified
fn main() {
    // check what was run, either arcboot build or arcboot test

    // only allow <= 3 args or if arg[1] != test or build, exit
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 3 || (args[1] != "build" && args[1] != "test") {
        exit(1);
    }

    let res_map = read_env();

    // immutable references
    let out_dir = &res_map["OUT_DIR"];
    // ! NOTE: only support a single .S file for now. Can simply make it a HashMap<String, Vec<String>> instead
    // and individually specify the types of these vars. Vec<String> for asm_files and output_objs
    let asm_files = &res_map["ASM_FILES"];
    let linker_script = &res_map["LINK_SCRIPT"];
    let output_objs = &res_map["OUT_OBJ"];

    // if build, take the config file kernel.build and build it
    if args[1] == "build" {
        let mut __arch_build: Arch = default_arch_build;

        // collect the arch, if not specified, assume spectro/riscv64
        if args.contains(&"aarch64".to_string()) {
            __arch_build = Arch::Aarch64;
        }

        let mut arch_build = match __arch_build {
            Arch::Aarch64 => "aarch64-none-elf",
            Arch::Riscv64 => "riscv64gc-unknown-none-elf",
        };

        // check if a build config was passed
        let build_config = check_build_config(args.as_slice());

        // make a list of files to be linked (.o assembled and kernel .a)
        let mut to_link = [str!(out_dir) + "kernel.a", str!(out_dir) + output_objs];

        // build
        let build = Build::new(__arch_build)
            .rust_build(arch_build, build_config, out_dir)
            .assemble(asm_files, &output_objs)
            .link(&to_link, linker_script, &output_objs);

        // make a test to test out example/kernel.build
    }

    // when testing, build the with the --test flag instead of the --debug or --release flag
    if args[1] == "test" {
        let QEMU = "qemu-system-riscv64";

        Command::new("cargo")
            .arg("rustc")
            .arg("--test")
            .arg("-- --nocapture");

        // then run it on qemu like normal. Im not sure if the stdout will be captured, so maybe specify --nocapture above
        Command::new(QEMU).arg("");
    }

    // when "run" is specified without a config, pass it to cargo as `run` by itself, and it should run the previously built cfg or the default one
    // when run with a config, build first with that config then run
    if args[1] == "run" {
        let QEMU = "qemu-system-riscv64";

        Command::new("cargo")
            .arg("rustc")
            .arg("--test")
            .arg("-- --nocapture");

        // then run it on qemu like normal. Im not sure if the stdout will be captured, so maybe specify --nocapture above
        Command::new(QEMU).arg("");
    }
}

// returns the build config
#[inline(always)]
fn check_build_config<'a>(to_check: &'a [String]) -> &'a str {
    for _st in to_check {
        if to_check.contains(&"--release".to_string()) {
            return "--release";
        };
        if to_check.contains(&"--debug".to_string()) {
            return "--debug";
        };
        if to_check.contains(&"--test".to_string()) {
            return "--test";
        };
    }
    // default, release
    "--release"
}
