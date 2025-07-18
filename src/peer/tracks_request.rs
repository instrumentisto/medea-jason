//! [MediaStreamConstraints][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints

use std::{collections::HashMap, rc::Rc};

use derive_more::with_trait::Display;
use medea_client_api_proto::{MediaSourceKind, TrackId};
use tracerr::Traced;

use crate::{
    media::{
        AudioSource, AudioTrackConstraints, DeviceVideoTrackConstraints,
        DisplayVideoTrackConstraints, MediaKind, MediaStreamSettings,
        TrackConstraints, VideoSource, track::local,
    },
    platform,
    utils::Caused,
};

/// Errors that may occur when validating [`TracksRequest`] or
/// parsing [`local::Track`]s.
#[derive(Caused, Clone, Copy, Debug, Display, Eq, PartialEq)]
#[cause(error = platform::Error)]
pub enum TracksRequestError {
    /// [`TracksRequest`] contains multiple Device [`AudioTrackConstraints`].
    #[display("only one device audio track is allowed in SimpleTracksRequest")]
    TooManyDeviceAudioTracks,

    /// [`TracksRequest`] contains multiple Display [`AudioTrackConstraints`].
    #[display("only one display audio track is allowed in SimpleTracksRequest")]
    TooManyDisplayAudioTracks,

    /// [`TracksRequest`] contains multiple [`DeviceVideoTrackConstraints`].
    #[display("only one device video track is allowed in SimpleTracksRequest")]
    TooManyDeviceVideoTracks,

    /// [`TracksRequest`] contains multiple [`DisplayVideoTrackConstraints`].
    #[display("only one display video track is allowed in SimpleTracksRequest")]
    TooManyDisplayVideoTracks,

    /// [`TracksRequest`] contains no track constraints at all.
    #[display("SimpleTracksRequest should have at least one track")]
    NoTracks,

    /// Provided multiple device audio [`local::Track`]s.
    #[display("provided multiple device audio MediaStreamTracks")]
    ExpectedDeviceAudioTracks,

    /// Provided multiple display audio [`local::Track`]s.
    #[display("provided multiple display audio MediaStreamTracks")]
    ExpectedDisplayAudioTracks,

    /// Provided multiple device video [`local::Track`]s.
    #[display("provided multiple device video MediaStreamTracks")]
    ExpectedDeviceVideoTracks,

    /// Provided multiple display video [`local::Track`]s.
    #[display("provided multiple display video MediaStreamTracks")]
    ExpectedDisplayVideoTracks,

    /// Audio track fails to satisfy specified constraints.
    #[display("provided audio track does not satisfy specified constraints")]
    InvalidAudioTrack,

    /// Video track fails to satisfy specified constraints.
    #[display("provided video track does not satisfy specified constraints")]
    InvalidVideoTrack,
}

/// Representation of [MediaStreamConstraints][1] object.
///
/// It's used for invoking [getUserMedia()][2] to specify what kinds of tracks
/// should be included into returned `MediaStream`, and, optionally,
/// to establish constraints for those track's settings.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
/// [2]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [3]: https://w3.org/TR/mediacapture-streams#mediastream
#[derive(Debug, Default)]
pub struct TracksRequest {
    /// Device [`AudioTrackConstraints`] of [`local::Track`]s to be applied.
    device_audio: HashMap<TrackId, AudioTrackConstraints>,

    /// Display [`AudioTrackConstraints`] of [`local::Track`]s to be applied.
    display_audio: HashMap<TrackId, AudioTrackConstraints>,

    /// [`DeviceVideoTrackConstraints`] of [`local::Track`]s to be applied.
    device_video: HashMap<TrackId, DeviceVideoTrackConstraints>,

    /// [`DisplayVideoTrackConstraints`] of [`local::Track`]s to be applied.
    display_video: HashMap<TrackId, DisplayVideoTrackConstraints>,
}

impl TracksRequest {
    /// Adds track request to this [`TracksRequest`].
    pub fn add_track_request<T: Into<TrackConstraints>>(
        &mut self,
        track_id: TrackId,
        caps: T,
    ) {
        match caps.into() {
            TrackConstraints::Audio(audio) => match audio {
                AudioSource::Device(device) => {
                    drop(self.device_audio.insert(track_id, device));
                }
                AudioSource::Display(display) => {
                    drop(self.display_audio.insert(track_id, display));
                }
            },
            TrackConstraints::Video(video) => match video {
                VideoSource::Device(device) => {
                    drop(self.device_video.insert(track_id, device));
                }
                VideoSource::Display(display) => {
                    drop(self.display_video.insert(track_id, display));
                }
            },
        }
    }
}

/// Subtype of [`TracksRequest`], which can have maximum one track of each kind
/// and must have at least one track of any kind.
#[derive(Debug)]
pub struct SimpleTracksRequest {
    /// Display [`AudioTrackConstraints`] of a [`local::Track`] to be applied.
    display_audio: Option<(TrackId, AudioTrackConstraints)>,

    /// Device [`AudioTrackConstraints`] of a [`local::Track`] to be applied.
    device_audio: Option<(TrackId, AudioTrackConstraints)>,

    /// [`DisplayVideoTrackConstraints`] of a [`local::Track`] to be applied.
    display_video: Option<(TrackId, DisplayVideoTrackConstraints)>,

    /// [`DisplayVideoTrackConstraints`] of a [`local::Track`] to be applied.
    device_video: Option<(TrackId, DeviceVideoTrackConstraints)>,
}

impl SimpleTracksRequest {
    /// Parses [`local::Track`]s and returns [`HashMap`] with [`TrackId`]s
    /// and [`local::Track`]s.
    ///
    /// # Errors
    ///
    /// - [`TracksRequestError::InvalidAudioTrack`] when some audio track from
    ///   the provided [`local::Track`]s not satisfies contained constrains.
    /// - [`TracksRequestError::ExpectedDeviceAudioTracks`] when the provided
    ///   [`HashMap`] doesn't have the expected device audio track.
    /// - [`TracksRequestError::ExpectedDisplayAudioTracks`] when the provided
    ///   [`HashMap`] doesn't have the expected display audio track.
    /// - [`TracksRequestError::InvalidVideoTrack`] when some device video track
    ///   from the provided [`HashMap`] doesn't satisfy contained constrains.
    /// - [`TracksRequestError::ExpectedDeviceVideoTracks`] when the provided
    ///   [`HashMap`] doesn't have the expected device video track.
    /// - [`TracksRequestError::InvalidVideoTrack`] when some display video
    ///   track from the provided [`HashMap`] doesn't satisfy contained
    ///   constrains.
    /// - [`TracksRequestError::ExpectedDisplayVideoTracks`] when the provided
    ///   [`HashMap`] doesn't have the expected display video track.
    pub async fn parse_tracks(
        &self,
        tracks: Vec<Rc<local::Track>>,
    ) -> Result<HashMap<TrackId, Rc<local::Track>>, Traced<TracksRequestError>>
    {
        use TracksRequestError::{InvalidAudioTrack, InvalidVideoTrack};

        let mut parsed_tracks = HashMap::new();

        let mut display_video_tracks = Vec::new();
        let mut device_video_tracks = Vec::new();
        let mut device_audio_tracks = Vec::new();
        let mut display_audio_tracks = Vec::new();
        for track in tracks {
            match track.kind() {
                MediaKind::Audio => match track.media_source_kind() {
                    MediaSourceKind::Device => {
                        device_audio_tracks.push(track);
                    }
                    MediaSourceKind::Display => {
                        display_audio_tracks.push(track);
                    }
                },
                MediaKind::Video => match track.media_source_kind() {
                    MediaSourceKind::Device => {
                        device_video_tracks.push(track);
                    }
                    MediaSourceKind::Display => {
                        display_video_tracks.push(track);
                    }
                },
            }
        }

        if let Some((id, device_audio)) = &self.device_audio {
            if let Some(track) = device_audio_tracks.into_iter().next() {
                if device_audio.satisfies(track.as_ref()).await {
                    drop(parsed_tracks.insert(*id, track));
                } else {
                    return Err(tracerr::new!(InvalidAudioTrack));
                }
            }
        }

        if let Some((id, display_audio)) = &self.display_audio {
            if let Some(track) = display_audio_tracks.into_iter().next() {
                if display_audio.satisfies(track.as_ref()).await {
                    drop(parsed_tracks.insert(*id, track));
                } else {
                    return Err(tracerr::new!(InvalidAudioTrack));
                }
            }
        }

        if let Some((id, device_video)) = &self.device_video {
            if let Some(track) = device_video_tracks.into_iter().next() {
                if device_video.satisfies(track.as_ref()).await {
                    drop(parsed_tracks.insert(*id, track));
                } else {
                    return Err(tracerr::new!(InvalidVideoTrack));
                }
            }
        }

        if let Some((id, display_video)) = &self.display_video {
            if let Some(track) = display_video_tracks.into_iter().next() {
                if display_video.satisfies(track.as_ref()).await {
                    drop(parsed_tracks.insert(*id, track));
                } else {
                    return Err(tracerr::new!(InvalidVideoTrack));
                }
            }
        }

        Ok(parsed_tracks)
    }

    /// Merges [`SimpleTracksRequest`] with provided [`MediaStreamSettings`].
    ///
    /// Applies new settings if possible, meaning that if this
    /// [`SimpleTracksRequest`] does not have some constraint, then it will be
    /// applied from [`MediaStreamSettings`].
    ///
    /// # Errors
    ///
    /// - [`TracksRequestError::ExpectedDeviceAudioTracks`] when
    ///   [`SimpleTracksRequest`] contains Device [`AudioTrackConstraints`],
    ///   but the provided [`MediaStreamSettings`] doesn't and these Device
    ///   [`AudioTrackConstraints`] are important.
    /// - [`TracksRequestError::ExpectedDisplayAudioTracks`] when
    ///   [`SimpleTracksRequest`] contains Display [`AudioTrackConstraints`],
    ///   but the provided [`MediaStreamSettings`] doesn't and these Display
    ///   [`AudioTrackConstraints`] are important.
    /// - [`TracksRequestError::ExpectedDeviceVideoTracks`] when
    ///   [`SimpleTracksRequest`] contains [`DeviceVideoTrackConstraints`], but
    ///   the provided [`MediaStreamSettings`] doesn't and these
    ///   [`DeviceVideoTrackConstraints`] are important.
    /// - [`TracksRequestError::ExpectedDisplayVideoTracks`] when
    ///   [`SimpleTracksRequest`] contains [`DisplayVideoTrackConstraints`], but
    ///   the provided [`MediaStreamSettings`] doesn't and these
    ///   [`DisplayVideoTrackConstraints`] are important.
    pub fn merge<T: Into<MediaStreamSettings>>(
        &mut self,
        other: T,
    ) -> Result<(), Traced<TracksRequestError>> {
        let other = other.into();

        if let Some((_, device_audio_caps)) = &self.device_audio {
            if !other.is_device_audio_enabled() {
                if device_audio_caps.required() {
                    return Err(tracerr::new!(
                        TracksRequestError::ExpectedDeviceAudioTracks
                    ));
                }
                drop(self.device_audio.take());
            }
        }
        if let Some((_, display_audio_caps)) = &self.display_audio {
            if !other.is_display_audio_enabled() {
                if display_audio_caps.required() {
                    return Err(tracerr::new!(
                        TracksRequestError::ExpectedDisplayAudioTracks
                    ));
                }
                drop(self.display_audio.take());
            }
        }
        if let Some((_, device_video_caps)) = &self.device_video {
            if !other.is_device_video_enabled() {
                if device_video_caps.required() {
                    return Err(tracerr::new!(
                        TracksRequestError::ExpectedDeviceVideoTracks
                    ));
                }
                drop(self.device_video.take());
            }
        }
        if let Some((_, display_video_caps)) = &self.display_video {
            if !other.is_display_video_enabled() {
                if display_video_caps.required() {
                    return Err(tracerr::new!(
                        TracksRequestError::ExpectedDisplayVideoTracks
                    ));
                }
                drop(self.display_video.take());
            }
        }

        if other.is_device_audio_enabled() {
            if let Some((_, device_audio)) = self.device_audio.as_mut() {
                device_audio.merge(other.get_device_audio().clone());
            }
        }
        if other.is_display_audio_enabled() {
            if let Some((_, display_audio)) = self.display_audio.as_mut() {
                display_audio.merge(other.get_display_audio().clone());
            }
        }
        if other.is_display_video_enabled() {
            if let Some((_, display_video)) = self.display_video.as_mut() {
                if let Some(other_display_video) = other.get_display_video() {
                    display_video.merge(other_display_video.clone());
                }
            }
        }
        if other.is_device_video_enabled() {
            if let Some((_, device_video)) = self.device_video.as_mut() {
                if let Some(other_device_video) = other.get_device_video() {
                    device_video.merge(other_device_video.clone());
                }
            }
        }

        Ok(())
    }
}

impl TryFrom<TracksRequest> for SimpleTracksRequest {
    type Error = TracksRequestError;

    fn try_from(value: TracksRequest) -> Result<Self, Self::Error> {
        use TracksRequestError::{
            NoTracks, TooManyDeviceAudioTracks, TooManyDeviceVideoTracks,
            TooManyDisplayAudioTracks, TooManyDisplayVideoTracks,
        };

        #[expect(clippy::else_if_without_else, reason = "more readable")]
        if value.device_video.len() > 1 {
            return Err(TooManyDeviceVideoTracks);
        } else if value.display_video.len() > 1 {
            return Err(TooManyDisplayVideoTracks);
        } else if value.device_audio.len() > 1 {
            return Err(TooManyDeviceAudioTracks);
        } else if value.display_audio.len() > 1 {
            return Err(TooManyDisplayAudioTracks);
        } else if value.device_video.is_empty()
            && value.display_video.is_empty()
            && value.device_audio.is_empty()
            && value.display_video.is_empty()
        {
            return Err(NoTracks);
        }

        let mut req = Self {
            device_audio: None,
            display_audio: None,
            device_video: None,
            display_video: None,
        };
        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for (id, device) in value.device_audio {
            drop(req.device_audio.replace((id, device)));
        }
        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for (id, display) in value.display_audio {
            drop(req.display_audio.replace((id, display)));
        }
        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for (id, device) in value.device_video {
            drop(req.device_video.replace((id, device)));
        }
        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for (id, display) in value.display_video {
            drop(req.display_video.replace((id, display)));
        }

        Ok(req)
    }
}

impl From<&SimpleTracksRequest> for MediaStreamSettings {
    fn from(request: &SimpleTracksRequest) -> Self {
        let mut constraints = Self::new();

        if let Some((_, device_audio)) = &request.device_audio {
            constraints.device_audio(device_audio.clone());
        }

        if let Some((_, display_audio)) = &request.display_audio {
            constraints.display_audio(display_audio.clone());
        }

        if let Some((_, device_video)) = &request.device_video {
            constraints.device_video(device_video.clone());
        }

        if let Some((_, display_video)) = &request.display_video {
            constraints.display_video(display_video.clone());
        }

        constraints
    }
}
