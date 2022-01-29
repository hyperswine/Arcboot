// expose certain things to Cargo.toml in [deps.arcboot]
// hook onto test or something

// link your kernel crate, call your build function, then call this build function when you run `cargo arcbuild`
// the build function you specify compiles your kernel crate to a staticlib as kernel.a
// this build function then runs after that, 