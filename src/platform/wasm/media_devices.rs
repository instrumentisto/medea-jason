//! [MediaDevices][1] functionality.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediadevices

#![allow(clippy::unwrap_used)]
// TODO: False positive in function signatures, try remove on 1.77 Rust upgrade.
#![allow(clippy::let_underscore_untyped, let_underscore_drop)]

use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::JsFuture;

use tracerr::Traced;
use web_sys::{Event, MediaDevices as SysMediaDevices};

use crate::{
    media::{InvalidOutputAudioDeviceIdError, MediaSourceKind, MicVolumeError},
    platform::{
        utils::EventListener, DisplayMediaStreamConstraints, Error,
        GetUserMediaError, MediaDeviceInfo, MediaDisplayInfo,
        MediaStreamConstraints, MediaStreamTrack,
    },
};

use super::window;

impl From<Error> for GetUserMediaError {
    fn from(err: Error) -> Self {
        let message = err.message().to_lowercase();

        if message.contains("audio") {
            Self::Audio(err)
        } else if message.contains("video") {
            Self::Video(err)
        } else {
            Self::Unknown(err)
        }
    }
}

/// Media devices controller.
#[derive(Debug)]
pub struct MediaDevices {
    /// Underlying [`SysMediaDevices`], used for media devices management.
    devices: Rc<SysMediaDevices>,

    /// [`EventListener`] for the `devicechange` event of the underlying
    /// [`SysMediaDevices`].
    on_device_change_listener:
        RefCell<Option<EventListener<SysMediaDevices, Event>>>,
}

impl Default for MediaDevices {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaDevices {
    /// Returns new [`MediaDevices`].
    ///
    /// # Panics
    ///
    /// If failed to get [`SysMediaDevices`] from the browser.
    #[must_use]
    pub fn new() -> Self {
        let devices = window()
            .navigator()
            .media_devices()
            .map_err(Error::from)
            .map_err(tracerr::wrap!())
            .unwrap();
        Self {
            devices: Rc::new(devices),
            on_device_change_listener: RefCell::new(None),
        }
    }

    /// Collects information about the User Agent's available media input
    /// devices.
    ///
    /// Adapter for the [MediaDevices.enumerateDevices()][1] function.
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
    pub async fn enumerate_devices(
        &self,
    ) -> Result<Vec<MediaDeviceInfo>, Traced<Error>> {
        let devices = JsFuture::from(
            self.devices
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

    /// Unimplemented on WASM targets.
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::unused_async)] // for platform code uniformity
    pub async fn enumerate_displays(
        &self,
    ) -> Result<Vec<MediaDisplayInfo>, Traced<Error>> {
        unimplemented!()
    }

    /// Prompts a user for a permission to use a media input which produces
    /// [`MediaStreamTrack`]s containing the requested types of media.
    ///
    /// Adapter for the [MediaDevices.getUserMedia()][1] function.
    ///
    /// # Errors
    ///
    /// With [`Error`] if [MediaDevices.getUserMedia()][1] returns error or
    /// cannot get [MediaDevices][2].
    ///
    /// # Panics
    ///
    /// If [`js_sys::Array`] returned from [MediaDevices.getUserMedia()][1]
    /// contains something that is not [`web_sys::MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/mediacapture-streams#mediadevices
    pub async fn get_user_media(
        &self,
        caps: MediaStreamConstraints,
    ) -> Result<Vec<MediaStreamTrack>, Traced<GetUserMediaError>> {
        let stream = JsFuture::from(
            self.devices
                .get_user_media_with_constraints(&caps.into())
                .map_err(Error::from)
                .map_err(tracerr::from_and_wrap!())?,
        )
        .await
        .map(web_sys::MediaStream::from)
        .map_err(Error::from)
        .map_err(tracerr::from_and_wrap!())?;

        Ok(js_sys::try_iter(&stream.get_tracks())
            .unwrap()
            .unwrap()
            .map(|tr| {
                MediaStreamTrack::new(
                    tr.unwrap(),
                    Some(MediaSourceKind::Device),
                )
            })
            .collect())
    }

    /// Prompts a user to select and grant a permission to capture contents of a
    /// display or portion thereof (such as a single window) as vector of
    /// [`MediaStreamTrack`]s.
    ///
    /// Adapter for the [MediaDevices.getDisplayMedia()][1] function.
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
        &self,
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
            .map(|tr| {
                MediaStreamTrack::new(
                    tr.unwrap(),
                    Some(MediaSourceKind::Display),
                )
            })
            .collect())
    }

    /// This method should be unreachable, because this functional is
    /// implemented on the Dart side of Jason only.
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
        &self,
        _: String,
    ) -> Result<(), Traced<InvalidOutputAudioDeviceIdError>> {
        unreachable!(
            "`set_output_audio_id()` is implemented on the Dart side, \
             so this method call is unreachable",
        )
    }

    /// Subscribes onto the [`MediaDevices`]'s `devicechange` event.
    ///
    /// # Panics
    ///
    /// If `devicechange` event listener binding fails.
    pub fn on_device_change<F>(&self, f: Option<F>)
    where
        F: 'static + FnMut(),
    {
        if let Some(mut f) = f {
            drop(
                self.on_device_change_listener.borrow_mut().replace(
                    EventListener::new_mut(
                        Rc::clone(&self.devices),
                        "devicechange",
                        move |_| f(),
                    )
                    .unwrap(),
                ),
            );
        }
    }

    /// Always returns `false` since accessing microphone cannot be implemented
    /// on web platform.
    #[allow(clippy::unused_async)]
    pub async fn microphone_volume_is_available(&self) -> bool {
        false
    }

    /// This method should be unreachable, because cannot be implemented on web
    /// platform.
    ///
    /// # Errors
    ///
    /// Never.
    ///
    /// # Panics
    ///
    /// Always.
    #[allow(clippy::unused_async)]
    pub async fn microphone_volume(
        &self,
    ) -> Result<i64, Traced<MicVolumeError>> {
        unreachable!(
            "`microphone_volume()` cannot be implemented on web platform",
        )
    }

    /// This method should be unreachable, because cannot be implemented on web
    /// platform.
    ///
    /// # Errors
    ///
    /// Never.
    ///
    /// # Panics
    ///
    /// Always.
    #[allow(clippy::unused_async)]
    pub async fn set_microphone_volume(
        &self,
        _: i64,
    ) -> Result<(), Traced<MicVolumeError>> {
        unreachable!(
            "`set_microphone_volume()` cannot be implemented on web platform",
        )
    }
}
