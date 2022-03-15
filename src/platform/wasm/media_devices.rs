//! [MediaDevices][1] functionality.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediadevices

use wasm_bindgen_futures::JsFuture;

use tracerr::Traced;

use crate::{
    media::InvalidOutputAudioDeviceIdError,
    platform::{
        DisplayMediaStreamConstraints, Error, MediaDeviceInfo,
        MediaStreamConstraints, MediaStreamTrack,
    },
};

use super::window;

/// Collects information about the User Agent's available media input devices.
///
/// Adapter for a [MediaDevices.enumerateDevices()][1] function.
///
/// # Errors
///
/// With [`Error`] if [MediaDevices.enumerateDevices()][1] returns error or
/// cannot get [MediaDevices][2].
///
/// # Panics
///
/// If [`js_sys::Array`] returned from [MediaDevices.enumerateDevices()][1]
/// contains something that is not [`web_sys::MediaDeviceInfo`].
///
/// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-enumeratedevices
/// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
pub async fn enumerate_devices() -> Result<Vec<MediaDeviceInfo>, Traced<Error>>
{
    let devices = window()
        .navigator()
        .media_devices()
        .map_err(Error::from)
        .map_err(tracerr::wrap!())?;
    let devices = JsFuture::from(
        devices
            .enumerate_devices()
            .map_err(Error::from)
            .map_err(tracerr::wrap!())?,
    )
    .await
    .map_err(Error::from)
    .map_err(tracerr::wrap!())?;

    Ok(js_sys::Array::from(&devices)
        .values()
        .into_iter()
        .map(|info| {
            let info = web_sys::MediaDeviceInfo::from(info.unwrap());
            MediaDeviceInfo::from(info)
        })
        .collect())
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
/// # Panics
///
/// If [`js_sys::Array`] returned from [MediaDevices.getUserMedia()][1]
/// contains something that is not [`web_sys::MediaStreamTrack`].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
pub async fn get_user_media(
    caps: MediaStreamConstraints,
) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
    let media_devices = window()
        .navigator()
        .media_devices()
        .map_err(Error::from)
        .map_err(tracerr::wrap!())?;

    let stream = JsFuture::from(
        media_devices
            .get_user_media_with_constraints(&caps.into())
            .map_err(Error::from)
            .map_err(tracerr::wrap!())?,
    )
    .await
    .map(web_sys::MediaStream::from)
    .map_err(Error::from)
    .map_err(tracerr::wrap!())?;

    Ok(js_sys::try_iter(&stream.get_tracks())
        .unwrap()
        .unwrap()
        .map(|tr| MediaStreamTrack::from(tr.unwrap()))
        .collect())
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
/// # Panics
///
/// If [`js_sys::Array`] returned from [MediaDevices.getDisplayMedia()][1]
/// contains something that is not [`web_sys::MediaStreamTrack`].
///
/// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
/// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
pub async fn get_display_media(
    caps: DisplayMediaStreamConstraints,
) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
    let media_devices = window()
        .navigator()
        .media_devices()
        .map_err(Error::from)
        .map_err(tracerr::wrap!())?;

    let stream = JsFuture::from(
        media_devices
            .get_display_media_with_constraints(&caps.into())
            .map_err(Error::from)
            .map_err(tracerr::wrap!())?,
    )
    .await
    .map(web_sys::MediaStream::from)
    .map_err(Error::from)
    .map_err(tracerr::wrap!())?;

    Ok(js_sys::try_iter(&stream.get_tracks())
        .unwrap()
        .unwrap()
        .map(|tr| MediaStreamTrack::from(tr.unwrap()))
        .collect())
}

/// This method should be unreachable, because this functional is implemented on
/// the Dart side of Jason only.
///
/// # Errors
///
/// Never.
///
/// # Panics
///
/// Always.
#[allow(clippy::unused_async)]
pub async fn set_output_audio_id(
    _: String,
) -> Result<(), Traced<InvalidOutputAudioDeviceIdError>> {
    unreachable!(
        "`set_output_audio_id()` is implemented on the Dart side,\
         so this method call is unreachable",
    )
}
