//! Representation of [MediaDevices][0].
//!
//! [0]: https://w3.org/TR/mediacapture-streams#mediadevices

use std::convert::TryInto as _;

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
        /// Returns information about available media input devices.
        pub fn enumerate_devices() -> Dart_Handle;

        /// Prompts a user for permissions to use a media input device,
        /// producing a vector of [MediaStreamTrack][1]s containing the
        /// requested types of media.
        ///
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn get_user_media(constraints: Dart_Handle) -> Dart_Handle;

        /// Prompts a user to select and grant permissions to capture contents
        /// of a display or portion thereof (such as a single window), producing
        /// a vector of [MediaStreamTrack][1]s containing the requested types
        /// of media.
        ///
        /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn get_display_media(constraints: Dart_Handle) -> Dart_Handle;
    }
}

/// Collects information about available media input devices.
///
/// Adapter for a [MediaDevices.enumerateDevices()][1] function.
///
/// # Errors
///
/// If [MediaDevices.enumerateDevices()][1] errors itself or unable to get
/// [MediaDevices][2].
///
/// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-enumeratedevices
/// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
pub async fn enumerate_devices() -> Result<Vec<InputDeviceInfo>, Traced<Error>>
{
    let devices = FutureFromDart::execute::<DartHandle>(unsafe {
        media_devices::enumerate_devices()
    })
    .await
    .map(DartList::from)
    .map_err(tracerr::wrap!())?;

    let len = devices.length();
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        let val = devices.get(i).unwrap();
        if let Ok(v) = val.try_into() {
            result.push(v);
        }
    }
    Ok(result)
}

/// Prompts a user for permissions to use a media input device, producing a
/// [`Vec`] of [`MediaStreamTrack`]s containing the requested types of media.
///
/// Adapter for a [MediaDevices.getUserMedia()][1] function.
///
/// # Errors
///
/// If [MediaDevices.getUserMedia()][1] errors itself or unable to get
/// [MediaDevices][2].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
pub async fn get_user_media(
    caps: MediaStreamConstraints,
) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
    let tracks = FutureFromDart::execute::<DartHandle>(unsafe {
        media_devices::get_user_media(caps.into())
    })
    .await
    .map_err(tracerr::wrap!())?;

    Ok(DartList::from(tracks).into())
}

/// Prompts a user to select and grant permissions to capture contents of a
/// display or portion thereof (such as a single window), producing a [`Vec`] of
/// [`MediaStreamTrack`]s containing the requested types of media.
///
/// Adapter for a [MediaDevices.getDisplayMedia()][1] function.
///
/// # Errors
///
/// If [MediaDevices.getDisplayMedia()][1] errors itself or unable to get
/// [MediaDevices][2].
///
/// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
/// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
pub async fn get_display_media(
    caps: DisplayMediaStreamConstraints,
) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
    let tracks = FutureFromDart::execute::<DartHandle>(unsafe {
        media_devices::get_display_media(caps.into())
    })
    .await
    .map_err(tracerr::wrap!())?;

    Ok(DartList::from(tracks).into())
}
