//! [MediaDeviceInfo][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams/#device-info

use std::convert::TryFrom;

use dart_sys::Dart_Handle;
use derive_more::From;

use crate::{
    api::DartValueArg, media::MediaKind,
    platform::dart::utils::handle::DartHandle,
};

type DeviceIdFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<String>>;

type LabelFunction = extern "C" fn(Dart_Handle) -> DartValueArg<Option<String>>;

type GroupIdFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<String>>;

type KindFunction = extern "C" fn(Dart_Handle) -> DartValueArg<Option<i32>>;

/// Stores pointer to the [`DeviceIdFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut DEVICE_ID_FUNCTION: Option<DeviceIdFunction> = None;

/// Stores pointer to the [`LabelFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut LABEL_FUNCTION: Option<LabelFunction> = None;

/// Stores pointer to the [`GroupIdFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GROUP_ID_FUNCTION: Option<GroupIdFunction> = None;

/// Stores pointer to the [`KindFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut KIND_FUNCTION: Option<KindFunction> = None;

/// Registers the provided [`DeviceIdFunction`] as [`DEVICE_ID_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_InputDeviceInfo__device_id(
    f: DeviceIdFunction,
) {
    DEVICE_ID_FUNCTION = Some(f);
}

/// Registers the provided [`LabelFunction`] as [`LABEL_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_InputDeviceInfo__label(f: LabelFunction) {
    LABEL_FUNCTION = Some(f);
}

/// Registers the provided [`GroupIdFunction`] as [`GROUP_ID_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_InputDeviceInfo__group_id(
    f: GroupIdFunction,
) {
    GROUP_ID_FUNCTION = Some(f);
}

/// Registers the provided [`KindFunction`] as [`KIND_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_InputDeviceInfo__kind(f: KindFunction) {
    KIND_FUNCTION = Some(f);
}

/// Representation of [MediaDeviceInfo][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
#[derive(Clone, Debug, From)]
pub struct InputDeviceInfo(DartHandle);

impl InputDeviceInfo {
    /// Returns unique identifier for the represented device.
    #[must_use]
    pub fn device_id(&self) -> String {
        // Device ID should be always Some
        Option::try_from(unsafe { DEVICE_ID_FUNCTION.unwrap()(self.0.get()) })
            .unwrap()
            .unwrap()
    }

    /// Returns kind of the represented device.
    ///
    /// This representation of [MediaDeviceInfo][1] ONLY for input device.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#device-info
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        // Kind should be always Some
        Option::<i32>::try_from(unsafe { KIND_FUNCTION.unwrap()(self.0.get()) })
            .unwrap()
            .unwrap()
            .into()
    }

    /// Returns label describing the represented device (for example
    /// "External USB Webcam").
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn label(&self) -> String {
        // Label should be always Some
        Option::try_from(unsafe { LABEL_FUNCTION.unwrap()(self.0.get()) })
            .unwrap()
            .unwrap()
    }

    /// Returns group identifier of the represented device.
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
    #[must_use]
    pub fn group_id(&self) -> String {
        // Group ID should be always Some
        Option::try_from(unsafe { GROUP_ID_FUNCTION.unwrap()(self.0.get()) })
            .unwrap()
            .unwrap()
    }
}
