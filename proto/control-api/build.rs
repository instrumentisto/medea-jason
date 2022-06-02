//! Build script generating library code out of specs.

#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    clippy::as_conversions,
    clippy::branches_sharing_code,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::else_if_without_else,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::index_refutable_slice,
    clippy::iter_with_drain,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::only_used_in_recursion,
    clippy::option_if_let_else,
    clippy::panic_in_result_fn,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    future_incompatible,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "grpc")]
    grpc::compile()?;

    Ok(())
}

/// gRPC Protobuf specs compilation.
#[cfg(feature = "grpc")]
mod grpc {
    use std::{error::Error, fs, io};

    /// Path to Protobuf source files.
    const GRPC_DIR: &str = "src/grpc";

    /// Compiles gRPC protobuf specs to Rust source files.
    ///
    /// Specs will be generated only if you've deleted old generated specs.
    ///
    /// For rebuilding you may simply execute:
    /// ```bash
    /// make cargo.gen crate=medea-control-api-proto
    /// ```
    /// in the root of the project.
    pub(crate) fn compile() -> Result<(), Box<dyn Error>> {
        let proto_names = ProtoNames::load()?;
        let grpc_spec_files = proto_names.get_grpc_spec_files();
        let out_files = proto_names.get_out_files();

        grpc_spec_files
            .iter()
            .chain(out_files.iter())
            .for_each(|filename| {
                println!("cargo:rerun-if-changed={filename}");
            });

        for filename in &out_files {
            if let Err(err) = fs::File::open(filename) {
                if err.kind() != io::ErrorKind::NotFound {
                    return Err(err.to_string().into());
                }

                tonic_build::configure()
                    .out_dir(GRPC_DIR)
                    .build_client(cfg!(feature = "client"))
                    .build_server(cfg!(feature = "server"))
                    .compile(&grpc_spec_files, &[GRPC_DIR.to_owned()])?;
            }
        }

        Ok(())
    }

    /// All names of Protobuf specs from [`GRPC_DIR`] directory.
    ///
    /// This entity just stores file stems (for `api.proto`'s filename file stem
    /// is `api` for example) of all files from [`GRPC_DIR`] that have `.proto`
    /// extension.
    struct ProtoNames(Vec<String>);

    impl ProtoNames {
        /// Loads [`ProtoNames`] from [`GRPC_DIR`] directory.
        fn load() -> io::Result<Self> {
            let proto_names = fs::read_dir(GRPC_DIR)?
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|entry| entry.path())
                .filter(|path| {
                    path.extension().map_or(false, |ext| {
                        path.is_file() && ext.to_string_lossy() == *"proto"
                    })
                })
                .filter_map(|path| {
                    path.file_stem()
                        .map(|stem| stem.to_string_lossy().to_string())
                })
                .collect();
            Ok(Self(proto_names))
        }

        /// Returns paths to all Protobuf files from [`GRPC_DIR`].
        fn get_grpc_spec_files(&self) -> Vec<String> {
            self.0
                .iter()
                .map(|name| format!("{}/{}.proto", GRPC_DIR, name))
                .collect()
        }

        /// Returns paths to files which will be generated by [`tonic`] after
        /// compilation of Protobuf specs from [`GRPC_DIR`].
        fn get_out_files(&self) -> Vec<String> {
            self.0
                .iter()
                .map(|filename| format!("{}/{}.rs", GRPC_DIR, filename))
                .collect()
        }
    }
}
