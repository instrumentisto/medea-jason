//! DTOs for the Medea's Control API mock server.

#![allow(clippy::module_name_repetitions)]
#![forbid(non_ascii_idents, unsafe_code)]

pub mod callback;
pub mod endpoint;
pub mod member;
pub mod room;

use std::collections::HashMap;

use derive_more::From;
use serde::{Deserialize, Serialize};

pub use crate::{
    endpoint::{
        AudioSettings, Endpoint, P2pMode, PublishPolicy, VideoSettings, WebRtcPlayEndpoint,
        WebRtcPublishEndpoint,
    },
    member::{Credentials, Member},
    room::{Room, RoomElement},
};

/// Response which returns sids.
///
/// Used for create methods.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateResponse {
    /// URIs with which [Jason] can connect `Member`s.
    ///
    /// [Jason]: https://github.com/instrumentisto/medea/tree/master/jason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sids: Option<HashMap<String, String>>,

    /// Error if something happened on [Control API]'s side.
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

/// Response which can return only error (if any).
///
/// Used for delete methods.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    /// Error if something happened on [Control API]'s side.
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

/// Error object. Returns when some error happened on [Control API]'s side.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    /// Medea's Control API error code.
    pub code: u32,

    /// Text of error.
    pub text: String,

    /// Element's ID with which error happened.
    pub element: String,
}

/// Union of all elements which exists in [Medea].
///
/// [Medea]: https://github.com/instrumentisto/medea
#[derive(Debug, Deserialize, From, Serialize)]
#[serde(tag = "kind")]
pub enum Element {
    Member(Member),
    WebRtcPublishEndpoint(WebRtcPublishEndpoint),
    WebRtcPlayEndpoint(WebRtcPlayEndpoint),
    Room(Room),
}

/// Response on request for get `Element` request.
#[derive(Debug, Deserialize, Serialize)]
pub struct SingleGetResponse {
    /// Requested element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<Element>,

    /// [`ErrorResponse`] if some error happened on [Control API]'s side.
    /// Otherwise `None`.
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
