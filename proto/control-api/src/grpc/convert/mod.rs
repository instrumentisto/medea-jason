//! Conversions between crate types and the ones generated from [gRPC] specs.
//!
//! [gRPC]: https://grpc.io

mod api;
mod callback;

use derive_more::with_trait::{Display, Error, From};

use crate::{
    control::ParseFidError, endpoint::web_rtc_play::LocalSrcUriParseError,
};

use super::CallbackUrlParseError;

/// Possible errors of deserializing types from [gRPC] spec.
///
/// [gRPC]: https://grpc.io
#[derive(Debug, Display, Error, From)]
pub enum ProtobufError {
    /// Error of parsing a [`LocalSrcUri`] of a [`WebRtcPlay`] media
    /// [`Element`].
    ///
    /// [`Element`]: crate::Element
    /// [`LocalSrcUri`]: crate::endpoint::web_rtc_play::LocalSrcUri
    /// [`WebRtcPlay`]: crate::endpoint::WebRtcPlay
    #[display("Source URI parse error: {_0}")]
    LocalSrcUriParseErr(LocalSrcUriParseError),

    /// [`Element`] is expected to be of another type.
    ///
    /// [`Element`]: crate::Element
    #[display("`{_1}` media element expected to be of type `{_0}`")]
    #[from(ignore)]
    ExpectedElement(&'static str, Box<str>),

    /// [`Element`] is expected to be specified.
    ///
    /// [`Element`]: crate::Element
    #[display("Expected media element, but none specified")]
    NoElement,

    /// [`Element`] is expected to be specified for a [`Fid`].
    ///
    /// [`Element`]: crate::Element
    /// [`Fid`]: crate::Fid
    #[display("Expected media element for `{_0}`, but none specified")]
    #[from(ignore)]
    NoElementForId(#[error(not(source))] Box<str>),

    /// Error of parsing a [`CallbackUrl`].
    ///
    /// [`CallbackUrl`]: super::CallbackUrl
    #[display("gRPC callback URL parse error: {_0}")]
    CallbackUrlParseErr(CallbackUrlParseError),

    /// Some [`Element`] specifies invalid [`Duration`].
    ///
    /// [`Duration`]: std::time::Duration
    /// [`Element`]: crate::Element
    #[display(
        "`Element(id: {_0})` specifies field `{_1}` with invalid duration"
    )]
    #[from(ignore)]
    InvalidDuration(Box<str>, &'static str),

    /// Error of parsing a [`Fid`].
    ///
    /// [`Fid`]: crate::Fid
    #[display("FID parse error: {_0}")]
    ParseFidErr(ParseFidError),

    /// Error of parsing a [`DateTime`].
    ///
    /// [`DateTime`]: time::OffsetDateTime
    #[display("`DateTime` parse error: {_0}")]
    TimeParseErr(time::error::Parse),

    /// Such API call is unimplemented.
    #[display("API call is unimplemented")]
    Unimplemented,
}

impl From<ProtobufError> for tonic::Status {
    fn from(err: ProtobufError) -> Self {
        match &err {
            ProtobufError::LocalSrcUriParseErr(_)
            | ProtobufError::ExpectedElement(_, _)
            | ProtobufError::NoElement
            | ProtobufError::NoElementForId(_)
            | ProtobufError::CallbackUrlParseErr(_)
            | ProtobufError::InvalidDuration(_, _)
            | ProtobufError::ParseFidErr(_)
            | ProtobufError::TimeParseErr(_) => {
                Self::invalid_argument(err.to_string())
            }
            ProtobufError::Unimplemented => {
                Self::unimplemented(err.to_string())
            }
        }
    }
}
