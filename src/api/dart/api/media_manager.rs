//! External handle to a [`MediaManager`].

use flutter_rust_bridge::{DartOpaque, frb};
use futures::TryFutureExt as _;
use send_wrapper::SendWrapper;
use tracerr::Traced;

#[cfg(doc)]
use crate::media::MediaManager;
use crate::{
    api::{
        Error as DartError,
        api::{
            ApiMediaDeviceDetails, ApiMediaDisplayDetails,
            ApiMediaStreamSettings, LocalMediaTrack,
        },
    },
    media::{self as core},
    platform::{self, utils::dart_future::IntoDartFuture as _},
};

/// External handle to a [`MediaManager`].
///
/// [`MediaManager`] performs all media acquisition requests
/// ([getUserMedia()][1]/[getDisplayMedia()][2]) and stores all received tracks
/// for further re-usage.
///
/// [`MediaManager`] stores weak references to [`local::Track`]s, so if there
/// are no strong references to some track, then this track is stopped and
/// deleted from the [`MediaManager`].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
#[derive(Debug)]
#[frb(opaque)]
pub struct MediaManagerHandle(
    pub(crate) SendWrapper<core::MediaManagerHandleImpl>,
);

impl From<core::MediaManagerHandleImpl> for MediaManagerHandle {
    fn from(value: core::MediaManagerHandleImpl) -> Self {
        Self(SendWrapper::new(value))
    }
}

impl MediaManagerHandle {
    /// Returns [`LocalMediaTrack`]s objects, built from the provided
    /// [`ApiMediaStreamSettings`].
    #[frb(sync)]
    #[must_use]
    pub fn init_local_tracks(
        &self,
        caps: ApiMediaStreamSettings,
    ) -> DartOpaque {
        let manager = self.0.clone();

        async move { manager.init_local_tracks(caps.into()).await }
            .map_ok(|tracks| {
                tracks
                    .into_iter()
                    .map(LocalMediaTrack::from)
                    .collect::<Vec<_>>()
            })
            .into_dart_future()
            .into_dart_opaque()
    }

    /// Returns a list of [`ApiMediaDeviceDetails`] objects representing
    /// available media input and devices, such as microphones, cameras, and
    /// so forth.
    #[frb(sync)]
    #[must_use]
    pub fn enumerate_devices(&self) -> DartOpaque {
        let manager = self.0.clone();

        let result = async move {
            Ok::<Vec<_>, Traced<core::EnumerateDevicesError>>(
                manager
                    .enumerate_devices()
                    .await?
                    .into_iter()
                    .map(|v| ApiMediaDeviceDetails {
                        kind: v.kind(),
                        device_id: v.device_id(),
                        label: v.label(),
                        group_id: v.group_id(),
                        is_failed: v.is_failed(),
                    })
                    .collect(),
            )
        }
        .into_dart_future()
        .into_dart_opaque();

        result
    }

    /// Returns a list of [`ApiMediaDisplayDetails`] objects representing
    /// available sources that can be used for screen capturing.
    #[frb(sync)]
    #[must_use]
    pub fn enumerate_displays(&self) -> DartOpaque {
        let manager = self.0.clone();

        async move {
            Ok::<Vec<_>, Traced<core::EnumerateDisplaysError>>(
                manager
                    .enumerate_displays()
                    .await?
                    .into_iter()
                    .map(|v| ApiMediaDisplayDetails {
                        device_id: v.device_id(),
                        title: v.title(),
                    })
                    .collect(),
            )
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Switches the current output audio device to the device with the provided
    /// `device_id`.
    #[frb(sync)]
    #[must_use]
    pub fn set_output_audio_id(&self, device_id: String) -> DartOpaque {
        let manager = self.0.clone();

        async move {
            manager
                .set_output_audio_id(device_id)
                .await
                .map_err(tracerr::map_from_and_wrap!())?;
            Ok::<_, Traced<core::InvalidOutputAudioDeviceIdError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Sets the microphone volume level in percents.
    #[frb(sync)]
    #[must_use]
    pub fn set_microphone_volume(&self, level: i64) -> DartOpaque {
        let manager = self.0.clone();

        async move {
            manager
                .set_microphone_volume(level)
                .await
                .map_err(tracerr::map_from_and_wrap!())?;
            Ok::<_, Traced<core::MicVolumeError>>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Indicates whether it's possible to access microphone volume settings.
    #[frb(sync)]
    #[must_use]
    pub fn microphone_volume_is_available(&self) -> DartOpaque {
        let manager = self.0.clone();

        async move { manager.microphone_volume_is_available().await }
            .into_dart_future()
            .into_dart_opaque()
    }

    /// Returns the current microphone volume level in percents.
    #[frb(sync)]
    #[must_use]
    pub fn microphone_volume(&self) -> DartOpaque {
        let manager = self.0.clone();

        async move { manager.microphone_volume().await }
            .into_dart_future()
            .into_dart_opaque()
    }

    /// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
    ///
    /// Sets an ideal [frameRate][1] constraint.
    ///
    /// # Errors
    ///
    /// If [`MediaManagerHandle::on_device_change()`] errors.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    #[frb(sync)]
    pub fn on_device_change(&self, cb: DartOpaque) -> Result<(), DartOpaque> {
        let manager = self.0.clone();
        manager
            .on_device_change(platform::Function::new(cb))
            .map_err(DartError::from)
            .map_err(Into::into)
    }
}
