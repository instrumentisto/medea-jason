#![cfg_attr(any(doc, test), doc = include_str!("../README.md"))]
#![cfg_attr(not(any(doc, test)), doc = env!("CARGO_PKG_NAME"))]
#![deny(nonstandard_style, rustdoc::all, trivial_casts, trivial_numeric_casts)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    clippy::absolute_paths,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::as_conversions,
    clippy::as_pointer_underscore,
    clippy::as_ptr_cast_mut,
    clippy::assertions_on_result_states,
    clippy::branches_sharing_code,
    clippy::cfg_not_test,
    clippy::clear_with_drain,
    clippy::clone_on_ref_ptr,
    clippy::coerce_container_to_any,
    clippy::collection_is_never_read,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::default_union_representation,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_include_without_cfg,
    clippy::doc_paragraphs_missing_punctuation,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::empty_enum_variants_with_brackets,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::infinite_loop,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_over_hash_type,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::large_stack_frames,
    clippy::let_underscore_untyped,
    clippy::literal_string_with_formatting_args,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::map_with_unused_argument_over_ranges,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::needless_collect,
    clippy::needless_pass_by_ref_mut,
    clippy::needless_raw_strings,
    clippy::non_zero_suggestions,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pathbuf_init_then_push,
    clippy::pedantic,
    clippy::precedence_bits,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::read_zero_byte_vec,
    clippy::redundant_clone,
    clippy::redundant_test_prefix,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_and_then,
    clippy::same_name_method,
    clippy::semicolon_inside_block,
    clippy::set_contains_or_insert,
    clippy::shadow_unrelated,
    clippy::significant_drop_in_scrutinee,
    clippy::significant_drop_tightening,
    clippy::single_option_map,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::too_long_first_doc_paragraph,
    clippy::trailing_empty_array,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::uninhabited_references,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unnecessary_struct_initialization,
    clippy::unused_peekable,
    clippy::unused_result_ok,
    clippy::unused_trait_names,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::volatile_composites,
    clippy::while_float,
    clippy::wildcard_enum_match_arm,
    ambiguous_negative_literals,
    closure_returning_async_block,
    future_incompatible,
    impl_trait_redundant_captures,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    redundant_lifetimes,
    rust_2018_idioms,
    single_use_lifetimes,
    unit_bindings,
    unnameable_types,
    unreachable_pub,
    unstable_features,
    unused,
    variant_size_differences
)]
#![expect( // TODO: Needs refactoring.
    clippy::module_name_repetitions,
    clippy::unimplemented,
    clippy::unwrap_used,
    unreachable_pub,
    reason = "needs refactoring"
)]

pub mod api;
pub mod callback;
pub mod client;

use actix_web::rt;
use clap::Parser as _;

/// Control API protocol re-exported definitions.
pub mod proto {
    pub use crate::api::{
        CreateResponse, Element, ErrorResponse, Response, SingleGetResponse,
        endpoint::{
            AudioSettings, Endpoint, P2pMode, PublishPolicy, VideoSettings,
            WebRtcPlayEndpoint, WebRtcPublishEndpoint,
        },
        member::{Credentials, Member},
        room::{Room, RoomElement},
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

    init_tracing();

    rt::System::new().block_on(async move {
        let callback_server = callback::server::run(&opts);
        api::run(&opts, callback_server).await;
    });
}

/// Initializes [`tracing`] backend and all the tools relying on it:
/// - Global structured logger with the required [`tracing::Level`].
pub fn init_tracing() {
    use std::io;

    use tracing_subscriber::{
        EnvFilter, Layer as _, filter::filter_fn, fmt,
        layer::SubscriberExt as _, util::SubscriberInitExt as _,
    };

    /// [`tracing::Level`]s outputted to [`io::stderr`].
    const STDERR_LEVELS: &[tracing::Level] =
        &[tracing::Level::WARN, tracing::Level::ERROR];

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_thread_names(true)
                .compact()
                .with_ansi(true)
                .with_writer(io::stderr)
                .with_filter(EnvFilter::from_default_env())
                .with_filter(filter_fn(|meta| {
                    meta.is_span() || STDERR_LEVELS.contains(meta.level())
                })),
        )
        .with(
            fmt::layer()
                .with_thread_names(true)
                .compact()
                .with_ansi(true)
                .with_filter(EnvFilter::from_default_env())
                .with_filter(filter_fn(|meta| {
                    meta.is_span() || !STDERR_LEVELS.contains(meta.level())
                })),
        )
        .init();
}
