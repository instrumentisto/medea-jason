//! [gRPC]-based [Control API] for [Medea].
//!
//! [gRPC]: https://grpc.io
//! [Medea]: https://github.com/instrumentisto/medea
//! [Control API]: https://tinyurl.com/yxsqplq7

#[cfg(feature = "client")]
mod client;
mod conversions;
#[cfg(feature = "server")]
mod server;

#[allow(
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#[rustfmt::skip]
pub mod api;
#[allow(
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#[rustfmt::skip]
pub mod callback;

pub use self::conversions::{
    CallbackUrl, CallbackUrlParseError, TryFromProtobufError,
};
#[cfg(feature = "client")]
#[doc(inline)]
pub use self::{
    api::control_api_client::ControlApiClient,
    callback::callback_client::CallbackClient,
    client::{CallbackClientError, ControlClientError},
};
#[cfg(feature = "server")]
#[doc(inline)]
pub use self::{
    api::control_api_server::ControlApiServer,
    callback::callback_server::CallbackServer,
};
