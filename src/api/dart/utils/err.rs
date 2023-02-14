//! Facilities for creating Dart exceptions from Rust.

use std::ptr;

use dart_sys::Dart_Handle;
use derive_more::Into;
use medea_macro::dart_bridge;

use crate::{
    api::{
        box_dart_handle,
        err::{
            EnumerateDevicesException, FormatException, InternalException,
            InvalidOutputAudioDeviceIdException, LocalMediaInitException,
            MediaSettingsUpdateException, MediaStateTransitionException,
            MicVolumeException, RpcClientException, StateError,
        },
    },
    platform::{self, utils::string_into_c_str},
};

#[dart_bridge("flutter/lib/src/native/ffi/exception.g.dart")]
mod exception {
    use std::ptr;

    use dart_sys::Dart_Handle;
    use libc::c_char;

    use crate::api::DartValue;

    use super::DartError;

    /// Invokes other Dart closures that accept a [`DartValue`] argument.
    extern "C" {
        /// Returns a new Dart [`StateError`] with the provided message.
        ///
        /// [`StateError`]: https://api.dart.dev/dart-core/StateError-class.html
        pub fn new_state_error(message: ptr::NonNull<c_char>) -> Dart_Handle;

        /// Returns a new Dart [`FormatException`][1] with the provided message.
        ///
        /// [1]: https://api.dart.dev/dart-core/FormatException-class.html
        pub fn new_format_exception(
            message: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Returns a new Dart [`LocalMediaInitException`] with the provided
        /// error `kind`, `message`, `cause` and `stacktrace`.
        pub fn new_local_media_init_exception(
            kind: i64,
            message: ptr::NonNull<c_char>,
            cause: DartValue,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Returns a new Dart [`EnumerateDevicesException`] with the provided
        /// error `cause` and `stacktrace`.
        pub fn new_enumerate_devices_exception(
            cause: DartError,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Returns a new Dart [`RpcClientException`] with the provided error
        /// `kind`, `message`, `cause` and `stacktrace`.
        pub fn new_rpc_client_exception(
            kind: i64,
            message: ptr::NonNull<c_char>,
            cause: DartValue,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Returns a new Dart [`MediaStateTransitionException`] with the
        /// provided error `message` and `stacktrace`.
        pub fn new_media_state_transition_exception(
            message: ptr::NonNull<c_char>,
            stacktrace: ptr::NonNull<c_char>,
            kind: i64,
        ) -> Dart_Handle;

        /// Returns a new Dart [`InternalException`] with the provided error
        /// `message`, `cause` and `stacktrace`.
        pub fn new_internal_exception(
            message: ptr::NonNull<c_char>,
            cause: DartValue,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Returns a new Dart [`MediaSettingsUpdateException`] with the
        /// provided error `message`, `cause` and `rolled_back` property.
        pub fn new_media_settings_update_exception(
            message: ptr::NonNull<c_char>,
            cause: DartError,
            rolled_back: bool,
        ) -> Dart_Handle;

        /// Returns a new Dart [`InvalidOutputAudioDeviceIdException`] with the
        /// provided `trace` property.
        pub fn new_invalid_output_audio_device_id_exception(
            trace: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Returns a new Dart [`MicVolumeException`] with the provided `cause`
        /// and `trace` properties.
        pub fn new_mic_volume_exception(
            cause: DartError,
            trace: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Returns a new Dart `NativePanicException`.
        ///
        /// Returned [`Dart_Handle`] will be recognized by Dart runtime as an
        /// error, so `Dart_IsError` function will return `true` on the returned
        /// [`Dart_Handle`].
        pub fn throw_panic_exception() -> Dart_Handle;
    }
}

/// Creates and returns a new Dart `NativePanicException`.
///
/// Returned [`Dart_Handle`] will be recognized by Dart runtime as an error, so
/// `Dart_IsError` function will return `true` on the returned [`Dart_Handle`].
#[must_use]
pub unsafe fn new_panic_error() -> Dart_Handle {
    exception::throw_panic_exception()
}

/// An error that can be returned from Rust to Dart.
#[allow(missing_copy_implementations)] // not trivially copyable
#[derive(Debug, Into)]
#[repr(transparent)]
pub struct DartError(ptr::NonNull<Dart_Handle>);

impl DartError {
    /// Creates a new [`DartError`] from the provided [`Dart_Handle`].
    fn new(handle: Dart_Handle) -> Self {
        Self(unsafe { box_dart_handle(handle) })
    }
}

impl From<platform::Error> for DartError {
    fn from(err: platform::Error) -> Self {
        Self::new(err.get_handle())
    }
}

impl From<StateError> for DartError {
    fn from(err: StateError) -> Self {
        unsafe {
            Self::new(exception::new_state_error(string_into_c_str(
                err.message(),
            )))
        }
    }
}

impl From<LocalMediaInitException> for DartError {
    fn from(err: LocalMediaInitException) -> Self {
        unsafe {
            Self::new(exception::new_local_media_init_exception(
                err.kind() as i64,
                string_into_c_str(err.message()),
                err.cause().map(Self::from).into(),
                string_into_c_str(err.trace()),
            ))
        }
    }
}

impl From<EnumerateDevicesException> for DartError {
    fn from(err: EnumerateDevicesException) -> Self {
        unsafe {
            Self::new(exception::new_enumerate_devices_exception(
                err.cause().into(),
                string_into_c_str(err.trace()),
            ))
        }
    }
}

impl From<InvalidOutputAudioDeviceIdException> for DartError {
    fn from(err: InvalidOutputAudioDeviceIdException) -> Self {
        unsafe {
            Self::new(exception::new_invalid_output_audio_device_id_exception(
                string_into_c_str(err.trace()),
            ))
        }
    }
}

impl From<MicVolumeException> for DartError {
    fn from(err: MicVolumeException) -> Self {
        unsafe {
            Self::new(exception::new_mic_volume_exception(
                err.cause().into(),
                string_into_c_str(err.trace()),
            ))
        }
    }
}

impl From<FormatException> for DartError {
    fn from(err: FormatException) -> Self {
        unsafe {
            Self::new(exception::new_format_exception(string_into_c_str(
                err.message(),
            )))
        }
    }
}

impl From<RpcClientException> for DartError {
    fn from(err: RpcClientException) -> Self {
        unsafe {
            Self::new(exception::new_rpc_client_exception(
                err.kind() as i64,
                string_into_c_str(err.message()),
                err.cause().map(Self::from).into(),
                string_into_c_str(err.trace()),
            ))
        }
    }
}

impl From<MediaStateTransitionException> for DartError {
    fn from(err: MediaStateTransitionException) -> Self {
        unsafe {
            Self::new(exception::new_media_state_transition_exception(
                string_into_c_str(err.message()),
                string_into_c_str(err.trace()),
                err.kind() as i64,
            ))
        }
    }
}

impl From<InternalException> for DartError {
    fn from(err: InternalException) -> Self {
        unsafe {
            Self::new(exception::new_internal_exception(
                string_into_c_str(err.message()),
                err.cause().map(Self::from).into(),
                string_into_c_str(err.trace()),
            ))
        }
    }
}

impl From<MediaSettingsUpdateException> for DartError {
    fn from(err: MediaSettingsUpdateException) -> Self {
        unsafe {
            Self::new(exception::new_media_settings_update_exception(
                string_into_c_str(err.message()),
                err.cause(),
                err.rolled_back(),
            ))
        }
    }
}
