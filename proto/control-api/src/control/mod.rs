//! [`ControlApi`] definitions.
//!
//! [`ControlApi`]: Api

// TODO: Remove once annoying false positive is fixed:
//       https://github.com/rust-lang/rust-clippy/issues/6902
#![allow(clippy::use_self)]

pub mod endpoint;
pub mod member;
pub mod room;

use std::collections::HashMap;

use async_trait::async_trait;
use derive_more::{Display, From};

pub use self::{endpoint::Endpoint, member::Member, room::Room};

/// [Control API] used to control [Medea] server.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
/// [Medea]: https://git.instrumentisto.com/streaming/medea
#[async_trait(?Send)]
pub trait Api {
    /// Creates a new [`Room`].
    ///
    /// # Errors
    ///
    /// With [`ErrorCode::RoomAlreadyExists`] if a [`Room`] with the provided
    /// [`Id`] already exists.
    ///
    /// [`Id`]: room::Id
    async fn create_room(&self, spec: Room) -> Result<member::Sids, ErrorResponse>;

    /// Applies changes to the already existing [`Room`], or creates a new one
    /// in case there is no [`Room`] with the provided [`Id`].
    ///
    /// [`Id`]: room::Id
    async fn apply_room(&self, spec: Room) -> Result<member::Sids, ErrorResponse>;

    /// Creates a new [`Member`] in already existing [`Room`].
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::RoomNotFound`] if a [`Room`] with the provided
    ///   [`room::Id`] doesn't exist;
    /// - With [`ErrorCode::MemberAlreadyExists`] if a [`Member`] with the
    ///   provided [`member::Id`] already exists.
    async fn create_room_member(
        &self,
        room_id: room::Id,
        spec: Member,
    ) -> Result<member::Sids, ErrorResponse>;

    /// Applies changes to the already existing [`Member`], or creates a new one
    /// in case there is no [`Member`] with the provided [`Id`].
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::RoomNotFound`] if a [`Room`] with the provided
    ///   [`room::Id`] doesn't exist.
    ///
    /// [`Id`]: member::Id
    async fn apply_room_member(
        &self,
        room_id: room::Id,
        spec: Member,
    ) -> Result<member::Sids, ErrorResponse>;

    /// Creates a new [`Endpoint`] for already existing [`Member`].
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::RoomNotFound`] if a [`Room`] with the provided
    ///   [`room::Id`] doesn't exist;
    /// - With [`ErrorCode::MemberNotFound`] if a [`Member`] with the provided
    ///   [`member::Id`] doesn't exist;
    /// - With [`ErrorCode::EndpointAlreadyExists`] if an [`Endpoint`] with the
    ///   provided [`endpoint::Id`] already exists.
    async fn create_room_endpoint(
        &self,
        room_id: room::Id,
        member_id: member::Id,
        spec: Endpoint,
    ) -> Result<member::Sids, ErrorResponse>;

    /// Applies changes to the already existing [`Endpoint`], or creates a new
    /// one in case there is no [`Endpoint`] with the provided [`Id`].
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::RoomNotFound`] if a [`Room`] with the provided
    ///   [`room::Id`] doesn't exist;
    /// - With [`ErrorCode::MemberNotFound`] if a [`Member`] with the provided
    ///   [`member::Id`] doesn't exist.
    ///
    /// [`Id`]: endpoint::Id
    async fn apply_room_endpoint(
        &self,
        room_id: room::Id,
        member_id: member::Id,
        spec: Endpoint,
    ) -> Result<member::Sids, ErrorResponse>;

    /// Deletes [`Elements`] with provided [`StatefulFid`]s.
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::NoElement`] if `fids` is empty;
    /// - With [`ErrorCode::ElementIdMismatch`] if `fids` contains multiple
    ///   [`room::Id`]s.
    async fn delete_elements(
        &self,
        fids: Vec<Fid>,
    ) -> Result<member::Sids, ErrorResponse>;

    /// Returns [`Elements`] by their [`StatefulFid`]s.
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::RoomNotFound`], [`ErrorCode::MemberNotFound`] or
    ///   [`ErrorCode::EndpointNotFound`] if an [`Element`] with the provided
    ///   `ID` doesn't exist.
    async fn get_elements(
        &self,
        fids: Vec<Fid>,
    ) -> Result<Elements, ErrorResponse>;

    /// Checks healthiness of this media server.
    async fn healthz(&self, ping: Ping) -> Result<Pong, ErrorResponse>;
}

/// Possible media elements forming a media pipeline.
#[derive(Clone, Debug, From)]
pub enum Element {
    /// [`Room`] media element.
    Room(Room),

    /// [`Member`] media element.
    Member(Member),

    /// [`Endpoint`] media element.
    Endpoint(Endpoint),
}

/// Collection of uniquely identified [`Element`]s.
pub type Elements = HashMap<Fid, Element>;

/// FID (Full ID) is a composition of media [`Element`] IDs referring to some
/// [`Element`] on a whole media server uniquely.
#[derive(Clone, Debug)]
pub enum Fid {
    /// FID of a [`Room`].
    Room {
        /// Unique ID of the [`Room`].
        id: room::Id,
    },

    /// FID of a [`Member`].
    Member {
        /// ID of the [`Member`] in the [`Room`].
        id: member::Id,

        /// Unique ID of the [`Room`].
        room_id: room::Id,
    },

    /// FID of an [`Endpoint`].
    Endpoint {
        /// ID of the [`Endpoint`] of the [`Member`].
        id: endpoint::Id,

        /// Unique ID of the [`Room`].
        room_id: room::Id,

        /// ID of the [`Member`] in the [`Room`].
        member_id: member::Id,
    },
}

/// [`Ping`] message received by a media server periodically for probing its
/// healthiness.
///
/// Each new [`Ping`] should increase its nonce, starting with `0`.
#[derive(Clone, Copy, Debug)]
pub struct Ping(pub u32);

/// [`Pong`] message send by a media server in response to a received [`Ping`]
/// message.
///
/// Contains nonce of the answered [`Ping`] message.
#[derive(Clone, Copy, Debug)]
pub struct Pong(pub u32);

/// Medea's [`ControlApi`] error response.
///
/// [`ControlApi`]: Api
#[derive(Clone, Debug)]
pub struct ErrorResponse {
    /// [`ErrorCode`] which will be returned with code and message.
    pub code: ErrorCode,

    /// [`Element`] `ID` where some error happened. May be empty.
    pub element_id: Option<String>,

    /// All [`ErrorCode`]s have [`Display`] implementation. And this
    /// implementation will be used if this field is [`None`]. But
    /// sometimes we want to add some error explanation. Then we set this
    /// field to [`Some`] and this text will be added to
    /// [`Display`] implementation's text.
    ///
    /// By default this field should be [`None`].
    ///
    /// For providing error explanation use [`ErrorResponse::with_explanation`]
    /// method.
    ///
    /// [`Display`]: std::fmt::Display
    pub explanation: Option<String>,
}

impl ErrorResponse {
    /// New [`ErrorResponse`] with [`ErrorCode`] and [`Element`] `ID`.
    pub fn new<T: ToString>(error_code: ErrorCode, element_id: &T) -> Self {
        Self {
            code: error_code,
            element_id: Some(element_id.to_string()),
            explanation: None,
        }
    }

    /// New [`ErrorResponse`] only with [`ErrorCode`].
    #[must_use]
    pub const fn without_id(error_code: ErrorCode) -> Self {
        Self {
            code: error_code,
            element_id: None,
            explanation: None,
        }
    }

    /// [`ErrorResponse`] for all unexpected errors.
    ///
    /// Provide unexpected `Error` to this function.
    /// This error will be printed with [`Display`] implementation
    /// of provided `Error` as error explanation.
    ///
    /// [`Display`]: std::fmt::Display
    pub fn unexpected<B: ToString>(unknown_error: &B) -> Self {
        Self {
            code: ErrorCode::UnexpectedError,
            explanation: Some(unknown_error.to_string()),
            element_id: None,
        }
    }

    /// [`ErrorResponse`] with some additional info.
    ///
    /// With this method you can add additional text to error message of
    /// [`ErrorCode`].
    #[must_use]
    pub const fn with_explanation(
        error_code: ErrorCode,
        explanation: String,
        id: Option<String>,
    ) -> Self {
        Self {
            code: error_code,
            explanation: Some(explanation),
            element_id: id,
        }
    }
}

/// Codes of possible [`ControlApi`] errors.
///
/// [`ControlApi`]: Api
#[derive(Clone, Copy, Debug, Display)]
#[repr(u16)]
pub enum ErrorCode {
    /// Unimplemented API call.
    ///
    /// This code should go with additional explanation of what exactly is
    /// unimplemented (see [`ErrorResponse::explanation()`]).
    ///
    /// Code: `1000`
    #[display(fmt = "Unimplemented API call")]
    UnimplementedCall = 1000,

    /// Request doesn't contain any [`Element`]s.
    ///
    /// Code: `1001`
    #[display(fmt = "Request doesn't contain any `Element`s")]
    NoElement = 1001,

    /// Provided [`Fid`] cannot point to the provided [`Element`].
    ///
    /// Code: `1002`
    #[display(fmt = "Provided FID cannot point to provided `Element`")]
    ElementIdMismatch = 1002,

    /// [`Room`] not found.
    ///
    /// Code: `1003`
    #[display(fmt = "`Room` not found")]
    RoomNotFound = 1003,

    /// [`Member`] not found.
    ///
    /// Code: `1004`
    #[display(fmt = "`Member` not found")]
    MemberNotFound = 1004,

    /// [`Endpoint`] not found.
    ///
    /// Code: `1005`
    #[display(fmt = "`Endpoint` not found")]
    EndpointNotFound = 1005,

    /// [`Room`] element is expected in pipeline.
    ///
    /// Code: `1006`
    #[display(fmt = "Expecting `Room` element but it's not")]
    NotRoomInSpec = 1006,

    /// [`Member`] element is expected in pipeline.
    ///
    /// Code: `1007`
    #[display(fmt = "Expecting `Member` element but it's not")]
    NotMemberInSpec = 1007,

    /// Invalid source [URI] in [`endpoint::WebRtcPlay`].
    ///
    /// Code: `1008`
    ///
    /// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
    #[display(fmt = "Invalid source URI in `WebRtcPlayEndpoint`")]
    InvalidSrcUri = 1008,

    /// Provided not source [URI] in [`endpoint::WebRtcPlay`].
    ///
    /// Code: `1009`
    ///
    /// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
    #[display(fmt = "Provided not source URI in `WebRtcPlayEndpoint`")]
    NotSourceUri = 1009,

    /// [`Element`]'s [URI] doesn't have `local://` scheme.
    ///
    /// Code: `1010`
    ///
    /// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
    #[display(fmt = "Element's URI don't have 'local://' prefix.")]
    ElementIdIsNotLocal = 1010,

    /// Provided element's `FID`/`URI` with too many paths.
    ///
    /// Code: `1011`
    #[display(fmt = "You provided element's FID/URI with too many paths.")]
    ElementIdIsTooLong = 1011,

    /// Missing some fields in source `URI` of [`endpoint::WebRtcPublish`].
    ///
    /// Code: __1012__.
    #[display(
        fmt = "Missing some fields in source URI of WebRtcPublishEndpoint."
    )]
    MissingFieldsInSrcUri = 1012,

    /// Empty [`Element`] ID.
    ///
    /// Code: __1013__.
    #[display(fmt = "Provided empty element ID.")]
    EmptyElementId = 1013,

    /// Provided empty elements `FID`s list.
    ///
    /// Code: __1014__.
    #[display(fmt = "Provided empty elements FIDs list.")]
    EmptyElementsList = 1014,

    /// Provided not the same [`room::Id`]s in elements `ID`s. Probably you try
    /// use [`ControlApi::delete_elements()`] method for elements with
    /// different [`room::Id`]s.
    ///
    /// Code: __1015__.
    ///
    /// [`ControlApi::delete_elements()`]: Api::delete_elements()
    #[display(fmt = "Provided not the same Room IDs in elements IDs. \
                     Probably you try use 'Delete' method for elements with \
                     different Room IDs")]
    ProvidedNotSameRoomIds = 1015,

    /// [`Room`] with provided [`Id`] already exists.
    ///
    /// Code: __1016__.
    ///
    /// [`Id`]: room::Id
    #[display(fmt = "Room with provided FID already exists.")]
    RoomAlreadyExists = 1016,

    /// [`Member`] with provided [`Id`] already exists.
    ///
    /// Code: __1017__.
    ///
    /// [`Id`]: member::Id
    #[display(fmt = "Member with provided FID already exists.")]
    MemberAlreadyExists = 1017,

    /// [`Endpoint`] with provided [`Id`] already exists.
    ///
    /// Code: __1018__.
    ///
    /// [`Id`]: endpoint::Id
    #[display(fmt = "Endpoint with provided FID already exists.")]
    EndpointAlreadyExists = 1018,

    /// Missing path in some reference to the Medea element.
    ///
    /// Code: __1019__.
    #[display(fmt = "Missing path in some reference to the Medea element.")]
    MissingPath = 1019,

    /// Missing host in callback `URL`.
    ///
    /// Code: __1020__.
    #[display(fmt = "Missing host in callback URL.")]
    MissingHostInCallbackUrl = 1020,

    /// Unsupported callback `URL` protocol.
    ///
    /// Code: __1021__.
    #[display(fmt = "Unsupported callback URL protocol.")]
    UnsupportedCallbackUrlProtocol = 1021,

    /// Invalid callback `URL`.
    ///
    /// Code: __1022__.
    #[display(fmt = "Invalid callback URL.")]
    InvalidCallbackUrl = 1022,

    /// Encountered negative duration.
    ///
    /// Code: __1023__.
    #[display(fmt = "Encountered negative duration")]
    NegativeDuration = 1023,

    /// Unexpected server error.
    ///
    /// Use this [`ErrorCode`] only with [`ErrorResponse::unexpected`]
    /// function. In error text with this code should be error message
    /// which explain what exactly goes wrong
    /// ([`ErrorResponse::unexpected`] does this).
    ///
    /// Code: __2000__.
    #[display(fmt = "Unexpected error happened.")]
    UnexpectedError = 2000,
}

impl From<ErrorCode> for u16 {
    #[allow(clippy::as_conversions)]
    fn from(code: ErrorCode) -> Self {
        code as Self
    }
}

impl From<ErrorCode> for u32 {
    fn from(code: ErrorCode) -> Self {
        u16::from(code).into()
    }
}
