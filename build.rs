//! Compiles `trampoline.c` and links it into the final library.

#![forbid(non_ascii_idents, unsafe_code)]

use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=CLIPPY_ARGS");
    if env::var("CLIPPY_ARGS").is_ok() {
        return;
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");
        println!(
            "cargo:rustc-link-arg=-Wl,-install_name,\
             @rpath/libmedea_jason.dylib"
        );
    }
}
