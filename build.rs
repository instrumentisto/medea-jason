//! Compiles `trampoline.c` and links it into the final library.

#![forbid(non_ascii_idents, unsafe_code)]

use std::env;

use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

fn case(str: &str) -> String {
    let mut res = String::new();
    let mut first = true;
    let mut space = false;
    for i in str.chars() {
        if first {
            res.push_str(&i.to_uppercase().to_string());
            first = false;
            continue;
        }
        if space {
            res.push_str(&i.to_uppercase().to_string());
            space = false;
            continue;
        }
        if i == '_' {
            space = true;
            continue;
        } else {
            res.push(i);
        }
    }
    res
}

fn gen_args(
    files: &[&str],
) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let args = files
        .iter()
        .map(|file| {
            (
                format!("src/api/dart/{}_api.rs", file),
                format!("flutter/lib/src/native/ffi/{}_api.g.dart", file),
                format!("src/{}_api_g.rs", file),
                format!("{}Api", case(file)),
            )
        })
        .collect::<Vec<_>>();
    let mut a1 = vec![];
    let mut a2 = vec![];
    let mut a3 = vec![];
    let mut a4 = vec![];
    for (a, b, c, d) in args {
        a1.push(a);
        a2.push(b);
        a3.push(c);
        a4.push(d);
    }
    (a1, a2, a3, a4)
}

fn main() {
    let (rust_input, dart_output, rust_output, class_name) = gen_args(&[
        "jason",
        "audio_track_constraints",
        "connection_handle",
        "device_video_track_constraints",
        "display_video_track_constraints",
        "local_media_track",
        "media_device_info",
        "media_display_info",
        "media_manager_handle",
        "media_stream_settings",
    ]);
    // Tell Cargo that if the input Rust code changes, to rerun this build
    // script.
    println!("cargo:rerun-if-changed={}", "src/lib.rs");
    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input,
        // Path of output generated Dart code
        dart_output,
        // Path of output Rust code
        rust_output: Some(rust_output),
        // Class name of each Rust block of api
        class_name: Some(class_name),
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
