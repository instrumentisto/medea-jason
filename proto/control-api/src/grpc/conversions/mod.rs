//! Definitions of conversions from [`ControlApi`] spec into generated from
//! `protobuf` via [`tonic-build`] and vise-versa.
//!
//! [`ControlApi`]: crate::ControlApi

// TODO: Remove once annoying false positive is fixed:
//       https://github.com/rust-lang/rust-clippy/issues/6902
#![allow(clippy::use_self)]

mod api;
mod callback;

use std::str::FromStr;

use derive_more::{Display, Error, From, Into};
use url::Url;

use crate::{
    control::ParseFidError, endpoint::web_rtc_play::LocalSrcUriParseError,
    grpc::api as proto,
};

/// `URL` of the gRPC [`CallbackClient`].
///
/// [`CallbackClient`]: crate::CallbackClient
#[derive(Clone, Debug, Display, Eq, Hash, Into, PartialEq)]
#[display(fmt = "grpc://{}", _0)]
pub struct CallbackUrl(String);

impl CallbackUrl {
    /// Returns `HTTP` address for gRPC callback client.
    ///
    /// If you wish to get address with protocol - just use [`Display`]
    /// implementation.
    #[must_use]
    pub fn http_addr(&self) -> String {
        format!("http://{}", self.0)
    }
}

impl FromStr for CallbackUrl {
    type Err = CallbackUrlParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(&value)?;
        let url_scheme = url.scheme();
        let host = url.host().ok_or(CallbackUrlParseError::MissingHost)?;
        let host = url
            .port()
            .map_or_else(|| host.to_string(), |port| format!("{host}:{port}"));

        match url_scheme {
            "grpc" => Ok(Self(host)),
            _ => Err(CallbackUrlParseError::UnsupportedScheme),
        }
    }
}

/// Error of [`CallbackUrl`] parsing.
#[derive(Clone, Copy, Debug, Display, Error, From)]
pub enum CallbackUrlParseError {
    /// Failed to parse URL.
    UrlParseErr(url::ParseError),

    /// URL is missing host.
    #[display(fmt = "Missing host")]
    MissingHost,

    /// URL contains unsupported scheme.
    #[display(fmt = "Unsupported URL scheme")]
    UnsupportedScheme,
}

// impl From<CallbackUrlParseError> for ErrorResponse {
//     fn from(err: CallbackUrlParseError) -> Self {
//         use CallbackUrlParseError::{
//             MissingHost, UnsupportedScheme, UrlParseErr,
//         };
//
//         match err {
//             MissingHost => {
//                 Self::without_id(ErrorCode::MissingHostInCallbackUrl)
//             }
//             UnsupportedScheme => {
//                 Self::without_id(ErrorCode::UnsupportedCallbackUrlProtocol)
//             }
//             UrlParseErr(_) =>
// Self::without_id(ErrorCode::InvalidCallbackUrl),         }
//     }
// }

/// Errors which may occur while deserializing protobuf spec.
#[derive(Debug, Display, Error, From)]
pub enum TryFromProtobufError {
    /// Error while parsing [`SrcUri`] of [`WebRtcPlay`].
    ///
    /// [`WebRtcPlay`]: crate::endpoint::WebRtcPlay
    /// [`SrcUri`]: crate::endpoint::web_rtc_play::SrcUri
    #[display(fmt = "Src uri parse error: {:?}", _0)]
    SrcUriError(LocalSrcUriParseError),

    /// [`Room`] element doesn't have [`Member`] element. Currently this is
    /// unimplemented.
    ///
    /// [`Member`]: crate::Member
    /// [`Room`]: crate::Room
    #[display(fmt = "Expected element of type [{}]. Id [{}]", _0, _1)]
    #[from(ignore)]
    ExpectedOtherElement(String, String),

    /// Element is [`None`], but expected [`Some`].
    #[display(fmt = "Element is None, expected Some. Id [{}]", _0)]
    #[from(ignore)]
    EmptyElement(#[error(not(source))] String),

    /// Error while [`CallbackUrl`] parsing.
    #[display(fmt = "Error while parsing gRPC callback URL. {}", _0)]
    CallbackUrlParseErr(CallbackUrlParseError),

    /// Some element from a spec contains negative [`Duration`], but it's not
    /// supported.
    ///
    /// [`Duration`]: std::time::Duration
    #[display(
        fmt = "Element(id: {}) contains negative duration field `{}`",
        _0,
        _1
    )]
    #[from(ignore)]
    NegativeDuration(String, &'static str),

    /// TODO
    #[display(fmt = "FID is too long: {}", _0)]
    FidIsTooLong(#[error(not(source))] String),

    /// TODO
    Fid(ParseFidError),

    /// TODO
    Url(url::ParseError),

    /// TODO
    UnimplementedCall,
}

impl From<TryFromProtobufError> for proto::Error {
    fn from(_: TryFromProtobufError) -> Self {
        todo!()
    }
}

// impl From<TryFromProtobufError> for ErrorResponse {
//     fn from(err: TryFromProtobufError) -> Self {
//         use TryFromProtobufError as E;
//
//         match err {
//             E::SrcUriError(e) => e.into(),
//             E::CallbackUrlParseErr(e) => e.into(),
//             E::ExpectedOtherElement(element, id) => Self::with_explanation(
//                 ErrorCode::ElementIdMismatch,
//                 format!(
//                     "Provided fid can not point to element of type
// [{element}]",                 ),
//                 Some(id),
//             ),
//             E::EmptyElement(id) => Self::with_explanation(
//                 ErrorCode::NoElement,
//                 String::from("No element was provided"),
//                 Some(id),
//             ),
//             E::NegativeDuration(id, f) => Self::with_explanation(
//                 ErrorCode::NegativeDuration,
//                 format!(
//                     "Element(id: {id}) contains negative duration field
// `{f}`",                 ),
//                 Some(id),
//             ),
//         }
//     }
// }
