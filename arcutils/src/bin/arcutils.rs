use std::env;
use std::process::{exit, Command};

use arcutils::builder::*;
use arcutils::readenv::*;

const DEFAULT_ARCH: Arch = Arch::Riscv64;

const BUILD_CFG: [&str; 3] = ["--release", "--debug", "--test"];

fn main() {
    // SECTION 0: CHECKS

    let allowed_commands = ["build", "test", "run", "flash"];

    // Check what was run, either arcboot build | test | flash
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() > 3 || !allowed_commands.contains(&&args[1].as_ref()) {
        exit(1);
    }

    /*
        SECTION A: BUILD THE KERNEL LIBRARY
    */

    // Take the config file kernel.build and build it
    if args[1] == "build" {
        // by default, env path should be in support/
        // if 'spectro' or 'pi4b' is not specified, assumee 'spectro'
        let mut arch_build_path = "support/spectro.build";
        if args.contains(&"pi4b".to_string()) {
            arch_build_path = "support/pi4b.build";
        }

        let res_map = read_env(arch_build_path);

        // immutable references
        let out_dir = &res_map["OUT_DIR"];
        let asm_files = &res_map["ASM_FILES"];
        let linker_script = &res_map["LINK_SCRIPT"];
        // if there are multiple asm_files and output_obj, then compile each at a time.
        // or better, just specify asm_files and compile to out_dir/<name>.o
        let output_objs = &res_map["OUT_OBJ"];
        let link_objs = &res_map["LINK_OBJ"];
        let output_img = &res_map["OUT_IMG"];

        let mut __arch_build: Arch = DEFAULT_ARCH;

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
        let mut to_link = vec![];
        // split on spaces
        let outs: Vec<&str> = link_objs.split(' ').collect();
        for o in outs {
            to_link.push(o.to_string());
        }

        // debug
        println!("to_link = {:?}", to_link);

        // build
        let build = Build::new(__arch_build)
            .rust_build(arch_build, build_config, out_dir)
            .assemble(asm_files, &output_objs)
            .link(&to_link, linker_script, &output_img);
    }

    /*
        SECTION B: ARCTEST
    */

    // TODO: uses --feature arcboot and runs it on qemu
    if args[1] == "test" {
        exit(1);

        let QEMU = "qemu-system-riscv64";

        // *NOTE: will build the kernel in `arctest` mode with its own EFI stub and set println = UART instead of fd = 1 (usually the main console)

        Command::new("cargo")
            .arg("rustc")
            .arg("--feature")
            .arg("arctest");

        // then run it on qemu like normal. Im not sure if the stdout will be captured, so maybe specify --nocapture above
        Command::new(QEMU).arg("");
    }

    /*
        SECTION C: RUN A BUILT KERNEL IMAGE
    */

    // TODO: run with either spectro/pi4b on QEMU using a prebuilt kernel .a and arcboot .o
    // if not found, will attempt to run `arcboot build` first, which should generate the output in build/
    if args[1] == "run" {
        exit(1);
    }

    /*
        SECTION D: FLASH ARCBOOT BL
    */

    // TODO: flash arcboot bl for a certain arch, arm, riscv, x86 onto a clean GPT drive as a single partition
    // TODO: package an arcboot .exe and neutron/quantii .exe and any .config ascii files in a dir and create an ISO, then flash onto the disk as two separate partitions
    if args[1] == "flash" {
        Command::new("flash");

        // OPTION 1: install arcboot bl by itself. Basically creates a FAT32 partition of around 200MB
        // then copies the files required by UEFI like /EFI/boot/* and any extra config files in the root dir

        // OPTION 2: Recommended for neutron systems. Packages arcboot bl and neutron lib into a single filesystem (FAT32). Arcboot bl still does the job of locating the neutron image on disk, but stored on the same FAT32 partition
        // easy to get started and up, the other stuff that neutron/quantii would use, e.g. the root filesystem and other /mnt/ disks can be other partitions on the same or different disks
    }
}

// Returns the build config
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

/*
    TESTS
*/

#[test]
fn test_cmd() {
    // set cli_args and call collect_cli_args()
}
