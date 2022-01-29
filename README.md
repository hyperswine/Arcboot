## ArcBoot
Tools to build a crate and link with a bootloader (asm). And to create a bootable image for `cargo test`.
Supports RISC-V and .

Best to be used as a build dependency and configured in `Cargo.toml` under `deps.arcboot`.

Specifically optimised to building Spectro and Pi4B. Lack of options and stuff to keep things self contained, simple and easy to modify.

- only works for shells with `sh` linked
