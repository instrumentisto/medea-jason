//! Acquiring and storing [`local::Track`]s.

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use derive_more::{Display, From};
use medea_client_api_proto::MediaSourceKind;
use tracerr::Traced;

use crate::{
    media::{
        track::MediaStreamTrackState, MediaKind, MediaStreamSettings,
        MultiSourceTracksConstraints,
    },
    platform,
    utils::Caused,
};

use super::track::local;

/// Errors returned from the [`MediaManagerHandle::enumerate_devices()`] method.
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = "platform::Error")]
pub enum EnumerateDevicesError {
    /// Occurs if the `enumerateDevices` request fails.
    #[display(fmt = "MediaDevices.enumerateDevices() failed: {}", _0)]
    Failed(platform::Error),

    /// [`MediaManagerHandle`]'s inner [`Weak`] pointer cannot be upgraded.
    #[display(fmt = "MediaManagerHandle is in detached state")]
    Detached,
}

/// Errors returned from the [`MediaManagerHandle::init_local_tracks()`] method.
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = "platform::Error")]
pub enum InitLocalTracksError {
    /// [`MediaManagerHandle`]'s inner [`Weak`] pointer could not be upgraded.
    #[display(fmt = "MediaManagerHandle is in detached state")]
    Detached,

    /// Occurs if the [getUserMedia][1] request fails.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    #[display(fmt = "Failed to get local tracks: {}", _0)]
    GetUserMediaFailed(#[cause] GetUserMediaError),

    /// Occurs if the [getDisplayMedia()][1] request fails.
    ///
    /// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    #[display(fmt = "Failed to get local tracks: {}", _0)]
    GetDisplayMediaFailed(#[cause] GetDisplayMediaError),
}

/// Error returned from the [`MediaManagerHandle::set_output_audio_id`] method.
///
/// Occurs if the provided audio output device ID is incorrect.
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "Invalid audio device ID provided")]
pub struct InvalidOutputAudioDeviceIdError;

/// Error returned from the [`MediaManagerHandle::set_microphone_volume`]
/// method.
///
/// Occurs if there is no possibility to set the
/// volume for the current `audio input device` or `system`, the `Audio
/// Device Module` or the `Microphone` is not initialized or there is no
/// connected `audio input devices` at all.
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "Can't set the microphone volume")]
pub struct SetMicrophoneVolumeError;

/// Error returned from the
/// [`MediaManagerHandle::microphone_volume_is_available`] method.
///
/// Occurs if it the `Audio Device Module` or the `Microphone` is not
/// initialized or there is no connected `audio input devices` at all.
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "Can't check if microphone volume setting is available")]
pub struct MicrophoneVolumeIsAvailableError;

/// Error returned from the [`MediaManagerHandle::microphone_volume`] method.
///
/// Occurs if it the `Audio Device Module` or the `Microphone` is not
/// initialized or there is no connected `audio input devices` at all.
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "Can't get the microphone volume level")]
pub struct MicrophoneVolumeError;

/// Error indicating about a [`MediaManagerHandle`] in detached state.
#[derive(Clone, Copy, Debug, Display)]
#[display(fmt = "MediaManagerHandle is in detached state")]
pub struct HandleDetachedError;

/// Error occurring when [`local::Track`] was [`ended`][1] right after
/// [getUserMedia()][2] or [getDisplayMedia()][3] request.
///
/// [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
/// [2]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
#[derive(Clone, Debug, Display)]
#[display(fmt = "{} track is ended", _0)]
struct LocalTrackIsEndedError(MediaKind);

/// Errors occurring when [getUserMedia()][1] request fails.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = "platform::Error")]
pub enum GetUserMediaError {
    /// [getUserMedia()][1] request failed.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    #[display(fmt = "MediaDevices.getUserMedia() failed: {}", _0)]
    PlatformRequestFailed(platform::GetUserMediaError),

    /// [`local::Track`] was [`ended`][1] right after [getUserMedia()][2] or
    /// [getDisplayMedia()][3] request.
    ///
    /// [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    /// [2]: https://tinyurl.com/rnxcavf
    /// [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    #[display(fmt = "New {} local track was ended", _0)]
    LocalTrackIsEnded(MediaKind),
}

impl From<LocalTrackIsEndedError> for GetUserMediaError {
    fn from(err: LocalTrackIsEndedError) -> Self {
        Self::LocalTrackIsEnded(err.0)
    }
}

/// Error occurring when [getDisplayMedia()][1] request fails.
///
/// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = "platform::Error")]
pub enum GetDisplayMediaError {
    /// [getDisplayMedia()][1] request failed.
    ///
    /// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    #[display(fmt = "MediaDevices.getDisplayMedia() failed: {}", _0)]
    PlatformRequestFailed(platform::Error),

    /// [`local::Track`] was [`ended`][1] right after [getUserMedia()][2] or
    /// [getDisplayMedia()][3] request.
    ///
    /// [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    /// [2]: https://tinyurl.com/rnxcavf
    /// [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    #[display(fmt = "New {} local track was ended", _0)]
    LocalTrackIsEnded(MediaKind),
}

impl From<LocalTrackIsEndedError> for GetDisplayMediaError {
    fn from(err: LocalTrackIsEndedError) -> Self {
        Self::LocalTrackIsEnded(err.0)
    }
}

/// [`MediaManager`] performs all media acquisition requests
/// ([getUserMedia()][1]/[getDisplayMedia()][2]) and stores all received tracks
/// for further reusage.
///
/// [`MediaManager`] stores weak references to
/// [`local::Track`]s, so if there are no strong references to some track,
/// then this track is stopped and deleted from [`MediaManager`].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
#[derive(Debug, Default)]
pub struct MediaManager(Rc<InnerMediaManager>);

/// Actual data of [`MediaManager`].
#[derive(Debug, Default)]
struct InnerMediaManager {
    /// Obtained tracks storage
    tracks: Rc<RefCell<HashMap<String, Weak<local::Track>>>>,

    /// Media devices platform controller.
    media_devices: platform::MediaDevices,
}

impl InnerMediaManager {
    /// Subscribes onto the `devicechange` event of this [`InnerMediaManager`].
    pub fn on_device_change(&self, cb: platform::Function<()>) {
        self.media_devices.on_device_change(Some(move || {
            cb.call0();
        }));
    }

    /// Returns a list of [`platform::MediaDeviceInfo`] objects.
    async fn enumerate_devices(
        &self,
    ) -> Result<Vec<platform::MediaDeviceInfo>, Traced<platform::Error>> {
        self.media_devices
            .enumerate_devices()
            .await
            .map_err(tracerr::wrap!())
    }

    /// Obtains [`local::Track`]s based on a provided
    /// [`MediaStreamSettings`]. This can be the tracks that were acquired
    /// earlier, or new tracks, acquired via [getUserMedia()][1] or/and
    /// [getDisplayMedia()][2] requests.
    ///
    /// # Errors
    ///
    /// With [`InitLocalTracksError::GetUserMediaFailed`] if [getUserMedia()][1]
    /// request failed.
    ///
    /// With [`InitLocalTracksError::GetDisplayMediaFailed`] if
    /// [getDisplayMedia()][2] request failed.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    async fn get_tracks(
        &self,
        mut caps: MediaStreamSettings,
    ) -> Result<Vec<(Rc<local::Track>, bool)>, Traced<InitLocalTracksError>>
    {
        let tracks_from_storage = self
            .get_from_storage(&mut caps)
            .into_iter()
            .map(|t| (t, false));
        match caps.into() {
            None => Ok(tracks_from_storage.collect()),
            Some(MultiSourceTracksConstraints::Display(caps)) => {
                Ok(tracks_from_storage
                    .chain(
                        self.get_display_media(caps)
                            .await
                            .map_err(tracerr::map_from_and_wrap!())?
                            .into_iter()
                            .map(|t| (t, true)),
                    )
                    .collect())
            }
            Some(MultiSourceTracksConstraints::Device(caps)) => {
                Ok(tracks_from_storage
                    .chain(
                        self.get_user_media(caps)
                            .await
                            .map_err(tracerr::map_from_and_wrap!())?
                            .into_iter()
                            .map(|t| (t, true)),
                    )
                    .collect())
            }
            Some(MultiSourceTracksConstraints::DeviceAndDisplay(
                device_caps,
                display_caps,
            )) => {
                let device_tracks = self
                    .get_user_media(device_caps)
                    .await
                    .map_err(tracerr::map_from_and_wrap!())?;
                let display_tracks = self
                    .get_display_media(display_caps)
                    .await
                    .map_err(tracerr::map_from_and_wrap!())?;
                Ok(tracks_from_storage
                    .chain(
                        device_tracks
                            .into_iter()
                            .chain(display_tracks.into_iter())
                            .map(|t| (t, true)),
                    )
                    .collect())
            }
        }
    }

    /// Tries to find [`local::Track`]s that satisfies [`MediaStreamSettings`],
    /// from tracks that were acquired earlier to avoid redundant
    /// [getUserMedia()][1]/[getDisplayMedia()][2] calls.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    fn get_from_storage(
        &self,
        caps: &mut MediaStreamSettings,
    ) -> Vec<Rc<local::Track>> {
        // cleanup weak links
        self.tracks
            .borrow_mut()
            .retain(|_, track| Weak::strong_count(track) > 0);

        let mut tracks = Vec::new();
        let storage: Vec<_> = self
            .tracks
            .borrow()
            .iter()
            .map(|(_, track)| Weak::upgrade(track).unwrap())
            .collect();

        if caps.is_audio_enabled() {
            let track = storage
                .iter()
                .find(|&track| caps.get_audio().satisfies(track.as_ref()))
                .cloned();

            if let Some(t) = track {
                caps.set_audio_publish(false);
                tracks.push(t);
            }
        }

        tracks.extend(
            storage
                .iter()
                .filter(|&track| {
                    caps.unconstrain_if_satisfies_video(track.as_ref())
                })
                .cloned(),
        );

        tracks
    }

    /// Obtains new [`local::Track`]s making [getUserMedia()][1] call, saves
    /// received tracks weak refs to storage, returns list of tracks strong
    /// refs.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    async fn get_user_media(
        &self,
        caps: platform::MediaStreamConstraints,
    ) -> Result<Vec<Rc<local::Track>>, Traced<GetUserMediaError>> {
        let tracks = self
            .media_devices
            .get_user_media(caps)
            .await
            .map_err(tracerr::map_from_and_wrap!())?;

        let tracks = self
            .parse_and_save_tracks(tracks, MediaSourceKind::Device)
            .map_err(tracerr::map_from_and_wrap!())?;

        Ok(tracks)
    }

    /// Obtains [`local::Track`]s making [getDisplayMedia()][1] call, saves
    /// received tracks weak refs to storage, returns list of tracks strong
    /// refs.
    ///
    /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    async fn get_display_media(
        &self,
        caps: platform::DisplayMediaStreamConstraints,
    ) -> Result<Vec<Rc<local::Track>>, Traced<GetDisplayMediaError>> {
        let tracks = self
            .media_devices
            .get_display_media(caps)
            .await
            .map_err(tracerr::map_from_and_wrap!())?;

        let track = self
            .parse_and_save_tracks(tracks, MediaSourceKind::Display)
            .map_err(tracerr::map_from_and_wrap!())?;

        Ok(track)
    }

    /// Retrieves tracks from provided [`platform::MediaStreamTrack`]s, saves
    /// tracks weak references in [`MediaManager`] tracks storage.
    ///
    /// # Errors
    ///
    /// With [`LocalTrackIsEndedError`] if at least one track from the provided
    /// [`platform::MediaStreamTrack`]s is in [`ended`][1] state.
    ///
    /// In case of error all tracks are ended and are not saved in
    /// [`MediaManager`]'s tracks storage.
    ///
    /// [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    fn parse_and_save_tracks(
        &self,
        tracks: Vec<platform::MediaStreamTrack>,
        kind: MediaSourceKind,
    ) -> Result<Vec<Rc<local::Track>>, Traced<LocalTrackIsEndedError>> {
        let mut storage = self.tracks.borrow_mut();

        // Tracks returned by getDisplayMedia()/getUserMedia() request should be
        // `live`. Otherwise, we should err without caching tracks in
        // `MediaManager`. Tracks will be stopped on `Drop`.
        for track in &tracks {
            if track.ready_state() != MediaStreamTrackState::Live {
                return Err(tracerr::new!(LocalTrackIsEndedError(
                    track.kind()
                )));
            }
        }

        let tracks = tracks
            .into_iter()
            .map(|tr| Rc::new(local::Track::new(tr, kind)))
            .inspect(|track| {
                drop(storage.insert(track.id(), Rc::downgrade(track)));
            })
            .collect();

        Ok(tracks)
    }

    /// Switches the current audio output device to the device with the provided
    /// `device_id`.
    ///
    /// # Errors
    ///
    /// With [`InvalidOutputAudioDeviceIdError`] if the provided `device_id` is
    /// not available.
    async fn set_output_audio_id(
        &self,
        device_id: String,
    ) -> Result<(), Traced<InvalidOutputAudioDeviceIdError>> {
        #[allow(clippy::map_err_ignore)]
        self.media_devices
            .set_output_audio_id(device_id)
            .await
            .map_err(|_| tracerr::new!(InvalidOutputAudioDeviceIdError))
    }

    /// Sets the microphone volume level in percents.
    ///
    /// # Errors
    ///
    /// With [`SetMicrophoneVolumeError`] if there is no possibility to set the
    /// volume for the current `audio input device` or `system`, the `Audio
    /// Device Module` or the `Microphone` is not initialized or there is no
    /// connected `audio input devices` at all.
    async fn set_microphone_volume(
        &self,
        level: i64,
    ) -> Result<(), Traced<SetMicrophoneVolumeError>> {
        #[allow(clippy::map_err_ignore)]
        self.media_devices
            .set_microphone_volume(level)
            .await
            .map_err(|_| tracerr::new!(SetMicrophoneVolumeError))
    }

    /// Indicates if it is possible to set the microphone volume.
    ///
    /// # Errors
    ///
    /// With [`MicrophoneVolumeIsAvailableError`] if it the `Audio Device
    /// Module` or the `Microphone` is not initialized or there is no connected
    /// `audio input devices` at all.
    async fn microphone_volume_is_available(
        &self,
    ) -> Result<bool, Traced<MicrophoneVolumeIsAvailableError>> {
        #[allow(clippy::map_err_ignore)]
        self.media_devices
            .microphone_volume_is_available()
            .await
            .map_err(|_| tracerr::new!(MicrophoneVolumeIsAvailableError))
    }

    /// Gets the current microphone volume level in percents.
    ///
    /// # Errors
    ///
    /// With [`MicrophoneVolumeError`] if it the `Audio Device
    /// Module` or the `Microphone` is not initialized or there is no connected
    /// `audio input devices` at all.
    async fn microphone_volume(
        &self,
    ) -> Result<i64, Traced<MicrophoneVolumeError>> {
        #[allow(clippy::map_err_ignore)]
        self.media_devices
            .microphone_volume()
            .await
            .map_err(|_| tracerr::new!(MicrophoneVolumeError))
    }
}

impl MediaManager {
    /// Obtains [`local::Track`]s based on a provided [`MediaStreamSettings`].
    /// This can be the tracks that were acquired earlier, or new tracks,
    /// acquired via [getUserMedia()][1] or/and [getDisplayMedia()][2] requests.
    ///
    /// # Errors
    ///
    /// With [`InitLocalTracksError::GetUserMediaFailed`] if [getUserMedia()][1]
    /// request failed.
    ///
    /// With [`InitLocalTracksError::GetDisplayMediaFailed`] if
    /// [getDisplayMedia()][2] request failed.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    pub async fn get_tracks<I: Into<MediaStreamSettings>>(
        &self,
        caps: I,
    ) -> Result<Vec<(Rc<local::Track>, bool)>, Traced<InitLocalTracksError>>
    {
        self.0
            .get_tracks(caps.into())
            .await
            .map_err(tracerr::wrap!())
    }

    /// Instantiates a new [`MediaManagerHandle`] for external usage.
    #[must_use]
    pub fn new_handle(&self) -> MediaManagerHandle {
        MediaManagerHandle(Rc::downgrade(&self.0))
    }
}

/// External handle to a [`MediaManager`].
///
/// [`MediaManager`] performs all media acquisition requests
/// ([getUserMedia()][1]/[getDisplayMedia()][2]) and stores all received tracks
/// for further reusage.
///
/// [`MediaManager`] stores weak references to [`local::Track`]s, so if there
/// are no strong references to some track, then this track is stopped and
/// deleted from [`MediaManager`].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
#[derive(Clone, Debug)]
pub struct MediaManagerHandle(Weak<InnerMediaManager>);

impl MediaManagerHandle {
    /// Returns a list of [`platform::MediaDeviceInfo`] objects representing
    /// available media input and devices, such as microphones, cameras, and so
    /// forth.
    ///
    /// # Errors
    ///
    /// See [`EnumerateDevicesError`] for details.
    pub async fn enumerate_devices(
        &self,
    ) -> Result<Vec<platform::MediaDeviceInfo>, Traced<EnumerateDevicesError>>
    {
        let this = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(EnumerateDevicesError::Detached))?;
        this.enumerate_devices()
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Returns [`local::LocalMediaTrack`]s objects, built from the provided
    /// [`MediaStreamSettings`].
    ///
    /// # Errors
    ///
    /// See [`InitLocalTracksError`] for details.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    pub async fn init_local_tracks(
        &self,
        caps: MediaStreamSettings,
    ) -> Result<Vec<local::LocalMediaTrack>, Traced<InitLocalTracksError>> {
        let this = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(InitLocalTracksError::Detached))?;
        this.get_tracks(caps)
            .await
            .map(|tracks| {
                tracks
                    .into_iter()
                    .map(|(t, _)| local::LocalMediaTrack::new(t))
                    .collect::<Vec<_>>()
            })
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Switches the current audio output device to the device with the provided
    /// `device_id`.
    ///
    /// # Errors
    ///
    /// With [`InvalidOutputAudioDeviceIdError`] if the provided `device_id` is
    /// not available.
    pub async fn set_output_audio_id(
        &self,
        device_id: String,
    ) -> Result<(), Traced<InvalidOutputAudioDeviceIdError>> {
        let this = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(InvalidOutputAudioDeviceIdError))?;
        this.set_output_audio_id(device_id)
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Sets the microphone volume level in percents.
    ///
    /// # Errors
    ///
    /// With [`SetMicrophoneVolumeError`] if there is no possibility to set the
    /// volume for the current `audio input device` or `system`, the `Audio
    /// Device Module` or the `Microphone` is not initialized or there is no
    /// connected `audio input devices` at all.
    pub async fn set_microphone_volume(
        &self,
        level: i64,
    ) -> Result<(), Traced<SetMicrophoneVolumeError>> {
        let this = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(SetMicrophoneVolumeError))?;
        this.set_microphone_volume(level)
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Indicates if it is possible to set the microphone volume.
    ///
    /// # Errors
    ///
    /// With [`MicrophoneVolumeIsAvailableError`] if it the `Audio Device
    /// Module` or the `Microphone` is not initialized or there is no connected
    /// `audio input devices` at all.
    pub async fn microphone_volume_is_available(
        &self,
    ) -> Result<bool, Traced<MicrophoneVolumeIsAvailableError>> {
        let this = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(MicrophoneVolumeIsAvailableError))?;
        this.microphone_volume_is_available()
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Gets the current microphone volume level in percents.
    ///
    /// # Errors
    ///
    /// With [`MicrophoneVolumeIsAvailableError`] if it the `Audio Device
    /// Module` or the `Microphone` is not initialized or there is no connected
    /// `audio input devices` at all.
    pub async fn microphone_volume(
        &self,
    ) -> Result<i64, Traced<MicrophoneVolumeError>> {
        let this = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(MicrophoneVolumeError))?;
        this.microphone_volume()
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Subscribes onto the `devicechange` event of this [`MediaManagerHandle`].
    ///
    /// # Errors
    ///
    /// If the underlying [`MediaManagerHandle`] is dropped.
    pub fn on_device_change(
        &self,
        cb: platform::Function<()>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        let this = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(HandleDetachedError))?;
        this.on_device_change(cb);
        Ok(())
    }
}
