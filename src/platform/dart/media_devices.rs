//! [MediaDevices][1] functionality.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediadevices

use medea_macro::dart_bridge;
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

#[dart_bridge("flutter/lib/src/native/platform/media_devices.g.dart")]
mod media_devices {
    use dart_sys::Dart_Handle;

    extern "C" {
        /// Returns information about the User Agent's available media input
        /// devices.
        pub fn enumerate_devices() -> Dart_Handle;

        /// Prompts a user for a permission to use a media input which produces
        /// vector of [`MediaStreamTrack`]s containing the requested
        /// types of media.
        pub fn get_user_media(constraints: Dart_Handle) -> Dart_Handle;

        /// Prompts a user to select and grant a permission to capture contents
        /// of a display or portion thereof (such as a single window) as
        /// vector of [`MediaStreamTrack`].
        pub fn get_display_media(constraints: Dart_Handle) -> Dart_Handle;
    }
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
        media_devices::enumerate_devices()
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
        media_devices::get_user_media(caps.into())
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
        media_devices::get_display_media(caps.into())
    })
    .await
    .unwrap();

    Ok(DartList::from(tracks).into())
}
