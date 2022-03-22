//! External API errors.

// TODO: See https://github.com/rustwasm/wasm-bindgen/pull/2719
#![allow(clippy::use_self)]

use std::borrow::Cow;

#[cfg(not(target_os = "windows"))]
use wasm_bindgen::prelude::wasm_bindgen;

use tracerr::{Trace, Traced};

use crate::{
    api::Error,
    connection,
    media::{
        self, EnumerateDevicesError, GetDisplayMediaError, GetUserMediaError,
        InitLocalTracksError, InvalidOutputAudioDeviceIdError,
    },
    peer::{
        sender::CreateError, InsertLocalTracksError, LocalMediaError,
        UpdateLocalStreamError,
    },
    platform,
    room::{
        self, ChangeMediaStateError, ConstraintsUpdateError, RoomJoinError,
    },
    rpc::{rpc_session::ConnectionLostReason, ReconnectError, SessionError},
    utils::Caused,
};

/// Error thrown when the operation wasn't allowed by the current state of the
/// object.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct StateError {
    /// Message describing the problem.
    message: Cow<'static, str>,

    /// Stacktrace of this [`StateError`].
    trace: Trace,
}

impl StateError {
    /// Creates a new [`StateError`] with the provided `message` and `trace`.
    #[must_use]
    pub fn new<T: Into<Cow<'static, str>>>(message: T, trace: Trace) -> Self {
        Self {
            message: message.into(),
            trace,
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl StateError {
    /// Returns message describing the problem.
    #[must_use]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    /// Returns native stacktrace of this [`StateError`].
    #[must_use]
    pub fn trace(&self) -> String {
        self.trace.to_string()
    }
}

/// Possible error kinds of a [`LocalMediaInitException`].
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum LocalMediaInitExceptionKind {
    /// Occurs if the [getUserMedia()][1] request failed.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    GetUserMediaFailed,

    /// Occurs if the [getDisplayMedia()][1] request failed.
    ///
    /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    GetDisplayMediaFailed,

    /// Occurs when local track is [`ended`][1] right after [getUserMedia()][2]
    /// or [getDisplayMedia()][3] request.
    ///
    /// [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    /// [2]: https://tinyurl.com/rnxcavf
    /// [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    LocalTrackIsEnded,
}

/// Exception thrown when accessing media devices.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct LocalMediaInitException {
    /// Concrete error kind of this [`LocalMediaInitException`].
    kind: LocalMediaInitExceptionKind,

    /// Error message describing the problem.
    message: Cow<'static, str>,

    /// [`platform::Error`] causing this [`LocalMediaInitException`].
    cause: Option<platform::Error>,

    /// Stacktrace of this [`LocalMediaInitException`].
    trace: Trace,
}

impl LocalMediaInitException {
    /// Creates a new [`LocalMediaInitException`] from the provided error
    /// `kind`, `message`, optional `cause` and `trace`.
    #[must_use]
    pub fn new<M: Into<Cow<'static, str>>>(
        kind: LocalMediaInitExceptionKind,
        message: M,
        cause: Option<platform::Error>,
        trace: Trace,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            cause,
            trace,
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl LocalMediaInitException {
    /// Returns concrete error kind of this [`LocalMediaInitException`].
    #[must_use]
    pub fn kind(&self) -> LocalMediaInitExceptionKind {
        self.kind
    }

    /// Returns an error message describing the problem.
    #[must_use]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    /// Returns [`platform::Error`] causing this [`LocalMediaInitException`].
    #[must_use]
    pub fn cause(&self) -> Option<platform::Error> {
        self.cause.clone()
    }

    /// Returns stacktrace of this [`LocalMediaInitException`].
    #[must_use]
    pub fn trace(&self) -> String {
        self.trace.to_string()
    }
}

/// Exception thrown when cannot get info of available media devices.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct EnumerateDevicesException {
    /// [`platform::Error`] causing this [`EnumerateDevicesException`].
    cause: platform::Error,

    /// Stacktrace of this [`EnumerateDevicesException`].
    trace: Trace,
}

impl EnumerateDevicesException {
    /// Creates a new [`EnumerateDevicesException`] from the provided error
    /// `cause` and `trace`.
    #[must_use]
    pub fn new(cause: platform::Error, trace: Trace) -> Self {
        Self { cause, trace }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl EnumerateDevicesException {
    /// Returns [`platform::Error`] causing this [`EnumerateDevicesException`].
    #[must_use]
    pub fn cause(&self) -> platform::Error {
        self.cause.clone()
    }

    /// Returns stacktrace of this [`EnumerateDevicesException`].
    #[must_use]
    pub fn trace(&self) -> String {
        self.trace.to_string()
    }
}

/// Exception thrown when cannot change output audio device ID.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct InvalidOutputAudioDeviceIdException {
    /// Stacktrace of this [`InvalidOutputAudioDeviceIdException`].
    trace: Trace,
}

impl InvalidOutputAudioDeviceIdException {
    /// Creates a new [`InvalidOutputAudioDeviceIdException`] from the provided
    /// error [`Trace`].
    #[must_use]
    pub fn new(trace: Trace) -> Self {
        Self { trace }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl InvalidOutputAudioDeviceIdException {
    /// Returns stacktrace of this [`InvalidOutputAudioDeviceIdException`].
    #[must_use]
    pub fn trace(&self) -> String {
        self.trace.to_string()
    }
}

/// Possible error kinds of a [`RpcClientException`].
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RpcClientExceptionKind {
    /// Connection with a server was lost.
    ///
    /// This usually means that some transport error occurred, so a client can
    /// continue performing reconnecting attempts.
    ConnectionLost,

    /// Could not authorize an RPC session.
    ///
    /// This usually means that authentication data a client provides is
    /// obsolete.
    AuthorizationFailed,

    /// RPC session has been finished. This is a terminal state.
    SessionFinished,
}

/// Exceptions thrown from a RPC client that implements messaging with media
/// server.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct RpcClientException {
    /// Concrete error kind of this [`RpcClientException`].
    kind: RpcClientExceptionKind,

    /// Error message describing the problem.
    message: Cow<'static, str>,

    /// [`platform::Error`] causing this [`RpcClientException`].
    cause: Option<platform::Error>,

    /// Stacktrace of this [`RpcClientException`].
    trace: Trace,
}

impl RpcClientException {
    /// Creates a new [`RpcClientException`] from the provided error `kind`,
    /// `message`, optional `cause` and `trace`.
    #[must_use]
    pub fn new<M: Into<Cow<'static, str>>>(
        kind: RpcClientExceptionKind,
        message: M,
        cause: Option<platform::Error>,
        trace: Trace,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            cause,
            trace,
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl RpcClientException {
    /// Returns concrete error kind of this [`RpcClientException`].
    #[must_use]
    pub fn kind(&self) -> RpcClientExceptionKind {
        self.kind
    }

    /// Returns an error message describing the problem.
    #[must_use]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    /// Returns [`platform::Error`] causing this [`RpcClientException`].
    #[must_use]
    pub fn cause(&self) -> Option<platform::Error> {
        self.cause.clone()
    }

    /// Returns stacktrace of this [`RpcClientException`].
    #[must_use]
    pub fn trace(&self) -> String {
        self.trace.to_string()
    }
}

/// Jason's internal exception.
///
/// This is either a programmatic error or some unexpected platform component
/// failure that cannot be handled in any way.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct InternalException {
    /// Error message describing the problem.
    message: Cow<'static, str>,

    /// [`platform::Error`] causing this [`RpcClientException`].
    cause: Option<platform::Error>,

    /// Stacktrace of this [`InternalException`].
    trace: Trace,
}

impl InternalException {
    /// Creates a new [`InternalException`] from the provided error `message`,
    /// `trace` and an optional `cause`.
    #[must_use]
    pub fn new<T: Into<Cow<'static, str>>>(
        message: T,
        cause: Option<platform::Error>,
        trace: Trace,
    ) -> Self {
        Self {
            message: message.into(),
            trace,
            cause,
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl InternalException {
    /// Returns an error message describing the problem.
    #[must_use]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    /// Returns [`platform::Error`] causing this [`RpcClientException`].
    #[must_use]
    pub fn cause(&self) -> Option<platform::Error> {
        self.cause.clone()
    }

    /// Returns stacktrace of this [`InternalException`].
    #[must_use]
    pub fn trace(&self) -> String {
        self.trace.to_string()
    }
}

/// Exception thrown when a string or some other data doesn't have an expected
/// format and cannot be parsed or processed.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct FormatException(Cow<'static, str>);

impl FormatException {
    /// Creates a new [`FormatException`] with the provided `message` describing
    /// the problem.
    #[must_use]
    pub fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Self(message.into())
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl FormatException {
    /// Returns an error message describing of the problem.
    #[must_use]
    pub fn message(&self) -> String {
        self.0.to_string()
    }
}

/// Kind of a [`MediaStateTransitionException`].
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Clone, Copy, Debug)]
pub enum MediaStateTransitionExceptionKind {
    /// Media state of a [`Sender`] transits to an opposite of the requested
    /// one.
    ///
    /// [`Sender`]: crate::peer::media::Sender
    OppositeState,

    /// Requested state transition is not allowed by [`Sender`]'s settings.
    ///
    /// [`Sender`]: crate::peer::media::Sender
    ProhibitedState,
}

/// Exception thrown when the requested media state transition could not be
/// performed.
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct MediaStateTransitionException {
    /// Error message describing the problem.
    message: Cow<'static, str>,

    /// Concrete error kind of this [`MediaStateTransitionException`].
    kind: MediaStateTransitionExceptionKind,

    /// Stacktrace of this [`MediaStateTransitionException`].
    trace: Trace,
}

impl MediaStateTransitionException {
    /// Creates a new [`MediaStateTransitionException`] from the provided error
    /// `message` and `trace`.
    #[must_use]
    pub fn new<T: Into<Cow<'static, str>>>(
        message: T,
        trace: Trace,
        kind: MediaStateTransitionExceptionKind,
    ) -> Self {
        Self {
            message: message.into(),
            trace,
            kind,
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl MediaStateTransitionException {
    /// Returns an error message describing the problem.
    #[must_use]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    /// Returns stacktrace of this [`MediaStateTransitionException`].
    #[must_use]
    pub fn trace(&self) -> String {
        self.trace.to_string()
    }

    /// Returns concrete error kind of this [`MediaStateTransitionException`].
    #[must_use]
    pub fn kind(&self) -> MediaStateTransitionExceptionKind {
        self.kind
    }
}

/// Errors occurring in [`RoomHandle::set_local_media_settings()`][1] method.
///
/// [1]: crate::api::RoomHandle::set_local_media_settings
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
#[derive(Debug)]
pub struct MediaSettingsUpdateException {
    /// Error message describing the problem.
    message: Cow<'static, str>,

    /// Original [`ChangeMediaStateError`] that was encountered while updating
    /// local media settings.
    cause: Traced<ChangeMediaStateError>,

    /// Whether media settings were successfully rolled back after new settings
    /// application failed.
    rolled_back: bool,
}

impl MediaSettingsUpdateException {
    /// Creates a new [`MediaSettingsUpdateException`] from the provided error
    /// `message`, `cause` and `rolled_back` property.
    #[must_use]
    pub fn new<T: Into<Cow<'static, str>>>(
        message: T,
        cause: Traced<ChangeMediaStateError>,
        rolled_back: bool,
    ) -> Self {
        Self {
            message: message.into(),
            rolled_back,
            cause,
        }
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_unit))]
#[cfg_attr(not(target_os = "windows"), wasm_bindgen)]
impl MediaSettingsUpdateException {
    /// Returns an error message describing the problem.
    #[must_use]
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    /// Returns the original [`ChangeMediaStateError`] that was encountered
    /// while updating local media settings.
    #[must_use]
    pub fn cause(&self) -> Error {
        self.cause.clone().into()
    }

    /// Returns whether media settings were successfully rolled back after new
    /// settings application failed.
    #[must_use]
    pub fn rolled_back(&self) -> bool {
        self.rolled_back
    }
}

impl From<Traced<media::HandleDetachedError>> for Error {
    fn from(err: Traced<media::HandleDetachedError>) -> Self {
        let (err, trace) = err.split();
        StateError::new(err.to_string(), trace).into()
    }
}

impl From<Traced<connection::HandleDetachedError>> for Error {
    fn from(err: Traced<connection::HandleDetachedError>) -> Self {
        let (err, trace) = err.split();
        StateError::new(err.to_string(), trace).into()
    }
}

impl From<Traced<room::HandleDetachedError>> for Error {
    fn from(err: Traced<room::HandleDetachedError>) -> Self {
        let (err, trace) = err.split();
        StateError::new(err.to_string(), trace).into()
    }
}

impl From<Traced<EnumerateDevicesError>> for Error {
    fn from(err: Traced<EnumerateDevicesError>) -> Self {
        let (err, stacktrace) = err.split();
        match err {
            EnumerateDevicesError::Failed(err) => {
                EnumerateDevicesException::new(err, stacktrace).into()
            }
            EnumerateDevicesError::Detached => {
                StateError::new(err.to_string(), stacktrace).into()
            }
        }
    }
}

impl From<Traced<InvalidOutputAudioDeviceIdError>> for Error {
    fn from(err: Traced<InvalidOutputAudioDeviceIdError>) -> Self {
        let (_, trace) = err.split();
        InvalidOutputAudioDeviceIdException::new(trace).into()
    }
}

impl From<Traced<InitLocalTracksError>> for Error {
    fn from(err: Traced<InitLocalTracksError>) -> Self {
        use GetDisplayMediaError as Gdm;
        use GetUserMediaError as Gum;
        use InitLocalTracksError as Err;
        use LocalMediaInitExceptionKind as Kind;

        let (err, stacktrace) = err.split();
        let message = err.to_string();

        let (kind, cause) = match err {
            Err::Detached => {
                return StateError::new(message, stacktrace).into()
            }
            Err::GetUserMediaFailed(Gum::PlatformRequestFailed(cause)) => {
                (Kind::GetUserMediaFailed, Some(cause))
            }
            Err::GetDisplayMediaFailed(Gdm::PlatformRequestFailed(cause)) => {
                (Kind::GetDisplayMediaFailed, Some(cause))
            }
            Err::GetUserMediaFailed(Gum::LocalTrackIsEnded(_))
            | Err::GetDisplayMediaFailed(Gdm::LocalTrackIsEnded(_)) => {
                (Kind::LocalTrackIsEnded, None)
            }
        };

        LocalMediaInitException::new(kind, message, cause, stacktrace).into()
    }
}

impl From<Traced<ReconnectError>> for Error {
    fn from(err: Traced<ReconnectError>) -> Self {
        let (err, trace) = err.split();

        match err {
            ReconnectError::Detached => {
                StateError::new(err.to_string(), trace).into()
            }
            ReconnectError::Session(err) => Traced::compose(err, trace).into(),
        }
    }
}

impl From<Traced<SessionError>> for Error {
    fn from(err: Traced<SessionError>) -> Self {
        use ConnectionLostReason as Reason;
        use RpcClientExceptionKind as Kind;
        use SessionError as SE;

        let (err, trace) = err.split();
        let message = err.to_string();

        let mut cause = None;
        let kind = match err {
            SE::SessionFinished(_) => Some(Kind::SessionFinished),
            SE::NoCredentials
            | SE::SessionUnexpectedlyDropped
            | SE::NewConnectionInfo => None,
            SE::RpcClient(e) => {
                cause = e.cause();
                None
            }
            SE::AuthorizationFailed => Some(Kind::AuthorizationFailed),
            SE::ConnectionLost(reason) => {
                if let Reason::ConnectError(e) = reason {
                    cause = e.into_inner().cause();
                };
                Some(Kind::ConnectionLost)
            }
        };

        if let Some(rpc_kind) = kind {
            RpcClientException::new(rpc_kind, message, cause, trace).into()
        } else {
            InternalException::new(message, cause, trace).into()
        }
    }
}

impl From<Traced<RoomJoinError>> for Error {
    fn from(err: Traced<RoomJoinError>) -> Self {
        let (err, trace) = err.split();
        let message = err.to_string();

        match err {
            RoomJoinError::Detached | RoomJoinError::CallbackNotSet(_) => {
                StateError::new(message, trace).into()
            }
            RoomJoinError::ConnectionInfoParse(_) => {
                FormatException::new(message).into()
            }
            RoomJoinError::SessionError(err) => {
                Traced::compose(err, trace).into()
            }
        }
    }
}

impl From<Traced<ChangeMediaStateError>> for Error {
    fn from(err: Traced<ChangeMediaStateError>) -> Self {
        let (err, trace) = err.split();
        let message = err.to_string();

        match err {
            ChangeMediaStateError::Detached => {
                StateError::new(err.to_string(), trace).into()
            }
            ChangeMediaStateError::CouldNotGetLocalMedia(err) => {
                Traced::compose(err, trace).into()
            }
            ChangeMediaStateError::ProhibitedState(_) => {
                MediaStateTransitionException::new(
                    message,
                    trace,
                    MediaStateTransitionExceptionKind::ProhibitedState,
                )
                .into()
            }
            ChangeMediaStateError::TransitionIntoOppositeState(_) => {
                MediaStateTransitionException::new(
                    message,
                    trace,
                    MediaStateTransitionExceptionKind::OppositeState,
                )
                .into()
            }
            ChangeMediaStateError::InvalidLocalTracks(_)
            | ChangeMediaStateError::InsertLocalTracksError(_) => {
                InternalException::new(message, None, trace).into()
            }
        }
    }
}

impl From<ConstraintsUpdateError> for Error {
    fn from(err: ConstraintsUpdateError) -> Self {
        let message = err.to_string();

        let (err, rolled_back) = match err {
            ConstraintsUpdateError::Recovered(err) => (err, true),
            ConstraintsUpdateError::RecoverFailed {
                recover_reason, ..
            } => (recover_reason, false),
            ConstraintsUpdateError::Errored(err) => (err, false),
        };

        MediaSettingsUpdateException::new(message, err, rolled_back).into()
    }
}

impl From<Traced<LocalMediaError>> for Error {
    fn from(err: Traced<LocalMediaError>) -> Self {
        use InsertLocalTracksError as IE;
        use LocalMediaError as ME;
        use UpdateLocalStreamError as UE;

        let (err, trace) = err.split();
        let message = err.to_string();

        match err {
            ME::UpdateLocalStreamError(err) => match err {
                UE::CouldNotGetLocalMedia(err) => {
                    Traced::compose(err, trace).into()
                }
                UE::InvalidLocalTracks(_)
                | UE::InsertLocalTracksError(
                    IE::InvalidMediaTrack | IE::NotEnoughTracks,
                ) => InternalException::new(message, None, trace).into(),
                UE::InsertLocalTracksError(IE::CouldNotInsertLocalTrack(_)) => {
                    InternalException::new(message, None, trace).into()
                }
            },
            ME::SenderCreateError(CreateError::TransceiverNotFound(_)) => {
                InternalException::new(message, None, trace).into()
            }
            ME::SenderCreateError(CreateError::CannotDisableRequiredSender) => {
                MediaStateTransitionException::new(
                    message,
                    trace,
                    MediaStateTransitionExceptionKind::ProhibitedState,
                )
                .into()
            }
        }
    }
}
