use std::env;
use std::process::exit;
use std::process::Command;
use std::fs;

use dotenv::*;

// assumes you are running this in the root of your kernel with Cargo.toml visible
fn main() {
    // check what was run, either arcboot build or arcboot test

    // only allow <= 3 args or if arg[1] != test or build, exit
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 || (args[1].neq("build") && args[1].neq("test"))  {
        exit(1);
    }

    // if 3rd arg is --release, --debug, --test, keep in mind

    // if build, take the config file kernel.build and build it
    if args[1].eq("build") {
        let config_str = fs.read_to_string("kernel.build").expect("Could not read file, does it exist or perhaps not readable?");

        // TODO: just use dotenv, but keep kernel.build idea with variables:
        // OUT_DIR, which '/.a' gets appended
        // ASM_FILES, a space separated list of non-special character files to be assembled and linked to the final program. Maybe enclosed within double quotes "a.asm b.asm c.asm"
        // LINK_SCRIPT, a file "linker.ld"

        // analyse what DIR equals to

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

}
