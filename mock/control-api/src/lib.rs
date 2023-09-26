#![doc = include_str!("../README.md")]
#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::all,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents)]
#![warn(
    clippy::as_conversions,
    clippy::as_ptr_cast_mut,
    clippy::assertions_on_result_states,
    clippy::branches_sharing_code,
    clippy::clear_with_drain,
    clippy::clone_on_ref_ptr,
    clippy::collection_is_never_read,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::derive_partial_eq_without_eq,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::index_refutable_slice,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::large_stack_frames,
    clippy::let_underscore_untyped,
    clippy::lossy_float_literal,
    clippy::manual_clamp,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::needless_raw_strings,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_clone,
    clippy::redundant_type_annotations,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::semicolon_inside_block,
    clippy::shadow_unrelated,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::tuple_array_conversions,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unnecessary_struct_initialization,
    clippy::unneeded_field_pattern,
    clippy::unused_peekable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    future_incompatible,
    invalid_reference_casting,
    let_underscore_drop,
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
    unused_tuple_struct_fields,
    variant_size_differences
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::unimplemented,
    clippy::unwrap_used,
    unreachable_pub
)]

pub mod api;
pub mod callback;
pub mod client;
pub mod prelude;

use clap::Parser as _;
use slog::{o, Drain};
use slog_scope::GlobalLoggerGuard;

/// Control API protocol re-exported definitions.
pub mod proto {
    pub use crate::api::{
        endpoint::{
            AudioSettings, Endpoint, P2pMode, PublishPolicy, VideoSettings,
            WebRtcPlayEndpoint, WebRtcPublishEndpoint,
        },
        member::{Credentials, Member},
        room::{Room, RoomElement},
        CreateResponse, Element, ErrorResponse, Response, SingleGetResponse,
    };
}

/// CLI options.
#[derive(Clone, Debug, clap::Parser)]
#[command(about, version)]
pub struct Cli {
    /// Address to host medea-control-api-mock-server on.
    #[arg(long, short, default_value = "0.0.0.0:8000")]
    pub addr: Box<str>,

    /// Address to Medea's gRPC control API.
    #[arg(long, short, default_value = "http://0.0.0.0:6565")]
    pub medea_addr: Box<str>,

    /// Address to host gRPC Control API Callback service on.
    #[arg(long, short, default_value = "0.0.0.0")]
    pub callback_host: Box<str>,

    /// Port to listen by gRPC Control API Callback service.
    #[arg(long, short = 'p', default_value_t = 9099)]
    pub callback_port: u16,
}

/// Runs RESTful mock server.
pub fn run() {
    drop(dotenv::dotenv().ok());

    let opts = Cli::parse();

    let _log_guard = init_logger();

    actix_web::rt::System::new().block_on(async move {
        let callback_server = callback::server::run(&opts);
        api::run(&opts, callback_server).await;
    });
}

/// Initializes [`slog`] logger outputting logs with a [`slog_term`]'s
/// decorator.
///
/// # Panics
///
/// If [`slog_stdlog`] fails to [initialize](slog_stdlog::init).
pub fn init_logger() -> GlobalLoggerGuard {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_envlogger::new(drain).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());
    let scope_guard = slog_scope::set_global_logger(logger);
    slog_stdlog::init()
        .unwrap_or_else(|e| panic!("Failed to initialize `slog_stdlog`: {e}"));

    scope_guard
}
