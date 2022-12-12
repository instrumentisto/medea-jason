//! Compiles `trampoline.c` and links it into the final library.

#![forbid(non_ascii_idents, unsafe_code)]

use std::env;

use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

fn main() {
    // Tell Cargo that if the input Rust code changes, to rerun this build
    // script.
    println!("cargo:rerun-if-changed=src/api/dart/jason_api.rs");
    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec!["src/api/dart/jason_api.rs".to_owned()],
        
        // Path of output generated Dart code
        dart_output: vec!["flutter/lib/src/native/ffi/jason_api.g.dart".to_owned()],
        // Path of output Rust code
        rust_output: Some(vec!["src/jason_api_g.rs".to_owned()]),
        // Class name of each Rust block of api
        class_name: Some(vec!["JasonApi".to_owned()]),
        dart_format_line_length: 80,
        skip_add_mod_to_lib: true,
        // for other options use defaults
        ..Default::default()
    };
    // get opts from raw opts
    let configs = config_parse(raw_opts);

    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }

    println!("cargo:rerun-if-env-changed=CLIPPY_ARGS");
    if env::var("CLIPPY_ARGS").is_ok() {
        return;
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.11");
        println!(
            "cargo:rustc-link-arg=-Wl,-install_name,\
             @rpath/libmedea_jason.dylib"
        );
    }

    if let Ok("wasm32-unknown-unknown") = env::var("TARGET").as_deref() {
        return;
    }

    println!("cargo:rerun-if-changed=src/platform/dart/api_dl/trampoline.c");
    cc::Build::new()
        .file("src/platform/dart/api_dl/trampoline.c")
        .compile("trampoline");
}
