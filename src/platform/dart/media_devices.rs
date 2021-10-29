//! [MediaDevices][1] functionality.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediadevices

use dart_sys::Dart_Handle;
use tracerr::Traced;

use crate::platform::{
    dart::utils::{
        dart_future::FutureFromDart, handle::DartHandle, list::DartList,
    },
    Error,
};

use super::{
    constraints::{DisplayMediaStreamConstraints, MediaStreamConstraints},
    input_device_info::InputDeviceInfo,
    media_track::MediaStreamTrack,
};

type EnumerateDevicesFunction = extern "C" fn() -> Dart_Handle;

type GetUserMediaFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

type GetDisplayMediaFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Stores pointer to the [`EnumerateDevicesFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ENUMERATE_DEVICES_FUNCTION: Option<EnumerateDevicesFunction> = None;

/// Stores pointer to the [`GetUserMediaFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_USER_MEDIA_FUNCTION: Option<GetUserMediaFunction> = None;

/// Stores pointer to the [`GetDisplayMediaFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_DISPLAY_MEDIA_FUNCTION: Option<GetDisplayMediaFunction> = None;

/// Registers the provided [`EnumerateDevicesFunction`] as
/// [`ENUMERATE_DEVICES_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaDevices__enumerate_devices(
    f: EnumerateDevicesFunction,
) {
    ENUMERATE_DEVICES_FUNCTION = Some(f);
}

/// Registers the provided [`GetUserMediaFunction`] as
/// [`GET_USER_MEDIA_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaDevices__get_user_media(
    f: GetUserMediaFunction,
) {
    GET_USER_MEDIA_FUNCTION = Some(f);
}

/// Registers the provided [`GetDisplayMediaFunction`] as
/// [`GET_DISPLAY_MEDIA_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaDevices__get_display_media(
    f: GetUserMediaFunction,
) {
    GET_DISPLAY_MEDIA_FUNCTION = Some(f);
}

/// Collects information about the User Agent's available media input devices.
///
/// Adapter for a [MediaDevices.enumerateDevices()][1] function.
///
/// # Errors
///
/// With [`Error`] if [MediaDevices.enumerateDevices()][1] returns error or
/// cannot get [MediaDevices][2].
///
/// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-enumeratedevices
pub async fn enumerate_devices() -> Result<Vec<InputDeviceInfo>, Traced<Error>>
{
    let devices = FutureFromDart::execute::<DartHandle>(unsafe {
        ENUMERATE_DEVICES_FUNCTION.unwrap()()
    })
    .await
    .unwrap();
    Ok(DartList::from(devices).into())
}

/// Prompts a user for a permission to use a media input which produces vector
/// of [`MediaStreamTrack`]s containing the requested types of media.
///
/// Adapter for a [MediaDevices.getUserMedia()][1] function.
///
/// # Errors
///
/// With [`Error`] if [MediaDevices.getUserMedia()][1] returns error or cannot
/// get [MediaDevices][2].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
pub async fn get_user_media(
    caps: MediaStreamConstraints,
) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
    let tracks = FutureFromDart::execute::<DartHandle>(unsafe {
        GET_USER_MEDIA_FUNCTION.unwrap()(caps.into())
    })
    .await
    .unwrap();
    Ok(DartList::from(tracks).into())
}

/// Prompts a user to select and grant a permission to capture contents of a
/// display or portion thereof (such as a single window) as vector of
/// [`MediaStreamTrack`].
///
/// Adapter for a [MediaDevices.getDisplayMedia()][1] function.
///
/// # Errors
///
/// With [`Error`] if [MediaDevices.getDisplayMedia()][1] returns error or
/// cannot get [MediaDevices][2].
///
/// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
pub async fn get_display_media(
    caps: DisplayMediaStreamConstraints,
) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
    let tracks = FutureFromDart::execute::<DartHandle>(unsafe {
        GET_DISPLAY_MEDIA_FUNCTION.unwrap()(caps.into())
    })
    .await
    .unwrap();
    Ok(DartList::from(tracks).into())
}
