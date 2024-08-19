//! [gRPC]-based [Control API] implementation.
//!
//! [gRPC]: https://grpc.io
//! [Control API]: https://tinyurl.com/yxsqplq7

#[cfg(feature = "client")]
mod client;
mod convert;
#[cfg(feature = "server")]
mod server;

#[allow(
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::style,
    let_underscore_drop,
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
    clippy::style,
    let_underscore_drop,
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

use std::str::FromStr;

use derive_more::{Display, Error, From, Into};
use url::Url;

#[doc(inline)]
pub use self::convert::ProtobufError;
#[cfg(feature = "client")]
#[doc(inline)]
pub use self::{
    api::control_api_client::ControlApiClient,
    callback::callback_server::CallbackServer as CallbackApiServer,
    client::ControlApiClientError,
};
#[cfg(feature = "server")]
#[doc(inline)]
pub use self::{
    api::control_api_server::ControlApiServer,
    callback::callback_client::CallbackClient as CallbackApiClient,
    server::CallbackApiClientError,
};

/// URL representing a [gRPC] callback implementing [`CallbackApi`].
///
/// [`CallbackApi`]: crate::CallbackApi
/// [gRPC]: https://grpc.io
#[derive(Clone, Debug, Display, Eq, Hash, Into, PartialEq)]
#[display("grpc://{_0}")]
#[into(String)]
pub struct CallbackUrl(Url);

impl CallbackUrl {
    /// Converts this [`CallbackUrl`] into the one with `http://` scheme.
    #[must_use]
    pub fn to_http(&self) -> Url {
        let mut url = self.0.clone();
        url.set_scheme("http").unwrap_or_else(|()| unreachable!());
        url
    }
}

impl FromStr for CallbackUrl {
    type Err = CallbackUrlParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value)?;
        if url.scheme() != "grpc" {
            return Err(CallbackUrlParseError::WrongScheme);
        }
        if url.host().is_none() {
            return Err(CallbackUrlParseError::MissingHost);
        }
        Ok(Self(url))
    }
}

/// Error of parsing a [`CallbackUrl`].
#[derive(Clone, Copy, Debug, Display, Error, From)]
pub enum CallbackUrlParseError {
    /// Error of parsing the provided [`Url`].
    #[display("Invalid URL: {_0}")]
    UrlParseErr(url::ParseError),

    /// [`Url`] is missing host.
    #[display("Missing host")]
    MissingHost,

    /// [`Url`] contains unsupported scheme.
    #[display("Only `grpc://` scheme is allowed")]
    WrongScheme,
}
