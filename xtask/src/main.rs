use std::{env, process::Command};

fn help() {
    print!(
        "
    ===cargo xtask===

    Commands
        help - display this message

        qemu - build arcboot and run using qemu-system-<arch>
            [ --debug | --release ]
            --arch [ arm | riscv | x86 ] (default is arm)

        build - build both arcboot and arcutils
            --output-dir (default is build/arcboot.app and build/arcutils.app)
            --arch [ arm | riscv | x86 ] (default is arm)

        debug - build arcboot in debug mode and attach gdb to the runner
            --runner [ qemu | hw ] (default is qemu)

        test - simply runs cargo test on both arcboot and arcutils


    "
    )
}

fn build() {
    // change to arcboot dir
    env::set_current_dir("arcboot").unwrap();

    Command::new("cargo")
        .args(["install", "--path", "../build"])
        .status()
        .expect("something went wrong with cargo");
    
    
}

fn main() {
    println!("Not implemented");
}
