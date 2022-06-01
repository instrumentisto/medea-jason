//! Definitions of conversions from [`ControlApi`] spec into generated from
//! `protobuf` spec via [`tonic-build`] and vise-versa.
//!
//! [`ControlApi`]: crate::ControlApi

// TODO: Remove once annoying false positive is fixed:
//       https://github.com/rust-lang/rust-clippy/issues/6902
#![allow(clippy::use_self)]

mod api;
mod callback;

use std::str::FromStr;

use derive_more::{Display, Error, From, Into};
use time::error::{Format as TimeFormatError, Parse as TimeParseError};
use url::Url;

use crate::{
    control::ParseFidError, endpoint::web_rtc_play::LocalSrcUriParseError,
};

/// URL of the gRPC [`CallbackApi`].
///
/// [`CallbackApi`]: crate::CallbackApi
#[derive(Clone, Debug, Display, Eq, From, Hash, Into, PartialEq)]
#[display(fmt = "grpc://{}", _0)]
#[from(types(String))]
#[into(owned(types(String)))]
pub struct CallbackUrl(Box<str>);

impl CallbackUrl {
    /// Returns HTTP address for gRPC callback client.
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
        let url = Url::parse(value)?;
        let url_scheme = url.scheme();
        let host = url.host().ok_or(CallbackUrlParseError::MissingHost)?;
        let host = url
            .port()
            .map_or_else(|| host.to_string(), |port| format!("{host}:{port}"));

        match url_scheme {
            "grpc" => Ok(host.into()),
            _ => Err(CallbackUrlParseError::UnsupportedScheme),
        }
    }
}

/// Error of [`CallbackUrl`] parsing.
#[derive(Clone, Copy, Debug, Display, Error, From)]
pub enum CallbackUrlParseError {
    /// Error while parsing [`Url`].
    #[display(fmt = "Error while parsing URL: {}", _0)]
    UrlParseErr(url::ParseError),

    /// URL is missing host.
    #[display(fmt = "Missing host")]
    MissingHost,

    /// URL contains unsupported scheme.
    #[display(fmt = "Unsupported URL scheme")]
    UnsupportedScheme,
}

/// Errors which may occur while deserializing protobuf spec.
#[derive(Debug, Display, Error, From)]
pub enum TryFromProtobufError {
    /// Error while parsing [`LocalSrcUri`] of [`WebRtcPlay`].
    ///
    /// [`WebRtcPlay`]: crate::endpoint::WebRtcPlay
    /// [`LocalSrcUri`]: crate::endpoint::web_rtc_play::LocalSrcUri
    #[display(fmt = "Src uri parse error: {:?}", _0)]
    LocalSrcUriParseError(LocalSrcUriParseError),

    /// [`Room`] element doesn't have [`Member`] element. Currently this is
    /// unimplemented.
    ///
    /// [`Member`]: crate::Member
    /// [`Room`]: crate::Room
    #[display(fmt = "Expected element of type [{}]. Id [{}]", _0, _1)]
    #[from(ignore)]
    ExpectedOtherElement(&'static str, String),

    /// Element is [`None`], but expected [`Some`].
    #[display(fmt = "Element is None, expected Some.")]
    EmptyElement,

    /// Element is [`None`], but expected [`Some`].
    #[display(fmt = "Element is None, expected Some. Id [{}]", _0)]
    #[from(ignore)]
    EmptyElementId(#[error(not(source))] String),

    /// Error while [`CallbackUrl`] parsing.
    #[display(fmt = "Error while parsing gRPC callback URL: {}", _0)]
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

    /// Error while parsing [`Fid`].
    ///
    /// [`Fid`]: crate::Fid
    #[display(fmt = "Error while parsing FID: {}", _0)]
    ParseFidError(ParseFidError),

    /// Error while formatting [`DateTime`].
    ///
    /// [`DateTime`]: time::OffsetDateTime
    #[display(fmt = "Error while formatting DateTime: {}", _0)]
    TimeFormatError(TimeFormatError),

    /// Error while parsing [`DateTime`].
    ///
    /// [`DateTime`]: time::OffsetDateTime
    #[display(fmt = "Error while parsing DateTime: {}", _0)]
    TimeParseError(TimeParseError),

    /// API call is unimplemented.
    #[display(fmt = "API call is unimplemented")]
    UnimplementedCall,
}

impl From<TryFromProtobufError> for tonic::Status {
    fn from(err: TryFromProtobufError) -> Self {
        match &err {
            TryFromProtobufError::LocalSrcUriParseError(_)
            | TryFromProtobufError::ExpectedOtherElement(_, _)
            | TryFromProtobufError::EmptyElement
            | TryFromProtobufError::EmptyElementId(_)
            | TryFromProtobufError::CallbackUrlParseErr(_)
            | TryFromProtobufError::NegativeDuration(_, _)
            | TryFromProtobufError::ParseFidError(_)
            | TryFromProtobufError::TimeFormatError(_)
            | TryFromProtobufError::TimeParseError(_) => {
                tonic::Status::invalid_argument(err.to_string())
            }
            TryFromProtobufError::UnimplementedCall => {
                tonic::Status::unimplemented(err.to_string())
            }
        }
    }
}
