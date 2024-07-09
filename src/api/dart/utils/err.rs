//! Facilities for creating Dart exceptions from Rust.

use std::ptr;

use dart_sys::Dart_Handle;
use derive_more::Into;
use flutter_rust_bridge::DartOpaque;
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

    use crate::{api::DartValue, platform::Error};

    use super::DartError;

    /// Invokes other Dart closures that accept a [`DartValue`] argument.
    extern "C" {
        /// Returns a new Dart [`StateError`] with the provided message.
        ///
        /// [`StateError`]: https://api.dart.dev/dart-core/StateError-class.html
        pub fn new_state_error(
            message: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`FormatException`][1] with the provided message.
        ///
        /// [1]: https://api.dart.dev/dart-core/FormatException-class.html
        pub fn new_format_exception(
            message: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`LocalMediaInitException`] with the provided
        /// error `kind`, `message`, `cause` and `stacktrace`.
        pub fn new_local_media_init_exception(
            kind: i64,
            message: ptr::NonNull<c_char>,
            cause: DartValue,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`EnumerateDevicesException`] with the provided
        /// error `cause` and `stacktrace`.
        pub fn new_enumerate_devices_exception(
            cause: DartError,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`RpcClientException`] with the provided error
        /// `kind`, `message`, `cause` and `stacktrace`.
        pub fn new_rpc_client_exception(
            kind: i64,
            message: ptr::NonNull<c_char>,
            cause: DartValue,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`MediaStateTransitionException`] with the
        /// provided error `message` and `stacktrace`.
        pub fn new_media_state_transition_exception(
            message: ptr::NonNull<c_char>,
            stacktrace: ptr::NonNull<c_char>,
            kind: i64,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`InternalException`] with the provided error
        /// `message`, `cause` and `stacktrace`.
        pub fn new_internal_exception(
            message: ptr::NonNull<c_char>,
            cause: DartValue,
            stacktrace: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`MediaSettingsUpdateException`] with the
        /// provided error `message`, `cause` and `rolled_back` property.
        pub fn new_media_settings_update_exception(
            message: ptr::NonNull<c_char>,
            cause: DartError,
            rolled_back: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`InvalidOutputAudioDeviceIdException`] with the
        /// provided `trace` property.
        pub fn new_invalid_output_audio_device_id_exception(
            trace: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart [`MicVolumeException`] with the provided `cause`
        /// and `trace` properties.
        pub fn new_mic_volume_exception(
            cause: DartError,
            trace: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns a new Dart `NativePanicException`.
        pub fn new_panic_exception() -> Result<Dart_Handle, Error>;
    }
}

/// Creates and returns a new Dart `NativePanicException`.
#[must_use]
pub unsafe fn new_panic_error() -> Dart_Handle {
    #[allow(clippy::unwrap_used)]
    unsafe { exception::new_panic_exception() }.unwrap()
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

impl From<DartError> for DartOpaque {
    fn from(val: DartError) -> Self {
        let boxed = unsafe { Box::from_raw(val.0.as_ptr()) };
        unsafe { Self::new_non_droppable(*boxed) }
    }
}

impl From<platform::Error> for DartError {
    fn from(err: platform::Error) -> Self {
        Self::new(err.get_handle())
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<StateError> for DartError {
    fn from(err: StateError) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_state_error(string_into_c_str(err.message()))
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<LocalMediaInitException> for DartError {
    fn from(err: LocalMediaInitException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_local_media_init_exception(
                err.kind() as i64,
                string_into_c_str(err.message()),
                err.cause().map(Self::from).into(),
                string_into_c_str(err.trace()),
            )
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<EnumerateDevicesException> for DartError {
    fn from(err: EnumerateDevicesException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_enumerate_devices_exception(
                err.cause().into(),
                string_into_c_str(err.trace()),
            )
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<InvalidOutputAudioDeviceIdException> for DartError {
    fn from(err: InvalidOutputAudioDeviceIdException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_invalid_output_audio_device_id_exception(
                string_into_c_str(err.trace()),
            )
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<MicVolumeException> for DartError {
    fn from(err: MicVolumeException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_mic_volume_exception(
                err.cause().into(),
                string_into_c_str(err.trace()),
            )
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<FormatException> for DartError {
    fn from(err: FormatException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_format_exception(string_into_c_str(err.message()))
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<RpcClientException> for DartError {
    fn from(err: RpcClientException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_rpc_client_exception(
                err.kind() as i64,
                string_into_c_str(err.message()),
                err.cause().map(Self::from).into(),
                string_into_c_str(err.trace()),
            )
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<MediaStateTransitionException> for DartError {
    fn from(err: MediaStateTransitionException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_media_state_transition_exception(
                string_into_c_str(err.message()),
                string_into_c_str(err.trace()),
                err.kind() as i64,
            )
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<InternalException> for DartError {
    fn from(err: InternalException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_internal_exception(
                string_into_c_str(err.message()),
                err.cause().map(Self::from).into(),
                string_into_c_str(err.trace()),
            )
        }
        .unwrap();

        Self::new(exception)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<MediaSettingsUpdateException> for DartError {
    fn from(err: MediaSettingsUpdateException) -> Self {
        #[allow(clippy::unwrap_used)]
        let exception = unsafe {
            exception::new_media_settings_update_exception(
                string_into_c_str(err.message()),
                err.cause(),
                err.rolled_back(),
            )
        }
        .unwrap();

        Self::new(exception)
    }
}
