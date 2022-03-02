#![doc = include_str!("../README.md")]
#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents)]
#![warn(
    clippy::branches_sharing_code,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::index_refutable_slice,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
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
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::trailing_empty_array,
    clippy::trivial_regex,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
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
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results
)]
#![allow(clippy::module_name_repetitions)] // TODO: Make this unnecessary.

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
#[clap(about, version)]
pub struct Cli {
    /// Address to host medea-control-api-mock-server on.
    #[clap(long, short, default_value = "0.0.0.0:8000")]
    pub addr: String,

    /// Address to Medea's gRPC control API.
    #[clap(long, short, default_value = "http://0.0.0.0:6565")]
    pub medea_addr: String,

    /// Address to host gRPC Control API Callback service on.
    #[clap(long, short, default_value = "0.0.0.0")]
    pub callback_host: String,

    /// Port to listen by gRPC Control API Callback service.
    #[clap(long, short = 'p', default_value_t = 9099)]
    pub callback_port: u16,
}

/// Runs RESTful mock server.
pub fn run() {
    drop(dotenv::dotenv().ok());

    let opts = Cli::parse();

    let _log_guard = init_logger();

    actix_web::rt::System::new().block_on(async move {
        let callback_server = callback::server::run(&opts).await;
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
    slog_stdlog::init().unwrap();

    scope_guard
}
