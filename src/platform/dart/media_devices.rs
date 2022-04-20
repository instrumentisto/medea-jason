//! Representation of [MediaDevices][0].
//!
//! [0]: https://w3.org/TR/mediacapture-streams#mediadevices

use medea_macro::dart_bridge;
use tracerr::Traced;

use crate::{
    api::string_into_c_str,
    media::MediaSourceKind,
    platform::{
        dart::utils::{
            dart_future::FutureFromDart, handle::DartHandle, list::DartList,
        },
        utils::callback::Callback,
        Error,
    },
};

use super::{
    constraints::{DisplayMediaStreamConstraints, MediaStreamConstraints},
    media_device_info::MediaDeviceInfo,
    media_track::MediaStreamTrack,
};

#[dart_bridge("flutter/lib/src/native/platform/media_devices.g.dart")]
mod media_devices {
    use std::{os::raw::c_char, ptr};

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

        /// Switches the current output audio device to the device with the
        /// provided `device_id`.
        pub fn set_output_audio_id(
            device_id: ptr::NonNull<c_char>,
        ) -> Dart_Handle;

        /// Subscribes onto the `MediaDevices`'s `devicechange` event.
        pub fn on_device_change(cb: Dart_Handle);
    }
}

/// Media devices controller.
#[derive(Clone, Copy, Debug, Default)]
pub struct MediaDevices;

impl MediaDevices {
    /// Collects information about available media input devices.
    ///
    /// Adapter for the [MediaDevices.enumerateDevices()][1] function.
    ///
    /// # Errors
    ///
    /// If [MediaDevices.enumerateDevices()][1] errors itself or unable to get
    /// [MediaDevices][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-enumeratedevices
    /// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
    pub async fn enumerate_devices(
        &self,
    ) -> Result<Vec<MediaDeviceInfo>, Traced<Error>> {
        let devices = unsafe {
            FutureFromDart::execute::<DartHandle>(
                media_devices::enumerate_devices(),
            )
            .await
        }
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

    /// Prompts a user for permissions to use a media input device, producing
    /// [`MediaStreamTrack`]s containing the requested types of media.
    ///
    /// Adapter for the [MediaDevices.getUserMedia()][1] function.
    ///
    /// # Errors
    ///
    /// If [MediaDevices.getUserMedia()][1] errors itself or unable to get
    /// [MediaDevices][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
    pub async fn get_user_media(
        &self,
        caps: MediaStreamConstraints,
    ) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
        let tracks = unsafe {
            FutureFromDart::execute::<DartHandle>(
                media_devices::get_user_media(caps.into()),
            )
            .await
        }
        .map_err(tracerr::wrap!())?;

        let tracks = Vec::from(DartList::from(tracks))
            .into_iter()
            .map(|track| {
                MediaStreamTrack::new(track, Some(MediaSourceKind::Device))
            })
            .collect();

        Ok(tracks)
    }

    /// Prompts a user to select and grant permissions to capture contents of a
    /// display or portion thereof (such as a single window), producing
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
        &self,
        caps: DisplayMediaStreamConstraints,
    ) -> Result<Vec<MediaStreamTrack>, Traced<Error>> {
        let tracks = unsafe {
            FutureFromDart::execute::<DartHandle>(
                media_devices::get_display_media(caps.into()),
            )
            .await
        }
        .map_err(tracerr::wrap!())?;

        let tracks = Vec::from(DartList::from(tracks))
            .into_iter()
            .map(|track| {
                MediaStreamTrack::new(track, Some(MediaSourceKind::Display))
            })
            .collect();

        Ok(tracks)
    }

    /// Switches the current output audio device to the device with the provided
    /// `device_id`.
    ///
    /// # Errors
    ///
    /// If output audio device with the provided `device_id` is not available.
    pub async fn set_output_audio_id(
        &self,
        device_id: String,
    ) -> Result<(), Traced<Error>> {
        unsafe {
            FutureFromDart::execute::<()>(media_devices::set_output_audio_id(
                string_into_c_str(device_id),
            ))
            .await
        }
        .map_err(tracerr::wrap!())
    }

    /// Subscribes onto the [`MediaDevices`]'s `devicechange` event.
    pub fn on_device_change<F>(&self, handler: Option<F>)
    where
        F: 'static + FnMut(),
    {
        if let Some(mut h) = handler {
            unsafe {
                media_devices::on_device_change(
                    Callback::from_fn_mut(move |_: ()| {
                        h();
                    })
                    .into_dart(),
                );
            };
        }
    }
}
