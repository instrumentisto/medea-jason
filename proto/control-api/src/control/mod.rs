//! [`ControlApi`] definitions.
//!
//! [`ControlApi`]: Api

pub mod endpoint;
pub mod member;
pub mod room;

use std::collections::HashMap;

use async_trait::async_trait;
use derive_more::Display;

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
    async fn create_room(&self, spec: Room) -> Result<Sids, ErrorResponse>;

    /// Applies changes to the already existing [`Room`], or creates a new one
    /// in case there is no [`Room`] with the provided [`Id`].
    ///
    /// [`Id`]: room::Id
    async fn apply_room(&self, spec: Room) -> Result<Sids, ErrorResponse>;

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
    ) -> Result<Sids, ErrorResponse>;

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
    ) -> Result<Sids, ErrorResponse>;

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
    ) -> Result<Sids, ErrorResponse>;

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
    ) -> Result<Sids, ErrorResponse>;

    /// Deletes [`Elements`] with provided [`StatefulFid`]s.
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::NoElement`] if `fids` is empty;
    /// - With [`ErrorCode::ElementIdMismatch`] if `fids` contains multiple
    ///   [`room::Id`]s.
    async fn delete_elements(
        &self,
        fids: Vec<StatefulFid>,
    ) -> Result<Sids, ErrorResponse>;

    /// Returns [`Elements`] by their [`StatefulFid`]s.
    ///
    /// # Errors
    ///
    /// - With [`ErrorCode::RoomNotFound`], [`ErrorCode::MemberNotFound`] or
    ///   [`ErrorCode::EndpointNotFound`] if an [`Element`] with the provided
    ///   `ID` doesn't exist.
    async fn get_elements(
        &self,
        fids: Vec<StatefulFid>,
    ) -> Result<Elements, ErrorResponse>;

    /// Checks media server healthiness.
    async fn healthz(&self, ping: Ping) -> Result<Pong, ErrorResponse>;
}

/// [`Element`]s returned from [`ControlApi::get_elements()`].
///
/// [`ControlApi::get_elements()`]: Api::get_elements()
pub type Elements = HashMap<StatefulFid, Element>;

/// [`Sid`]s used by [`Member`]s to connect to a [Medea] server via
/// [Client API].
///
/// [`Sid`]: member::Sid
/// [Client Api]: https://tinyurl.com/266y74tf
/// [Medea]: https://git.instrumentisto.com/streaming/medea
pub type Sids = HashMap<member::Id, member::Sid>;

/// Single element returned by [`ControlApi::get_elements()`].
///
/// [`ControlApi::get_elements()`]: Api::get_elements()
#[derive(Clone, Debug)]
pub enum Element {
    /// [`Room`] element.
    Room(Room),

    /// [`Member`] element.
    Member(Member),

    /// [`Endpoint`] element.
    Endpoint(Endpoint),
}

/// `FID` (full `ID`, or `fid` in [`ControlApi`]) is a composition of media
/// [`Element`]s `ID`s, which refers to some media element on a whole server
/// uniquely.
///
/// [`ControlApi`]: Api
#[derive(Clone, Debug)]
pub enum StatefulFid {
    /// [`Room`]'s `FID`.
    Room {
        /// Unique [`Room`] `ID`.
        id: room::Id,
    },

    /// [`Member`]'s `FID`.
    Member {
        /// Unique [`Member`] `ID`.
        id: member::Id,

        /// Unique [`Room`] `ID`.
        room_id: room::Id,
    },

    /// [`Endpoint`]s `FID`.
    Endpoint {
        /// Unique [`Endpoint`] `ID`.
        id: endpoint::Id,

        /// Unique [`Room`] `ID`.
        room_id: room::Id,

        /// Unique [`Member`] `ID`.
        member_id: member::Id,
    },
}

/// [`Ping`] message received by media server periodically for probing its
/// healthiness.
#[derive(Clone, Copy, Debug)]
pub struct Ping(pub u32);

/// [`Pong`] message send by media server in response to received [`Ping`]
/// message.
#[derive(Clone, Copy, Debug)]
pub struct Pong(pub u32);

/// Medea's [`ControlApi`] error response.
///
/// [`ControlApi`]: Api
#[derive(Clone, Debug)]
pub struct ErrorResponse {
    /// [`ErrorCode`] which will be returned with code and message.
    pub error_code: ErrorCode,

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
            error_code,
            element_id: Some(element_id.to_string()),
            explanation: None,
        }
    }

    /// New [`ErrorResponse`] only with [`ErrorCode`].
    #[must_use]
    pub const fn without_id(error_code: ErrorCode) -> Self {
        Self {
            error_code,
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
            error_code: ErrorCode::UnexpectedError,
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
            error_code,
            explanation: Some(explanation),
            element_id: id,
        }
    }
}

/// [Medea]'s [`ControlApi`] errors.
///
/// [`ControlApi`]: Api
/// [Medea]: https://git.instrumentisto.com/streaming/medea
#[derive(Clone, Copy, Debug, Display)]
#[repr(u16)]
pub enum ErrorCode {
    /// Unimplemented API call.
    ///
    /// This code should be with additional text which explains what
    /// exactly unimplemented (you can do it with
    /// [`ErrorResponse::with_explanation`] function).
    ///
    /// Code: __1000__.
    #[display(fmt = "Unimplemented API call.")]
    UnimplementedCall = 1000,

    /// Request doesn't contain any [`Elements`].
    ///
    /// Code: __1001__.
    #[display(fmt = "Request doesn't contain any elements")]
    NoElement = 1001,

    /// Provided `FID` can't point to provided element.
    ///
    /// Code: __1002__.
    #[display(fmt = "Provided fid can't point to provided element")]
    ElementIdMismatch = 1002,

    /// [`Room`] not found.
    ///
    /// Code: __1003__.
    #[display(fmt = "Room not found.")]
    RoomNotFound = 1003,

    /// [`Member`] not found.
    ///
    /// Code: __1004__.
    #[display(fmt = "Member not found.")]
    MemberNotFound = 1004,

    /// [`Endpoint`] not found.
    ///
    /// Code: __1005__.
    #[display(fmt = "Endpoint not found.")]
    EndpointNotFound = 1005,

    /// Medea expects [`Room`] element in pipeline but received not him.
    ///
    /// Code: __1006__.
    #[display(fmt = "Expecting Room element but it's not.")]
    NotRoomInSpec = 1006,

    /// Medea expects [`Member`] element in pipeline but received not him.
    ///
    /// Code: __1007__.
    #[display(fmt = "Expected Member element but it's not.")]
    NotMemberInSpec = 1007,

    /// Invalid source URI in [`endpoint::WebRtcPlay`].
    ///
    /// Code: __1008__.
    #[display(fmt = "Invalid source URI in 'WebRtcPlayEndpoint'.")]
    InvalidSrcUri = 1008,

    /// Provided not source URI in [`endpoint::WebRtcPlay`].
    ///
    /// Code: __1009__.
    #[display(fmt = "Provided not source URI in 'WebRtcPlayEndpoint'.")]
    NotSourceUri = 1009,

    /// Element's URI don't have `local://` prefix.
    ///
    /// Code: __1010__.
    #[display(fmt = "Element's URI don't have 'local://' prefix.")]
    ElementIdIsNotLocal = 1010,

    /// Provided element's `FID`/`URI` with too many paths.
    ///
    /// Code: __1011__.
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
    /// ([`ErrorResponse::unexpected`] do this).
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
